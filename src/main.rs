use base64::{engine::general_purpose::STANDARD, Engine};
use clap::{Parser, Subcommand};
use core_graphics::display::CGDisplay;
use core_graphics::event::{
    CGEvent, CGEventFlags, CGEventTapLocation, CGEventType, CGKeyCode, CGMouseButton,
    KeyCode, ScrollEventUnit,
};
use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};
use core_graphics::geometry::{CGPoint, CGRect};
use screencapturekit::prelude::*;
use screencapturekit::screenshot_manager::{CGImage, SCScreenshotManager};
use std::io::{self, Write};
use std::process::exit;
use std::thread;
use std::time::Duration;

#[derive(Parser)]
#[command(name = "nano-use", about = "Minimal macOS computer-use toolkit")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
  Screenshot,
  Click { x: f64, y: f64 },
  #[command(name = "right_click")]
  RightClick { x: f64, y: f64 },
  #[command(name = "double_click")]
  DoubleClick { x: f64, y: f64 },
  Drag { x1: f64, y1: f64, x2: f64, y2: f64 },
  Type { text: String },
  Keypress { keys: String },
  Scroll { x: f64, y: f64, dy: i32 },
}

fn main() {
  if let Err(err) = run() {
    eprintln!("{err}");
    exit(1);
  }
}

fn run() -> Result<(), String> {
  let cli = Cli::parse();
  match cli.command {
    Command::Screenshot => screenshot(),
    Command::Click { x, y } => click(x, y, CGMouseButton::Left),
    Command::RightClick { x, y } => click(x, y, CGMouseButton::Right),
    Command::DoubleClick { x, y } => double_click(x, y),
    Command::Drag { x1, y1, x2, y2 } => drag(x1, y1, x2, y2),
    Command::Type { text } => type_text(&text),
    Command::Keypress { keys } => keypress(&keys),
    Command::Scroll { x, y, dy } => scroll(x, y, dy),
  }
}

fn fail<T>(msg: impl Into<String>) -> Result<T, String> {
  Err(msg.into())
}

fn desktop_bounds() -> Result<CGRect, String> {
  let ids = CGDisplay::active_displays().map_err(|_| "无法获取显示器列表".to_string())?;
  if ids.is_empty() {
    return fail("未检测到活动显示器");
  }

  let mut bounds = CGDisplay::new(ids[0]).bounds();
  for id in ids.iter().skip(1) {
    bounds = rect_union(bounds, CGDisplay::new(*id).bounds());
  }
  Ok(bounds)
}

fn rect_union(a: CGRect, b: CGRect) -> CGRect {
  let ax2 = a.origin.x + a.size.width;
  let ay2 = a.origin.y + a.size.height;
  let bx2 = b.origin.x + b.size.width;
  let by2 = b.origin.y + b.size.height;
  let x = a.origin.x.min(b.origin.x);
  let y = a.origin.y.min(b.origin.y);
  CGRect::new(
    &CGPoint::new(x, y),
    &core_graphics::geometry::CGSize::new(ax2.max(bx2) - x, ay2.max(by2) - y),
  )
}

fn bounds_label(bounds: &CGRect) -> String {
  format!(
    "{}x{} (origin: {}, {})",
    bounds.size.width, bounds.size.height, bounds.origin.x, bounds.origin.y
  )
}

fn validate_point(x: f64, y: f64) -> Result<(), String> {
  let bounds = desktop_bounds()?;
  let within_x = x >= bounds.origin.x && x < bounds.origin.x + bounds.size.width;
  let within_y = y >= bounds.origin.y && y < bounds.origin.y + bounds.size.height;
  if within_x && within_y {
    Ok(())
  } else {
    fail(format!(
      "坐标 ({x}, {y}) 越界，屏幕尺寸 {}",
      bounds_label(&bounds)
    ))
  }
}

fn validate_points(points: &[(f64, f64)]) -> Result<(), String> {
  for &(x, y) in points {
    validate_point(x, y)?;
  }
  Ok(())
}

fn event_source() -> Result<CGEventSource, String> {
  CGEventSource::new(CGEventSourceStateID::HIDSystemState)
    .map_err(|_| "无法创建 CGEventSource".to_string())
}

fn post_mouse(
  event_type: CGEventType,
  point: CGPoint,
  button: CGMouseButton,
) -> Result<(), String> {
  let source = event_source()?;
  let event = CGEvent::new_mouse_event(source, event_type, point, button)
    .map_err(|_| "无法创建鼠标事件".to_string())?;
  event.post(CGEventTapLocation::HID);
  Ok(())
}

fn screenshot() -> Result<(), String> {
  let bounds = desktop_bounds()?;
  let png_bytes = capture_desktop_png(&bounds)?;
  let encoded = STANDARD.encode(png_bytes);
  io::stdout()
    .write_all(encoded.as_bytes())
    .map_err(|e| format!("写入 stdout 失败: {e}"))?;
  io::stdout()
    .write_all(b"\n")
    .map_err(|e| format!("写入 stdout 失败: {e}"))?;
  Ok(())
}

fn capture_desktop_png(bounds: &CGRect) -> Result<Vec<u8>, String> {
  use screencapturekit::cg::CGRect as ScRect;
  let rect = ScRect::new(
    bounds.origin.x,
    bounds.origin.y,
    bounds.size.width,
    bounds.size.height,
  );
  if let Ok(image) = SCScreenshotManager::capture_image_in_rect(rect) {
    return image_to_png_bytes(&image);
  }

  capture_desktop_stitched(bounds)
}

fn capture_desktop_stitched(bounds: &CGRect) -> Result<Vec<u8>, String> {
  let content = SCShareableContent::get().map_err(|e| format!("获取屏幕内容失败: {e}"))?;
  let displays = content.displays();
  if displays.is_empty() {
    return fail("未检测到可捕获的显示器");
  }

  if displays.len() == 1 {
    let image = capture_display(&displays[0], bounds)?;
    return image_to_png_bytes(&image);
  }

  let scale = display_scale(&displays[0]);
  let width = (bounds.size.width * scale).round() as u32;
  let height = (bounds.size.height * scale).round() as u32;
  let mut canvas = vec![0u8; (width * height * 4) as usize];

  for display in &displays {
    let image = capture_display(display, bounds)?;
    let frame = display.frame();
    let rgba = image
      .rgba_data()
      .map_err(|e| format!("读取截图像素失败: {e}"))?;
    let img_w = image.width() as u32;
    let img_h = image.height() as u32;
    let offset_x = ((frame.x - bounds.origin.x) * scale).round() as i64;
    let offset_y = ((frame.y - bounds.origin.y) * scale).round() as i64;

    blit_rgba(
      &mut canvas,
      width,
      height,
      &rgba,
      img_w,
      img_h,
      offset_x,
      offset_y,
    );
  }

  encode_rgba_png(width, height, &canvas)
}

fn display_scale(display: &screencapturekit::shareable_content::SCDisplay) -> f64 {
  let pixel_w = display.width().max(1) as f64;
  let frame = display.frame();
  pixel_w / frame.width.max(1.0)
}

fn capture_display(
  display: &screencapturekit::shareable_content::SCDisplay,
  _bounds: &CGRect,
) -> Result<CGImage, String> {
  let filter = SCContentFilter::create()
    .with_display(display)
    .with_excluding_windows(&[])
    .build();

  let frame = display.frame();
  let scale = display_scale(display);
  let width = (frame.width * scale).round() as u32;
  let height = (frame.height * scale).round() as u32;
  let config = SCStreamConfiguration::new()
    .with_width(width.max(1))
    .with_height(height.max(1));

  SCScreenshotManager::capture_image(&filter, &config)
    .map_err(|e| format!("截图失败: {e}"))
}

fn blit_rgba(
  canvas: &mut [u8],
  canvas_w: u32,
  canvas_h: u32,
  src: &[u8],
  src_w: u32,
  src_h: u32,
  offset_x: i64,
  offset_y: i64,
) {
  for y in 0..src_h {
    let dst_y = offset_y + y as i64;
    if dst_y < 0 || dst_y >= canvas_h as i64 {
      continue;
    }
    for x in 0..src_w {
      let dst_x = offset_x + x as i64;
      if dst_x < 0 || dst_x >= canvas_w as i64 {
        continue;
      }
      let src_idx = ((y * src_w + x) * 4) as usize;
      let dst_idx = ((dst_y as u32 * canvas_w + dst_x as u32) * 4) as usize;
      if src_idx + 3 < src.len() && dst_idx + 3 < canvas.len() {
        canvas[dst_idx..dst_idx + 4].copy_from_slice(&src[src_idx..src_idx + 4]);
      }
    }
  }
}

fn image_to_png_bytes(image: &CGImage) -> Result<Vec<u8>, String> {
  let rgba = image
    .rgba_data()
    .map_err(|e| format!("读取截图像素失败: {e}"))?;
  encode_rgba_png(image.width() as u32, image.height() as u32, &rgba)
}

fn encode_rgba_png(width: u32, height: u32, rgba: &[u8]) -> Result<Vec<u8>, String> {
  let mut buf = Vec::new();
  {
    let mut encoder = png::Encoder::new(&mut buf, width, height);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder
      .write_header()
      .map_err(|e| format!("PNG 编码失败: {e}"))?;
    writer
      .write_image_data(rgba)
      .map_err(|e| format!("PNG 编码失败: {e}"))?;
  }
  Ok(buf)
}

fn click(x: f64, y: f64, button: CGMouseButton) -> Result<(), String> {
  validate_point(x, y)?;
  let point = CGPoint::new(x, y);
  let (down, up) = match button {
    CGMouseButton::Right => (CGEventType::RightMouseDown, CGEventType::RightMouseUp),
    _ => (CGEventType::LeftMouseDown, CGEventType::LeftMouseUp),
  };
  post_mouse(down, point, button)?;
  post_mouse(up, point, button)?;
  Ok(())
}

fn double_click(x: f64, y: f64) -> Result<(), String> {
  validate_point(x, y)?;
  let point = CGPoint::new(x, y);
  click(x, y, CGMouseButton::Left)?;
  thread::sleep(Duration::from_millis(50));
  post_mouse(CGEventType::LeftMouseDown, point, CGMouseButton::Left)?;
  post_mouse(CGEventType::LeftMouseUp, point, CGMouseButton::Left)?;
  Ok(())
}

fn drag(x1: f64, y1: f64, x2: f64, y2: f64) -> Result<(), String> {
  validate_points(&[(x1, y1), (x2, y2)])?;
  let start = CGPoint::new(x1, y1);
  let end = CGPoint::new(x2, y2);
  post_mouse(CGEventType::LeftMouseDown, start, CGMouseButton::Left)?;
  thread::sleep(Duration::from_millis(10));
  post_mouse(CGEventType::LeftMouseDragged, end, CGMouseButton::Left)?;
  thread::sleep(Duration::from_millis(10));
  post_mouse(CGEventType::LeftMouseUp, end, CGMouseButton::Left)?;
  Ok(())
}

fn type_text(text: &str) -> Result<(), String> {
  if text.is_empty() {
    return fail("type 命令不接受空字符串");
  }
  let source = event_source()?;
  let event = CGEvent::new_keyboard_event(source, 0, true)
    .map_err(|_| "无法创建键盘事件".to_string())?;
  event.set_string(text);
  event.post(CGEventTapLocation::HID);
  Ok(())
}

fn keypress(keys: &str) -> Result<(), String> {
  let normalized = keys.trim().to_ascii_lowercase();
  if normalized.is_empty() {
    return fail("keypress 命令不接受空字符串");
  }

  let parts: Vec<&str> = normalized.split('+').map(str::trim).collect();
  let mut flags = CGEventFlags::empty();
  let mut main_key = None;

  for part in parts {
    match part {
      "cmd" | "command" => flags |= CGEventFlags::CGEventFlagCommand,
      "ctrl" | "control" => flags |= CGEventFlags::CGEventFlagControl,
      "alt" | "option" => flags |= CGEventFlags::CGEventFlagAlternate,
      "shift" => flags |= CGEventFlags::CGEventFlagShift,
      "fn" | "function" => flags |= CGEventFlags::CGEventFlagSecondaryFn,
      key => {
        if main_key.is_some() {
          return fail(format!("无法解析按键组合: {keys}"));
        }
        main_key = Some(key);
      }
    }
  }

  let main_key = main_key.ok_or_else(|| format!("无法解析按键组合: {keys}"))?;
  let keycode = resolve_keycode(main_key)?;

  let source = event_source()?;
  let down = CGEvent::new_keyboard_event(source.clone(), keycode, true)
    .map_err(|_| "无法创建键盘事件".to_string())?;
  down.set_flags(flags);
  down.post(CGEventTapLocation::HID);

  let up = CGEvent::new_keyboard_event(source, keycode, false)
    .map_err(|_| "无法创建键盘事件".to_string())?;
  up.set_flags(flags);
  up.post(CGEventTapLocation::HID);
  Ok(())
}

fn resolve_keycode(name: &str) -> Result<CGKeyCode, String> {
  let code = match name {
    "return" | "enter" => KeyCode::RETURN,
    "tab" => KeyCode::TAB,
    "space" => KeyCode::SPACE,
    "delete" | "backspace" => KeyCode::DELETE,
    "escape" | "esc" => KeyCode::ESCAPE,
    "up" | "up_arrow" => KeyCode::UP_ARROW,
    "down" | "down_arrow" => KeyCode::DOWN_ARROW,
    "left" | "left_arrow" => KeyCode::LEFT_ARROW,
    "right" | "right_arrow" => KeyCode::RIGHT_ARROW,
    "home" => KeyCode::HOME,
    "end" => KeyCode::END,
    "pageup" | "page_up" => KeyCode::PAGE_UP,
    "pagedown" | "page_down" => KeyCode::PAGE_DOWN,
    "forward_delete" | "fwd_delete" => KeyCode::FORWARD_DELETE,
    "f1" => KeyCode::F1,
    "f2" => KeyCode::F2,
    "f3" => KeyCode::F3,
    "f4" => KeyCode::F4,
    "f5" => KeyCode::F5,
    "f6" => KeyCode::F6,
    "f7" => KeyCode::F7,
    "f8" => KeyCode::F8,
    "f9" => KeyCode::F9,
    "f10" => KeyCode::F10,
    "f11" => KeyCode::F11,
    "f12" => KeyCode::F12,
    other if other.len() == 1 => {
      let ch = other.chars().next().unwrap();
      us_keycode_for_char(ch).ok_or_else(|| format!("未知按键: {name}"))?
    }
    other => return fail(format!("未知按键: {other}")),
  };
  Ok(code)
}

fn us_keycode_for_char(ch: char) -> Option<CGKeyCode> {
  let lower = ch.to_ascii_lowercase();
  match lower {
    'a' => Some(0x00),
    'b' => Some(0x0B),
    'c' => Some(0x08),
    'd' => Some(0x02),
    'e' => Some(0x0E),
    'f' => Some(0x03),
    'g' => Some(0x05),
    'h' => Some(0x04),
    'i' => Some(0x22),
    'j' => Some(0x26),
    'k' => Some(0x28),
    'l' => Some(0x25),
    'm' => Some(0x2E),
    'n' => Some(0x2D),
    'o' => Some(0x1F),
    'p' => Some(0x23),
    'q' => Some(0x0C),
    'r' => Some(0x0F),
    's' => Some(0x01),
    't' => Some(0x11),
    'u' => Some(0x20),
    'v' => Some(0x09),
    'w' => Some(0x0D),
    'x' => Some(0x07),
    'y' => Some(0x10),
    'z' => Some(0x06),
    '0' => Some(0x1D),
    '1' => Some(0x12),
    '2' => Some(0x13),
    '3' => Some(0x14),
    '4' => Some(0x15),
    '5' => Some(0x17),
    '6' => Some(0x16),
    '7' => Some(0x1A),
    '8' => Some(0x1C),
    '9' => Some(0x19),
    '-' => Some(0x1B),
    '=' => Some(0x18),
    '[' => Some(0x21),
    ']' => Some(0x1E),
    '\\' => Some(0x2A),
    ';' => Some(0x29),
    '\'' => Some(0x27),
    ',' => Some(0x2B),
    '.' => Some(0x2F),
    '/' => Some(0x2C),
    '`' => Some(0x32),
    _ => None,
  }
}

fn scroll(x: f64, y: f64, dy: i32) -> Result<(), String> {
  validate_point(x, y)?;
  CGDisplay::warp_mouse_cursor_position(CGPoint::new(x, y))
    .map_err(|_| format!("无法移动鼠标到 ({x}, {y})"))?;
  let source = event_source()?;
  let event = CGEvent::new_scroll_event(
    source,
    ScrollEventUnit::PIXEL,
    1,
    dy,
    0,
    0,
  )
  .map_err(|_| "无法创建滚轮事件".to_string())?;
  event.post(CGEventTapLocation::HID);
  Ok(())
}

pub mod colors;

use ctru::{
	prelude::*,
	services::{
		gfx::{Flush, Screen, Swap},
		gspgpu::FramebufferFormat,
	},
};
use egui::{Event, Modifiers, MouseWheelUnit, PointerButton, Pos2, Rect, TouchPhase, Vec2};
use egui_software_backend::{BufferMutRef, ColorFieldOrder, EguiSoftwareRender};
use std::time::Instant;

fn main() {
	let mut apt = Apt::new().unwrap();
	let mut hid = Hid::new().unwrap();
	let gfx = Gfx::new().unwrap();
	let mut screen = gfx.bottom_screen.borrow_mut();

	unsafe { ctru_sys::osSetSpeedupEnable(true) };
	apt.set_app_cpu_time_limit(45)
		.expect("Failed to enable system core");

	screen.set_double_buffering(true);
	screen.set_framebuffer_format(FramebufferFormat::Rgba8);
	// screen.set_wide_mode(true);
	screen.swap_buffers();

	let (width, height) = {
		let framebuffer = screen.raw_framebuffer();
		(framebuffer.height, framebuffer.width)
	};

	let buffer = &mut vec![[0u8; 4]; width * height];
	let mut buffer_ref = BufferMutRef::new(buffer, width, height);
	let ctx = egui::Context::default();
	// ctx.set_zoom_factor(0.5);
	let mut demo = egui_demo_lib::DemoWindows::default();
	let mut sw_render = EguiSoftwareRender::new(ColorFieldOrder::Rgba);

	let start = Instant::now();
	let mut old_touch: (u16, u16) = (0, 0);
	while apt.main_loop() {
		hid.scan_input();

		let keys = hid.keys_down();
		let keys_held = hid.keys_held();
		let keys_up = hid.keys_up();
		if keys.intersects(KeyPad::START | KeyPad::Y) {
			break;
		}
		let mut events = Vec::new();

		if keys.contains(KeyPad::ZL) {
			events.push(Event::Zoom(-0.5));
		} else if keys.contains(KeyPad::ZR) {
			events.push(Event::Zoom(0.5));
		}

		let mut scroll_x = 0.0;
		let mut scroll_y = 0.0;

		if keys.contains(KeyPad::DPAD_DOWN) {
			scroll_y = -1.0;
		} else if keys.contains(KeyPad::DPAD_UP) {
			scroll_y = 1.0;
		}

		if keys.contains(KeyPad::DPAD_LEFT) {
			scroll_x = -1.0;
		} else if keys.contains(KeyPad::DPAD_RIGHT) {
			scroll_x = 1.0;
		}

		if scroll_x != 0.0 || scroll_y != 0.0 {
			events.push(Event::MouseWheel {
				unit: MouseWheelUnit::Line,
				delta: Vec2 {
					x: scroll_x,
					y: scroll_y,
				},
				phase: TouchPhase::Move,
				modifiers: Modifiers::default(),
			});
		};

		let touch: (u16, u16) = hid.touch_position();
		if keys_up.contains(KeyPad::TOUCH) {
			events.push(Event::PointerButton {
				pos: Pos2::new(old_touch.0 as f32, old_touch.1 as f32),
				button: PointerButton::Primary,
				pressed: false,
				modifiers: Modifiers::default(),
			});
			events.push(Event::PointerGone);
		} else if keys_held.contains(KeyPad::TOUCH) {
			let pos = Pos2::new(touch.0 as f32, touch.1 as f32);
			events.push(Event::PointerButton {
				pos,
				button: PointerButton::Primary,
				pressed: true,
				modifiers: Modifiers::default(),
			});
			events.push(Event::PointerMoved(pos));
		} else if keys.contains(KeyPad::TOUCH) {
			let pos = Pos2::new(touch.0 as f32, touch.1 as f32);
			events.push(Event::PointerButton {
				pos,
				button: PointerButton::Primary,
				pressed: true,
				modifiers: Modifiers::default(),
			});
			events.push(Event::PointerMoved(pos));
			events.push(Event::PointerButton {
				pos: Pos2::new(old_touch.0 as f32, old_touch.1 as f32),
				button: PointerButton::Primary,
				pressed: false,
				modifiers: Modifiers::default(),
			});
			events.push(Event::PointerGone);
		}
		old_touch = touch;

		let input = egui::RawInput {
			screen_rect: Some(Rect::from_min_size(
				Default::default(),
				Vec2::new(width as f32, height as f32),
			)),
			events,
			time: Some(start.elapsed().as_secs_f64()),
			..egui::RawInput::default()
		};

		let out = ctx.run_ui(input, |ui| {
			demo.ui(ui);
		});
		let primitives = ctx.tessellate(out.shapes, out.pixels_per_point);
		// buffer_ref.data.iter_mut().for_each(|x| *x = [0; 4]);
		unsafe {
			std::ptr::write_bytes(
				buffer_ref.data as *mut _ as *mut u8,
				0,
				buffer_ref.data.len() * 4,
			);
		}
		sw_render.render(
			&mut buffer_ref,
			&primitives,
			&out.textures_delta,
			out.pixels_per_point,
		);

		let frame_buffer = screen.raw_framebuffer();
		unsafe {
			let src = std::slice::from_raw_parts(
				buffer_ref.data.as_ptr() as *const u8,
				buffer_ref.data.len() * 4,
			);

			for y in 0..height {
				for x in 0..width {
					let draw_y = (height - 1) - y;
					let draw_x = x;

					let read_index = (y * width + x) * 4;
					let draw_index = (draw_x * height + draw_y) * 4;

					frame_buffer
						.ptr
						.add(draw_index)
						.copy_from_nonoverlapping(src.as_ptr().add(read_index), 4);
				}
			}
		}

		screen.flush_buffers();
		screen.swap_buffers();

		gfx.wait_for_vblank();
	}
}

/*
 * Copyright (C) 2026 Speykious <speykious.dev>
 *
 * This file is part of Fluster.
 *
 * Fluster is free software: you can redistribute it and/or modify it under the
 * terms of the GNU Affero General Public License as published by the Free
 * Software Foundation, either version 3 of the License, or (at your option) any
 * later version.
 *
 * Fluster is distributed in the hope that it will be useful, but WITHOUT ANY
 * WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR
 * A PARTICULAR PURPOSE. See the GNU Affero General Public License for more
 * details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with Fluster. If not, see <https://www.gnu.org/licenses/>.
 */

use std::sync::Arc;

use dioxus_devtools::subsecond;
use image::ImageFormat;
use tracing_subscriber::filter::LevelFilter;
use uing::{glam::Vec2, windowing::UingApp};
use winit::{
	dpi::LogicalSize,
	event::KeyEvent,
	event_loop::{ControlFlow, EventLoop},
	window::{Icon, Theme, Window, WindowAttributes},
};

use crate::gui::{FlusterApp, MainScene};

mod gui;

rust_i18n::i18n!("locales", fallback = "en");

const FLUSTER_LOGO_64X64: &[u8] = include_bytes!("../assets/images/fluster-logo-64x64.png");

fn main() {
	tracing_subscriber::fmt()
		.event_format(
			tracing_subscriber::fmt::format()
				.with_ansi(true)
				.with_file(false)
				.with_level(true)
				.with_source_location(false)
				.with_line_number(false)
				.with_thread_names(true)
				.compact(),
		)
		.with_max_level(LevelFilter::INFO)
		.init();

	#[cfg(debug_assertions)]
	dioxus_devtools::connect_subsecond();

	subsecond::register_handler(Arc::new(|| {
		println!("refreshing locales");
		rust_i18n::i18n!("locales", fallback = "en");
	}));

	let icon_img = image::load_from_memory_with_format(FLUSTER_LOGO_64X64, ImageFormat::Png).unwrap();
	let width = icon_img.width();
	let height = icon_img.height();

	let event_loop = EventLoop::new().unwrap();
	event_loop.set_control_flow(ControlFlow::Wait);

	let mut app = FlusterApp::new(
		WindowAttributes::default()
			.with_active(true)
			.with_theme(Some(Theme::Light))
			.with_title("Fluster")
			.with_window_icon(Some(Icon::from_rgba(icon_img.into_bytes(), width, height).unwrap()))
			.with_resizable(true)
			.with_inner_size(LogicalSize::new(1280, 720)),
		make_scene,
	);

	event_loop.run_app(&mut app).unwrap();
}

fn make_scene(window: Arc<Window>, scale_factor: f32, viewport: Vec2) -> Box<dyn UingApp<KeyEvent>> {
	Box::new(MainScene::new(window, scale_factor, viewport))
}

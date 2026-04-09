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
use uing::glam::{UVec2, Vec2, uvec2, vec2};
use uing::windowing::{ImeEvent, Modifiers, MouseButton, TouchPhase, UingApp};
use uing::{frienderer::Renderer, windowing::winit::GraphicsContext};
use uing::{handle_hot_reload_patch_request, request_hot_reload_patch};
use winit::application::ApplicationHandler;
use winit::event::{ElementState, Ime, MouseScrollDelta, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::{
	event::KeyEvent,
	window::{Window, WindowAttributes},
};

pub struct WinitUingAppState {
	window: Arc<Window>,
	scale_factor: f32,
	renderer: Renderer,
	uing_app: Box<dyn UingApp<KeyEvent>>,
}

pub struct FlusterApp {
	win_attribs: WindowAttributes,
	graphics_context: GraphicsContext,

	state: Option<WinitUingAppState>,
	make_uing_app: fn(Arc<Window>, f32, Vec2) -> Box<dyn UingApp<KeyEvent>>,

	viewport: UVec2,
}

impl FlusterApp {
	pub fn new(
		win_attribs: WindowAttributes,
		make_scene: fn(Arc<Window>, f32, Vec2) -> Box<dyn UingApp<KeyEvent>>,
	) -> Self {
		let graphics_context = GraphicsContext::new(&win_attribs);

		Self {
			win_attribs,
			graphics_context,

			state: None,
			make_uing_app: make_scene,

			viewport: UVec2::default(),
		}
	}
}

impl ApplicationHandler for FlusterApp {
	fn resumed(&mut self, event_loop: &ActiveEventLoop) {
		let (window, renderer) = match (self.graphics_context).create_window(event_loop, &self.win_attribs) {
			Ok(window) => window,
			Err(e) => {
				tracing::error!("Error: {e}");
				event_loop.exit();
				return;
			}
		};

		let hr_window = Arc::clone(&window);
		subsecond::register_handler(Arc::new(move || {
			request_hot_reload_patch();
			hr_window.request_redraw();
		}));

		let win_size = window.inner_size();
		self.viewport = uvec2(win_size.width, win_size.height);

		let scale_factor = window.scale_factor() as f32;
		let scene = (self.make_uing_app)(Arc::clone(&window), scale_factor, self.viewport.as_vec2());

		window.request_redraw();

		self.state = Some(WinitUingAppState {
			window,
			scale_factor,
			renderer,
			uing_app: scene,
		});
	}

	fn window_event(
		&mut self,
		event_loop: &winit::event_loop::ActiveEventLoop,
		_window_id: winit::window::WindowId,
		event: WindowEvent,
	) {
		match event {
			WindowEvent::Resized(size) if size.width != 0 && size.height != 0 => {
				self.graphics_context.resize(size);
				self.viewport = UVec2::new(size.width, size.height);
				if let Some(state) = self.state.as_mut() {
					state.window.request_redraw();
					(state.renderer).resize(self.viewport.x as i32, self.viewport.y as i32, state.scale_factor);
				}
			}

			WindowEvent::CursorMoved { position, .. } => {
				if let Some(state) = self.state.as_mut() {
					state.window.request_redraw();
					(state.uing_app).on_cursor_moved(vec2(position.x as f32, position.y as f32) / state.scale_factor);
				}
			}

			WindowEvent::MouseWheel { delta, phase, .. } => {
				if let Some(s) = self.state.as_mut() {
					s.window.request_redraw();

					let delta = match delta {
						MouseScrollDelta::LineDelta(x, y) => vec2(x, y) * 64.0,
						MouseScrollDelta::PixelDelta(pos) => vec2(pos.x as f32, pos.y as f32),
					};

					let phase = match phase {
						winit::event::TouchPhase::Started => TouchPhase::Started,
						winit::event::TouchPhase::Moved => TouchPhase::Moved,
						winit::event::TouchPhase::Ended => TouchPhase::Ended,
						winit::event::TouchPhase::Cancelled => TouchPhase::Cancelled,
					};

					s.uing_app.on_mouse_wheel(delta, phase);
				}
			}

			WindowEvent::MouseInput { state, button, .. } => {
				if let Some(s) = self.state.as_mut() {
					s.window.request_redraw();

					let button = match button {
						winit::event::MouseButton::Left => MouseButton::Left,
						winit::event::MouseButton::Right => MouseButton::Right,
						winit::event::MouseButton::Middle => MouseButton::Middle,
						winit::event::MouseButton::Back => MouseButton::Back,
						winit::event::MouseButton::Forward => MouseButton::Forward,
						winit::event::MouseButton::Other(b) => MouseButton::Other(b),
					};

					s.uing_app.on_mouse_button(state == ElementState::Pressed, button);
				}
			}

			WindowEvent::CloseRequested => event_loop.exit(),

			WindowEvent::Ime(ime) => {
				let ime = match ime {
					Ime::Enabled => ImeEvent::Enabled,
					Ime::Preedit(text, range) => ImeEvent::Preedit(text, range),
					Ime::Commit(text) => ImeEvent::Commit(text),
					Ime::Disabled => ImeEvent::Disabled,
				};

				if let Some(state) = self.state.as_mut() {
					state.window.request_redraw();
					state.uing_app.on_ime(&ime);
				}
			}

			WindowEvent::ModifiersChanged(modifiers) => {
				if let Some(state) = self.state.as_mut() {
					state.window.request_redraw();

					let modifiers = modifiers.state();
					state.uing_app.on_modifiers_changed(Modifiers {
						shift: modifiers.shift_key(),
						ctrl: modifiers.control_key(),
						alt: modifiers.alt_key(),
						meta: modifiers.super_key(),
					});
				}
			}

			WindowEvent::KeyboardInput {
				event: ref key_event, ..
			} => {
				if let Some(state) = self.state.as_mut() {
					state.window.request_redraw();
					state.uing_app.on_key_input(key_event);
				}
			}

			WindowEvent::RedrawRequested => {
				if let Some(state) = self.state.as_mut() {
					handle_hot_reload_patch_request(|| {
						state.uing_app.ui_mut().request_widget_tree_reset();
						state.uing_app.ui_mut().request_next_frame();
					});

					let scale_factor = state.scale_factor;

					let viewport = self.viewport.as_vec2() / scale_factor;
					(state.uing_app).on_redraw(&mut state.renderer, viewport, scale_factor);

					self.graphics_context.swap_buffers();

					if state.uing_app.ui_mut().next_frame_requested() {
						state.window.request_redraw();
					}
				}
			}

			_ => {}
		};
	}
}

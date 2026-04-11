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
use uing::{
	TextInputAction, UiContext, WidgetReaction,
	frienderer::{Renderer, TextureHandle},
	glam::Vec2,
	windowing::{UingApp, winit::WinitPlatformInteractions},
};
use winit::{
	event::{ElementState, KeyEvent},
	keyboard::{Key, NamedKey},
	window::Window,
};

use crate::gui::theme::{Theme, theme_dark};
pub use context::*;

mod context;
mod theme;

#[allow(unused)]
#[warn(unused_imports)]
mod components {
	pub mod button;
	pub mod checkbox;
	pub mod icon;
	pub mod input_field;
	pub mod login_card;
	pub mod select;
}

mod pages {
	mod login;
	mod register;

	pub use login::login;
	pub use register::register;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum PageId {
	Login,
	Register,
}

static FLUSTER_LOGO_128_PNG: &[u8] = include_bytes!("../assets/images/fluster-logo-nocircle-128x128.png");

struct SceneState {
	theme: Theme,

	logo_128: Option<TextureHandle>,
	page_id: PageId,

	thing: Option<usize>,
	thonk: bool,
}

pub struct MainScene {
	ui: UiContext,
	state: SceneState,
}

impl MainScene {
	pub fn new(window: Arc<Window>, scale_factor: f32, viewport: Vec2) -> Self {
		rust_i18n::set_locale("en");

		let theme = theme_dark();

		let mut ui = UiContext::new(Box::new(WinitPlatformInteractions::new(window)), scale_factor, viewport);
		ui.color_text_cursor = theme.text_primary;
		ui.color_focus = theme.brand_primary;
		ui.color_text_select = theme.border_color_focus;

		// Ok let's try this as an experiment
		ui.act_on_press = true;

		const IBM_PLEX_SANS: &[u8] = include_bytes!("../assets/fonts/IBMPlexSans-VariableFont_wdth,wght.ttf");
		const BRICOLAGE_GROTESQUE: &[u8] = include_bytes!("../assets/fonts/BricolageGrotesque-ExtraBold.trimmed.ttf");
		const PHOSPHOR_FILL_ICONS: &[u8] = include_bytes!("../assets/fonts/Phosphor-Fill.trimmed.ttf");
		const PHOSPHOR_BOLD_ICONS: &[u8] = include_bytes!("../assets/fonts/Phosphor-Bold.trimmed.ttf");
		ui.register_font(Arc::new(IBM_PLEX_SANS));
		ui.register_font(Arc::new(BRICOLAGE_GROTESQUE));
		ui.register_font(Arc::new(PHOSPHOR_FILL_ICONS));
		ui.register_font(Arc::new(PHOSPHOR_BOLD_ICONS));

		Self {
			ui,
			state: SceneState {
				theme,
				logo_128: None,
				page_id: PageId::Login,

				thing: None,
				thonk: false,
			},
		}
	}

	fn interface(state: &mut SceneState, ui: &mut UiContext, renderer: &mut Renderer) -> WidgetReaction {
		match state.page_id {
			PageId::Login => pages::login(state, ui, renderer),
			PageId::Register => pages::register(state, ui, renderer),
		}
	}
}

impl UingApp<KeyEvent> for MainScene {
	fn ui_mut(&mut self) -> &mut UiContext {
		&mut self.ui
	}

	fn on_key_input(&mut self, key_event: &winit::event::KeyEvent) -> bool {
		let ui = &mut self.ui;

		'text_input: {
			let text = key_event.text.as_ref().map(|s| s.as_str()).unwrap_or_default();

			let action = match (key_event.state, key_event.logical_key.as_ref()) {
				(ElementState::Pressed, Key::Named(NamedKey::Backspace)) => match ui.modifier_ctrl() {
					true => TextInputAction::DeleteLeftWord,
					false => TextInputAction::DeleteLeftChar,
				},
				(ElementState::Pressed, Key::Named(NamedKey::Delete)) => match ui.modifier_ctrl() {
					true => TextInputAction::DeleteRightWord,
					false => TextInputAction::DeleteRightChar,
				},
				(ElementState::Pressed, Key::Named(NamedKey::ArrowLeft)) => match ui.modifier_ctrl() {
					true => TextInputAction::MoveLeftWord,
					false => TextInputAction::MoveLeftChar,
				},
				(ElementState::Pressed, Key::Named(NamedKey::ArrowRight)) => match ui.modifier_ctrl() {
					true => TextInputAction::MoveRightWord,
					false => TextInputAction::MoveRightChar,
				},
				(ElementState::Pressed, Key::Named(NamedKey::ArrowUp)) => match ui.modifier_ctrl() {
					true => TextInputAction::MoveUpParagraph,
					false => TextInputAction::MoveUpLine,
				},
				(ElementState::Pressed, Key::Named(NamedKey::ArrowDown)) => match ui.modifier_ctrl() {
					true => TextInputAction::MoveDownParagraph,
					false => TextInputAction::MoveDownLine,
				},
				(ElementState::Pressed, Key::Named(NamedKey::Enter)) if ui.modifier_shift() => TextInputAction::NewLine,
				(ElementState::Pressed, Key::Named(NamedKey::Enter)) => TextInputAction::SubmitPress,
				(ElementState::Released, Key::Named(NamedKey::Enter)) => TextInputAction::SubmitRelease,
				(ElementState::Pressed, Key::Named(NamedKey::Tab)) => break 'text_input,
				(ElementState::Pressed, key) => match ui.modifier_ctrl() {
					true => match key {
						Key::Character("a") => TextInputAction::SelectAll,
						Key::Character("c") => TextInputAction::Copy,
						Key::Character("v") => TextInputAction::Paste,
						_ => break 'text_input,
					},
					false if text.is_empty() => break 'text_input,
					false => TextInputAction::Commit,
				},

				_ => break 'text_input,
			};

			if ui.on_text_input_action(action, ui.modifier_shift(), text) {
				return true;
			}
		};

		match (key_event.state, key_event.logical_key.as_ref()) {
			(ElementState::Released, Key::Named(NamedKey::Tab)) => {
				if ui.modifier_shift() {
					ui.focus_on_prev();
				} else {
					ui.focus_on_next();
				}
			}
			(state, Key::Named(NamedKey::Enter | NamedKey::Space)) => {
				ui.set_focus_pressed(state == ElementState::Pressed);
			}
			(ElementState::Released, Key::Named(NamedKey::Escape)) => {
				ui.focus_on(None);
			}
			(ElementState::Released, Key::Named(NamedKey::F12)) => {
				ui.inspector.toggle();
			}
			_ => return false,
		}

		true
	}

	fn on_redraw(&mut self, renderer: &mut Renderer, viewport: Vec2, scale_factor: f32) {
		subsecond::call(|| {
			self.ui.start_frame();
			{
				self.ui.resize(viewport);

				let state = &mut self.state;
				(self.ui).build_interface(renderer, |ui, renderer| Self::interface(state, ui, renderer));

				self.ui.solve_layout(scale_factor);
				self.ui.react();

				self.ui.draw_widgets(renderer, scale_factor);
				self.ui.make_renderer_draw(renderer);
			}
			self.ui.end_frame();
		});
	}
}

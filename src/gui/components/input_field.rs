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

use std::{cell::Cell, rc::Rc};

use uing::{
	Anchor, UiContext, WidgetKey, WidgetReaction,
	glam::Vec4,
	parley::FontWeight,
	prelude::{FILL, HUG_ROUND, Wf},
	wk,
};

use crate::gui::{
	components::{
		button::{ButtonBuilder, ButtonSize, ButtonStyle},
		icon::{PhosphorIcon, phosphor_icon_fill},
	},
	theme::{FONT_SIZE_SM, RADIUS_LG, SPACING_1, SPACING_2_5, SPACING_3, SPACING_4, Theme},
};

#[inline(always)]
pub fn input_field<'a>() -> InputFieldBuilder<'a> {
	InputFieldBuilder::new()
}

#[inline(always)]
pub fn text_input(ui: &mut UiContext, theme: &Theme, key: WidgetKey) -> WidgetReaction {
	InputFieldBuilder::new().build(ui, theme, key)
}

#[inline(always)]
pub fn password_input(ui: &mut UiContext, theme: &Theme, key: WidgetKey) -> WidgetReaction {
	InputFieldBuilder::new().password(true).build(ui, theme, key)
}

#[inline(always)]
pub fn text_input_field(ui: &mut UiContext, theme: &Theme, key: WidgetKey, label: &str) -> WidgetReaction {
	InputFieldBuilder::new().label(Some(label)).build(ui, theme, key)
}

#[inline(always)]
pub fn password_input_field(ui: &mut UiContext, theme: &Theme, key: WidgetKey, label: &str) -> WidgetReaction {
	InputFieldBuilder::new()
		.label(Some(label))
		.password(true)
		.build(ui, theme, key)
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ElementStatus {
	#[default]
	Neutral,
	Error,
}

#[derive(Debug, Default)]
pub struct InputFieldBuilder<'a> {
	password: bool,
	icon_start: Option<PhosphorIcon>,
	icon_end: Option<PhosphorIcon>,
	label: Option<&'a str>,
	placeholder: Option<&'a str>,
	subtext: Option<&'a str>,
	status: ElementStatus,
}

impl<'a> InputFieldBuilder<'a> {
	#[inline(always)]
	pub fn new() -> Self {
		Self::default()
	}

	#[inline(always)]
	pub fn password(self, password: bool) -> Self {
		Self { password, ..self }
	}

	#[inline(always)]
	pub fn icon_start(self, icon_start: Option<PhosphorIcon>) -> Self {
		Self { icon_start, ..self }
	}

	#[inline(always)]
	pub fn icon_end(self, icon_end: Option<PhosphorIcon>) -> Self {
		Self { icon_end, ..self }
	}

	#[inline(always)]
	pub fn label(self, label: Option<&'a str>) -> Self {
		Self { label, ..self }
	}

	#[inline(always)]
	pub fn placeholder(self, placeholder: Option<&'a str>) -> Self {
		Self { placeholder, ..self }
	}

	#[inline(always)]
	pub fn subtext(self, subtext: Option<&'a str>) -> Self {
		Self { subtext, ..self }
	}

	pub fn build(self, ui: &mut UiContext, theme: &Theme, key: WidgetKey) -> WidgetReaction {
		let (accent_color, subtext_color, border_color) = match self.status {
			ElementStatus::Neutral => (
				theme.text_primary,
				theme.text_tertiary,
				theme.background_modifier_accent,
			),
			ElementStatus::Error => (theme.accent_danger, theme.accent_danger, theme.accent_danger),
		};

		let field = (ui.build_widget(wk!([key])))
			.flex_col(SPACING_1)
			.size_wh(FILL, HUG_ROUND)
			.build();
		{
			if let Some(label) = self.label {
				let label_props = theme.tp_regular(FONT_SIZE_SM).font_weight(FontWeight::MEDIUM);
				let label = ui.text(wk!([key]), label, &label_props).size_fill().build();
				ui.add_child(field, label);
			}

			let visibility_state = if self.password {
				ui.use_state(key, || Cell::new(false))
			} else {
				Rc::new(Cell::new(true))
			};

			let input = ui.text_input(
				key,
				|b| {
					b.pad_hv(SPACING_4, SPACING_2_5)
						.add_flags(Wf::PASSWD_INPUT & !visibility_state.get())
						.color(theme.form_surface_background)
						.border(border_color, Vec4::splat(1.0), Vec4::splat(RADIUS_LG))
						.flex_row(SPACING_3)
						.size_wh(FILL, HUG_ROUND)
				},
				&theme.tp_regular(FONT_SIZE_SM),
				self.placeholder.unwrap_or_default(),
				theme.text_tertiary,
			);
			{
				if let Some(icon_start) = self.icon_start {
					let icon = phosphor_icon_fill(ui, wk!([key]), icon_start, FONT_SIZE_SM, accent_color)
						.center()
						.build();
					ui.add_child_to_start(input, icon);
				}

				if let Some(icon_end) = self.icon_end {
					let icon = phosphor_icon_fill(ui, wk!([key]), icon_end, FONT_SIZE_SM, accent_color)
						.center()
						.build();
					ui.add_child(input, icon);
				}

				if self.password {
					let icon = match visibility_state.get() {
						true => PhosphorIcon::EyeSlash,
						false => PhosphorIcon::Eye,
					};

					let toggle = ButtonBuilder::new()
						.size(ButtonSize::Text)
						.style(ButtonStyle::GhostMuted)
						.width(HUG_ROUND)
						.icon_start(Some(icon))
						.anchorigin(Anchor::CENTER)
						.build(ui, theme, wk!([key]));
					if toggle.clicked() {
						visibility_state.set(!visibility_state.get());
					}
					ui.add_child(input, toggle);
				}
			}
			ui.add_child(field, input);

			if let Some(subtext) = self.subtext {
				let subtext_props = theme.tp_regular(FONT_SIZE_SM).color(subtext_color);
				let label = ui.text(wk!([key]), subtext, &subtext_props).size_fill().build();
				ui.add_child(field, label);
			}
		}
		field
	}
}

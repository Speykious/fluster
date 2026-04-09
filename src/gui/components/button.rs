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

use uing::{
	Anchor, UiContext, WidgetDim, WidgetKey, WidgetPadding, WidgetReaction, components::ButtonColors, glam::Vec4,
	parley::FontWeight, prelude::HUG_ROUND, wk,
};

use crate::gui::{
	components::icon::{PhosphorIcon, phosphor_icon_fill},
	theme::{FONT_SIZE_SM, SPACING_1_5, SPACING_2, SPACING_2_5, SPACING_4, TRANSPARENT_BLACK, Theme},
};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ButtonSize {
	#[default]
	Regular,
	Small,
	Text,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ButtonStyle {
	#[default]
	Primary,
	Secondary,
	Ghost,
	GhostMuted,
	Inverted,
	Outline,
	Link,
}

#[derive(Debug)]
pub struct ButtonBuilder<'a> {
	icon_start: Option<PhosphorIcon>,
	icon_end: Option<PhosphorIcon>,
	label: Option<&'a str>,
	style: ButtonStyle,
	size: ButtonSize,
	width: WidgetDim,
	anchorigin: Anchor,
}

impl<'a> Default for ButtonBuilder<'a> {
	fn default() -> Self {
		Self {
			icon_start: Default::default(),
			icon_end: Default::default(),
			label: Default::default(),
			style: Default::default(),
			size: Default::default(),
			width: HUG_ROUND,
			anchorigin: Default::default(),
		}
	}
}

impl<'a> ButtonBuilder<'a> {
	#[inline(always)]
	pub fn new() -> Self {
		Self::default()
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
	pub fn style(self, style: ButtonStyle) -> Self {
		Self { style, ..self }
	}

	#[inline(always)]
	pub fn size(self, size: ButtonSize) -> Self {
		Self { size, ..self }
	}

	#[inline(always)]
	pub fn width(self, width: WidgetDim) -> Self {
		Self { width, ..self }
	}

	#[inline(always)]
	pub fn anchorigin(self, anchorigin: Anchor) -> Self {
		Self { anchorigin, ..self }
	}

	pub fn build(self, ui: &mut UiContext, theme: &Theme, key: WidgetKey) -> WidgetReaction {
		let btn_theme = match self.style {
			ButtonStyle::Primary => &theme.button_primary,
			ButtonStyle::Secondary => &theme.button_secondary,
			ButtonStyle::Ghost => &theme.button_ghost,
			ButtonStyle::GhostMuted => &theme.button_ghost_muted,
			ButtonStyle::Inverted => &theme.button_inverted,
			ButtonStyle::Outline => &theme.button_outline,
			ButtonStyle::Link => &theme.button_link,
		};

		let mut button_colors = ButtonColors {
			regular: (btn_theme.idle.bg, btn_theme.idle.border),
			hovered: (btn_theme.hover.bg, btn_theme.hover.border),
			pressed: (btn_theme.press.bg, btn_theme.press.border),
		};

		let (padding, font_size) = match self.size {
			ButtonSize::Regular => (WidgetPadding::hv(SPACING_4, SPACING_2_5), FONT_SIZE_SM),
			ButtonSize::Small => (WidgetPadding::hv(SPACING_2_5, SPACING_1_5), FONT_SIZE_SM),
			ButtonSize::Text => (WidgetPadding::default(), FONT_SIZE_SM),
		};

		if self.size == ButtonSize::Text {
			button_colors.regular = (TRANSPARENT_BLACK, TRANSPARENT_BLACK);
			button_colors.hovered = (TRANSPARENT_BLACK, TRANSPARENT_BLACK);
			button_colors.pressed = (TRANSPARENT_BLACK, TRANSPARENT_BLACK);
		}

		let container = (ui.btn_box(key, button_colors))
			.size_wh(self.width, HUG_ROUND)
			.padding(padding)
			.anchorigin(self.anchorigin)
			.border_width(Vec4::splat(btn_theme.border_width))
			.border_radius(Vec4::splat(btn_theme.border_radius))
			.build();
		{
			if self.icon_start.is_some() || self.label.is_some() || self.icon_end.is_some() {
				let subcontainer = (ui.build_widget(wk!([key])))
					.flex_row(SPACING_2)
					.size_hug_round()
					.center()
					.build();
				{
					let fg = if container.pressed() {
						btn_theme.press.fg
					} else if container.hovered() {
						btn_theme.hover.fg
					} else {
						btn_theme.idle.fg
					};

					if let Some(icon_start) = self.icon_start {
						let icon = phosphor_icon_fill(ui, wk!([key]), icon_start, font_size, fg)
							.center()
							.build();
						ui.add_child(subcontainer, icon);
					}

					if let Some(label) = self.label {
						let is_link = self.style == ButtonStyle::Link;
						let weight = if is_link {
							FontWeight::NORMAL
						} else {
							FontWeight::SEMI_BOLD
						};

						let mut tp = theme.tp_regular(font_size).color(fg).font_weight(weight);
						tp.underline = is_link && container.hovered();
						let text = ui.text(wk!([key]), label, &tp).center().build();
						ui.add_child(subcontainer, text);
					}

					if let Some(icon_end) = self.icon_end {
						let icon = phosphor_icon_fill(ui, wk!([key]), icon_end, font_size, fg)
							.center()
							.build();
						ui.add_child(subcontainer, icon);
					}
				}
				ui.add_child(container, subcontainer);
			}
		}
		container
	}
}

#[inline(always)]
pub fn button_primary(ui: &mut UiContext, theme: &Theme, key: WidgetKey, label: &str) -> WidgetReaction {
	ButtonBuilder::new()
		.style(ButtonStyle::Primary)
		.label(Some(label))
		.build(ui, theme, key)
}

#[inline(always)]
pub fn button_secondary(ui: &mut UiContext, theme: &Theme, key: WidgetKey, label: &str) -> WidgetReaction {
	ButtonBuilder::new()
		.style(ButtonStyle::Secondary)
		.label(Some(label))
		.build(ui, theme, key)
}

#[inline(always)]
pub fn button_transparent(ui: &mut UiContext, theme: &Theme, key: WidgetKey, label: &str) -> WidgetReaction {
	ButtonBuilder::new()
		.style(ButtonStyle::Ghost)
		.label(Some(label))
		.build(ui, theme, key)
}

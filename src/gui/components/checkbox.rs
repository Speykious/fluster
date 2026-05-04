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
	UiContext, WidgetKey, WidgetReaction,
	glam::{Vec4, vec2},
	wk,
};
use winit::window::CursorIcon;

use crate::gui::{
	components::icon::{PhosphorIcon, phosphor_icon_bold},
	theme::{TRANSPARENT_BLACK, Theme},
};

pub fn checkbox(ui: &mut UiContext, theme: &Theme, key: WidgetKey, state: &mut bool) -> WidgetReaction {
	let (bg_color, border_color) = if *state {
		(theme.brand_primary, theme.brand_primary)
	} else {
		(TRANSPARENT_BLACK, theme.background_modifier_accent)
	};

	let container = (ui.build_widget(key))
		.size_fixed(24.0, 24.0)
		.color(bg_color)
		.border(border_color, Vec4::ONE, Vec4::splat(4.0))
		.cursor_icons(CursorIcon::Pointer, CursorIcon::Pointer)
		.clickable()
		.focusable()
		.build();
	{
		if container.clicked() {
			*state = !*state;
		}

		if *state {
			let check = phosphor_icon_bold(ui, wk!([key]), PhosphorIcon::Check, 18.0, theme.text_primary)
				.pos(vec2(0.0, -2.0))
				.center()
				.build();
			ui.add_child(container, check);
		}
	}
	container
}

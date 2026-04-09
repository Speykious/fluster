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

use std::{cell::Cell, f32::consts::TAU};

use uing::{
	TextProps, UiContext, Widget, WidgetBuilder, WidgetKey,
	frienderer::{Color, DrawCommand, RRect, Renderer},
	glam::{Vec2, Vec4, vec2},
};

use crate::gui::theme::{BLACK, TRANSPARENT_WHITE};

/// Subset of Phosphor Icons, only ones used within Fluster.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PhosphorIcon {
	Browser = 0xe0f4,
	Check = 0xe182,
	Eye = 0xe220,
	EyeSlash = 0xe224,
	Key = 0xe2d6,
	Control = 0xeca6,
}

pub fn phosphor_icon_fill(
	ui: &mut UiContext,
	key: WidgetKey,
	icon: PhosphorIcon,
	font_size: f32,
	color: Color,
) -> WidgetBuilder<'_> {
	let s = char::from_u32(icon as u32).unwrap().to_string();
	ui.text(key, &s, &tp_icon_fill(font_size).color(color))
}

pub fn phosphor_icon_bold(
	ui: &mut UiContext,
	key: WidgetKey,
	icon: PhosphorIcon,
	font_size: f32,
	color: Color,
) -> WidgetBuilder<'_> {
	let s = char::from_u32(icon as u32).unwrap().to_string();
	ui.text(key, &s, &tp_icon_bold(font_size).color(color))
}

pub const fn tp_icon_fill(font_size: f32) -> TextProps<'static> {
	TextProps {
		font_stack: "Phosphor-Fill",
		font_size,
		color: BLACK,
		..TextProps::new()
	}
}

pub const fn tp_icon_bold(font_size: f32) -> TextProps<'static> {
	TextProps {
		font_stack: "Phosphor-Bold",
		font_size,
		color: BLACK,
		..TextProps::new()
	}
}

pub fn loading_anim(ui: &mut UiContext, key: WidgetKey, size: f32, color: Color) -> WidgetBuilder<'_> {
	let anim_state = ui.use_state(key, || Cell::new(0_f32));
	anim_state.set((anim_state.get() + ui.dt()) % TAU);
	ui.request_next_frame(); // this element is constantly animated

	fn render_loading(ui: &UiContext, renderer: &mut Renderer, widget: &Widget) {
		let rect = widget.solved_rect();
		let center = rect.pos + 0.5 * rect.size;

		let circle_gap = rect.size.x / 20.0;
		let circle_size = (rect.size.x - 4.0 * circle_gap) / 3.0;
		let border_radius = Vec4::splat(circle_size / 2.0);
		let fill_color = widget.props.color;
		let stroke_color = fill_color & TRANSPARENT_WHITE;

		let anim_state = ui.get_state::<Cell<f32>>(widget.key).unwrap();
		let t = anim_state.get() * 6.0;

		let amplitude = 4.0;
		let sin_offset = 0.4;

		let y = center.y - 0.5 * circle_size;
		let y1 = y - amplitude * ((1.0 + sin_offset) * t.sin() - sin_offset).max(0.0);
		let y2 = y - amplitude * ((1.0 + sin_offset) * t.cos() - sin_offset).max(0.0);
		let y3 = y - amplitude * ((1.0 + sin_offset) * -t.cos() - sin_offset).max(0.0);

		let x1 = center.x - 0.5 * circle_size;
		let x2 = x1 - (circle_gap + circle_size);
		let x3 = x1 + (circle_gap + circle_size);

		renderer.push_draw_commands(&[
			DrawCommand::RRect(RRect {
				pos: vec2(x1, y1),
				size: Vec2::splat(circle_size),
				border_radius,
				border_width: Vec4::ZERO,
				fill_color,
				stroke_color,
				box_blur: 0.0,
			}),
			DrawCommand::RRect(RRect {
				pos: vec2(x2, y2),
				size: Vec2::splat(circle_size),
				border_radius,
				border_width: Vec4::ZERO,
				fill_color,
				stroke_color,
				box_blur: 0.0,
			}),
			DrawCommand::RRect(RRect {
				pos: vec2(x3, y3),
				size: Vec2::splat(circle_size),
				border_radius,
				border_width: Vec4::ZERO,
				fill_color,
				stroke_color,
				box_blur: 0.0,
			}),
		]);
	}

	let mut element = (ui.build_widget(key))
		.size_fixed(size, size)
		.custom_render(render_loading);

	// Set color without activating the DRAW_BACKGROUND flag
	element.props_mut().color = color;

	element
}

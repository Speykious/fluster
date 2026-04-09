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
	UiContext, WidgetReaction,
	frienderer::Renderer,
	glam::Vec4,
	prelude::{FILL_RESPECT_HUG, fixed},
	wk,
};

use crate::gui::{
	FLUSTER_LOGO_128_PNG, SceneState,
	theme::{RADIUS_2XL, SPACING_6, SPACING_8, SPACING_12},
};

#[derive(Debug, Clone, Copy)]
pub struct LoginCardReaction {
	pub card: WidgetReaction,
	pub form: WidgetReaction,
}

pub fn login_card(state: &mut SceneState, ui: &mut UiContext, renderer: &mut Renderer) -> LoginCardReaction {
	let form;

	let login_box_width = 896.0;

	let card = (ui.build_widget(wk!()))
		.flex_row(0.0)
		.size_hug_round()
		.center()
		.color(state.theme.background_secondary)
		.border_radius(Vec4::splat(RADIUS_2XL))
		.build();
	{
		let logo_side = (ui.build_widget(wk!()))
			.size_wh(fixed(login_box_width / 3.0), FILL_RESPECT_HUG)
			.border(state.theme.background_modifier_accent, Vec4::Y, Vec4::ZERO)
			.pad_hv(SPACING_8, SPACING_12)
			.build();
		{
			let logo_inside = (ui.build_widget(wk!()))
				.flex_col(SPACING_6)
				.size_hug_round()
				.center()
				.build();
			{
				let logo_128 =
					*(state.logo_128).get_or_insert_with(|| ui.upload_png_from_memory(renderer, FLUSTER_LOGO_128_PNG));

				let logo_128 = (ui.build_widget(wk!()))
					.image(logo_128)
					.size_fixed(128.0, 128.0)
					.color(state.theme.brand_primary)
					.border_color(state.theme.brand_primary)
					.border_radius(Vec4::splat(64.0))
					.center()
					.build();
				ui.add_child(logo_inside, logo_128);

				let tp = state
					.theme
					.tp_regular(42.0)
					.font_stack("Bricolage Grotesque ExtraBold")
					.letter_spacing(-1.8);
				let fluxer_logo_text = ui.text(wk!(), "Fluster", &tp).build();
				ui.add_child(logo_inside, fluxer_logo_text);
			}
			ui.add_child(logo_side, logo_inside);
		}
		ui.add_child(card, logo_side);

		form = (ui.build_widget(wk!()))
			.size_wh(fixed(login_box_width * 2.0 / 3.0), FILL_RESPECT_HUG)
			.pad_hv(SPACING_12, SPACING_12)
			.flex_col(SPACING_6)
			.build();
		ui.add_child(card, form);
	}

	LoginCardReaction { card, form }
}

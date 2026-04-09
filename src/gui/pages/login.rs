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

use rust_i18n::t;
use uing::{
	UiContext, WidgetReaction,
	frienderer::Renderer,
	parley::FontWeight,
	prelude::{FILL, HUG_ROUND, fixed},
	wk,
};

use crate::gui::{
	PageId, SceneState,
	components::{
		button::{ButtonBuilder, ButtonSize, ButtonStyle},
		icon::PhosphorIcon,
		input_field::{password_input_field, text_input_field},
		login_card::login_card,
	},
	theme::{FONT_SIZE_SM, LAYOUT_GAP, LAYOUT_GAP_SM},
};

pub fn login(state: &mut SceneState, ui: &mut UiContext, renderer: &mut Renderer) -> WidgetReaction {
	let window_frame = (ui.build_widget(wk!()))
		.color(state.theme.brand_primary)
		.size_fill()
		.build();
	{
		let login_box = login_card(state, ui, renderer);
		{
			let tp = state.theme.tp_regular(20.0).font_weight(FontWeight::SEMI_BOLD);
			let welcome_back = ui.text(wk!(), &t!("login.welcome_back"), &tp).center().build();
			ui.add_child(login_box.form, welcome_back);

			let email_fieldset = text_input_field(ui, &state.theme, wk!(), &t!("login.email"));
			ui.add_child(login_box.form, email_fieldset);

			let password_fieldset = password_input_field(ui, &state.theme, wk!(), &t!("login.password"));
			ui.add_child(login_box.form, password_fieldset);

			let tp = state.theme.tp_regular(FONT_SIZE_SM).color(state.theme.text_tertiary);
			let label = ui.text(wk!(), &t!("login.forgot_password"), &tp).build();
			ui.add_child(login_box.form, label);

			let login_btn =
				ButtonBuilder::new()
					.label(Some(&t!("login.log_in")))
					.width(FILL)
					.build(ui, &state.theme, wk!());
			ui.add_child(login_box.form, login_btn);

			let divider = (ui.build_widget(wk!()))
				.size_wh(FILL, HUG_ROUND)
				.flex_row(LAYOUT_GAP)
				.build();
			{
				let line = (ui.build_widget(wk!()))
					.size_wh(FILL, fixed(1.0))
					.color(state.theme.background_modifier_accent)
					.center()
					.build();
				ui.add_child(divider, line);

				let tp = state.theme.tp_regular(FONT_SIZE_SM).color(state.theme.text_tertiary);
				let label_or = ui.text(wk!(), &t!("login.or"), &tp).build();
				ui.add_child(divider, label_or);

				let line = (ui.build_widget(wk!()))
					.size_wh(FILL, fixed(1.0))
					.color(state.theme.background_modifier_accent)
					.center()
					.build();
				ui.add_child(divider, line);
			}
			ui.add_child(login_box.form, divider);

			let other_options = (ui.build_widget(wk!()))
				.flex_col(LAYOUT_GAP)
				.size_wh(FILL, HUG_ROUND)
				.build();
			{
				let buttons = (ui.build_widget(wk!()))
					.flex_col(LAYOUT_GAP_SM)
					.size_wh(FILL, HUG_ROUND)
					.build();
				{
					let passkey_btn = ButtonBuilder::new()
						.style(ButtonStyle::Secondary)
						.icon_start(Some(PhosphorIcon::Key))
						.label(Some(&t!("login.log_in_with_passkey")))
						.width(FILL)
						.build(ui, &state.theme, wk!());
					ui.add_child(buttons, passkey_btn);

					let browser_btn = ButtonBuilder::new()
						.style(ButtonStyle::Secondary)
						.icon_start(Some(PhosphorIcon::Browser))
						.label(Some(&t!("login.log_in_via_browser")))
						.width(FILL)
						.build(ui, &state.theme, wk!());
					ui.add_child(buttons, browser_btn);
				}
				ui.add_child(other_options, buttons);

				let need_account = (ui.build_widget(wk!())).size_hug_round().flex_row(6.0).build();
				{
					let tp = state.theme.tp_regular(FONT_SIZE_SM).color(state.theme.text_tertiary);
					let label = ui.text(wk!(), &t!("login.need_account"), &tp).center().build();
					ui.add_child(need_account, label);

					let register_btn = ButtonBuilder::new()
						.style(ButtonStyle::Link)
						.size(ButtonSize::Text)
						.label(Some(&t!("login.register")))
						.build(ui, &state.theme, wk!());
					if register_btn.clicked() {
						state.page_id = PageId::Register;
					}
					ui.add_child(need_account, register_btn);
				}
				ui.add_child(other_options, need_account);
			}
			ui.add_child(login_box.form, other_options);
		}
		ui.add_child(window_frame, login_box.card);
	}
	window_frame
}

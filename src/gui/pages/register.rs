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
	UiContext, WidgetDim, WidgetReaction,
	frienderer::Renderer,
	parley::FontWeight,
	prelude::{FILL, HUG_ROUND},
	wk,
};

use crate::gui::{
	PageId, SceneState,
	components::{
		button::{ButtonBuilder, ButtonSize, ButtonStyle},
		checkbox::checkbox,
		input_field::{InputFieldBuilder, input_field, password_input_field, text_input_field},
		login_card::login_card,
		select::select,
	},
	theme::{FONT_SIZE_SM, SPACING_2},
};

pub fn register(state: &mut SceneState, ui: &mut UiContext, renderer: &mut Renderer) -> WidgetReaction {
	let window_frame = (ui.build_widget(wk!()))
		.color(state.theme.brand_primary)
		.size_fill()
		.build();
	{
		let login_box = login_card(state, ui, renderer);
		{
			let tp = state.theme.tp_regular(20.0).font_weight(FontWeight::SEMI_BOLD);
			let welcome_back = ui.text(wk!(), &t!("register.create_an_account"), &tp).center().build();
			ui.add_child(login_box.form, welcome_back);

			let email_fieldset = text_input_field(ui, &state.theme, wk!(), &t!("login.email"));
			ui.add_child(login_box.form, email_fieldset);

			let display_name_fieldset =
				text_input_field(ui, &state.theme, wk!(), &t!("register.display_name_optional"));
			ui.add_child(login_box.form, display_name_fieldset);

			let username_fieldset = InputFieldBuilder::new()
				.label(Some(&t!("register.username_optional")))
				.subtext(Some(&t!("register.four_digit_tag_footer")))
				.build(ui, &state.theme, wk!());
			ui.add_child(login_box.form, username_fieldset);

			let password_fieldset = password_input_field(ui, &state.theme, wk!(), &t!("login.password"));
			ui.add_child(login_box.form, password_fieldset);

			let confirm_password_fieldset =
				password_input_field(ui, &state.theme, wk!(), &t!("register.confirm_password"));
			ui.add_child(login_box.form, confirm_password_fieldset);

			let date_of_birth_fieldset_group = (ui.build_widget(wk!()))
				.size_wh(FILL, HUG_ROUND)
				.flex_col(SPACING_2)
				.build();
			{
				let tp = state.theme.tp_regular(FONT_SIZE_SM).font_weight(FontWeight::MEDIUM);
				let dob_label = ui.text(wk!(), &t!("register.date_of_birth"), &tp).size_fill().build();
				ui.add_child(date_of_birth_fieldset_group, dob_label);

				let fieldsets = (ui.build_widget(wk!()))
					.size_wh(FILL, HUG_ROUND)
					.flex_row(SPACING_2)
					.build();
				{
					let year_input = input_field()
						.subtext(Some(&t!("calendar.year")))
						.build(ui, &state.theme, wk!());
					ui.add_child(fieldsets, year_input);

					let select_container = (ui.build_widget(wk!()))
						.size_wh(WidgetDim::fill_frac(2.0), HUG_ROUND)
						.build();
					{
						let choices: &[&str] = &[
							&t!("calendar.january"),
							&t!("calendar.february"),
							&t!("calendar.march"),
							&t!("calendar.april"),
							&t!("calendar.may"),
							&t!("calendar.june"),
							&t!("calendar.july"),
							&t!("calendar.august"),
							&t!("calendar.september"),
							&t!("calendar.october"),
							&t!("calendar.november"),
							&t!("calendar.december"),
						];
						let placeholder: &str = &t!("calendar.month");

						let select_month = select(ui, &state.theme, wk!(), placeholder, choices, &mut state.thing);
						ui.add_child(select_container, select_month);
					}
					ui.add_child(fieldsets, select_container);

					let day_input = input_field()
						.subtext(Some(&t!("calendar.day")))
						.build(ui, &state.theme, wk!());
					ui.add_child(fieldsets, day_input);
				}
				ui.add_child(date_of_birth_fieldset_group, fieldsets);
			}
			ui.add_child(login_box.form, date_of_birth_fieldset_group);

			let agree_tos_privacy = (ui.build_widget(wk!())).size_hug_round().flex_row(SPACING_2).build();
			{
				let agree = checkbox(ui, &state.theme, wk!(), &mut state.thonk);
				ui.add_child(agree_tos_privacy, agree);

				let tos_privacy = (ui.build_widget(wk!())).size_hug_round().flex_row(4.0).center().build();
				{
					let tp = state.theme.tp_regular(FONT_SIZE_SM);

					let text = ui.text(wk!(), &t!("register.tos_agreement_p1"), &tp).build();
					ui.add_child(tos_privacy, text);

					let tos_btn = ButtonBuilder::new()
						.style(ButtonStyle::Link)
						.size(ButtonSize::Text)
						.label(Some(&t!("register.terms_of_service")))
						.build(ui, &state.theme, wk!());
					if tos_btn.clicked() {
						let _ = open::that("https://fluxer.app/terms");
					}
					ui.add_child(tos_privacy, tos_btn);

					let text = ui.text(wk!(), &t!("register.tos_agreement_p2"), &tp).build();
					ui.add_child(tos_privacy, text);

					let privacy_policy_btn = ButtonBuilder::new()
						.style(ButtonStyle::Link)
						.size(ButtonSize::Text)
						.label(Some(&t!("register.privacy_policy")))
						.build(ui, &state.theme, wk!());
					if privacy_policy_btn.clicked() {
						let _ = open::that("https://fluxer.app/privacy");
					}
					ui.add_child(tos_privacy, privacy_policy_btn);
				}
				ui.add_child(agree_tos_privacy, tos_privacy);
			}
			ui.add_child(login_box.form, agree_tos_privacy);

			let create_account_btn = ButtonBuilder::new()
				.label(Some(&t!("register.create_account")))
				.width(FILL)
				.build(ui, &state.theme, wk!());
			ui.add_child(login_box.form, create_account_btn);

			let already_have_account = (ui.build_widget(wk!())).size_hug_round().flex_row(6.0).build();
			{
				let tp = state.theme.tp_regular(FONT_SIZE_SM).color(state.theme.text_tertiary);
				let label = (ui.text(wk!(), &t!("register.already_have_account"), &tp))
					.center()
					.build();
				ui.add_child(already_have_account, label);

				let login_btn = ButtonBuilder::new()
					.style(ButtonStyle::Link)
					.size(ButtonSize::Text)
					.label(Some(&t!("register.login")))
					.build(ui, &state.theme, wk!());
				if login_btn.clicked() {
					state.page_id = PageId::Login;
				}
				ui.add_child(already_have_account, login_btn);
			}
			ui.add_child(login_box.form, already_have_account);
		}
		ui.add_child(window_frame, login_box.card);
	}
	window_frame
}

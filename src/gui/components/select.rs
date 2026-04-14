use std::{cell::Cell, f32::consts::PI};

use uing::{
	Anchor, UiContext, WidgetKey, WidgetReaction,
	anime::AnimSodProps,
	components::ButtonColors,
	glam::{Vec2, Vec4},
	prelude::{Af, FILL, HUG_ROUND, fixed},
	wk,
};
use winit::window::CursorIcon;

use crate::gui::{
	components::icon::{PhosphorIcon, phosphor_icon_bold},
	theme::{
		FONT_SIZE_SM, RADIUS_LG, SPACING_1, SPACING_2, SPACING_2_5, SPACING_3, SPACING_4, TRANSPARENT_WHITE, Theme,
	},
};

pub fn select(
	ui: &mut UiContext,
	theme: &Theme,
	key: WidgetKey,
	placeholder: &str,
	choices: &[&str],
	selected_elem: &mut Option<usize>,
) -> WidgetReaction {
	let selected_state = ui.use_state(key, || Cell::new(false));

	let select_container = ui.build_widget(key).size_wh(FILL, HUG_ROUND).build();
	{
		let select_subcontainer = (ui.build_widget(wk!([key])))
			.size_wh(FILL, HUG_ROUND)
			.flex_col(SPACING_1)
			.build();
		{
			let select_box = (ui.build_widget(wk!([key])))
				.pad_hv(SPACING_4, SPACING_2_5)
				.color(theme.form_surface_background)
				.border(
					theme.background_modifier_accent,
					Vec4::splat(1.0),
					Vec4::splat(RADIUS_LG),
				)
				.flex_row(SPACING_3)
				.size_wh(FILL, HUG_ROUND)
				.cursor_icons(CursorIcon::Pointer, CursorIcon::Pointer)
				.clickable()
				.focusable()
				.build();
			{
				if select_box.clicked() {
					selected_state.set(!selected_state.get());
				}

				let (text, text_color) = if let Some(n) = *selected_elem {
					(choices[n], theme.text_primary)
				} else {
					(placeholder, theme.text_tertiary)
				};

				let tp = (theme.tp_regular(FONT_SIZE_SM)).color(text_color);
				let label = (ui.text(wk!([key]), text, &tp))
					.center_left()
					.size_wh(FILL, HUG_ROUND)
					.build();
				ui.add_child(select_box, label);

				let icon_color = if select_box.hovered() {
					theme.text_primary
				} else {
					theme.text_tertiary
				};

				let icon = phosphor_icon_bold(ui, wk!([key]), PhosphorIcon::Control, FONT_SIZE_SM, icon_color)
					.anchor_origin(Anchor::CENTER, Anchor { x: 0.5, y: 0.37 })
					.rotation(if selected_state.get() { 0.0 } else { -PI })
					.anim_flags(Af::ROTATION)
					.anim_pos_curve(AnimSodProps::new(8.0, 1.0, 0.0))
					.build();
				ui.add_child(select_box, icon);
			}
			ui.add_child(select_subcontainer, select_box);
		}
		ui.add_child(select_container, select_subcontainer);

		let dropdown_anchor = ui.build_widget(wk!([key])).size_wh(FILL, fixed(0.0)).build();
		{
			if selected_state.get() {
				let dropdown = (ui.build_widget(wk!([key])))
					.tag("Dropdown")
					.clip_content()
					.bottom_left()
					.pos(SPACING_2 * Vec2::NEG_Y)
					.color(theme.form_surface_background)
					.border(
						theme.background_modifier_accent,
						Vec4::splat(1.0),
						Vec4::splat(RADIUS_LG),
					)
					.flex_col(0.0)
					.size_wh(FILL, fixed(300.0))
					.scrollable()
					.build();
				{
					let tp = (theme.tp_regular(FONT_SIZE_SM)).color(theme.text_primary);
					for (i, month) in choices.iter().enumerate() {
						let button_colors = ButtonColors {
							regular: (TRANSPARENT_WHITE, TRANSPARENT_WHITE),
							hovered: (TRANSPARENT_WHITE | 0x40, TRANSPARENT_WHITE),
							pressed: (TRANSPARENT_WHITE | 0x20, TRANSPARENT_WHITE),
						};

						let item = (ui.btn_box(wk!([key], i), button_colors))
							.size_wh(FILL, HUG_ROUND)
							.pad_all(SPACING_2_5)
							.build();
						{
							if item.clicked() {
								*selected_elem = Some(i);
								selected_state.set(false);
							}

							let label = ui.text(wk!([key], i), month, &tp).center_left().size_fill().build();
							ui.add_child(item, label);
						}
						ui.add_child(dropdown, item);
					}
				}
				ui.add_child(dropdown_anchor, dropdown);
			}
		}
		ui.add_child(select_container, dropdown_anchor);
	}
	select_container
}

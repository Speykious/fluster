use uing::{UiContext, WidgetKey, WidgetReaction, glam::{Vec4, vec2}, wk};
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

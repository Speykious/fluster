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

#![allow(unused)]

use colorsys::{ColorTransform, Hsl, Rgb};
use uing::{
	TextProps,
	anime::AnimSodProps,
	frienderer::Color,
	glam::{Vec2, vec2},
	parley::{Alignment, FontWeight},
};

pub const TRANSPARENT_WHITE: Color = 0xffffff00;
pub const TRANSPARENT_BLACK: Color = 0x00000000;

pub const WHITE: Color = 0xffffffff;
pub const BLACK: Color = 0x000000ff;

pub const NATIVE_TITLEBAR_HEIGHT: f32 = 32.0;

pub const RADIUS_SM: f32 = 4.0;
pub const RADIUS_MD: f32 = 6.0;
pub const RADIUS_LG: f32 = 8.0;
pub const RADIUS_XL: f32 = 12.0;
pub const RADIUS_2XL: f32 = 16.0;

pub const MEDIA_BORDER_RADIUS: f32 = 4.0;
pub const FOOTER_ROW_HEIGHT: f32 = 72.0;

pub const INPUT_CONTAINER_PADDING: f32 = 10.0;
pub const INPUT_CONTAINER_MIN_HEIGHT: f32 = FOOTER_ROW_HEIGHT;
pub const INPUT_WRAPPER_PADDING_X: f32 = 8.0;
pub const INPUT_WRAPPER_PADDING_BOTTOM: f32 = 8.0;
pub const TEXTAREA_TOP_BAR_HEIGHT: f32 = 40.0;
pub const TEXTAREA_LINE_HEIGHT: f32 = 22.0;
pub const TEXTAREA_CONTENT_OFFSET: f32 = (USER_AREA_CONTENT_HEIGHT - TEXTAREA_LINE_HEIGHT) / 2.0;

pub const TYPING_INDICATOR_HEIGHT: f32 = 16.0;
pub const TYPING_PILL_HEIGHT: f32 = 20.0;
pub const SLOWMODE_INDICATOR_HEIGHT: f32 = TYPING_PILL_HEIGHT;
pub const SCROLLER_SPACER_HEIGHT: f32 = 28.0;
pub const TYPING_AVATAR_SIZE: f32 = 12.0;
pub const TYPING_INDICATOR_ANIMATION_SIZE: f32 = 20.0;
pub const TYPING_INDICATOR_GAP: f32 = 0.0;
pub const TEXTAREA_SIDE_BUTTON_PADDING: f32 = 5.5;
pub const TYPING_UPLOAD_COLUMN_WIDTH: f32 = USER_AREA_CONTENT_HEIGHT + TEXTAREA_SIDE_BUTTON_PADDING * 2.0;

pub const SPOILER_BORDER_RADIUS: f32 = 6.0;

pub const FONT_SIZE_XS: f32 = 12.0;
pub const FONT_SIZE_SM: f32 = 14.0;
pub const FONT_SIZE_MD: f32 = 16.0;

pub const CUSTOM_EMOJI_SIZE_EMOJI_MULTIPLIER: f32 = 1.375; // X times the font size I guess
pub const CUSTOM_EMOJI_SIZE_JUMBO_EMOJI: f32 = 48.0;
pub const EMOJI_SIZE_MULTIPLIER: f32 = CUSTOM_EMOJI_SIZE_EMOJI_MULTIPLIER;
pub const EMOJI_JUMBO_SIZE: f32 = CUSTOM_EMOJI_SIZE_JUMBO_EMOJI;

pub const FONT_EMOJI: &str =
	"'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol', 'Noto Color Emoji', system-ui, sans-serif";

pub const SPACING_0: f32 = 0.0;
pub const SPACING_1: f32 = 4.0;
pub const SPACING_1_5: f32 = 6.0;
pub const SPACING_2: f32 = 8.0;
pub const SPACING_2_5: f32 = 10.0;
pub const SPACING_3: f32 = 12.0;
pub const SPACING_4: f32 = 16.0;
pub const SPACING_5: f32 = 20.0;
pub const SPACING_6: f32 = 24.0;
pub const SPACING_8: f32 = 32.0;
pub const SPACING_10: f32 = 40.0;
pub const SPACING_12: f32 = 48.0;
pub const SPACING_16: f32 = 64.0;
pub const SPACING_20: f32 = 80.0;
pub const SPACING_24: f32 = 96.0;

pub const LAYOUT_GUILD_LIST_WIDTH: f32 = 72.0;
pub const LAYOUT_SIDEBAR_WIDTH: f32 = 270.0;
pub const LAYOUT_HEADER_HEIGHT: f32 = 56.0;
pub const LAYOUT_USER_AREA_HEIGHT: f32 = INPUT_CONTAINER_MIN_HEIGHT;
pub const LAYOUT_USER_AREA_RESERVED_HEIGHT: f32 = 0.0;
pub const LAYOUT_MOBILE_BOTTOM_NAV_RESERVED_HEIGHT: f32 = 0.0;
pub const USER_AREA_CONTENT_HEIGHT: f32 = 36.0;
pub const USER_AREA_PADDING_Y: f32 = (LAYOUT_USER_AREA_HEIGHT - USER_AREA_CONTENT_HEIGHT) / 2.0;
pub const USER_AREA_PADDING_X: f32 = SPACING_4;
pub const VOICE_CONNECTION_PADDING_Y: f32 = SPACING_2;
pub const FOOTER_ROW_PADDING_Y: f32 = USER_AREA_PADDING_Y;
pub const LAYOUT_HEADER_POPOUT_WIDTH: f32 = LAYOUT_SIDEBAR_WIDTH - (SPACING_4 * 2.0);

pub const LAYOUT_GAP_SM: f32 = SPACING_2;
pub const LAYOUT_GAP_MS: f32 = SPACING_3;
pub const LAYOUT_GAP: f32 = SPACING_4;
pub const LAYOUT_GAP_LG: f32 = SPACING_6;

pub const CONTENT_PADDING: f32 = SPACING_4;
pub const CONTENT_PADDING_SM: f32 = SPACING_3;
pub const CONTENT_PADDING_LG: f32 = SPACING_6;

pub const GUILD_ICON_SIZE: f32 = 44.0;
pub const GUILD_ICON_GAP: f32 = SPACING_2;

pub const MOBILE_BOTTOM_NAV_HEIGHT: f32 = 60.0;

const FLUXER_BRAND_PRIMARY: Hsl11 = hsl(242.0, 0.70, 0.55);
const FLUXER_BRAND_SECONDARY: Hsl11 = hsl(242.0, 0.60, 0.49);
const FLUSTER_BRAND_PRIMARY: Hsl11 = hsl(172.7, 0.28, 0.45);
const FLUSTER_BRAND_SECONDARY: Hsl11 = hsl(172.7, 0.18, 0.39);

pub const FOCUS_PRIMARY: Color = 0x00b0f4ff;

struct GlobalColorsFromTheme {
	pub form_surface_background: Hsl11,
	pub surface_interactive_hover_bg: Hsl11,
	pub surface_interactive_selected_bg: Hsl11,
	pub surface_interactive_selected_color: Hsl11,
	pub scrollbar_thumb_bg: Hsl11,
	pub scrollbar_thumb_bg_hover: Hsl11,
	pub scrollbar_track_bg: Hsl11,
}

fn global_colors_from_theme(
	background_secondary: Hsl11,
	background_tertiary: Hsl11,
	background_textarea: Hsl11,
	background_modifier_hover: Hsl11,
	background_modifier_selected: Hsl11,
	text_primary: Hsl11,
	text_secondary: Hsl11,
	text_tertiary: Hsl11,
) -> GlobalColorsFromTheme {
	GlobalColorsFromTheme {
		form_surface_background: background_tertiary,
		surface_interactive_hover_bg: background_modifier_hover,
		surface_interactive_selected_bg: background_modifier_selected,
		surface_interactive_selected_color: text_primary,
		scrollbar_thumb_bg: hsl_mix(background_textarea, 0.35, text_tertiary, 0.65),
		scrollbar_thumb_bg_hover: hsl_mix(background_textarea, 0.25, text_secondary, 0.75),
		scrollbar_track_bg: hsl_mix1(background_secondary, 0.80, hsl_transparent()),
	}
}

#[derive(Default, Clone)]
pub struct ButtonThemeSub {
	pub fg: Color,
	pub bg: Color,
	pub border: Color,
}

impl ButtonThemeSub {
	fn from_hsl(fg: Hsl11, bg: Hsl11, border: Hsl11) -> Self {
		Self {
			fg: fg.to_color(),
			bg: bg.to_color(),
			border: border.to_color(),
		}
	}
}

fn bts_hsl(fg: Hsl11, bg: Hsl11, border: Hsl11) -> ButtonThemeSub {
	ButtonThemeSub::from_hsl(fg, bg, border)
}

#[derive(Default, Clone)]
pub struct ButtonTheme {
	pub border_width: f32,
	pub border_radius: f32,

	pub idle: ButtonThemeSub,
	pub hover: ButtonThemeSub,
	pub press: ButtonThemeSub,
}

#[derive(Default, Clone)]
pub struct Theme {
	pub background_primary: Color,
	pub background_secondary: Color,
	pub background_secondary_lighter: Color,
	pub background_secondary_alt: Color,
	pub background_tertiary: Color,
	pub background_tertiary_hover: Color,
	pub background_channel_header: Color,
	pub guild_list_foreground: Color,
	pub background_header_secondary: Color,
	pub background_header_primary: Color,
	pub background_textarea: Color,
	pub background_header_primary_hover: Color,
	pub text_tertiary_secondary: Color,
	pub text_tertiary_muted: Color,
	pub text_tertiary: Color,
	pub text_primary_muted: Color,
	pub text_chat_muted: Color,
	pub text_secondary: Color,
	pub text_chat: Color,
	pub text_primary: Color,
	pub panel_control_bg: Color,
	pub panel_control_border: Color,
	pub panel_control_divider: Color,
	pub panel_control_highlight: Color,
	pub background_modifier_hover: Color,
	pub background_modifier_selected: Color,
	pub background_modifier_accent: Color,
	pub background_modifier_accent_focus: Color,
	pub control_button_normal_bg: Color,
	pub control_button_normal_text: Color,
	pub control_button_hover_bg: Color,
	pub control_button_hover_text: Color,
	pub control_button_active_bg: Color,
	pub control_button_active_text: Color,
	pub control_button_danger_text: Color,
	pub control_button_danger_hover_bg: Color,
	pub brand_primary: Color,
	pub brand_secondary: Color,
	pub brand_primary_light: Color,
	pub brand_primary_fill: Color,
	pub status_online: Color,
	pub status_idle: Color,
	pub status_dnd: Color,
	pub status_offline: Color,
	pub status_danger: Color,
	pub status_warning: Color,
	pub text_warning: Color,
	pub plutonium: Color,
	pub plutonium_hover: Color,
	pub plutonium_text: Color,
	pub plutonium_icon: Color,
	pub invite_verified_icon_color: Color,
	pub text_link: Color,
	pub text_on_brand_primary: Color,
	pub text_code: Color,
	pub text_selection: Color,
	pub markup_mention_text: Color,
	pub markup_mention_fill: Color,
	pub markup_mention_border: Color,
	pub markup_jump_link_text: Color,
	pub markup_jump_link_fill: Color,
	pub markup_jump_link_hover_fill: Color,
	pub markup_everyone_text: Color,
	pub markup_everyone_fill: Color,
	pub markup_everyone_border: Color,
	pub markup_here_text: Color,
	pub markup_here_fill: Color,
	pub markup_here_border: Color,
	pub markup_interactive_hover_text: Color,
	pub markup_interactive_hover_fill: Color,
	pub interactive_muted: Color,
	pub interactive_active: Color,
	pub button_primary_fill: Color,
	pub button_primary_active_fill: Color,
	pub button_primary_text: Color,
	pub button_secondary_fill: Color,
	pub button_secondary_active_fill: Color,
	pub button_secondary_text: Color,
	pub button_secondary_active_text: Color,
	pub button_danger_fill: Color,
	pub button_danger_active_fill: Color,
	pub button_danger_text: Color,
	pub button_danger_outline_border_width: f32,
	pub button_danger_outline_border_color: Color,
	pub button_danger_outline_text: Color,
	pub button_danger_outline_active_fill: Color,
	pub button_danger_outline_active_border: Color,
	pub button_ghost_text: Color,
	pub button_inverted_fill: Color,
	pub button_inverted_text: Color,
	pub button_outline_border_width: f32,
	pub button_outline_border_color: Color,
	pub button_outline_text: Color,
	pub button_outline_active_fill: Color,
	pub button_outline_active_border_width: f32,
	pub button_outline_active_border_color: Color,
	pub theme_border: Color,
	pub theme_border_width: f32,
	pub bg_primary: Color,
	pub bg_secondary: Color,
	pub bg_tertiary: Color,
	pub bg_hover: Color,
	pub bg_active: Color,
	pub bg_code: Color,
	pub bg_code_block: Color,
	pub bg_blockquote: Color,
	pub bg_table_header: Color,
	pub bg_table_row_odd: Color,
	pub bg_table_row_even: Color,
	pub border_color: Color,
	pub border_color_hover: Color,
	pub border_color_focus: Color,
	pub accent_primary: Color,
	pub accent_success: Color,
	pub accent_warning: Color,
	pub accent_danger: Color,
	pub accent_info: Color,
	pub accent_purple: Color,
	pub alert_note_color: Color,
	pub alert_tip_color: Color,
	pub alert_important_color: Color,
	pub alert_warning_color: Color,
	pub alert_caution_color: Color,
	pub shadow_sm: (Vec2, f32, Color),
	pub shadow_md: (Vec2, f32, Color),
	pub shadow_lg: (Vec2, f32, Color),
	pub shadow_xl: (Vec2, f32, Color),
	pub transition_fast: AnimSodProps,
	pub transition_normal: AnimSodProps,
	pub transition_slow: AnimSodProps,
	pub spoiler_overlay_color: Color,
	pub spoiler_overlay_hover_color: Color,
	pub scrollbar_thumb_bg: Color,
	pub scrollbar_thumb_bg_hover: Color,
	pub scrollbar_track_bg: Color,
	pub user_area_divider_color: Color,

	pub form_surface_background: Color,
	pub surface_interactive_hover_bg: Color,
	pub surface_interactive_selected_bg: Color,
	pub surface_interactive_selected_color: Color,

	pub button_primary: ButtonTheme,
	pub button_secondary: ButtonTheme,
	pub button_ghost: ButtonTheme,
	pub button_ghost_muted: ButtonTheme,
	pub button_inverted: ButtonTheme,
	pub button_outline: ButtonTheme,
	pub button_link: ButtonTheme,
}

impl Theme {
	pub fn tp_regular(&self, font_size: f32) -> TextProps<'static> {
		TextProps {
			font_stack: "'IBM Plex Sans', sans-serif",
			font_size,
			color: self.text_primary,
			..TextProps::new()
		}
	}
}

pub fn theme_dark() -> Theme {
	let background_primary = hsl(220.0, 0.13, 0.05);
	let background_secondary = hsl(220.0, 0.13, 0.11);
	let background_secondary_lighter = hsl(220.0, 0.13, 0.13);
	let background_secondary_alt = hsl(220.0, 0.13, 0.15);
	let background_tertiary = hsl(220.0, 0.13, 0.18);
	let background_tertiary_hover = hsl_mix1(hsl(220.0, 0.13, 0.18), 0.96, hsl(0.0, 0.0, 1.0));
	let background_channel_header = hsl(220.0, 0.13, 0.16);
	let guild_list_foreground = hsl(220.0, 0.13, 0.17);
	let background_header_secondary = hsl(220.0, 0.13, 0.20);
	let background_header_primary = hsl(220.0, 0.13, 0.20);
	let background_textarea = hsl(220.0, 0.13, 0.23);
	let background_header_primary_hover = hsl(220.0, 0.13, 0.25);
	let text_tertiary_secondary = hsl(220.0, 0.13, 0.52);
	let text_tertiary_muted = hsl(220.0, 0.13, 0.55);
	let text_tertiary = hsl(220.0, 0.13, 0.64);
	let text_primary_muted = hsl(220.0, 0.13, 0.78);
	let text_chat_muted = hsl(220.0, 0.13, 0.78);
	let text_secondary = hsl(220.0, 0.13, 0.89);
	let text_chat = hsl(220.0, 0.13, 0.93);
	let text_primary = hsl(220.0, 0.13, 0.96);
	let panel_control_bg = hsl_mix(background_secondary_alt, 0.80, hsl(220.0, 0.13, 0.02), 0.20);
	let panel_control_border = hsla(220.0, 0.30, 0.65, 0.45);
	let panel_control_divider = hsla(220.0, 0.30, 0.55, 0.35);
	let panel_control_highlight = hsla(0.0, 0.0, 1.00, 0.04);
	let background_modifier_hover = hsla(220.0, 0.13, 1.00, 0.05);
	let background_modifier_selected = hsla(220.0, 0.13, 1.00, 0.1);
	let background_modifier_accent = hsla(216.0, 0.168, 0.302, 1.0);
	let background_modifier_accent_focus = hsla(221.0, 0.15, 0.35, 1.0);
	let control_button_normal_bg = hsl_transparent();
	let control_button_normal_text = text_primary_muted;
	let control_button_hover_bg = hsl(220.0, 0.13, 0.22);
	let control_button_hover_text = text_primary;
	let control_button_active_bg = hsl(220.0, 0.13, 0.24);
	let control_button_active_text = text_primary;
	let control_button_danger_text = hsl(1.0, 0.77, 0.60);
	let control_button_danger_hover_bg = hsl(1.0, 0.77, 0.20);
	let brand_primary = FLUSTER_BRAND_PRIMARY;
	let brand_secondary = FLUSTER_BRAND_SECONDARY;
	let brand_primary_light = hsl(242.0, 1.00, 0.84);
	let brand_primary_fill = hsl(0.0, 0.0, 1.00);
	let text_on_brand_primary = hsl(0.0, 0.0, 0.98);
	let status_online = hsl(142.0, 0.76, 0.40);
	let status_idle = hsl(45.0, 0.93, 0.50);
	let status_dnd = hsl(0.0, 0.84, 0.60);
	let status_offline = hsl(218.0, 0.11, 0.65);
	let status_danger = hsl(1.0, 0.77, 0.55);
	let status_warning = status_idle;
	let text_warning = hsl(45.0, 0.93, 0.55);
	let plutonium = brand_primary;
	let plutonium_hover = brand_secondary;
	let plutonium_text = text_on_brand_primary;
	let plutonium_icon = hsl(38.0, 0.92, 0.50);
	let invite_verified_icon_color = text_on_brand_primary;
	let text_link = hsl(brand_primary.h, 1.00, 0.70);
	let text_code = hsl(340.0, 0.50, 0.90);
	let text_selection = hsla(210.0, 0.90, 0.70, 0.35);
	let markup_mention_text = text_link;
	let markup_mention_fill = hsl_mix1(text_link, 0.20, hsl_transparent());
	let markup_mention_border = hsla(210.0, 1.00, 0.70, 0.3);
	let markup_jump_link_text = text_link;
	let markup_jump_link_fill = hsl_mix1(text_link, 0.12, hsl_transparent());
	let markup_jump_link_hover_fill = hsl_mix1(text_link, 0.20, hsl_transparent());
	let markup_everyone_text = hsl(250.0, 0.80, 0.75);
	let markup_everyone_fill = hsl_mix1(hsl(250.0, 0.80, 0.75), 0.18, hsl_transparent());
	let markup_everyone_border = hsla(250.0, 0.80, 0.75, 0.3);
	let markup_here_text = hsl(45.0, 0.90, 0.70);
	let markup_here_fill = hsl_mix1(hsl(45.0, 0.90, 0.70), 0.18, hsl_transparent());
	let markup_here_border = hsla(45.0, 0.90, 0.70, 0.3);
	let markup_interactive_hover_text = text_link;
	let markup_interactive_hover_fill = hsl_mix1(text_link, 0.30, hsl_transparent());
	let interactive_muted = hsl_mix(hsl(228.0, 0.10, 0.35), 1.00, hsl(245.0, 1.00, 0.80), 0.40);
	let interactive_active = hsl_mix(hsl(0.0, 0.0, 1.00), 1.00, hsl(245.0, 1.00, 0.80), 0.40);
	let button_primary_fill = hsl(139.0, 0.55, 0.44);
	let button_primary_active_fill = hsl(136.0, 0.60, 0.38);
	let button_primary_text = hsl(0.0, 0.0, 1.00);
	let button_secondary_fill = hsla(0.0, 0.0, 1.00, 0.1);
	let button_secondary_active_fill = hsla(0.0, 0.0, 1.00, 0.15);
	let button_secondary_text = hsl(0.0, 0.0, 1.00);
	let button_secondary_active_text = button_secondary_text;
	let button_danger_fill = hsl(359.0, 0.70, 0.54);
	let button_danger_active_fill = hsl(359.0, 0.65, 0.45);
	let button_danger_text = hsl(0.0, 0.0, 1.00);
	let button_danger_outline_border_width = 1.0;
	let button_danger_outline_border_color = hsl(359.0, 0.70, 0.54);
	let button_danger_outline_text = hsl(0.0, 0.0, 1.00);
	let button_danger_outline_active_fill = hsl(359.0, 0.65, 0.48);
	let button_danger_outline_active_border = hsl_transparent();
	let button_ghost_text = hsl(0.0, 0.0, 1.00);
	let button_inverted_fill = hsl(0.0, 0.0, 1.00);
	let button_inverted_text = hsl(0.0, 0.0, 0.0);
	let button_outline_border_width = 1.0;
	let button_outline_border_color = hsla(0.0, 0.0, 1.00, 0.3);
	let button_outline_text = hsl(0.0, 0.0, 1.00);
	let button_outline_active_fill = hsla(0.0, 0.0, 1.00, 0.15);
	let button_outline_active_border_width = 1.0;
	let button_outline_active_border_color = hsla(0.0, 0.0, 1.00, 0.4);
	let theme_border = hsl_transparent();
	let theme_border_width = 0.0;
	let bg_primary = background_primary;
	let bg_secondary = background_secondary;
	let bg_tertiary = background_tertiary;
	let bg_hover = background_modifier_hover;
	let bg_active = background_modifier_selected;
	let bg_code = hsla(220.0, 0.13, 0.15, 0.8);
	let bg_code_block = background_secondary_alt;
	let bg_blockquote = background_secondary_alt;
	let bg_table_header = background_tertiary;
	let bg_table_row_odd = background_primary;
	let bg_table_row_even = background_secondary;
	let border_color = hsla(220.0, 0.13, 0.50, 0.2);
	let border_color_hover = hsla(220.0, 0.13, 0.50, 0.3);
	let border_color_focus = hsla(210.0, 0.90, 0.70, 0.45);
	let accent_primary = brand_primary;
	let accent_success = status_online;
	let accent_warning = status_idle;
	let accent_danger = status_dnd;
	let accent_info = text_link;
	let accent_purple = hsl(270.0, 0.80, 0.65);
	let alert_note_color = hsl(210.0, 1.00, 0.70);
	let alert_tip_color = hsl(142.0, 0.76, 0.45);
	let alert_important_color = hsl(270.0, 0.80, 0.65);
	let alert_warning_color = hsl(45.0, 0.93, 0.55);
	let alert_caution_color = hsl(359.0, 0.75, 0.60);
	let shadow_sm = (vec2(0.0, 1.0), 2.0, rgba(0, 0, 0, 0.1));
	let shadow_md = (vec2(0.0, 2.0), 4.0, rgba(0, 0, 0, 0.15));
	let shadow_lg = (vec2(0.0, 4.0), 8.0, rgba(0, 0, 0, 0.15));
	let shadow_xl = (vec2(0.0, 10.0), 20.0, rgba(0, 0, 0, 0.15));
	let transition_fast = AnimSodProps::new(7.0, 1.0, 0.0);
	let transition_normal = AnimSodProps::new(5.0, 1.0, 0.0);
	let transition_slow = AnimSodProps::new(3.0, 1.0, 0.0);
	let spoiler_overlay_color = rgba(0, 0, 0, 0.2);
	let spoiler_overlay_hover_color = rgba(0, 0, 0, 0.3);
	let scrollbar_thumb_bg = rgba(121, 122, 124, 0.4);
	let scrollbar_thumb_bg_hover = rgba(121, 122, 124, 0.7);
	let scrollbar_track_bg = hsl_transparent();
	let user_area_divider_color = hsl_mix1(background_modifier_hover, 0.70, hsl_transparent());

	let GlobalColorsFromTheme {
		form_surface_background,
		surface_interactive_hover_bg,
		surface_interactive_selected_bg,
		surface_interactive_selected_color,
		scrollbar_thumb_bg,
		scrollbar_thumb_bg_hover,
		scrollbar_track_bg,
	} = global_colors_from_theme(
		background_secondary,
		background_tertiary,
		background_textarea,
		background_modifier_hover,
		background_modifier_selected,
		text_primary,
		text_secondary,
		text_tertiary,
	);

	let button_primary = ButtonTheme {
		border_width: 0.0,
		border_radius: RADIUS_LG,
		idle: bts_hsl(text_primary, brand_primary, brand_primary),
		hover: bts_hsl(text_primary, brand_secondary, brand_secondary),
		press: bts_hsl(text_primary, brand_primary, brand_primary),
	};
	let button_secondary = ButtonTheme {
		border_width: 0.0,
		border_radius: RADIUS_LG,
		idle: bts_hsl(text_secondary, background_tertiary, background_tertiary),
		hover: bts_hsl(text_secondary, background_tertiary_hover, background_tertiary_hover),
		press: bts_hsl(text_secondary, background_tertiary, background_tertiary),
	};
	let button_ghost = ButtonTheme {
		border_width: 0.0,
		border_radius: RADIUS_LG,
		idle: bts_hsl(button_ghost_text, hsl_transparent(), hsl_transparent()),
		hover: bts_hsl(button_ghost_text, background_tertiary_hover, background_tertiary_hover),
		press: bts_hsl(button_ghost_text, background_tertiary, background_tertiary),
	};
	let button_ghost_muted = ButtonTheme {
		border_width: 0.0,
		border_radius: RADIUS_LG,
		idle: bts_hsl(text_tertiary, hsl_transparent(), hsl_transparent()),
		hover: bts_hsl(text_tertiary, background_tertiary_hover, background_tertiary_hover),
		press: bts_hsl(text_tertiary, background_tertiary, background_tertiary),
	};
	let button_inverted = ButtonTheme {
		border_width: 0.0,
		border_radius: RADIUS_LG,
		idle: bts_hsl(button_inverted_text, button_inverted_fill, button_inverted_fill),
		hover: bts_hsl(button_inverted_text, brand_secondary, brand_secondary),
		press: bts_hsl(button_inverted_text, button_inverted_fill, button_inverted_fill),
	};
	let button_outline = ButtonTheme {
		border_width: button_outline_border_width,
		border_radius: RADIUS_LG,
		idle: bts_hsl(button_outline_text, hsl_transparent(), button_outline_border_color),
		hover: bts_hsl(
			button_outline_text,
			button_outline_active_fill,
			button_outline_active_border_color,
		),
		press: bts_hsl(button_outline_text, hsl_transparent(), button_outline_border_color),
	};
	let button_link = ButtonTheme {
		border_width: 0.0,
		border_radius: 0.0,
		idle: bts_hsl(text_link, hsl_transparent(), hsl_transparent()),
		hover: bts_hsl(text_link, hsl_transparent(), hsl_transparent()),
		press: bts_hsl(text_link, hsl_transparent(), hsl_transparent()),
	};

	Theme {
		background_primary: background_primary.to_color(),
		background_secondary: background_secondary.to_color(),
		background_secondary_lighter: background_secondary_lighter.to_color(),
		background_secondary_alt: background_secondary_alt.to_color(),
		background_tertiary: background_tertiary.to_color(),
		background_tertiary_hover: background_tertiary_hover.to_color(),
		background_channel_header: background_channel_header.to_color(),
		guild_list_foreground: guild_list_foreground.to_color(),
		background_header_secondary: background_header_secondary.to_color(),
		background_header_primary: background_header_primary.to_color(),
		background_textarea: background_textarea.to_color(),
		background_header_primary_hover: background_header_primary_hover.to_color(),
		text_tertiary_secondary: text_tertiary_secondary.to_color(),
		text_tertiary_muted: text_tertiary_muted.to_color(),
		text_tertiary: text_tertiary.to_color(),
		text_primary_muted: text_primary_muted.to_color(),
		text_chat_muted: text_chat_muted.to_color(),
		text_secondary: text_secondary.to_color(),
		text_chat: text_chat.to_color(),
		text_primary: text_primary.to_color(),
		panel_control_bg: panel_control_bg.to_color(),
		panel_control_border: panel_control_border.to_color(),
		panel_control_divider: panel_control_divider.to_color(),
		panel_control_highlight: panel_control_highlight.to_color(),
		background_modifier_hover: background_modifier_hover.to_color(),
		background_modifier_selected: background_modifier_selected.to_color(),
		background_modifier_accent: background_modifier_accent.to_color(),
		background_modifier_accent_focus: background_modifier_accent_focus.to_color(),
		control_button_normal_bg: control_button_normal_bg.to_color(),
		control_button_normal_text: control_button_normal_text.to_color(),
		control_button_hover_bg: control_button_hover_bg.to_color(),
		control_button_hover_text: control_button_hover_text.to_color(),
		control_button_active_bg: control_button_active_bg.to_color(),
		control_button_active_text: control_button_active_text.to_color(),
		control_button_danger_text: control_button_danger_text.to_color(),
		control_button_danger_hover_bg: control_button_danger_hover_bg.to_color(),
		brand_primary: brand_primary.to_color(),
		brand_secondary: brand_secondary.to_color(),
		brand_primary_light: brand_primary_light.to_color(),
		brand_primary_fill: brand_primary_fill.to_color(),
		status_online: status_online.to_color(),
		status_idle: status_idle.to_color(),
		status_dnd: status_dnd.to_color(),
		status_offline: status_offline.to_color(),
		status_danger: status_danger.to_color(),
		status_warning: status_warning.to_color(),
		text_warning: text_warning.to_color(),
		plutonium: plutonium.to_color(),
		plutonium_hover: plutonium_hover.to_color(),
		plutonium_text: plutonium_text.to_color(),
		plutonium_icon: plutonium_icon.to_color(),
		invite_verified_icon_color: invite_verified_icon_color.to_color(),
		text_link: text_link.to_color(),
		text_on_brand_primary: text_on_brand_primary.to_color(),
		text_code: text_code.to_color(),
		text_selection: text_selection.to_color(),
		markup_mention_text: markup_mention_text.to_color(),
		markup_mention_fill: markup_mention_fill.to_color(),
		markup_mention_border: markup_mention_border.to_color(),
		markup_jump_link_text: markup_jump_link_text.to_color(),
		markup_jump_link_fill: markup_jump_link_fill.to_color(),
		markup_jump_link_hover_fill: markup_jump_link_hover_fill.to_color(),
		markup_everyone_text: markup_everyone_text.to_color(),
		markup_everyone_fill: markup_everyone_fill.to_color(),
		markup_everyone_border: markup_everyone_border.to_color(),
		markup_here_text: markup_here_text.to_color(),
		markup_here_fill: markup_here_fill.to_color(),
		markup_here_border: markup_here_border.to_color(),
		markup_interactive_hover_text: markup_interactive_hover_text.to_color(),
		markup_interactive_hover_fill: markup_interactive_hover_fill.to_color(),
		interactive_muted: interactive_muted.to_color(),
		interactive_active: interactive_active.to_color(),
		button_primary_fill: button_primary_fill.to_color(),
		button_primary_active_fill: button_primary_active_fill.to_color(),
		button_primary_text: button_primary_text.to_color(),
		button_secondary_fill: button_secondary_fill.to_color(),
		button_secondary_active_fill: button_secondary_active_fill.to_color(),
		button_secondary_text: button_secondary_text.to_color(),
		button_secondary_active_text: button_secondary_active_text.to_color(),
		button_danger_fill: button_danger_fill.to_color(),
		button_danger_active_fill: button_danger_active_fill.to_color(),
		button_danger_text: button_danger_text.to_color(),
		button_danger_outline_border_width,
		button_danger_outline_border_color: button_danger_outline_border_color.to_color(),
		button_danger_outline_text: button_danger_outline_text.to_color(),
		button_danger_outline_active_fill: button_danger_outline_active_fill.to_color(),
		button_danger_outline_active_border: button_danger_outline_active_border.to_color(),
		button_ghost_text: button_ghost_text.to_color(),
		button_inverted_fill: button_inverted_fill.to_color(),
		button_inverted_text: button_inverted_text.to_color(),
		button_outline_border_width,
		button_outline_border_color: button_outline_border_color.to_color(),
		button_outline_text: button_outline_text.to_color(),
		button_outline_active_fill: button_outline_active_fill.to_color(),
		button_outline_active_border_width,
		button_outline_active_border_color: button_outline_active_border_color.to_color(),
		theme_border: theme_border.to_color(),
		theme_border_width,
		bg_primary: bg_primary.to_color(),
		bg_secondary: bg_secondary.to_color(),
		bg_tertiary: bg_tertiary.to_color(),
		bg_hover: bg_hover.to_color(),
		bg_active: bg_active.to_color(),
		bg_code: bg_code.to_color(),
		bg_code_block: bg_code_block.to_color(),
		bg_blockquote: bg_blockquote.to_color(),
		bg_table_header: bg_table_header.to_color(),
		bg_table_row_odd: bg_table_row_odd.to_color(),
		bg_table_row_even: bg_table_row_even.to_color(),
		border_color: border_color.to_color(),
		border_color_hover: border_color_hover.to_color(),
		border_color_focus: border_color_focus.to_color(),
		accent_primary: accent_primary.to_color(),
		accent_success: accent_success.to_color(),
		accent_warning: accent_warning.to_color(),
		accent_danger: accent_danger.to_color(),
		accent_info: accent_info.to_color(),
		accent_purple: accent_purple.to_color(),
		alert_note_color: alert_note_color.to_color(),
		alert_tip_color: alert_tip_color.to_color(),
		alert_important_color: alert_important_color.to_color(),
		alert_warning_color: alert_warning_color.to_color(),
		alert_caution_color: alert_caution_color.to_color(),
		shadow_sm,
		shadow_md,
		shadow_lg,
		shadow_xl,
		transition_fast,
		transition_normal,
		transition_slow,
		spoiler_overlay_color,
		spoiler_overlay_hover_color,
		scrollbar_thumb_bg: scrollbar_thumb_bg.to_color(),
		scrollbar_thumb_bg_hover: scrollbar_thumb_bg_hover.to_color(),
		scrollbar_track_bg: scrollbar_track_bg.to_color(),
		user_area_divider_color: user_area_divider_color.to_color(),

		form_surface_background: form_surface_background.to_color(),
		surface_interactive_hover_bg: surface_interactive_hover_bg.to_color(),
		surface_interactive_selected_bg: surface_interactive_selected_bg.to_color(),
		surface_interactive_selected_color: surface_interactive_selected_color.to_color(),

		button_primary,
		button_secondary,
		button_ghost,
		button_ghost_muted,
		button_inverted,
		button_outline,
		button_link,
	}
}

#[derive(Default, Debug, Clone, Copy)]
struct Hsl11 {
	pub h: f32,
	pub s: f32,
	pub l: f32,
	pub a: f32,
}

impl Hsl11 {
	fn to_color(self) -> Color {
		let hsl = Hsl::new(self.h as f64, self.s as f64 * 100.0, self.l as f64 * 100.0, None);
		let [r, g, b]: [u8; 3] = Rgb::from(hsl).into();
		u32::from_be_bytes([r, g, b, (self.a * 255.0) as u8])
	}
}

/// H[0-260] S[0-1] L[0-1]
const fn hsl(h: f32, s: f32, l: f32) -> Hsl11 {
	Hsl11 { h, s, l, a: 1.0 }
}

/// H[0-260] S[0-1] L[0-1] A[0-1]
const fn hsla(h: f32, s: f32, l: f32, a: f32) -> Hsl11 {
	Hsl11 { h, s, l, a }
}

/// H[0-260] S[0-1] L[0-1]
const fn hsl_tup(c: (f32, f32, f32)) -> Hsl11 {
	hsl(c.0, c.1, c.2)
}

const fn hsl_mix1(chsl_a: Hsl11, ratio_a: f32, chsl_b: Hsl11) -> Hsl11 {
	hsl_mix(chsl_a, ratio_a, chsl_b, 1.0 - ratio_a)
}

const fn hsl_mix(chsl_a: Hsl11, ratio_a: f32, chsl_b: Hsl11, ratio_b: f32) -> Hsl11 {
	let h = chsl_a.h * ratio_a + chsl_b.h * ratio_b;
	let s = chsl_a.s * ratio_a + chsl_b.s * ratio_b;
	let l = chsl_a.l * ratio_a + chsl_b.l * ratio_b;
	let a = chsl_a.a * ratio_a + chsl_b.a * ratio_b;
	hsla(h, s, l, a)
}

const fn hsl_transparent() -> Hsl11 {
	Hsl11 {
		h: 0.0,
		s: 0.0,
		l: 0.0,
		a: 0.0,
	}
}

const fn rgba(r: u8, g: u8, b: u8, a: f32) -> Color {
	u32::from_be_bytes([r, g, b, (a * 255.0) as u8])
}

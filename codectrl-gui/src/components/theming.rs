use crate::{
    consts::{OTF_FONT_MONOSPACE, OTF_FONT_REGULAR},
    data::FontSizes,
};

use egui::{
    epaint::Shadow,
    style::{Selection, WidgetVisuals, Widgets},
    Color32, FontData, FontDefinitions, FontFamily, Stroke, TextStyle, Visuals,
};
use lazy_static::lazy_static;

// colours
pub const CODECTRL_GREEN: Color32 = Color32::from_rgb(66, 184, 156);
pub const CORNER_RADIUS: f32 = 5.0;
pub const DARK_BACKGROUND: Color32 = Color32::from_rgb(39, 39, 39);
pub const DARK_BACKGROUND_DARKER: Color32 = Color32::from_rgb(29, 29, 29);
pub const DARK_BACKGROUND_LIGHT: Color32 = Color32::from_rgb(49, 49, 49);
pub const DARK_BACKGROUND_LIGHTER: Color32 = Color32::from_rgb(69, 69, 69);
pub const DARK_FOREGROUND_COLOUR: Color32 = Color32::from_rgb(200, 200, 200);
pub const DARK_HEADER_FOREGROUND_COLOUR: Color32 = Color32::from_rgb(240, 240, 240);
pub const HOVERED_BACKGROUND: Color32 = Color32::from_rgb(156, 72, 91);
pub const AUTHENTURA_RED: Color32 = Color32::from_rgb(230, 55, 96);

const EXPANSION: f32 = 2.0;

lazy_static! {
    pub static ref DARK_FOREGROUND: Stroke = Stroke::new(1.4, DARK_FOREGROUND_COLOUR);
    pub static ref DARK_STROKE: Stroke = Stroke::new(0.5, Color32::BLACK);
}

pub fn fonts(font_sizes: FontSizes) -> FontDefinitions {
    let mut fonts = FontDefinitions::default();

    fonts
        .font_data
        .insert("regular".into(), FontData::from_static(OTF_FONT_REGULAR));

    fonts.font_data.insert(
        "monospace".into(),
        FontData::from_static(OTF_FONT_MONOSPACE),
    );

    fonts
        .fonts_for_family
        .get_mut(&FontFamily::Proportional)
        .unwrap()
        .insert(0, "regular".into());

    fonts
        .fonts_for_family
        .get_mut(&FontFamily::Monospace)
        .unwrap()
        .insert(0, "monospace".into());

    fonts
        .family_and_size
        .insert(TextStyle::Body, (FontFamily::Proportional, font_sizes.body));

    fonts.family_and_size.insert(
        TextStyle::Button,
        (FontFamily::Proportional, font_sizes.button),
    );

    fonts.family_and_size.insert(
        TextStyle::Heading,
        (FontFamily::Proportional, font_sizes.heading),
    );

    fonts.family_and_size.insert(
        TextStyle::Monospace,
        (FontFamily::Monospace, font_sizes.monospace),
    );

    // this may be confusing so I'll explain: since we don't use the small textstyle
    // anywhere within the gui, we can instead use it as an "extra" style. i.e, we
    // can make it extra large for the main_view_empty function to have a extra
    // large "codeCTRL" that would otherwise be hard to draw.
    fonts.family_and_size.insert(
        TextStyle::Small,
        (FontFamily::Proportional, font_sizes.extra_large),
    );

    fonts
}

pub fn dark_theme() -> Visuals {
    Visuals {
        dark_mode: true,
        override_text_color: Some(DARK_FOREGROUND_COLOUR),
        widgets: Widgets {
            noninteractive: WidgetVisuals {
                bg_fill: DARK_BACKGROUND,
                bg_stroke: *DARK_STROKE,
                corner_radius: CORNER_RADIUS,
                fg_stroke: *DARK_FOREGROUND,
                expansion: EXPANSION,
            },
            inactive: WidgetVisuals {
                bg_fill: DARK_BACKGROUND_LIGHTER,
                bg_stroke: *DARK_STROKE,
                corner_radius: CORNER_RADIUS,
                fg_stroke: *DARK_FOREGROUND,
                expansion: EXPANSION,
            },
            hovered: WidgetVisuals {
                bg_fill: HOVERED_BACKGROUND,
                bg_stroke: *DARK_STROKE,
                corner_radius: CORNER_RADIUS,
                fg_stroke: *DARK_FOREGROUND,
                expansion: EXPANSION,
            },
            active: WidgetVisuals {
                bg_fill: Color32::from_additive_luminance(100),
                bg_stroke: *DARK_STROKE,
                corner_radius: CORNER_RADIUS,
                fg_stroke: *DARK_FOREGROUND,
                expansion: EXPANSION,
            },
            open: WidgetVisuals {
                bg_fill: DARK_BACKGROUND,
                bg_stroke: *DARK_STROKE,
                corner_radius: CORNER_RADIUS,
                fg_stroke: *DARK_FOREGROUND,
                expansion: EXPANSION,
            },
        },
        selection: Selection {
            bg_fill: AUTHENTURA_RED,
            stroke: *DARK_STROKE,
        },
        faint_bg_color: DARK_BACKGROUND_LIGHT,
        extreme_bg_color: DARK_BACKGROUND_DARKER,
        code_bg_color: DARK_BACKGROUND_DARKER,
        window_corner_radius: 10.0,
        window_shadow: Shadow::small_light(),
        popup_shadow: Shadow::small_light(),
        text_cursor_width: 2.0,
        text_cursor_preview: true,
        collapsing_header_frame: true,
        ..Visuals::default()
    }
}

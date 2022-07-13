// Copyright © SixtyFPS GmbH <info@slint-ui.com>
// SPDX-License-Identifier: GPL-3.0-only OR LicenseRef-Slint-commercial

use i_slint_core::{graphics::FontRequest, Coord};

pub const DEFAULT_FONT_SIZE: f32 = 12.;

pub fn create_layout(
    font_request: FontRequest,
    scale_factor: f32,
    text: &str,
    text_style: Option<skia_safe::textlayout::TextStyle>,
    max_width: Option<Coord>,
) -> skia_safe::textlayout::Paragraph {
    // TODO: don't create the font collection, etc. every time
    let mut font_collection = skia_safe::textlayout::FontCollection::new();
    font_collection.set_default_font_manager(skia_safe::FontMgr::new(), None);

    let mut text_style = text_style.unwrap_or_default();

    if let Some(family_name) = font_request.family {
        text_style.set_font_families(&[family_name.as_str()]);
    }

    let pixel_size = font_request.pixel_size.unwrap_or(DEFAULT_FONT_SIZE) * scale_factor;

    // TODO: add more font properties
    text_style.set_font_size(pixel_size);

    let style = skia_safe::textlayout::ParagraphStyle::new();
    let mut builder = skia_safe::textlayout::ParagraphBuilder::new(&style, font_collection);
    builder.push_style(&text_style);
    builder.add_text(text);
    let mut paragraph = builder.build();
    paragraph.layout(max_width.unwrap_or(core::f32::MAX));
    paragraph
}
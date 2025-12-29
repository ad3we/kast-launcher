use crate::constants;
use eframe::{
        egui,
        epaint::text::{FontInsert, InsertFontFamily},
};
use std::{fs, path::Path, sync::Arc};

pub fn add_font(ctx: &egui::Context, user_font_path: String) {
        // user font or fallback
        let user_font_data = if !user_font_path.is_empty() {
                let path = Path::new(&user_font_path);
                match fs::read(path) {
                        Ok(bytes) => egui::FontData::from_owned(bytes),
                        Err(_) => egui::FontData::from_static(constants::FALLBACK_FONT_BYTES),
                }
        } else {
                egui::FontData::from_static(constants::FALLBACK_FONT_BYTES)
        };

        // static font used for monospace
        let fallback_font_data = egui::FontData::from_static(constants::FALLBACK_FONT_BYTES);

        ctx.add_font(FontInsert::new(
                "user_font",
                user_font_data,
                vec![
                        InsertFontFamily {
                                family: egui::FontFamily::Proportional,
                                priority: egui::epaint::text::FontPriority::Highest,
                        },
                        InsertFontFamily {
                                family: egui::FontFamily::Monospace,
                                priority: egui::epaint::text::FontPriority::Lowest, // still added but will override later
                        },
                ],
        ));

        ctx.add_font(FontInsert::new(
                "fallback_font",
                fallback_font_data,
                vec![InsertFontFamily {
                        family: egui::FontFamily::Monospace,
                        priority: egui::epaint::text::FontPriority::Highest, // ensure monospace uses fallback
                }],
        ));
}

pub fn replace_fonts(ctx: &egui::Context, user_font_path: String) {
        let user_font_data = if !user_font_path.is_empty() {
                let path = Path::new(&user_font_path);
                match fs::read(path) {
                        Ok(bytes) => egui::FontData::from_owned(bytes),
                        Err(_) => egui::FontData::from_static(constants::FALLBACK_FONT_BYTES),
                }
        } else {
                egui::FontData::from_static(constants::FALLBACK_FONT_BYTES)
        };

        let fallback_font_data = egui::FontData::from_static(constants::FALLBACK_FONT_BYTES);

        let mut fonts = egui::FontDefinitions::default();

        // insert fonts
        fonts
                .font_data
                .insert("user_font".to_owned(), Arc::new(user_font_data));
        fonts
                .font_data
                .insert("fallback_font".to_owned(), Arc::new(fallback_font_data));

        // set other families to user font
        fonts
                .families
                .entry(egui::FontFamily::Proportional)
                .or_default()
                .insert(0, "user_font".to_owned());

        // monospace uses only fallback font (monospace family will be used exclusively for icons)
        fonts
                .families
                .entry(egui::FontFamily::Monospace)
                .or_default()
                .insert(0, "fallback_font".to_owned());

        ctx.set_fonts(fonts);
}

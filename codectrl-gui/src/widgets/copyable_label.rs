use crate::TOASTS;

use egui::{RichText, WidgetText};
use egui_toast::ToastOptions;
use std::{
    borrow::Cow,
    time::{Duration, Instant},
};

pub struct CopyableLabel {
    data: WidgetText,
}

impl CopyableLabel {
    pub fn new<T: Into<WidgetText> + Clone>(data: T) -> Self {
        Self { data: data.into() }
    }

    pub fn new_monospace<'a>(data: impl Into<Cow<'a, str>>) -> Self {
        let rt = RichText::new(data.into()).monospace();

        Self { data: rt.into() }
    }
}

impl egui::Widget for CopyableLabel {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let binding = unsafe { &mut TOASTS };
        let toasts = binding.get_mut();

        let widget_text: WidgetText = self.data.clone().into();
        let widget_text = widget_text.text();

        let mut response =
            ui.add(egui::Label::new(self.data).sense(egui::Sense::click()));

        response |= response.clone().on_hover_ui_at_pointer(|ui| {
            ui.label("Click to copy");
        });

        if response.clicked() {
            ui.ctx().output().copied_text = widget_text.to_owned();
            dbg!(&ui.ctx().output().copied_text);

            if let Some(toasts) = toasts {
                toasts.get_mut().info(
                    format!("Copied to clipboard: \"{widget_text}\""),
                    ToastOptions {
                        show_icon: true,
                        expires_at: Some(Instant::now() + Duration::from_secs(4)),
                    },
                );
            }
        }

        response
    }
}

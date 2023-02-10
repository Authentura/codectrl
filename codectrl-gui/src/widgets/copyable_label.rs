#[cfg(not(target_arch = "wasm32"))]
use crate::TOASTS;

use egui::{RichText, WidgetText};
#[cfg(not(target_arch = "wasm32"))]
use egui_toast::ToastOptions;
use std::borrow::Cow;
#[cfg(not(target_arch = "wasm32"))]
use std::time::{Duration, Instant};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

pub struct CopyableLabel {
    data: WidgetText,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
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
        #[cfg(not(target_arch = "wasm32"))]
        let binding = unsafe { &mut TOASTS };
        #[cfg(not(target_arch = "wasm32"))]
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

            #[cfg(not(target_arch = "wasm32"))]
            if let Some(toasts) = toasts {
                toasts.get_mut().info(
                    format!("Copied to clipboard: \"{widget_text}\""),
                    ToastOptions {
                        show_icon: true,
                        expires_at: Some(Instant::now() + Duration::from_secs(4)),
                    },
                );
            }

            #[cfg(target_arch = "wasm32")]
            alert(&format!("Copied to clipboard: \"{widget_text}\""));
        }

        response
    }
}

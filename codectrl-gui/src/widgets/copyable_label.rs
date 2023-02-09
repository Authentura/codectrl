use egui::{RichText, WidgetText};

pub struct CopyableLabel {
    data: WidgetText,
}

impl CopyableLabel {
    pub fn new<T: Into<WidgetText> + Clone>(data: T) -> Self {
        Self { data: data.into() }
    }

    pub fn new_monospace(data: &str) -> Self {
        let rt = RichText::new(data).monospace();

        Self { data: rt.into() }
    }
}

impl egui::Widget for CopyableLabel {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let widget_text: WidgetText = self.data.clone().into();
        let widget_text = widget_text.text();

        let response = ui.label(self.data).on_hover_ui_at_pointer(|ui| {
            ui.label("Click to copy");
        });

        if response.clicked() {
            ui.ctx().output().copied_text = widget_text.to_owned();
        }

        response
    }
}

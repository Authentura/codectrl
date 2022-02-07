use egui::{CtxRef, Id};

pub fn message_preview_view(
    is_open: &mut bool,
    ctx: &CtxRef,
    message: &str,
    message_type: &str,
) {
    egui::Window::new("Message Preview")
        .id(Id::new("message_preview_view"))
        .resizable(true)
        .collapsible(false)
        .title_bar(true)
        .enabled(true)
        .default_size((300.0, 400.0))
        .min_width(300.0)
        .min_height(400.0)
        .open(is_open)
        .show(ctx, |ui| {
            let mut message = message.to_string();

            ui.horizontal(|ui| ui.label(format!("Message type: {}", message_type)));

            egui::ScrollArea::vertical()
                .max_height(ui.available_height())
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    ui.add(
                        egui::TextEdit::multiline(&mut message)
                            .code_editor()
                            .desired_width(ui.available_width()),
                    );
                });
        });
}

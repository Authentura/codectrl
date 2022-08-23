use crate::wrapper::WrapperMsg;

use authentura_egui_styling::{application_style, fonts, FontSizes};
use e::{Color32, Vec2};
use eframe::{App, Frame};
use egui::{self as e, Context};
use std::{cell::RefCell, sync::Arc};

#[derive(Default)]
pub struct Login<'a> {
    token: String,
    wrapper_msg: Arc<RefCell<WrapperMsg<'a>>>,
    host: String,
    port: String,
    is_local: bool,
}

impl<'a> Login<'a> {
    pub fn new(ctx: &Context, wrapper_msg: Arc<RefCell<WrapperMsg<'a>>>) -> Self {
        ctx.set_fonts(fonts());
        ctx.set_style(application_style(FontSizes::default()));

        Self {
            token: String::new(),
            wrapper_msg,
            host: String::from("127.0.0.1"),
            port: String::from("3002"),
            is_local: true,
        }
    }
}

impl<'a> App for Login<'a> {
    fn update(&mut self, ctx: &e::Context, frame: &mut Frame) {
        #[cfg(not(target_arch = "wasm32"))]
        egui::TopBottomPanel::top("top_bar")
            .resizable(false)
            .default_height(200.0)
            .show(ctx, |ui| {
                ui.add_space(4.0);

                ui.menu_button("File", |ui| {
                    ui.horizontal_wrapped(|ui| {
                        if ui.button("Quit").clicked() {
                            frame.quit();
                        }
                    })
                });
            });

        e::CentralPanel::default().show(ctx, |ui| {
            e::Grid::new("login_form")
                .min_col_width(ui.available_width() / 2.0)
                .spacing(Vec2::new(10.0, 10.0))
                .show(ui, |ui| {
                    ui.checkbox(&mut self.is_local, "Is local?");
                    ui.end_row();

                    if !self.is_local {
                        let responsive_row =
                            |ui: &mut e::Ui, text: &str, data: &mut String| {
                                ui.colored_label(
                                    if data.is_empty() {
                                        Color32::LIGHT_RED
                                    } else {
                                        ctx.style().visuals.text_color()
                                    },
                                    text,
                                );

                                ui.text_edit_singleline(data);
                                ui.end_row();
                            };

                        responsive_row(ui, "Token", &mut self.token);

                        responsive_row(ui, "Host", &mut self.host);

                        responsive_row(ui, "Port", &mut self.port);
                    }
                });

            if ui.button("Click me!").clicked() {
                // *self.wrapper_msg.borrow_mut() = WrapperMsg::Main {};
            }
        });
    }
}

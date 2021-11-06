use crate::common::{Received, Receiver};
use chrono::Local;
use egui::CtxRef;
use epi::{Frame, Storage};
use serde::{Deserialize, Serialize};
use std::{
    collections::VecDeque,
    sync::{mpsc::Receiver as Rx, Arc, Mutex, RwLock},
    thread::{Builder as ThreadBuilder, JoinHandle},
};

#[derive(Debug, Default, Deserialize, Serialize)]
struct GuiAppState {}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct GuiApp {
    #[serde(skip)]
    received: Received,
    #[serde(skip)]
    pub receiver: Receiver,
    #[serde(skip)]
    update_thread: Option<JoinHandle<()>>,
    data: GuiAppState,
    title: String,
    socket_address: String,
}

impl GuiApp {
    pub fn new(title: &str, receiver: Rx<String>, socket_address: String) -> Self {
        Self {
            received: Arc::new(RwLock::new(VecDeque::new())),
            receiver: Some(Arc::new(Mutex::new(receiver))),
            update_thread: None,
            data: GuiAppState::default(),
            title: title.into(),
            socket_address,
        }
    }
}

impl epi::App for GuiApp {
    fn update(&mut self, ctx: &CtxRef, _frame: &mut Frame<'_>) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(format!("Listening on: {}", self.socket_address));
        });

        egui::TopBottomPanel::bottom("")
            .resizable(true)
            .min_height(200.0)
            .default_height(250.0)
            .max_height(350.0)
            .show(ctx, |ui| {
                ui.with_layout(egui::Layout::left_to_right(), |ui| {
                    ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
                        ui.heading("Test");
                    });

                    egui::ScrollArea::vertical()
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            ui.with_layout(
                                egui::Layout::top_down(egui::Align::Min),
                                |ui| {
                                    for received in self.received.read().unwrap().iter() {
                                        ui.horizontal_wrapped(|ui| {
                                            ui.add(
                                                egui::Label::new(format!(
                                                    "Received ({}):",
                                                    &(received.1).to_rfc2822()
                                                ))
                                                .strong(),
                                            );
                                            ui.label(&received.0);
                                        });
                                    }
                                },
                            );
                        });
                });
            });
    }

    fn setup(
        &mut self,
        _ctx: &CtxRef,
        frame: &mut Frame<'_>,
        _storage: Option<&dyn Storage>,
    ) {
        let rx = Arc::clone(self.receiver.as_ref().unwrap());
        let received = Arc::clone(&self.received);
        let ctx = Arc::clone(&frame.repaint_signal());

        self.update_thread = Some(unsafe {
            ThreadBuilder::new()
                .name(format!("{}_update_thread", self.title))
                .spawn_unchecked(move || loop {
                    let recd = rx.lock().unwrap().recv();

                    if let Ok(recd) = recd {
                        received.write().unwrap().push_front((recd, Local::now()));
                        ctx.request_repaint();
                    }
                })
                .expect("Could not start codeCTRL update thread")
        });
    }

    fn name(&self) -> &str { self.title.as_str() }
}

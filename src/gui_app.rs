use egui::CtxRef;
use epi::{Frame, Storage};
use serde::{Deserialize, Serialize};
use std::{
    collections::VecDeque,
    sync::{mpsc::Receiver, Arc, Mutex, RwLock},
    thread::{Builder as ThreadBuilder, JoinHandle},
};

#[derive(Debug, Default, Deserialize, Serialize)]
struct GuiAppState {}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct GuiApp {
    #[serde(skip)]
    received: Arc<RwLock<VecDeque<String>>>,
    #[serde(skip)]
    pub receiver: Option<Arc<Mutex<Receiver<String>>>>,
    #[serde(skip)]
    update_thread: Option<JoinHandle<()>>,
    data: GuiAppState,
    title: String,
}

impl GuiApp {
    pub fn new(title: &str, receiver: Receiver<String>) -> Self {
        Self {
            received: Arc::new(RwLock::new(VecDeque::new())),
            receiver: Some(Arc::new(Mutex::new(receiver))),
            update_thread: None,
            data: GuiAppState::default(),
            title: title.into(),
        }
    }
}

impl epi::App for GuiApp {
    fn update(&mut self, ctx: &CtxRef, _frame: &mut Frame<'_>) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello");
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
                                        ui.label(format!("Received: {}", received));
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
                .name("codeCTRL_update_thread".into())
                .spawn_unchecked(move || loop {
                    let recd = rx.lock().unwrap().recv();

                    if let Ok(recd) = recd {
                        received.write().unwrap().push_front(recd);
                        ctx.request_repaint();
                    }
                })
                .expect("Could not start codeCTRL update thread")
        });

        // if let Some(storage) = storage {
        //     *self = epi::get_value(storage,
        // epi::APP_KEY).unwrap_or_default(); }
    }

    // fn save(&mut self, storage: &mut dyn Storage) {
    //     epi::set_value(storage, epi::APP_KEY, self);
    // }

    fn name(&self) -> &str { self.title.as_str() }
}

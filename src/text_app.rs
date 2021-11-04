use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
    },
};
use std::{
    collections::VecDeque,
    error::Error,
    io::{self, Stdout},
    sync::{mpsc::Receiver, Arc, Mutex, RwLock},
    thread::{Builder as ThreadBuilder, JoinHandle},
    time::{Duration, Instant},
};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

// pub struct TextAppState {}

pub struct TextApp {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    tick_rate: Duration,
    title: String,

    received: Arc<RwLock<VecDeque<String>>>,
    pub receiver: Option<Arc<Mutex<Receiver<String>>>>,
    update_thread: Option<JoinHandle<()>>,
    socket_address: String,
}

impl TextApp {
    pub fn new(
        title: &str,
        receiver: Receiver<String>,
        socket_address: String,
    ) -> Result<Self, Box<dyn Error>> {
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;

        Ok(Self {
            terminal,
            tick_rate: Duration::from_millis(250),
            title: title.into(),
            received: Arc::new(RwLock::new(VecDeque::new())),
            receiver: Some(Arc::new(Mutex::new(receiver))),
            update_thread: None,
            socket_address,
        })
    }

    fn start_update_thread(&mut self) {
        let rx = Arc::clone(self.receiver.as_ref().unwrap());
        let received = Arc::clone(&self.received);

        self.update_thread = Some(unsafe {
            ThreadBuilder::new()
                .name(format!("{}_update_thread", self.title))
                .spawn_unchecked(move || loop {
                    let recd = rx.lock().unwrap().recv();

                    if let Ok(recd) = recd {
                        received.write().unwrap().push_front(recd);
                    }
                })
                .unwrap_or_else(|_| {
                    panic!("Could not start {} update thread", self.title)
                })
        });
    }

    pub fn draw(&mut self) -> Result<(), Box<dyn Error>> {
        self.start_update_thread();

        enable_raw_mode()?;

        self.ui()?;

        disable_raw_mode()?;

        execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture,
        )?;

        self.terminal.show_cursor()?;

        Ok(())
    }

    fn ui(&mut self) -> io::Result<()> {
        let mut last_tick = Instant::now();
        let mut should_quit = false;

        loop {
            self.terminal.draw(|f| {
                let chunks = Layout::default()
                    .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
                    .split(f.size());

                let block = Block::default()
                    .title("Logs")
                    .borders(Borders::TOP)
                    .border_style(Style::default().fg(Color::White));

                let logs = self
                    .received
                    .read()
                    .unwrap()
                    .iter()
                    .map(|x| {
                        Spans::from(vec![
                            Span::styled(
                                "Received: ",
                                Style::default().add_modifier(Modifier::BOLD),
                            ),
                            Span::from(x.clone()),
                        ])
                    })
                    .collect::<Vec<Spans>>();

                let socket_paragraph = Paragraph::new(Spans::from(vec![
                    Span::styled(
                        "Listening on: ",
                        Style::default().add_modifier(Modifier::BOLD),
                    ),
                    Span::from(self.socket_address.clone()),
                ]));
                let paragraph = Paragraph::new(logs).block(block);

                f.render_widget(socket_paragraph, chunks[0]);
                f.render_widget(paragraph, chunks[1]);
            })?;

            let timeout = self
                .tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if crossterm::event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    if key.code == KeyCode::Esc {
                        should_quit = true;
                    }
                }
            }

            if last_tick.elapsed() >= self.tick_rate {
                last_tick = Instant::now();
            }

            if should_quit {
                return Ok(());
            }
        }
    }
}

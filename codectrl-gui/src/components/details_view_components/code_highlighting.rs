use egui::TextFormat;
use std::collections::HashMap;
use tree_sitter_highlight::{HighlightConfiguration, HighlightEvent, Highlighter};

const COMMENTS: egui::Color32 = egui::Color32::from_rgb(121, 121, 121);
const WHITE: egui::Color32 = egui::Color32::from_rgb(214, 214, 214);
const YELLOW: egui::Color32 = egui::Color32::from_rgb(255, 216, 102);
const GREEN: egui::Color32 = egui::Color32::from_rgb(169, 220, 118);
const ORANGE: egui::Color32 = egui::Color32::from_rgb(255, 97, 136);
const PURPLE: egui::Color32 = egui::Color32::from_rgb(158, 134, 200);
const PINK: egui::Color32 = egui::Color32::from_rgb(176, 82, 121);
const BLUE: egui::Color32 = egui::Color32::from_rgb(120, 220, 232);

fn create_config(lang: &str) -> Option<HighlightConfiguration> {
    match lang {
        "python" => {
            return Some(
                HighlightConfiguration::new(
                    tree_sitter_python::language(),
                    tree_sitter_python::HIGHLIGHT_QUERY,
                    "",
                    "",
                )
                .unwrap(),
            );
        },

        "rust" => {
            return Some(
                HighlightConfiguration::new(
                    tree_sitter_rust::language(),
                    tree_sitter_rust::HIGHLIGHT_QUERY,
                    "",
                    "",
                )
                .unwrap(),
            );
        },

        _ => return None,
    }
}

pub fn code_highlighter(code: &str, lang: &str) -> egui::text::LayoutJob {
    let highlight_data = HashMap::from([
        ("keyword", ORANGE),
        ("operator", ORANGE),
        ("escape", ORANGE),
        ("function.builtin", BLUE),
        ("function", BLUE),
        ("constant", PURPLE),
        ("variable", WHITE),
        ("number", PURPLE),
        ("string", YELLOW),
        ("comment", COMMENTS),
        ("type", GREEN),
        ("variable.builtin", ORANGE),
        ("punctuation.delimiter", ORANGE),
        ("function.method", BLUE),
        ("none", PINK),
    ]);
    let highlight_names = highlight_data.keys().collect::<Vec<_>>();

    let mut config = create_config(&lang[..]).unwrap();
    config.configure(&highlight_names);
    let mut highlighter = Highlighter::new();
    let highlights = highlighter
        .highlight(&config, code.as_bytes(), None, |_| None)
        .unwrap();

    let mut job = egui::text::LayoutJob::default();
    let mut current = egui::Color32::TEMPORARY_COLOR;

    for event in highlights {
        match event.unwrap() {
            HighlightEvent::Source { start, end } => {
                eprintln!("source: {}", &code[start..end]);
                job.append(
                    &code[start..end],
                    0.0,
                    TextFormat::simple(egui::TextStyle::Monospace, current),
                );
            },
            HighlightEvent::HighlightStart(s) => {
                eprintln!("highlight style started: {:?}", highlight_names[s.0]);
                current = highlight_data[highlight_names[s.0]];
            },
            HighlightEvent::HighlightEnd => {
                eprintln!("highlight style ended");
                current = WHITE;
            },
        }
    }
    job
}

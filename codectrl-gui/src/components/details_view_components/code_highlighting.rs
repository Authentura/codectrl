use egui::{CtxRef, RichText, TextStyle, Ui, TextFormat};
use tree_sitter_highlight::{
    Highlighter,
    HighlightConfiguration,
    HighlightEvent
};

// Background: (46, 46, 46); #2e2e2e
// Comments: (121, 121, 121); #797979
// White: (214, 214, 214); #d6d6d6
// Yellow: (229, 181, 103); #e5b567
// Green: (180, 210, 115); #b4d273
// Orange: (232, 125, 62); #e87d3e
// Purple: (158, 134, 200); #9e86c8
// Pink: (176, 82, 121); #b05279
// Blue: (108, 153, 187); #6c99bb

const bg = egui::Color32::from_rgb(46, 46, 46);
const comments = egui::Color32::from_rgb(121, 121, 121);
const white = egui::Color32::from_rgb(214, 214, 214);
const yellow = egui::Color32::from_rgb(180, 210, 115);
const green = egui::Color32::from_rgb(229, 181, 103);
const orange = egui::Color32::from_rgb(232, 125, 62);
const purple = egui::Color32::from_rgb(158, 134, 200);
const pink = egui::Color32::from_rgb(176, 82, 121);
const blue = egui::Color32::from_rgb(108, 153, 187);


pub fn code_highlighter(s: &str) -> egui::text::LayoutJob {

    // Remove Line number and things

    let mut code : String = "".to_string();
    for l in s.split('\n'){
        code.push_str(l[3..].trim());
        code.push('\n');
    }


    let highlight_names = &[
        "keyword",
        "function",
        "variable",
        "operator",
        "attribute",
        "constant",
        "property",
        "punctuation",
        "punctuation.bracket",
        "punctuation.delimiter",
        "string",
        "string.special",
        "tag",
        "number",
        "type",
        "function.builtin",
        "type.builtin",
        "variable.builtin",
        "variable.parameter",
    ];

    let mut javascript_config = HighlightConfiguration::new(
        tree_sitter_python::language(),
        tree_sitter_python::HIGHLIGHT_QUERY,
        tree_sitter_python::TAGGING_QUERY,
        ""
    ).unwrap();

    javascript_config.configure(highlight_names);


    let mut highlighter = Highlighter::new();

    let highlights = highlighter.highlight(
        &javascript_config,
        code.as_bytes(),
        None,
        |_| None
    ).unwrap();
    
    let color = [
        egui::Color32::from_rgb(248, 85, 82),
        egui::Color32::from_rgb(141, 161, 1),
        egui::Color32::from_rgb(230, 152, 117),
        egui::Color32::from_rgb(230, 152, 117),
        egui::Color32::from_rgb(141, 161, 1),
        egui::Color32::from_rgb(122, 132, 120),
    ];

    let mut job = egui::text::LayoutJob::default();
    let mut current = 0;

    for event in highlights {
        match event.unwrap() {
            HighlightEvent::Source {start, end} => {
                eprintln!("source: {}-{}", start, end);
                job.append(
                    &code[start..end],
                    0.0,
                    TextFormat::simple(egui::TextStyle::Monospace,color[current])
                )
            },
            HighlightEvent::HighlightStart(s) => {
                eprintln!("highlight style started: {:?}", highlight_names[s.0]);
                if s.0 < 4{
                    current = s.0
                }

            },
            HighlightEvent::HighlightEnd => {
                eprintln!("highlight style ended");
                current = 5
            },
        }
    }
    
    job
}
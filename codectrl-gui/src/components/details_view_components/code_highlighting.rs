use egui::{TextFormat};
use tree_sitter_highlight::{
    Highlighter,
    HighlightConfiguration,
    HighlightEvent
};
use std::collections::HashMap;
// Background: (46, 46, 46); #2e2e2e
// Comments: (121, 121, 121); #797979
// White: (214, 214, 214); #d6d6d6
// Yellow: (229, 181, 103); #e5b567
// Green: (180, 210, 115); #b4d273
// Orange: (232, 125, 62); #e87d3e
// Purple: (158, 134, 200); #9e86c8
// Pink: (176, 82, 121); #b05279
// Blue: (108, 153, 187); #6c99bb

const BG : egui::Color32 = egui::Color32::from_rgb(46, 46, 46);
const COMMENTS : egui::Color32 = egui::Color32::from_rgb(121, 121, 121);
const WHITE : egui::Color32 = egui::Color32::from_rgb(214, 214, 214);
const YELLOW : egui::Color32 = egui::Color32::from_rgb(255,216,102);
const GREEN : egui::Color32 = egui::Color32::from_rgb(169,220,118);
const ORANGE : egui::Color32 = egui::Color32::from_rgb(255,97,136);
const PURPLE : egui::Color32 = egui::Color32::from_rgb(158, 134, 200);
const PINK : egui::Color32 = egui::Color32::from_rgb(176, 82, 121);
const BLUE : egui::Color32 = egui::Color32::from_rgb(120,220,232);


pub fn code_highlighter(s: &str) -> egui::text::LayoutJob {

    // Remove Line number and things

    let mut code : String = "".to_string();
    for l in s.split('\n'){
        code.push_str(l[3..].trim());
        code.push('\n');
    }

    let highlight_data = HashMap::from([
        ("keyword",ORANGE),
        ("operator",ORANGE),
        ("escape",ORANGE),

        ("function.builtin",BLUE),
        ("function",BLUE),
        ("constant",PURPLE),

        ("variable",WHITE),
        ("number",PURPLE),
        ("string",YELLOW),

        ("comment", COMMENTS),
    ]);
    
    let highlight_names = Vec::from_iter(highlight_data.keys());
    // let highlight_names = &[
    //     "keyword",
    //     "function",
    //     "variable",
    //     "operator",
    //     "attribute",
    //     "constant",
    //     "property",
    //     "punctuation",
    //     "punctuation.bracket",
    //     "punctuation.delimiter",
    //     "string",
    //     "string.special",
    //     "tag",
    //     "number",
    //     "type",
    //     "function.builtin",
    //     "type.builtin",
    //     "variable.builtin",
    //     "variable.parameter",
    // ];
    
    // code = std::fs::read_to_string("codectrl-gui/src/components/details_view_components/syntax-check.py").expect("it doesnt work");

    let mut javascript_config = HighlightConfiguration::new(
        tree_sitter_python::language(),
        tree_sitter_python::HIGHLIGHT_QUERY,
        tree_sitter_python::TAGGING_QUERY,
        ""
    ).unwrap();

    javascript_config.configure(&highlight_names);

    let mut highlighter = Highlighter::new();

    let highlights = highlighter.highlight(
        &javascript_config,
        code.as_bytes(),
        None,
        |_| None
    ).unwrap();
    

    let mut job = egui::text::LayoutJob::default();
    let mut current = egui::Color32::TEMPORARY_COLOR;

    for event in highlights {
        match event.unwrap() {
            HighlightEvent::Source {start, end} => {
                eprintln!("source: {}", &code[start..end]);
                job.append(
                    &code[start..end],
                    0.0,
                    TextFormat::simple(egui::TextStyle::Monospace,current)
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
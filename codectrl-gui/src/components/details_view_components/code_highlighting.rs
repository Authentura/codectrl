use egui::TextFormat;
use syntect::{
    easy::HighlightLines,
    highlighting::{Style, ThemeSet},
    parsing::SyntaxSet,
    util::LinesWithEndings,
};

fn lang_to_short(lang: &str) -> &str {
    // Add more language here
    match lang {
        "rust" => "rs",
        "python" => "py",
        _ => "",
    }
}

pub fn code_highlighter(code: &str, lang: &str) -> egui::text::LayoutJob {
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let syntax = ps.find_syntax_by_extension(lang_to_short(lang)).unwrap();
    let mut h = HighlightLines::new(syntax, &ts.themes["base16-mocha.dark"]);
    let mut job = egui::text::LayoutJob::default();
    for line in LinesWithEndings::from(code) {
        let ranges: Vec<(Style, &str)> = h.highlight(line, &ps);
        for h in ranges {
            let (style, code) = h;
            job.append(
                code,
                0.0,
                TextFormat::simple(
                    egui::TextStyle::Monospace,
                    egui::Color32::from_rgb(
                        style.foreground.r,
                        style.foreground.g,
                        style.foreground.b,
                    ),
                ),
            );
        }
    }
    job
}
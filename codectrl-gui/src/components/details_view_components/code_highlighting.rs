use codectrl_protobuf_bindings::data::Log;
use egui::{text::LayoutJob, Color32, Context, FontSelection, TextFormat, TextStyle};
use std::path::Path;
use syntect::{
    easy::HighlightLines,
    highlighting::{Style, ThemeSet},
    parsing::SyntaxSet,
    util::LinesWithEndings,
};

pub fn code_highlighter(code: &str, log: &Log, ctx: &Context) -> LayoutJob {
    let syntax_set = SyntaxSet::load_defaults_newlines();
    let theme_set = ThemeSet::load_defaults();
    let font_id = FontSelection::Style(TextStyle::Monospace).resolve(&ctx.style());

    let syntax = if let Some(syntax) = syntax_set.find_syntax_by_name(&log.language) {
        syntax
    } else if let Ok(Some(syntax)) = syntax_set
        .find_syntax_for_file(Path::new(&log.file_name).extension().unwrap_or_default())
    {
        syntax
    } else {
        syntax_set.find_syntax_plain_text()
    };

    let mut highlight =
        HighlightLines::new(syntax, &theme_set.themes["Solarized (dark)"]);
    let mut job = LayoutJob::default();

    for line in LinesWithEndings::from(code) {
        let ranges: Vec<(Style, &str)> = highlight.highlight(line, &syntax_set);

        for h in ranges {
            let (style, code) = h;
            job.append(
                code,
                0.0,
                TextFormat::simple(
                    font_id.clone(),
                    Color32::from_rgb(
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

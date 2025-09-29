#![warn(clippy::all, rust_2018_idioms)]
use tree_sitter::{Language, LanguageError, Parser};

pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui);
}

pub struct TreeSitterEditor {
    language: String,
    code: String,
    parser: Parser,
}

impl TreeSitterEditor {
    pub fn new(language: String, code: String, parser: Parser) -> Self {
        Self {
            language,
            code,
            parser,
        }
    }

    pub fn set_language(&mut self, language: &Language) -> Result<(), LanguageError> {
        self.parser.set_language(language)
    }
}

impl Default for TreeSitterEditor {
    fn default() -> Self {
        Self {
            language: "javascript".to_owned(),
            code: "console.log('hello world')".to_owned(),
            parser: Parser::new(),
        }
    }
}

/// TODO: must make sure parser is properly initialised! serde cannot serialize it. need to create a set_parser method

impl View for TreeSitterEditor {
    fn ui(&mut self, ui: &mut egui::Ui) {
        let Self {
            language,
            code,
            parser,
        } = self;
        let res = ui.add(
            egui::TextEdit::multiline(code)
                .font(egui::TextStyle::Monospace)
                .desired_rows(10)
                .desired_width(f32::INFINITY),
        );

        if res.changed() {
            let text: String = "Hi there".to_owned();
            let bytes: &[u8] = text.as_ref();
            let len = bytes.len();
            parser.parse_with_options(
                &mut |i, _| (i < len).then(|| &bytes[i..]).unwrap_or_default(),
                None,
                None,
            );
        }
    }
}

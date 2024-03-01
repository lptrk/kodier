mod lexer;
use std::fs;

fn main() {
    let file_path = "src/assets/test.ts";

    let test_input = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Fehler beim Lesen der Datei {}: {}", file_path, err);
            return;
        }
    };

    lexer::print_tokens(&lexer::lex(&test_input));
}

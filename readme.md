kodier/
│
├── src/
│ ├── lexer/ --- Verzeichnis für Lexer-Module
│ │ ├── mod.rs --- Definiert das Lexer-Modul und integriert seine Bestandteile
│ │ └── token.rs --- Token-Definitionen und -Funktionen
│ │
│ ├── parser/ --- Verzeichnis für Parser-Module
│ │ ├── mod.rs --- Definiert das Parser-Modul und integriert seine Bestandteile
│ │ └── ast.rs --- AST-Strukturen und zugehörige Funktionen
│ │
│ ├── ast/ --- Verzeichnis für AST-Definitionen
│ │ └── mod.rs --- AST-Definitionen und -Konstruktoren
│ │
│ └── main.rs --- Hauptprogrammdatei, die Lexer und Parser zusammenführt
│
├── tests/ --- Verzeichnis für Integrationstests
│ ├── lexer_tests.rs
│ └── parser_tests.rs
│
├── Cargo.toml

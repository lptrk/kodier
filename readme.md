# kodier

A lexer for TypeScript written in Rust.

## Installation

To install the project, follow these steps:

1. Clone the repository: `git clone https://github.com/lptrk/kodier.git`
2. Navigate to the project directory: `cd kodier`
3. Run the application: `cargo run`

## Usage

Use the lexer to tokenize TypeScript code:

```rust
use kodier::{lex, print_tokens};

fn main() {
    let test_input = r#"
        interface Person {
            name: string;
            age: number;
        }

        class Student implements Person {
            name: string;
            age: number;

            constructor(name: string, age: number, grade: string) {
                this.name = name;
                this.age = age;
            }

            displayInfo() {
                console.log(`Name: ${this.name}, Age: ${this.age}, Grade: ${this.grade}`);
            }
        }

        const student1 = new Student('Alice', 20, 'A');
        const student2 = new Student('Bob', 22, 'B');

        student1.displayInfo();
        student2.displayInfo();
    "#;
    print_tokens(lex(test_input));
}
```

## License

This project is licensed under the terms of the MIT license. See the [LICENSE](LICENSE) file for details.

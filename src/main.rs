mod lexer;
fn main() {
    const TEST_INPUT: &str = r#"
    interface Shape {
        name: string;
        color: string;
        area(): number;
    }

    class Circle implements Shape {
        name: string;
        color: string;
        radius: number;

        constructor(name: string, color: string, radius: number) {
            this.name = name;
            this.color = color;
            this.radius = radius;
        }

        area(): number {
            return Math.PI * this.radius ** 2;
        }
    }

    class Rectangle implements Shape {
        name: string;
        color: string;
        width: number;
        height: number;

        constructor(name: string, color: string, width: number, height: number) {
            this.name = name;
            this.color = color;
            this.width = width;
            this.height = height;
        }

        area(): number {
            return this.width * this.height;
        }
    }

    const circle = new Circle("Circle", "red", 5);
    const rectangle = new Rectangle("Rectangle", "blue", 10, 8);

    console.log(circle.area());
    console.log(rectangle.area());
    "#;

    lexer::print_tokens(&lexer::lex(&TEST_INPUT));
}

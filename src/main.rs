mod circle;
mod trapezium;
mod rectangle;

use text_io::read;
fn main() {
    println!("Выберите фигуру:");
    println!("1. Прямоугольник");
    println!("2. Трапеция");
    println!("3. Окружность");

    let choice: u8 = read!();

    if choice == 1 {
        rectangle::process_rectangle(); 
    }
    if choice == 2 {
        trapezium::process_trapezium();
    }
    if choice == 3 {
        circle::process_circle();
    }
}

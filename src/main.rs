mod circle;
mod trapezium;

use text_io::read;
fn main() {
    println!("Выберите фигуру:");
    println!("1. Прямоугольник");
    println!("2. Треугольник");
    println!("3. Трапеция");
    println!("4. Окружность");

    let choice: u8 = read!();

    if choice == 1 {
        println!("Вы выбрали прямоугольник"); 
    }
    if choice == 2 {
        println!("Вы выбрали треугольник"); 
    }
    if choice == 3 {
        trapezium::process_trapezium();
    }
    if choice == 4 {
        circle::process_circle();
    }
}

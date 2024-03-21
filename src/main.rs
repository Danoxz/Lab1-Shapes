mod circle;
mod trapezium;
mod rectangle;

use text_io::read;
fn main() {
    println!("Выберите фигуру:");
    println!("1. Прямоугольник");
    println!("2. Трапеция");
    println!("3. Окружность");
    println!("4. Выход");
    let choice: u8 = read!();

    match choice {
        1 => rectangle::process_rectangle(),
        2 => trapezoid::process_trapezoid(),
        3 => circle::process_circle(),
        4 => break,
        _ => println!("Неверный выбор. Попробуйте еще раз."),
        }
}

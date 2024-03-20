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
}

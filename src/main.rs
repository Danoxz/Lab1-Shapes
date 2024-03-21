use text_io::read;
mod circle;
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
        println!("Вы выбрали трапецию"); 
    }
    if choice == 4 {
        circle::process_circle();
    }
}

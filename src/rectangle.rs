use text_io::read;

pub fn process_rectangle() {
    println!("Введите длину и ширину прямоугольника:");
    let length: f64 = read!();
    let width: f64 = read!();

    let perimeter: f64 = 2.0 * (length + width);
    let area: f64 = length * width;

    println!("Периметр: {}", perimeter);
    println!("Площадь: {}", area);
}
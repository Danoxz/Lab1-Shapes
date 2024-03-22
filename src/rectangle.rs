use text_io::read;

pub fn process_rectangle() {
    println!("Введите длину и ширину прямоугольника:");
    let length: f64 = read!();
    let width: f64 = read!();

    if length < 0.0 || width < 0.0{
        println!("Неккоректное значение");
    } else {
        let perimeter: f64 = 2.0 * (length + width);
        let area: f64 = length * width;
        let diagonal: f64 = (length.powi(2) + width.powi(2)).sqrt();
    
        println!("Периметр: {}", perimeter);
        println!("Площадь: {}", area);
        println!("Длина диагонали: {}", diagonal);
    }
}
use text_io::read;
use std::f64::consts::PI;

pub fn process_circle() {
    println!("Введите радиус окружности:");
    let radius: f64 = read!();
    let area: f64 = PI * radius.powi(2);

    let circumference: f64 = 2.0 * PI * radius;
    println!("Длина окружности: {}", circumference);
    println!("Площадь круга: {}", area);
}
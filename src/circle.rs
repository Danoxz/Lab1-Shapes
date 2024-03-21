use text_io::read;
use std::f64::consts::PI;

pub fn process_circle() {
    println!("Введите радиус окружности:");
    let radius: f64 = read!();

    let circumference: f64 = 2.0 * PI * radius;
    println!("Длина окружности: {}", circumference);
}
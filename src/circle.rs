use text_io::read;
use std::f64::consts::PI;

pub fn process_circle() {
    println!("Введите радиус окружности:");
    let radius: f64 = read!();
    println!("Введите угол сектора (в градусах):");
    let angle: f64 = read!();

    let circumference: f64 = 2.0 * PI * radius;
    let area: f64 = PI * radius.powi(2);
    let sector_area = PI * radius.powi(2) * (angle / 360.0);

    println!("Длина окружности: {}", circumference);
    println!("Площадь круга: {}", area);
    println!("Площадь сектора: {}", sector_area);
}
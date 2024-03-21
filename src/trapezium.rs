use text_io::read;

pub fn process_trapezium() {
    println!("Введите длины оснований и боковой стороны трапеции:");
    let base1: f64 = read!();
    let base2: f64 = read!();
    let side: f64 = read!();

    let perimeter: f64 = base1 + base2 + side * 2.0;
    let midline: f64 = (base1 - base2).abs() / 2.0;
    let height: f64 = (side.powi(2) - midline.powi(2)).sqrt();

    let area = 0.5 * (base1 + base2) * height;

    println!("Периметр: {}", perimeter);
    println!("Площадь: {}", area);
    println!("Средняя линия: {}", midline);
}
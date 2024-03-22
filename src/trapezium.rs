use text_io::read;

pub fn process_trapezium() {
    println!("Введите длины оснований и боковоых сторон трапеции:");
    let base1: f64 = read!();
    let base2: f64 = read!();
    let side1: f64 = read!();
    let side2: f64 = read!();

    let perimeter: f64 = base1 + base2 + side1 + side2;
    let midline: f64 = (base1 + base2) / 2.0;
    let height: f64 = (side1.powi(2) - (((base1 - base2).powi(2) + side1.powi(2) - side2.powi(2)) / (2.0 * (base1 - base2))).powi(2)).sqrt();

    let area: f64 = midline * height;

    println!("Периметр: {}", perimeter);
    println!("Площадь: {}", area);
    println!("Средняя линия: {}",midline);
}
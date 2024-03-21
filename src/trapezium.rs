use text_io::read;

pub fn process_trapezium() {
    println!("Введите длины оснований и боковой стороны трапеции:");
    let base1: f64 = read!();
    let base2: f64 = read!();
    let side: f64 = read!();

    let perimeter: f64 = base1 + base2 + side * 2.0;



    println!("Периметр: {}", perimeter);
}
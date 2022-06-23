mod celsius_fahrenheit;
mod fibonacci;
mod lyrics;

fn main() {
    let fahr_to_cel = celsius_fahrenheit::fahren_cels(true, 50, 20);
    let cel_to_fahr = celsius_fahrenheit::fahren_cels(false, 50, 20);
    println!("{fahr_to_cel}"); // 68
    println!("{cel_to_fahr}"); // 10

    fibonacci::fibonacci_f(5); // 8
}


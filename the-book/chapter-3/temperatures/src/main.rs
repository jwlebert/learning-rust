fn to_celsius(fahrenheit: &f32) -> f32 {
    (fahrenheit - 32.0) / 1.8
}

fn to_fahrenheit(celsius: &f32) -> f32 {
    celsius * 1.8 + 32.0
}

fn main() {
    let f1: f32 = 70.0;
    let c1: f32 = to_celsius(&f1);

    println!("f1 : {f1}, c1 : {c1}");
    println!("should be 70 : {}", to_fahrenheit(&c1));
}

const FREEZING_POINT: f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) / 1.8
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 1.8) + 32.0
}

fn main() {
    let mut ftemp = FREEZING_POINT;
    let mut ctemp;

    println!("Freezing Point in celsius: {}°C",fahrenheit_to_celsius(ftemp));

    let mut counter = 0;
    while counter < 5 {
        ftemp += 1.0;
        ctemp = fahrenheit_to_celsius(ftemp);
        println!("{}°C = {}°F", ctemp, celsius_to_fahrenheit(ctemp));
        counter += 1;
    }
}
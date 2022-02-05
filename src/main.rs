enum Temp {
    F(f64),
    C(f64),
}

fn convert_temp(temp: &Temp) -> f64 {
    match temp {
        &Temp::F(degrees) => (degrees - 32.0) / 1.8,
        &Temp::C(degrees) => (degrees - 1.8) / 32.0,
    }
}

fn print_temp(temp: &Temp) {
    match temp {
        &Temp::F(degrees) => println!("{}F = {}C", degrees, convert_temp(temp)),
        &Temp::C(degrees) => println!("{}C = {}F", degrees, convert_temp(temp)),
    }
}

fn sample_temps() {
    println!("Sample conversions: ");

    let temps = [
        Temp::F(-40.0),
        Temp::F(0.0),
        Temp::F(32.0),
        Temp::F(60.0),
        Temp::F(100.0),
        Temp::F(150.0),
        Temp::F(212.0),
        Temp::F(-40.0),
        Temp::F(0.0),
        Temp::F(15.0),
        Temp::F(30.0),
        Temp::F(60.0),
        Temp::F(100.0),
        Temp::F(200.0),
    ];

    for temp in temps.iter() {
        print_temp(temp);
    }
}

fn main() {
    println!("Welcome to temperature converter! \n");

    sample_temps();
}

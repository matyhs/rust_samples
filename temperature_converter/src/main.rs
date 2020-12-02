use std::io;

fn convert(temperature : i32, measurement : char) {
    match measurement {
        'C' | 'c' => {
            let temp = temperature as f32 * (9.0/5.0) + 32.0;
            println!("Temperature is {temp} degrees {label}", temp = temp.to_string(), label = "Celsius")
        },
        'F' | 'f' => {
            let temp = (temperature as f32 - 32.0) * (5.0/9.0);
            println!("Temperature is {temp} degrees {label}", temp = temp.to_string(), label = "Fahrenheit") 
        }
        _ => panic!("Unknown measurement of temperature")
    }
}

fn main() {
    println!("Please input the temperature: ");

    let mut temperature = String::new();
    
    io::stdin().read_line(&mut temperature).expect("Failed to read line");

    let temperature : i32 = temperature.trim().parse().expect("Please indicate a number");

    println!("Please input the measurement of temperature (F or C) to convert it to:");

    let mut measurement = String::new();

    io::stdin().read_line(&mut measurement).expect("Failed to read line");

    let measurement : char = measurement.trim().parse().expect("Please indicate measure in C or F");

    convert(temperature, measurement);
}

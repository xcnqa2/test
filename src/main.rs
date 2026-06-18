use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <temperature_in_fahrenheit>", args[0]);
        std::process::exit(1);
    }

    let fahrenheit_str = &args[1];
    let fahrenheit: f64 = match fahrenheit_str.parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Error: '{}' is not a valid number.", fahrenheit_str);
            std::process::exit(1);
        }
    };

    let (celsius, kelvin) = convert_fahrenheit(fahrenheit);

    println!("Fahrenheit: {:.2}°F", fahrenheit);
    println!("Celsius: {:.2}°C", celsius);
    println!("Kelvin: {:.2}K", kelvin);
}

fn convert_fahrenheit(f: f64) -> (f64, f64) {
    let c = (f - 32.0) * 5.0 / 9.0;
    let k = c + 273.15;
    (c, k)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_32() {
        let (c, k) = convert_fahrenheit(32.0);
        assert!((c - 0.0).abs() < 1e-10);
        assert!((k - 273.15).abs() < 1e-10);
    }

    #[test]
    fn test_convert_212() {
        let (c, k) = convert_fahrenheit(212.0);
        assert!((c - 100.0).abs() < 1e-10);
        assert!((k - 373.15).abs() < 1e-10);
    }
}

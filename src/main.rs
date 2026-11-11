use std::env::args;

fn main() {
    let args: Vec<String> = args().collect();
    
    if args.len()<2 {
        println!("Provide input");
        return;
    }
    let s = &args[1];

    let unit = s.chars().last().unwrap();
    let number = s.strip_suffix(unit).unwrap().parse::<f64>().unwrap();

    if unit=='d' {
        println!("{}",deg_to_rad(number))
    } else if unit=='r' {
        println!("{}",rad_to_deg(number))
    } else {
        println!("Invalid input")
    }
}

fn deg_to_rad(deg: f64) -> f64 {
    return deg * (std::f64::consts::PI / 180.0f64)
}

fn rad_to_deg(deg: f64) -> f64 {
    return deg * (180.0f64 / std::f64::consts::PI)
}

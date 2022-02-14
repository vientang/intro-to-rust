fn main() {
    let city_name = "Rustville";

    println!("The city of {}:\n", city_name);

    print_population(1_324_578, 114_293, 108_097);
}

fn print_population(adults: u64, kids: u32, buildings: u32) {
    // 💡 TIP: Use the `as` keyword to convert between numeric types!
    let population = (adults as f64) + (kids as f64);

    // 💡 TIP: To get a f64 answer here, both numerator and denominator must be f64 values
    let buildings_per_person = (buildings as f64) / population;

    println!("    Population: {}", population);
    println!("        Adults: {}", adults);
    println!("        Kids: {}", kids);
    println!("    Buildings: {}", buildings);
    println!("    Buildings per person: {}", buildings_per_person);

    if buildings_per_person >= (1.0 as f64) {
        println!("Everyone can have their own building!");
    } else {
        println!("Buildings must be shared!");
    }
}

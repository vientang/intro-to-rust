struct City {
    description: String,
    residents: u64,
    is_coastal: bool,
}

fn new_city(residents: u64, is_coastal: bool) -> City {
    if is_coastal {
        City {
            description: format!("a *coastal* city of approximately {} residents", residents),
            residents,
            is_coastal,
        }
    } else {
        let noncoastaldescription = format!(
            "This city can be described as a *non-coastal* city of approximately {} residents", 
            residents
        );
        panic!("{}", noncoastaldescription);
    }
}

fn main() {
    let rustville: City = new_city(1_324_578, true);

    println!("This city can be described as: {}.", rustville.description);

    if rustville.is_coastal {
        println!("It is a coastal city with {} residents.", rustville.residents);
    } else {
        println!("It is not a coastal city and there are {} residents", rustville.residents);
    }
}

struct Laptop {
    brand: String,
    unit_price: u32,
}

impl Laptop {
    fn total_cost(&self, quantity: u32) -> u32 {
        self.unit_price * quantity
    }
}

fn main() {
    // Create instances for each laptop brand
    let hp = Laptop {
        brand: String::from("HP"),
        unit_price: 650_000,
    };
    let ibm = Laptop {
        brand: String::from("IBM"),
        unit_price: 755_000,
    };
    let toshiba = Laptop {
        brand: String::from("Toshiba"),
        unit_price: 550_000,
    };
    let dell = Laptop {
        brand: String::from("Dell"),
        unit_price: 850_000,
    };

    // Calculate total cost for purchasing 3 laptops from each brand
    let total_cost = hp.total_cost(3)
        + ibm.total_cost(3)
        + toshiba.total_cost(3)
        + dell.total_cost(3);

    // Print the total cost
    println!("The total cost for purchasing 3 laptops from each brand is: ₦{}", total_cost);
}

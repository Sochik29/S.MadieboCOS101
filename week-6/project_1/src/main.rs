use std::io;

fn main() {
    // Display the menu
    println!("MenuPrice");
    println!("P = Poundo Yam/Edinkaiko Soup - N3,200");
    println!("F = Fried Rice & Chicken - N3,000");
    println!("A = Amala & Ewedu Soup - N2,500");
    println!("E = Eba & Egusi Soup - N2,000");
    println!("W = White Rice & Stew - N2,500");

    // Prices for each item
    let prices = [
        ('P', 3200),
        ('F', 3000),
        ('A', 2500),
        ('E', 2000),
        ('W', 2500),
    ];

    // Get user input for food type
    let mut food_type = String::new();
    println!("\nEnter the food type (P, F, A, E, W): ");
    io::stdin().read_line(&mut food_type).expect("Failed to read input");
    let food_type = food_type.trim().to_uppercase().chars().next().unwrap_or_default();

    // Validate food type
    let price_per_unit = match prices.iter().find(|&&(c, _)| c == food_type) {
        Some(&(_, price)) => price,
        None => {
            println!("Invalid food type entered.");
            return;
        }
    };

    // Get user input for quantity
    let mut quantity = String::new();
    println!("Enter the quantity: ");
    io::stdin().read_line(&mut quantity).expect("Failed to read input");
    let quantity: i32 = match quantity.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid quantity entered.");
            return;
        }
    };

    // Calculate total cost
    let mut total_cost = price_per_unit * quantity;
    
    // Apply discount if applicable
    if total_cost > 10_000 {
        let discount = total_cost as f32 * 0.05;
        total_cost -= discount as i32;
        println!("A 5% discount has been applied.");
    }

    // Display the total charges
    println!("\nTotal charges: N{}", total_cost);
}

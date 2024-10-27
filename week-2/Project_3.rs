fn main() {
    // Initial value of the TV
    let initial_value: f64 = 210_000.0; 

    // depreciation rate
    let depreciation_rate: f64 = 0.05;

    // Number of years
    let years: i32 = 3;

    // value of the TV after 3 years
    let depreciated_value: f64 = initial_value * (1.0 - depreciation_rate).powi(years);

    //result
    println!("The value of the TV after {} years is: N{:.2}", years, depreciated_value);
}

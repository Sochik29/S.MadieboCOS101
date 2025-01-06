use std::fs::File;
use std::io::Write;

fn main() {
    // Open or create a file named "drinks.txt"
    let mut file = File::create("drinks.txt").expect("Could not create file");

    // Write the categories and drinks into the file
    writeln!(file, "Lager:").expect("Could not write to file");
    writeln!(file, " - 33 Export").expect("Could not write to file");
    writeln!(file, " - Desperados").expect("Could not write to file");
    writeln!(file, " - Goldberg").expect("Could not write to file");
    writeln!(file, " - Gulder").expect("Could not write to file");
    writeln!(file, " - Heineken").expect("Could not write to file");
    writeln!(file, " - Star").expect("Could not write to file");
    writeln!(file).expect("Could not write to file"); // Blank line

    writeln!(file, "Stout:").expect("Could not write to file");
    writeln!(file, " - Legend").expect("Could not write to file");
    writeln!(file, " - Turbo King").expect("Could not write to file");
    writeln!(file, " - Williams").expect("Could not write to file");
    writeln!(file).expect("Could not write to file"); // Blank line

    writeln!(file, "Non-Alcoholic:").expect("Could not write to file");
    writeln!(file, " - Maltina").expect("Could not write to file");
    writeln!(file, " - Amstel Malta").expect("Could not write to file");
    writeln!(file, " - Malta Gold").expect("Could not write to file");
    writeln!(file, " - Fayrouz").expect("Could not write to file");

    println!("File 'drinks.txt' has been created successfully!");
}
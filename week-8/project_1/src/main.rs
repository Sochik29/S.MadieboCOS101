fn main() {
    // Names of interviewees
    let names = ["Ola", "Michael", "Kayima"];
    // Corresponding years of experience
    let experiences = [5, 10, 7];

    // Initialize variables to track the highest experience and name
    let mut max_experience = 0;
    let mut most_experienced_person = "";

    // Loop through the experiences array
    for i in 0..experiences.len() {
        if experiences[i] > max_experience {
            max_experience = experiences[i];
            most_experienced_person = names[i];
        }
    }

    // Print the result
    println!(
        "The person with the highest programming experience is {} with {} years.",
        most_experienced_person, max_experience
    );
}
fn main() {
    let name1 = "Ayomide Adesokan";
    println!("My name is {}", name1);

    // Find and replace
    let name2 = name1.replace("Ayomide", "Adebare");
    println!("You can also call me {}", name2);

    let faculty = "Faculty of Science and Technology";

    // Find and replace
    let school = faculty.replace("Faculty", "School");
    println!("I am a student of the {}", school);
}


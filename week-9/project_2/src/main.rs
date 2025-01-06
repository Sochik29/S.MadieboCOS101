use std::fs::File;
use std::io::Write;



fn main() {
    // Define student details in a vector
    let students = vec![
        Student {
            id: 1,
            name: String::from("John Doe"),
            department: String::from("Computer Science"),
            level: 200,
        },
        Student {
            id: 2,
            name: String::from("Jane Smith"),
            department: String::from("Electrical Engineering"),
            level: 300,
        },
        Student {
            id: 3,
            name: String::from("Mary Johnson"),
            department: String::from("Mechanical Engineering"),
            level: 100,
        },
    ];

    // Display the student details
    println!("Student Details:");
    for student in &students {
        println!(
            "ID: {}, Name: {}, Department: {}, Level: {}",
            student.id, student.name, student.department, student.level
        );
    }

    // Save the student details to a file
    let mut file = File::create("student_details.txt").expect("Unable to create file");
    writeln!(file, "Student Details:").expect("Could not write to file");
    for student in &students {
        writeln!(
            file,
            "ID: {}, Name: {}, Department: {}, Level: {}",
            student.id, student.name, student.department, student.level
        )
        .expect("Could not write to file");
    }

    println!("Student details have been saved to 'student_details.txt'");
}
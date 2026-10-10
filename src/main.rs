mod student;
mod student_statistics;
use crate::student::Student;
use crate::student_statistics::*;
fn main() {
    let student = Student {
        id: 1001,
        name: "Mary".to_string(),
        marks: vec![70.0, 65.0, 80.0],
    };

    println!("Total: {}", student_statistics::total_marks(&student));
    println!("Average: {}", student_statistics::average_marks(&student));
    println!("Passed: {}", student_statistics::passed(&student));
}
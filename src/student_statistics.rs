use crate::student::Student;
pub fn total_marks(student: &Student) -> f64 {
    student.marks.iter().sum()
}
pub fn average_marks(student: &Student) -> f64 {
    (total_marks(student) / student.marks.len() as f64).round()
}
pub fn passed(student: &Student) -> bool {
    average_marks(student) >= 50.0
}
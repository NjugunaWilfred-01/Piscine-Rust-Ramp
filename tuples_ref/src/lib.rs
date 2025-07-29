#[derive(Debug, PartialEq, Clone)]

pub struct Student{
    id: u32,
    f_name: String,
    l_name: String,
}

pub fn id(student: &Student) -> u32 {
    student.id
}

pub fn first_name(student: &Student) -> &str {
    student.f_name.as_str()
}

pub fn last_name(student: &Student) -> &str {
    student.l_name.as_str()
}
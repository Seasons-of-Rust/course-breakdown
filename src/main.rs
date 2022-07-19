use std::fs::File;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Course {
    code1: String,
    code2: String,
    code3: String,
    code4: String,
    section: String,
    kind: String,
    thing: String,
    start: String,
    end: String,
    start_date: String,
    end_date: String,
    num1: String,
    num2: String,
}

fn main() {
    // Load courses.csv
    let file = File::open("courses.csv").unwrap();
    let mut rdr = csv::Reader::from_reader(file);

    // Get the vec ready
    let mut courses: Vec<Course> = Vec::new();

    for course in rdr.deserialize() {
        courses.push(course.unwrap());
    }

    // Print courses
    for course in courses {
        dbg!(course);
    }
}

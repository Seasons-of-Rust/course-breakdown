use std::{
    collections::{HashMap, HashSet},
    fs::File,
};

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
struct Course {
    //202310,10015,AERO,2001,L1,LAB,TR,1135,1325,2023-01-09,2023-04-12,70,63
    semester: i32,
    crn: i32,
    subject: String,
    code: i32,
    section: String,
    kind: Kind,
    days: String,
    start: String,
    end: String,
    start_date: String,
    end_date: String,
    total_space: i32,
    taken_space: i32,
}

impl Course {
    pub fn get_timeslot_for_course(self) -> String {
        format!("{}{}{}", self.days, self.start, self.end)
    }

    /// Print out pretty text from a course ❗
    pub fn course_title() -> String {
        todo!()
    }
}

#[derive(Debug, Deserialize, Clone)]
enum Kind {
    LEC,
    TUT,
    LAB,
    PAN,
    GRP,
    FLM,
    PLA,
    WKS,
    SEM,
    IND,
    PRC,
    HON,
    WRK,
    FIE,
    CAP,
    RES,
    STU,
    OTH,
    REP,
    DIR,
    NTC
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

    
    let subject_map: HashMap<String, Course> = sort_courses(courses.clone());
    
    // Get all of the unique types in the kind column
    let kind_map: HashSet<String> = courses.iter().map(|c| c.kind.clone()).collect();
    
    return;

    timed_courses(&subject_map);

    ask_user_for_course();

    show_histogram(&subject_map);
}

fn show_histogram(courses: &HashMap<String, Course>) {
    // Assume that all of the courses have been broken down into 5 minute
    // timeslots

    // Use ascii characters to represent intensity

    // $@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\|()1{}[]?-_+~<>i!lI;:,"^`'.
}

fn ask_user_for_course() { // Allie
    // Get a course subject and code from the user
    println!("Please enter a course code: ");
    let courseCode =
    // Verify that it's in the courses
}

/// Do something with time
/// Worked on by ES
fn timed_courses(courses: &HashMap<String, Course>) {
    todo!()
}

/// Sort the course into a hashmap
/// Worked on by Alan
fn sort_courses(courses: Vec<Course>) -> HashMap<String, Course> {
    let course_hashmap = HashMap::new();
    // Iterate over each course
    let course_iterator = courses.iter();

    // If it's a key in the hashmap already, add it to the vec in there,
    // otherwise create a new vec
    for course in course_iterator {
        if course_hashmap.contains_key(course) {
            
        }
        else {
            let course_vector = 
        }
    }

    course_hashmap
}

fn order_timeslots(courses: Vec<Course>) {
    let mut timetable: HashMap<String, u32> = HashMap::new();

    for c in courses {
        let ccode = c.get_timeslot_for_course();
        timetable.insert(
            ccode.clone(),
            1 + if timetable.contains_key(&ccode) {
                timetable[&ccode]
            } else {
                0
            },
        );
    }
    // Thanks StackOverflow!
    let mut hash_vec: Vec<(&String, &u32)> = timetable.iter().collect();
    println!("{:?}", hash_vec);
    hash_vec.sort_by(|a, b| b.1.cmp(a.1));
    println!("Sorted: {:?}", hash_vec);
    let final_vec: Vec<String> = hash_vec.into_iter().map(|x, y| x.clone()).collect();
    final_vec
}

/// Ideas:
///
/// - Sorting courses into subjects - Alan
/// - Changing kind to enum - Lily (it loaded!)
/// - Fixing types that are imported - Michelle
///   - Extracting date from start_date string
/// - Finding the number of year-standing courses in each subject
/// - Do something with time - ES - what should I do with time
///   - Order timeslots during the week that are the most busy - ES
///     - Each 5 minute block is another slot
///       - Make a struct that reprents this ❗
///     - Can we make a histogram out of that? ❗
/// - Interface to see a course ❗
///   - You can see what other courses overlap with it
/// - Make a better general struct that can represent all of the data
///   - List of courses, hashmap of subjects, time block struct
struct boop {}

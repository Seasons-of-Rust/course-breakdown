use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io,
};

use chrono::{DateTime, Utc};
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
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>,
    total_space: i32,
    taken_space: i32,
}

impl Course {
    pub fn get_timeslot_for_course(self) -> String {
        format!("{}{}{}", self.days, self.start, self.end)
    }

    /// Print out pretty text from a course ❗
    pub fn course_title(self) -> String {
        let s: String = self.code.to_string();
        format!("{}{}", self.subject, s)
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
    NTC,
}
impl Kind {
    pub fn getFullString(self) -> &str {
        match self {
            // (?) means it's what copilot reccomended but idk if it's accurate,
            // at least it was and then copilot started putting in it's own
            // (?)s, so it's just as unsure as I am.
            Kind::LEC => "Lecture",
            Kind::TUT => "Tutorial",
            Kind::LAB => "Lab",
            Kind::PAN => "Practical (?)",
            Kind::GRP => "Group",
            Kind::FLM => "Film",
            Kind::PLA => "Placement",
            Kind::WKS => "Workshop",
            Kind::SEM => "Seminar",
            Kind::IND => "Individual",
            Kind::PRC => "uhhhhhh2",
            Kind::HON => "Honor (?)",
            Kind::WRK => "Work",
            Kind::FIE => "Field (?)",
            Kind::CAP => "Capstone (?)",
            Kind::RES => "Research",
            Kind::STU => "Student (?)",
            Kind::OTH => "Other (?)",
            Kind::REP => "Repeat (?)",
            Kind::DIR => "Direction (?)",
            Kind::NTC => "Notice (?)",
        }
    }
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

    // Get all of the courses in a subject
    let subject_map: HashMap<String, Course> = sort_courses(courses.clone());

    // Get all of the unique types in the kind column
    let kind_map: HashSet<String> = courses.iter().map(|c| c.kind.clone()).collect();

    return;

    timed_courses(&subject_map);

    ask_user_for_course();

    show_histogram(&courses);
}

fn ask_user_for_course() {
    // Get a course subject and code from the user
    println!("Please enter a course subject: ");
    let mut courseSub = String::new();

        io::stdin().read_line(&mut courseSub).expect("Error.");

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
        let course_vec = course_hashmap.entry(&course.subject).or_insert(Vec::new());
        course_vec.push(course);
        // let mut course_vector: Vec<Course> = Vec::new();
        // if course_hashmap.contains_key(&course.subject) == false {
        //     course_vector.push(course.clone());
        // } else {
        //     course_vector.push(course);
        // }
        // course_hashmap.insert(course.subject, course_vector);
    }

    course_hashmap
}

fn order_timeslots(courses: Vec<Course>) -> Vec<String> {
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
    // this is wrong will fix
    let final_vec: Vec<String> = hash_vec.iter().map(|x| x.1.clone()).collect();
    final_vec
}

struct TimeRange {
    start_time: u32,
    end_time: u32,
    day: u32,
}

impl TimeRange {
    pub fn intersect(self, other: Self) -> bool {
        other.day == self.day
            && (other.start_time >= self.end_time && other.start_time < self.end_time)
            || (other.start_time < self.end_time && self.start_time < other.end_time)
    }
}

/*
impl TimeRange {
    pub fn intersect(self, other: TimeRange); //fffffff :(
}
*/

fn show_histogram(courses: &Vec<Course>) {
    let mut timetable: HashMap<String, u32> = HashMap::new();
    for c in courses {
        let 
    }
    
    // Assume that all of the courses have been broken down into 5 minute
    // timeslots

    // Use ascii characters to represent intensity

    // $@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\|()1{}[]?-_+~<>i!lI;:,"^`'.
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

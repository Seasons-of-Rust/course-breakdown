use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io,
};

use chrono::{DateTime, Utc};
use serde::Deserialize;

/// A course being offered at Carleton. It's specific by the semester, and the
/// section
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
    start: i32,
    end: i32,
    start_date: String,
    end_date: String,
    total_space: i32,
    taken_space: i32,
}

impl Course {
    pub fn get_timeslot_for_course(self) -> String {
        format!("{}{}{}", self.days, self.start, self.end)
    }

    /// Print out pretty text from a course
    pub fn course_title(self) -> String {
        format!("{} {}", self.subject, self.code)
    }

    /// Print out a description of the course
    pub fn course_description(self) -> String {
        format!(
            "{} {} {} ({})\n{} {} - {}\n",
            self.subject,
            self.code,
            self.section,
            self.kind.getFullString(),
            self.days,
            self.start,
            self.end
        )
    }
}

/// The different course types
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
    /// Translate each type into a more verbose string n
    pub fn getFullString(self) -> String {
        let fullString = match self {
            // (?) means it's what copilot reccomended but idk if it's accurate,
            // at least it was and then copilot started putting in it's own
            // (?)s, so it's just as unsure as I am. LOL
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
            Kind::PRC => "Practical (?)",
            Kind::HON => "Honor (?)",
            Kind::WRK => "Work",
            Kind::FIE => "Field (?)",
            Kind::CAP => "Capstone (?)",
            Kind::RES => "Research",
            Kind::STU => "Student",
            Kind::OTH => "Other",
            Kind::REP => "Repeat (?)",
            Kind::DIR => "Direction (?)",
            Kind::NTC => "Notice (?)",
        };
        fullString.to_string()
    }
}

fn main() {
    // Load courses.csv
    let file = File::open("courses.csv").unwrap();
    let mut rdr = csv::Reader::from_reader(file);

    // Get the vec ready
    let mut courses: Vec<Course> = Vec::new();

    // Deserialize the reader, and create a course out of each row
    for course in rdr.deserialize() {
        courses.push(course.unwrap());
    }

    // Print out the first 10 courses
    for course in courses.iter().take(10) {
        println!("{}", course.clone().course_title());
    }

    // Get all of the courses in a subject
    let subject_map: HashMap<String, Vec<Course>> = sort_courses(courses.clone());

    // Get all of the unique types in the kind column
    // let kind_map: HashSet<String> = courses.iter().map(|c| c.kind.clone()).collect();

    timed_courses(&subject_map);

    //ask_user_for_course(&subject_map);

    show_histogram(&courses);
}

/// Get input from the user on which course they want to see the details of
fn ask_user_for_course(courses: &HashMap<String, Course>) {
    // Get a course subject and code from the user
    println!("Please enter a course subject: ");
    let mut course_sub = String::new();
    io::stdin().read_line(&mut course_sub).expect("Error.");
    // Verify that hashmap contains subject code
    //implement when sort_courses is final 
    println!("Please enter a course code: ");
    let mut course_code = String::new();
    io::stdin().read_line(&mut course_code).expect("Error.");
    // Verify that subject key contains code value 
}

/// Do something with time
/// Worked on by ES
fn timed_courses(courses: &HashMap<String, Vec<Course>>) {
    todo!()
}

/// Sort the course into a hashmap
fn sort_courses(courses: Vec<Course>) -> HashMap<String, Vec<Course>> {
    let mut course_hashmap = HashMap::new();

    // Iterate over each course
    for course in courses.iter() {
        // If it's a key in the hashmap already, add it to the vec in there,
        // otherwise create a new vec
        let course_vec = course_hashmap
            .entry(course.subject.clone())
            .or_insert(Vec::new());
        course_vec.push(course.clone());
    }

    course_hashmap
}

fn order_timeslots(courses: Vec<Course>) -> Vec<String> {
    let mut timetable: HashMap<String, u32> = HashMap::new();

    // Bucket courses together based on semester, day, start time + end time
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

    // Sort the vec by most full timeslots
    hash_vec.sort_by(|a, b| b.1.cmp(a.1));

    // Print it out
    println!("Sorted: {:?}", hash_vec);

    // Get the list of times inorder th
    let final_vec: Vec<String> = hash_vec.iter().map(|x| x.0.clone()).collect();
    final_vec
}

// this was pointless.... :(
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

fn show_histogram(courses: &Vec<Course>) {
    // we only need to count the intervals where the intersection occurs
    let mut timetable: HashMap<u32, u32> = HashMap::new();
    //WIP:
    for c in courses {
        let course_end: u32 = c.end.parse().unwrap();
        let course_start: u32 = c.start.parse().unwrap();
        let intervals = (course_end..course_start).step_by(5);
        println!("{:?}", intervals);
        timetable.extend(intervals.map(|x| (x, 1 + if timetable.contains_key(&x) {
                timetable[&x]
            } else {
                0
            })));    
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

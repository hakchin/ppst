use crate::models::academy::{AcademyInfo, Instructor, Program};

/// Get the default academy information
/// This contains the hard-coded academy data that would eventually come from a config file or database
pub fn get_academy_info() -> AcademyInfo {
    AcademyInfo {
        name: "☆별을셀".to_string(),
        tagline: "Excellence in Mathematics Education".to_string(),
        logo_path: "/static/images/logo.svg".to_string(),
        programs: get_programs(),
        instructors: get_instructors(),
    }
}

/// Get the list of programs offered
fn get_programs() -> Vec<Program> {
    vec![
        Program {
            name: "Full-Stack Web Development".to_string(),
            description: "Master modern web technologies including React, Node.js, databases, and cloud deployment. Build production-ready applications from front-end to back-end.".to_string(),
            icon_path: Some("/static/images/web-icon.svg".to_string()),
        },
        Program {
            name: "Systems Programming with Rust".to_string(),
            description: "Learn Rust programming language and systems-level development. Build high-performance, memory-safe applications for the modern web and beyond.".to_string(),
            icon_path: Some("/static/images/rust-icon.svg".to_string()),
        },
        Program {
            name: "Data Science & Machine Learning".to_string(),
            description: "Explore data analysis, visualization, and machine learning algorithms. Apply Python and modern ML frameworks to real-world datasets.".to_string(),
            icon_path: Some("/static/images/data-icon.svg".to_string()),
        },
    ]
}

/// Get the list of instructors
fn get_instructors() -> Vec<Instructor> {
    vec![
        Instructor {
            name: "Dr. Jane Smith".to_string(),
            credentials: "Ph.D. in Computer Science, MIT".to_string(),
            photo_path: "/static/images/instructors/jane.jpg".to_string(),
            bio: "20+ years of experience in software engineering and education. Former Principal Engineer at Google. Passionate about making complex concepts accessible to all learners.".to_string(),
        },
        Instructor {
            name: "Alex Johnson".to_string(),
            credentials: "M.S. Computer Science, Stanford University".to_string(),
            photo_path: "/static/images/instructors/alex.jpg".to_string(),
            bio: "Full-stack developer with 15 years in the industry. Built scalable systems at startups and Fortune 500 companies. Loves mentoring aspiring developers.".to_string(),
        },
        Instructor {
            name: "Maria Garcia".to_string(),
            credentials: "B.S. Software Engineering, UC Berkeley".to_string(),
            photo_path: "/static/images/instructors/maria.jpg".to_string(),
            bio: "Data scientist and machine learning specialist. Previously at Netflix working on recommendation systems. Dedicated to teaching practical ML applications.".to_string(),
        },
    ]
}

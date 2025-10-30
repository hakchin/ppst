/// Represents information about the PPST Academy
#[derive(Debug, Clone)]
pub struct AcademyInfo {
    /// Academy name
    pub name: String,
    /// Tagline/slogan
    pub tagline: String,
    /// Mission statement (1-3 paragraphs)
    pub mission: String,
    /// Logo path (relative to static/ directory)
    pub logo_path: String,
    /// List of programs offered
    pub programs: Vec<Program>,
    /// List of instructors
    pub instructors: Vec<Instructor>,
}

#[derive(Debug, Clone)]
pub struct Program {
    /// Program name
    pub name: String,
    /// Short description
    pub description: String,
    /// Icon or image path (optional)
    pub icon_path: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Instructor {
    /// Instructor full name
    pub name: String,
    /// Credentials (e.g., "Ph.D. in Computer Science")
    pub credentials: String,
    /// Profile photo path
    pub photo_path: String,
    /// Short bio (1-2 sentences)
    pub bio: String,
}

impl AcademyInfo {
    /// Returns default academy information with hard-coded content
    pub fn default() -> Self {
        Self {
            name: "PPST Academy".to_string(),
            tagline: "Excellence in Programming Education".to_string(),
            mission: r#"Our mission is to provide world-class programming education that empowers students to become confident, skilled software developers. We believe in hands-on learning, industry-relevant curriculum, and personalized mentorship. Through our comprehensive programs, we prepare students not just to code, but to think critically, solve complex problems, and build innovative solutions that make a real impact."#.to_string(),
            logo_path: "/static/images/logo.svg".to_string(),
            programs: vec![
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
            ],
            instructors: vec![
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
            ],
        }
    }
}

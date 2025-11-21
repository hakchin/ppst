/// Represents information about the PPST Academy
#[derive(Debug, Clone)]
pub struct AcademyInfo {
    /// Academy name
    pub name: String,
    /// Tagline/slogan
    pub tagline: String,
    /// Logo path (relative to static/ directory)
    #[allow(dead_code)]
    pub logo_path: String,
    /// List of programs offered
    pub programs: Vec<Program>,
    /// List of instructors
    pub instructors: Vec<Instructor>,
}

#[derive(Debug, Clone)]
pub struct Program {
    /// Program name
    #[allow(dead_code)]
    pub name: String,
    /// Short description
    #[allow(dead_code)]
    pub description: String,
    /// Icon or image path (optional)
    #[allow(dead_code)]
    pub icon_path: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Instructor {
    /// Instructor full name
    #[allow(dead_code)]
    pub name: String,
    /// Credentials (e.g., "Ph.D. in Computer Science")
    #[allow(dead_code)]
    pub credentials: String,
    /// Profile photo path
    #[allow(dead_code)]
    pub photo_path: String,
    /// Short bio (1-2 sentences)
    #[allow(dead_code)]
    pub bio: String,
}

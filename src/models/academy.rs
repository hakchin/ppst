/// Represents basic information about the PPST Academy
#[derive(Debug, Clone)]
pub struct AcademyInfo {
    /// Academy name
    pub name: String,
    /// Tagline/slogan
    pub tagline: String,
}

impl AcademyInfo {
    /// Returns default academy information
    pub fn default() -> Self {
        Self {
            name: "☆별을셀".to_string(),
            tagline: "Excellence in Mathematics Education".to_string(),
        }
    }
}

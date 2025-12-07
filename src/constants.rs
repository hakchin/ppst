//! Application-wide constants
//!
//! Single source of truth for contact information and other constants
//! used throughout the application.

/// Academy contact information
pub mod contact {
    /// Phone number
    pub const PHONE: &str = "010-5102-0841";

    /// Academy name
    pub const NAME: &str = "별을셀";

    /// Full academy name with star
    pub const FULL_NAME: &str = "★별을셀수학";

    /// Website URL
    pub const WEBSITE: &str = "https://starrystarry.kr";

    /// Full address
    pub const ADDRESS: &str = "경기도 군포시 번영로 489 중앙타워 2층";

    /// Address with academy name
    pub const ADDRESS_FULL: &str = "경기도 군포시 번영로 489 중앙타워 2층 ★별을셀수학";

    /// Address with lot number
    pub const ADDRESS_WITH_LOT: &str =
        "경기도 군포시 번영로 489 중앙타워 2층 ★별을셀수학 (지번: 산본동 1142-7)";
}

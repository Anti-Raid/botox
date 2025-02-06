pub mod cache;
pub mod crypto;
pub mod serenity_backports;
pub mod taskman;

pub use extract_map::ExtractMap;

type Error = Box<dyn std::error::Error + Send + Sync>;

mod help;
mod lang;
mod roll;
mod table;

pub struct Data;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
pub type Result = std::result::Result<(), Error>;

pub use help::help;
pub use roll::roll;

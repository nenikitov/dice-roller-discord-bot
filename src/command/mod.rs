mod help;
mod roll;
mod lang;

pub struct Data;

pub(self) type Error = Box<dyn std::error::Error + Send + Sync>;
pub(self) type Context<'a> = poise::Context<'a, Data, Error>;
pub(self) type Result = std::result::Result<(), Error>;

pub use help::help;
pub use roll::roll;

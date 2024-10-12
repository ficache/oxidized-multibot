// todo!
// This file will contain command enums
// that should be handled in main.rs
use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
pub enum UserCommands {
    #[command(description="Show this message", aliases = ["h", "?"])]
    Help,
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
pub enum AdminCommands {
    #[command(description="Show this message", aliases = ["ah", "a?"])]
    AHelp,

    #[command(description = "Clean your chat. Specify start message with reply")]
    Purge(u8),
}

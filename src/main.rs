use teloxide::{dispatching::HandlerExt, prelude::*, utils::command::BotCommands};
// Local import zone
mod command_handlers;
mod enums;
use crate::command_handlers::admin::*;
use crate::enums::*;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting throw dice bot...");

    let bot = Bot::from_env();

    let handler = Update::filter_message()
        .branch(
            dptree::entry()
                .filter_command::<UserCommands>()
                .endpoint(user_handler),
        )
        .branch(
            dptree::filter_async(|bot: Bot, msg: Message| async move {
                if let Ok(chat_member) =
                    &bot.get_chat_member(msg.chat.id, msg.from.unwrap().id).await
                {
                    chat_member.is_privileged()
                } else {
                    false
                }
            })
            .filter_command::<AdminCommands>()
            .endpoint(admin_handler),
        );

    Dispatcher::builder(bot, handler)
        .default_handler(|upd| async move {
            log::warn!("Unhandled update: {:?}", upd);
        })
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

async fn user_handler(bot: Bot, msg: Message, cmd: UserCommands) -> ResponseResult<()> {
    match cmd {
        UserCommands::Help => {
            bot.send_message(msg.chat.id, UserCommands::descriptions().to_string())
                .await?
        }
    };
    Ok(())
}

async fn admin_handler(bot: Bot, msg: Message, cmd: AdminCommands) -> ResponseResult<()> {
    match cmd {
        AdminCommands::AHelp => ahelp(bot, msg).await?,
        AdminCommands::Purge(amount) => purge(bot, msg, amount).await?,
    };
    Ok(())
}

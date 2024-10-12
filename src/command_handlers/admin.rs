use teloxide::{prelude::*, types::MessageId, utils::command::BotCommands};

use crate::enums::AdminCommands;

pub async fn ahelp(bot: Bot, msg: Message) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, AdminCommands::descriptions().to_string())
        .await?;
    Ok(())
}

pub async fn purge(bot: Bot, msg: Message, amount: u8) -> ResponseResult<()> {
    if let Some(reply) = msg.reply_to_message() {
        delete_loop(&bot, &reply, amount).await;
        bot.send_message(msg.chat.id, "Purged").await?;
    } else {
        delete_loop(&bot, &msg, amount).await;
        bot.send_message(msg.chat.id, "Purged").await?;
    }
    Ok(())
}

async fn delete_loop(bot: &Bot, msg: &Message, amount: u8) {
    let mut decrement_id: i32 = msg.id.to_string().parse().unwrap();
    let mut error_amount: u8 = 0;
    let mut amount = amount;
    loop {
        if (error_amount > 100) || (amount == 0) {
            break;
        }

        decrement_id -= 1;

        if let Err(_) = bot
            .delete_message(msg.chat.id, MessageId(decrement_id))
            .await
        {
            error_amount += 1;
        } else {
            error_amount = 0;
            amount -= 1;
        }
    }
}

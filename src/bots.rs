use teloxide::prelude::*;

pub async fn run_listener(token: String) -> ResponseResult<()> {
    let bot = Bot::new(token);

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        if let Some(user) = msg.from() {
            println!("Сообщение от: {} (ID: {})", user.first_name, user.id);
            bot.send_message(msg.chat.id, format!("Ваш ID: {}", user.id))
                .await?;
        }
        respond(())
    })
    .await;

    Ok(())
}

use log::info;
use teloxide::{prelude::*, types::Update, utils::command::BotCommands, RequestError};

use crate::config::Config;

use super::{
    handler::{inline::inlinequery_handler, start::info_handler},
    inline_handler::InlineHandler,
    message_handler::MessageHandler,
};

#[derive(BotCommands, Clone)]
#[command(description = "Commands:", rename_rule = "lowercase")]
pub enum Command {
    #[command(description = "What is this?!")]
    Help,
    #[command(description = "start")]
    Start,
}

pub async fn run() {
    info!("[bot] Starting bot...");

    let config = Config::new();
    let bot = Bot::new(&config.bot_token);

    bot.set_my_commands(Command::bot_commands())
        .await
        .expect("Failed to set bot commands");

    let handler = dptree::entry()
        .branch(
            Update::filter_message()
                .filter_command::<Command>()
                .endpoint(command_handler),
        )
        .branch(
            Update::filter_inline_query()
                .endpoint(inline_handler),
        );
        
    // .branch(Update::filter_callback_query().endpoint(callback_handler));

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

async fn command_handler(msg: Message, bot: Bot, cmd: Command) -> Result<(), RequestError> {
    let handler = &MessageHandler::new(&bot, &msg);
    match cmd {
        Command::Help => info_handler(handler).await,
        Command::Start => info_handler(handler).await,
    }
}

async fn inline_handler(query: InlineQuery, bot: Bot) -> Result<(), RequestError, > {
    let handler = &InlineHandler::new(&bot, &query);
    inlinequery_handler(handler).await
}

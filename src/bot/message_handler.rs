use teloxide::{
    payloads::SendMessageSetters,
    requests::Requester,
    types::Message,
    types::{InlineKeyboardMarkup, ParseMode::Html},
    Bot, RequestError,
};

pub struct MessageHandler<'a> {
    pub bot: &'a Bot,
    pub msg: &'a Message,
}

impl<'a> MessageHandler<'a> {
    pub fn new(bot: &'a Bot, msg: &'a Message) -> Self {
        Self { bot, msg }
    }

    pub async fn reply_text<T: ToString>(
        &self,
        text: T,
        markup: Option<InlineKeyboardMarkup>,
    ) -> Result<(), RequestError> {
        match markup {
            Some(markup) => self
                .bot
                .send_message(self.msg.chat.id, text.to_string())
                .reply_markup(markup)
                .parse_mode(Html)
                .await
                .map(|_| ()),
            None => self
                .bot
                .send_message(self.msg.chat.id, text.to_string())
                .parse_mode(Html)
                .await
                .map(|_| ()),
        }
    }
}

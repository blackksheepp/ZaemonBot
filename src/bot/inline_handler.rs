use log::error;
use teloxide::{
    payloads::AnswerInlineQuerySetters,
    requests::Requester,
    types::{InlineQuery, InlineQueryResult},
    Bot, RequestError,
};

pub struct InlineHandler<'a> {
    pub bot: &'a Bot,
    pub query: &'a InlineQuery,
}

impl<'a> InlineHandler<'a> {
    pub fn new(bot: &'a Bot, query: &'a InlineQuery) -> Self {
        Self { bot, query }
    }

    pub async fn answer<R: IntoIterator<Item = InlineQueryResult>>(
        &self,
        results: R,
    ) -> Result<(), RequestError> {
        match self
            .bot
            .answer_inline_query(self.query.id.to_string(), results)
            .switch_pm_parameter("howto")
            .switch_pm_text("How to Use? + Templates")
            .await
        {
            Ok(_) => Result::Ok(()),
            Err(e) => {
                error!("Failed to answer query. Error: {}", e);
                Result::Err(e)
            }
        }
    }
}

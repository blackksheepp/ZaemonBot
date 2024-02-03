use teloxide::types::{InlineKeyboardButton, InlineKeyboardButtonKind, InlineKeyboardMarkup};

use crate::{bot::message_handler::MessageHandler, database::models::UserDB};

const START_TEXT: &str = r#"Yoo! I'm working just fine.

<b>What do I do?</b>
- Use me in Inline Mode to create BuriBuriZaemon Stickers.

<b>Quick Trivia: </b>
- Use '<code>.</code>' to Seperate Upper and Bottom Line.
- example: <code>@zaemonbot upper line. bottom line</code>

<i>don't forget to join @JustCodingTm</i>"#;

const HELP_TEXT: &str = r#"<b>How to use?</b>
- Just use me in Inline Mode and, type anything you want, I'll generate a sticker for your text.
- example :- <code>@zaemonbot  what the dog doin?. bottom text</code>

<b>How to Choose Templates?</b>
- Just enter your text, and add 'Template Number' at the end of your text.
- example :- <code>@zaemonbot Yoo! I like this template 2</code>
- That '2' at the end will choose the template.
- If no number is given. The template would be random.

<i><a href="https://t.me/addstickers/gg1183697491_by_zaemonbot">click this to see all templates.</a></i>"#;

pub async fn info_handler<'a>(handler: &MessageHandler<'a>) -> Result<(), teloxide::RequestError> {
    let user = UserDB::from_user(&handler.msg.from().unwrap());
    if user.is_unique().await {
        let _ = user.save().await;
    }

    let mut deeplink = handler.msg.text().unwrap().to_string();
    deeplink = deeplink.replace("/start", "").replace("/help", "");

    if deeplink.len() > 0 {
        handler.reply_text(HELP_TEXT, None).await
    } else {
        let mut markup = InlineKeyboardMarkup::default();
        markup = markup.append_row([InlineKeyboardButton::new(
            "TAP For PREVIEW",
            InlineKeyboardButtonKind::SwitchInlineQuery(
                "kya haal hai bro. tell karo 1".to_string(),
            ),
        )]);
        handler.reply_text(START_TEXT, Some(markup)).await
    }

    
}

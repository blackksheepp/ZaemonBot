use indexmap::IndexMap;
use std::env::current_dir;
use std::fs;
use std::time::Duration;

use crate::bot::inline_handler::InlineHandler;
use crate::database::models::UserDB;
use crate::utils::image::position_text;

use log::error;
use teloxide::requests::Requester;
use teloxide::types::{InlineQueryResult, InlineQueryResultCachedSticker, InputFile, InputSticker};
use teloxide::RequestError;

fn templates<'a>(index: u32) -> &'a str {
    let mut tmplts = IndexMap::new();
    tmplts.insert("zaemon", "CAADBQADMQQAApZ-UFasgHDcn0O3ORYE");
    tmplts.insert("engremon", "CAADBQADMgQAApZ-UFbiNDMKa0-3OhYE");
    tmplts.insert("wetmon", "CAADBQADMwQAApZ-UFZ9LQ-d_IhAdRYE");
    tmplts.insert("sitmon", "CAADBQADOwQAApZ-UFb3YwX0CUY68hYE");
    tmplts.insert("susmon", "CAADBQADOgQAApZ-UFbIghFg-U3IyBYE");
    tmplts.insert("thinkmon", "CAADBQADNgQAApZ-UFYGP9_Eq7T7ZRYE");
    tmplts.insert("worriedmon", "CAADBQADTQQAApZ-UFYIq5YZB8heKxYE");
    tmplts.insert("curiousmon", "CAADBQADTgQAApZ-UFa04_ApX2AFhBYE");
    tmplts.insert("rtxon", "CAADBQADTwQAApZ-UFY7ReFlAhe45BYE");
    tmplts.insert("ganstamon", "CAADBQADUAQAApZ-UFaMEEYlresvIRYE");
    tmplts.insert("needanswersmon", "CAADBQADUQQAApZ-UFYHOanWQaKlRRYE");
    tmplts.insert("wrongpigmon", "CAADBQADUgQAApZ-UFavDL78mdcPtBYE");
    tmplts.insert("suprisedmon", "CAADBQADUwQAApZ-UFahYy1bN4mpRhYE");
    tmplts.insert("standinmon", "CAADBQADVAQAApZ-UFZxqVDfP6rgAhYE");
    tmplts.insert("sleepmodeon", "CAADBQADVQQAApZ-UFb13mrYxv5m-xYE");
    tmplts.insert("ranoutofnamesmon", "CAADBQADVgQAApZ-UFZ2ulwL63tZHBYE");
    tmplts.insert("sexymon", "CAADBQADVwQAApZ-UFYt87nL20henhYE");
    tmplts.insert("policimon", "CAADBQADWQQAApZ-UFZj8wThG568hRYE");
    tmplts.insert("searchmon", "CAADBQADWwQAApZ-UFbNAxmt_lky2RYE");
    tmplts.insert("scaredmon", "CAADBQADXAQAApZ-UFY3G7ADMCDGShYE");
    tmplts.insert("studymon", "CAADBQADXQQAApZ-UFauKwABRZpesnUWBA");
    tmplts.insert("eatingmon", "CAADBQADXwQAApZ-UFbxAYw-Wz6s9xYE");
    tmplts.insert("brocomeon", "CAADBQADYAQAApZ-UFZwqavqs7x1eBYE");

    tmplts.get_index(index as usize).unwrap().0
}

pub async fn inlinequery_handler<'a>(handler: &InlineHandler<'a>) -> Result<(), RequestError> {
    let user = UserDB::from_user(&handler.query.from);
    if user.is_unique().await {
        let _ = user.save().await;
    }

    let mut text = handler.query.query.to_string();
    if text.split_whitespace().count() == 0 {
        text = "Kya Haal hai Bro. Tell Karo 1".to_string();
    }

    let mut template = 25;
    let last = text.split_whitespace().last();
    if let Some(last) = last {
        if last.chars().all(|c| c.is_numeric()) {
            if let Ok(_) = last.parse::<u32>() {
                template = last.parse::<u32>().unwrap() - 1;
                text = text
                    .split_whitespace()
                    .take(text.split_whitespace().count() - 1)
                    .collect::<Vec<&str>>()
                    .join(" ");
            }
        }
    };

    let template_range = 0..23;
    template = if !template_range.contains(&template) {
        0
    } else {
        template
    };

    let set_name = "gg1183697491_by_zaemonbot";
    let pic = templates(template);

    match position_text(&handler.query.from.id.0, &text, pic) {
        Ok(mut sticker) => {
            match handler
                .bot
                .add_sticker_to_set(
                    handler.query.from.id,
                    set_name,
                    InputSticker::Png(InputFile::file(&sticker)),
                    "🐽",
                )
                .await
            {
                Ok(_) => {
                    let cd = current_dir().unwrap();
                    let file = cd.join(&sticker); 
                    match fs::remove_file(&file) {
                        Ok(_) => (),
                        Err(_) => (),
                    };
                },
                Err(e) => error!("Failed to add Sticker in Set. Error: {}", e),
            }

            match handler.bot.get_sticker_set(set_name).await {
                Ok(sticker_set) => {
                    sticker = sticker_set.stickers.last().unwrap().file.id.to_string();
                    let answer = handler
                        .answer([InlineQueryResult::CachedSticker(
                            InlineQueryResultCachedSticker::new(
                                format!("random|{}", handler.query.from.id),
                                &sticker,
                            ),
                        )])
                        .await;

                    match handler.bot.delete_sticker_from_set(&sticker).await {
                        Ok(_) => (),
                        Err(e) => error!("Failed to delete Sticker from Set. Error: {}", e),
                    }

                    

                    answer
                }
                Err(e) => {
                    error!("Failed to get Sticker Set. Error: {}", e);
                    Result::Err(e)
                }
            }
        }
        Err(_) => {
            error!("Failed to Write Text.");
            Result::Err(RequestError::RetryAfter(Duration::from_secs(5)))
        }
    }
}

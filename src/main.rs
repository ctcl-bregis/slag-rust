// SLAG - CTCL 2024
// File: src/main.rs
// Purpose: Main app code
// Modified: February 21, 2024
// Modified: March 7, 2024

use poise::serenity_prelude as serenity;
use log::{info, warn};
use sqlite;
use gethostname::gethostname;

mod users;
//use crate::users;

use slag::Data;
use slag::Error;
use slag::Context;

#[poise::command(slash_command)]
pub async fn placeholder(ctx: Context<'_>, #[description = "Selected user"] user: Option<serenity::User>,) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let token = std::fs::read_to_string("./token.txt").expect("Error when reading token.txt");
    // Going to need all of them
    let intents = serenity::GatewayIntents::privileged();

    let allcmds = vec![users::info()];
    //let allcmds = vec![crate::users::age()];
    
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: allcmds,
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    // This is here to know what system the bot is running on. Example status: "Watching from CTCL-WBPC1"
    let actname = String::from("from ") + &gethostname().into_string().unwrap();

    let client = serenity::ClientBuilder::new(token, intents).activity(serenity::ActivityData {name: actname, kind: serenity::ActivityType::Watching, state: None, url: None })
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap();
}
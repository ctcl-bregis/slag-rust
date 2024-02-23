// SLAG - CTCL 2024
// File: src/main.rs
// Purpose: Main app code
// Modified: February 21, 2024
// Modified: February 22, 2024

use poise::serenity_prelude as serenity;
use log::{info, warn};
use sqlite;
use gethostname::gethostname;

use users::users;

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Displays your or another user's account creation date
#[poise::command(slash_command, prefix_command)]
async fn age(ctx: Context<'_>, #[description = "Selected user"] user: Option<serenity::User>,) -> Result<(), Error> {
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

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![age()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    // This is here to know what system the bot is running on. Example status: "slag-rust on CTCL-WBPC1"
    let actname = String::from("slag-rust on ") + &gethostname().into_string().unwrap();

    let client = serenity::ClientBuilder::new(token, intents).activity(serenity::ActivityData {name: actname, kind: serenity::ActivityType::Playing, state: None, url: None })
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap();
}
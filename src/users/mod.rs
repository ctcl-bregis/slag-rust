// SLAG - CTCL 2024
// File: src/users.rs
// Purpose: Users commands
// Modified: February 22, 2024
// Modified: March 7, 2024

use poise::serenity_prelude::Event;
use poise::serenity_prelude as serenity;
use poise::async_trait;
use poise::serenity_prelude::{Embed, EmbedAuthor, Message, EventHandler, EmbedField};
use poise::serenity_prelude::builder::CreateEmbed;
use poise::serenity_prelude::{User, Member};
use std::borrow::Borrow;
use log::error;

use crate::Data;
use crate::Error;
use crate::Context;

fn user2embed(user: User) -> CreateEmbed {
    let mut embed = CreateEmbed::new()
        .title("User Information")
        .colour(user.accent_colour.unwrap());

    embed
}

fn member2embed(member: Member) -> CreateEmbed {
    let mut embed = CreateEmbed::new()
        .title("User Information")
        .colour(member.user.accent_colour.unwrap());

    embed
}

// Displays your or another user's account creation date
//#[poise::command(slash_command)]
//pub async fn age(ctx: Context<'_>, #[description = "Selected user"] user: Option<serenity::User>,) -> Result<(), Error> {
//    let u = user.as_ref().unwrap_or_else(|| ctx.author());
//    let response = format!("{}'s account was created at {}", u.name, u.created_at());
//    ctx.say(response).await?;
//    Ok(())
//}



#[poise::command(slash_command)]
pub async fn info(ctx: Context<'_>, #[description = "User"] selecteduser: Option<serenity::Member>) -> Result<(), Error> {
    // Default to the user that invoked the command
    let embed: CreateEmbed;
    if selecteduser.is_none() {
        if ctx.author_member().await.is_none() {
            let user: User = ctx.author().clone();
            embed = user2embed(user);
        } else {
            let user: Member = ctx.author_member().await.unwrap().into_owned();
            embed = member2embed(user);
        }
    } else {
        let user: Member = selecteduser.unwrap();
        embed = member2embed(user);
    }


    
    ctx.send(poise::CreateReply::default().embed(embed)).await?;

    Ok(())
}
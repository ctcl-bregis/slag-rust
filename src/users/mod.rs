// SLAG - CTCL 2024
// File: src/users/mod.rs
// Purpose: Users commands
// Modified: February 22, 2024
// Modified: March 15, 2024

use poise::serenity_prelude::{Colour, Embed, EmbedAuthor, EmbedField, EventHandler, Http, Message, Presence};
use poise::serenity_prelude::builder::CreateEmbed;
use poise::serenity_prelude::{User, Member};
use log::error;
use phf::phf_map;

use crate::Data;
use crate::Error;
use crate::Context;

const DATE_FORMAT: &str = "%B %d, %Y %I:%M %p";

pub struct Month {
    num: u8,
    days: u8
}

const MONTHS: phf::Map<&'static str, Month> = phf_map! {
    "january" => Month {num: 1, days: 31},
    "february" => Month {num: 2, days: 29},
    "march" => Month {num: 3, days: 31},
    "april" => Month {num: 4, days: 30},
    "may" => Month {num: 5, days: 31},
    "june" => Month {num: 6, days: 30},
    "july" => Month {num: 7, days: 31},
    "august" => Month {num: 8, days: 31},
    "september" => Month {num: 9, days: 30},
    "october" => Month {num: 10, days: 31},
    "november" => Month {num: 11, days: 30},
    "december" => Month {num: 12, days: 31}
};

#[poise::command(slash_command)]
pub async fn info(ctx: Context<'_>, #[description = "User"] selecteduser: Option<User>) -> Result<(), Error> {
    let user: User;
    let member: Option<Member>;

    // Default to the user that invoked the command
    if selecteduser.is_none() {
        // If not member
        user = ctx.author().clone();
        if ctx.author_member().await.is_some() {
            member = Some(ctx.author_member().await.unwrap().as_ref().clone());
        } else {
            member = None;
        }
    } else {
        // If the user is not a member of the current guild, just get User information
        member = match ctx.http().get_member(ctx.guild_id().unwrap(), selecteduser.as_ref().unwrap().id).await {
            Ok(e) => Some(e),
            Err(_) => None,
        };
        user = selecteduser.unwrap(); 
    }

    // Default to #7F7F7F
    let mut embed_color: Colour = Colour::from_rgb(127, 127, 127);
    if let Some(accent_colour) = user.accent_colour {
        embed_color = accent_colour;
    }

    let mut embed = CreateEmbed::new()
        .title(format!("User Information for {}", &user.name))
        .colour(embed_color);
    
    if member.as_ref().is_some() {
        // Member not being "None" is known now
        let umember = member.clone().unwrap();
        if umember.nick.is_some() {
            embed = embed.field("Nickname", umember.nick.unwrap(), true);
            embed = embed.field("Name", &user.name, true);
        } else {
            embed = embed.field("Name", &user.name, true);
        }
    } else {
        embed = embed.field("Name", &user.name, true);
    }
    embed = embed.field("User ID", user.id.to_string(), true);

    if member.as_ref().is_some() {
        embed = embed.field("Joined Guild", member.as_ref().unwrap().joined_at.unwrap().format(DATE_FORMAT).to_string(), false);
    }

    embed = embed.field("Joined Discord", user.created_at().format(DATE_FORMAT).to_string(), false);
    let userpresence: Presence = ctx.guild().unwrap().presences.get(&user.id).unwrap().clone();
    if userpresence.client_status.is_some() {
        let cs = userpresence.client_status.unwrap();
        if cs.desktop.is_some() {
            embed = embed.field("On Desktop", cs.desktop.unwrap().name(), true);
        } else {
            embed = embed.field("On Desktop", "offline", true);
        }
        if cs.mobile.is_some() {
            embed = embed.field("On Mobile", cs.mobile.unwrap().name(), true);
        } else {
            embed = embed.field("On Mobile", "offline", true);
        }
        if cs.web.is_some() {
            embed = embed.field("On Web", cs.web.unwrap().name(), true);
        } else {
            embed = embed.field("On Web", "offline", true);
        }
    }

    if user.email.is_some() {
        embed = embed.field("User Email", user.email.unwrap().to_string(), true);
    }
    
    ctx.send(poise::CreateReply::default().embed(embed)).await?;

    Ok(())
}
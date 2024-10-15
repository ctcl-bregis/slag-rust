// SLAG - CTCL 2024
// File: src/micron/mod.rs
// Purpose: Micron FBGA lookup and decoder module
// Modified: February 22, 2024
// Modified: August 27, 2024

use phf::phf_map;
use poise::serenity_prelude::{async_trait, EventHandler, Ready};
use regex::Regex;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::Data;
use crate::Error;
use crate::Context;

#[derive(Deserialize)]
struct MicronResponse {
    date: String,
    #[serde(alias = "response-code")]
    response_code: String,
    details: Vec<MicronResponseDetails>
}

#[derive(Deserialize)]
struct MicronResponseDetails {
    #[serde(alias = "part-number")]
    part_number: String,
    #[serde(alias = "part-key")]
    part_key: String,
    #[serde(alias = "part-name")]
    part_name: String, 
    #[serde(alias = "sub-category")]
    subcategory: String,
    #[serde(alias = "fbga-code")]
    fbga_code: String,
    pageurl: String
}

fn devinfo(pn: &str) -> String {


    "placeholder".to_string()
}

#[poise::command(slash_command)]
pub async fn fbga(ctx: Context<'_>, #[description = "FBGA code"] fbgacode: Option<String>) -> Result<(), Error> {
    

    Ok(())
}
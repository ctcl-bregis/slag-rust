// SLAG - CTCL 2024
// File: src/micron/mod.rs
// Purpose: Micron FBGA lookup and decoder module
// Modified: February 22, 2024
// Modified: March 20, 2024

use phf::phf_map;
use poise::serenity_prelude::{async_trait, EventHandler, Ready};
use regex::Regex;
use once_cell::sync::Lazy;

use crate::Data;
use crate::Error;
use crate::Context;

struct DramType<'a> {
    dtype: &'a str,
    voltage: &'a str,
}

// numdram.xlsx Rev: May 4, 2023
// DRAM and voltage key are combined here since types can share the same Product Family number, e.g. LPDDR2 and DDR3 both starting with MT41
const DRAM_TYPES: phf::Map<&'static str, DramType> = phf_map! {
    "40A" => DramType {dtype: "DDR4 SDRAM", voltage: "1.2"},
    "41J" => DramType {dtype: "DDR3 SDRAM", voltage: "1.5"},
    "41K" => DramType {dtype: "DDR3 SDRAM", voltage: "1.35"},
    "41L" => DramType {dtype: "LPDDR2 Mobile", voltage: "1.2"},
    "44K" => DramType {dtype: "RLDRAM3", voltage: "1.35"},
    "46V" => DramType {dtype: "DDR1 SDRAM", voltage: "2.5"},
    "46H" => DramType {dtype: "DDR1 SDRAM", voltage: "1.8"},
    "47H" => DramType {dtype: "DDR2 SDRAM", voltage: "1.35"},
    "48H" => DramType {dtype: "SDRAM", voltage: "1.8"},
    "48HC" => DramType {dtype: "LPSDRAM Mobile", voltage: "1.8"},
    "48L" => DramType {dtype: "SDR SDRAM", voltage: "3.3"},
    "49H" => DramType {dtype: "RLDRAM2", voltage: "1.8"},
    "51J" => DramType {dtype: "GDDR5", voltage: "1.5"},
    "51K" => DramType {dtype: "GDDR5", voltage: "1.4"},
    "52H" => DramType {dtype: "LPDDR3 Mobile", voltage: "1.8"},
    "52K" => DramType {dtype: "DDR3L Mobile", voltage: "1.35"},
    "52L" => DramType {dtype: "LPDDR3 Mobile", voltage: "1.2"},
    "53B" => DramType {dtype: "LPDRR4 Mobile", voltage: "1.1"},
    "53D" => DramType {dtype: "LPDDR4X Mobile", voltage: "1.1"},
    "53E" => DramType {dtype: "LPDDR4/LPDDR4X Mobile", voltage: "1.1"},
    "58K" => DramType {dtype: "GDDR5/GDDR5X", voltage: "1.35"},
    "58M" => DramType {dtype: "GDDR5X", voltage: "1.25"},
    "60B" => DramType {dtype: "DDR5 SDRAM", voltage: "1.1"},
    "61A" => DramType {dtype: "GDDR6", voltage: "1.2"},
    "61K" => DramType {dtype: "GDDR6/GDDR6X", voltage: "1.35"},
    "61M" => DramType {dtype: "GDDR6/GDDR6X", voltage: "1.25"},
    "62F" => DramType {dtype: "LPDDR5/LPDDR5X Mobile", voltage: "1.05"},
    "68A" => DramType {dtype: "GDDR7", voltage: "1.2"},
    "68B" => DramType {dtype: "GDDR7", voltage: "1.1"},
};

// CSN-11 Rev: AV 12/23 EN
const WEEKS: phf::Map<&'static str, &'static str> = phf_map! {
    "A" => "2",
    "B" => "4",
    "C" => "6",
    "D" => "8",
    "E" => "10",
    "F" => "12",
    "G" => "14",
    "H" => "16",
    "I" => "18",
    "J" => "20",
    "K" => "22",
    "L" => "24",
    "M" => "26",
    "N" => "28",
    "O" => "30",
    "P" => "32",
    "Q" => "34",
    "R" => "36",
    "S" => "38",
    "T" => "40",
    "U" => "42",
    "V" => "44",
    "W" => "46",
    "X" => "48",
    "Y" => "50",
    "Z" => "52"
};

// CSN-11 Rev: AV 12/23 EN
const LOCATIONS: phf::Map<&'static str, &'static str> = phf_map! {
    "1" => "USA",
    "2" => "Singapore",
    "3" => "Italy",
    "4" => "Japan",
    "5" => "China",
    "7" => "Taiwan",
    "8" => "Korea",
    "9" => "Mixed",
    "B" => "Israel",
    "C" => "Ireland",
    "D" => "Malaysia",
    "F" => "Philippines"
};

fn devinfo(pn: &str) -> String {


    "placeholder".to_string()
}

static DRAM_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(MT)([0-9]{2}[A-Z]{1,2})([1-9]{1,3})(K|M|G)([1-9]{1,2})([A-Z0-9]{2})?([A-Z0-9]{1,3})(-[0-9]{1,3}(?:E|B|H)?) ?([A-Z]{1,2})?([A-Z][A-Z])?(:[A-Z])?").unwrap());

#[poise::command(slash_command)]
pub async fn fbga(ctx: Context<'_>, #[description = "FBGA code"] fbgacode: Option<String>) -> Result<(), Error> {
    

    Ok(())
}

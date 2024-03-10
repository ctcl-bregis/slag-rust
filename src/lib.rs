// SLAG - CTCL 2024
// File: src/lib.rs
// Purpose: Commonly used types and functions
// Modified: March 7, 2024
// Modified: March 7, 2024

use poise::serenity_prelude::Embed;
use indexmap::IndexMap;

pub struct Data {} // User data, which is stored and accessible in all command invocations
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

pub mod users;
pub mod tags;
pub mod micron;

pub const DATE_FORMAT: &str = "%B %d, %Y %I:%M %p";

pub struct Month {
    num: u8,
    days: u8
}

pub fn remove_first(s: &str) -> Option<&str> {
    s.chars().next().map(|c| &s[c.len_utf8()..])
}


pub fn getmonths() -> IndexMap<String, Month> {
    let mut months = IndexMap::new();

    months.insert(String::from("january"), Month {num: 1, days: 31});
    months.insert(String::from("february"), Month {num: 2, days: 29});
    months.insert(String::from("march"), Month {num: 3, days: 31});
    months.insert(String::from("april"), Month {num: 4, days: 30});
    months.insert(String::from("may"), Month {num: 5, days: 31});
    months.insert(String::from("june"), Month {num: 6, days: 30});
    months.insert(String::from("july"), Month {num: 7, days: 31});
    months.insert(String::from("august"), Month {num: 8, days: 31});
    months.insert(String::from("september"), Month {num: 9, days: 30});
    months.insert(String::from("october"), Month {num: 10, days: 31});
    months.insert(String::from("november"), Month {num: 11, days: 30});
    months.insert(String::from("december"), Month {num: 12, days: 31});

    months
}
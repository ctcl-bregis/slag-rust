// SLAG - CTCL 2024
// File: src/lib.rs
// Purpose: Commonly used types and functions
// Modified: March 7, 2024
// Modified: March 16, 2024

pub struct Data {} // User data, which is stored and accessible in all command invocations
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

pub mod base;
pub mod users;
pub mod tags;
pub mod micron;

pub fn remove_first(s: &str) -> Option<&str> {
    s.chars().next().map(|c| &s[c.len_utf8()..])
}

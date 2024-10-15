// SLAG - CTCL 2024
// File: src/base/mod.rs
// Purpose: Base commands
// Modified: March 14, 2024
// Modified: August 27, 2024

use std::env;

use poise::serenity_prelude::{Embed, EmbedAuthor, Message, EventHandler, EmbedField, Colour, Http};
use poise::serenity_prelude::builder::CreateEmbed;
use poise::serenity_prelude::{User, Member};
use log::error;
use gethostname::gethostname;
use std::process::Command;

use crate::Data;
use crate::Error;
use crate::Context;

#[poise::command(slash_command)]
pub async fn sysinfo(ctx: Context<'_>) -> Result<(), Error> {
    let mut embed = CreateEmbed::new()
        .title("Host System Information")
        .colour(Colour::from(10066329));

    if gethostname().to_str().is_some() {
        embed = embed.field("System Host Name", gethostname().to_str().unwrap(), false);
    }

    embed = match env::var("hwcodename") {
        Ok(ev) => embed.field("Host system codename", ev, true),
        Err(_) => embed
    };

    embed = match env::var("hwshcodename") {
        Ok(ev) => embed.field("Host system shorthand codename", ev, true),
        Err(_) => embed
    };

    embed = match env::var("hwtype") {
        Ok(ev) => embed.field("Host system type", ev, true),
        Err(_) => embed
    };

    // uname -a should be available on Linux, Android, FreeBSD, OpenBSD, NetBSD, DragonFly BSD and MacOS
    if cfg!(any(target_os = "linux", target_os = "android", target_os = "freebsd", target_os = "openbsd", target_os = "netbsd", target_os = "dragonfly", target_os = "macos")) {
        let unamea: String = String::from_utf8(Command::new("uname").args(["-a"]).output().expect("failed to execute process").stdout).unwrap();
        embed = embed.field("uname -a", unamea, false);
    };

    // x86(-64) specific feature
    if cfg!(any(target_arch = "x86_64", target_arch = "x86")) {
        use raw_cpuid::CpuId;
        let cpuid = CpuId::new();

        let cpuvendor = cpuid.get_vendor_info().map(|vi| match vi.as_str() {
                "AuthenticAMD" => "AMD",
                "GenuineIntel" => "Intel",
                // Congrats to anyone who manages to run this bot on the following CPUs...
                "AMDisbetter!" => "AMD",
                "VIA VIA VIA " => "VIA",
                "GenuineTMx86" => "Transmeta",
                "TransmetaCPU" => "Transmeta",
                "CentaurHauls" => "VIA/Centaur",
                "CyrixInstead" => "Cyrix",
                "NexGenDriven" => "NexGen",
                "UMC UMC UMC " => "UMC",
                "SiS SiS SiS " => "SiS",
                "Geode by NSC" => "National Semiconductor", // Could this be AMD too?
                "RiseRiseRise" => "Rise",
                "Vortex86 SoC" => "DM&P",
                "MiSTer AO486" => "AO486",
                "GenuineAO486" => "AO486",
                "  Shanghai  " => "Zhaoxin (VIA)",
                "HygonGenuine" => "Hygon",
                "E2K MACHINE " => "MCST Elbrus",
                _ => "Unknown"
                // TODO: add hypervisor CPUs if needed
            });
        embed = match cpuvendor {
            Some(cpu) => embed.field("CPU Vendor", cpu, true),
            None => embed
        };
    
        embed = match cpuid.get_processor_brand_string() {
            Some(cpu) => embed.field("CPU", cpu.as_str(), true),
            None => embed
        };
    }

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}

pub async fn botinfo(ctx: Context<'_>) -> Result<(), Error> {
    let mut embed = CreateEmbed::new()
        .title("Bot Information")
        .colour(Colour::from(10066329));



    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}
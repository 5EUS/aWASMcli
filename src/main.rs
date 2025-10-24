mod cli;

use cli::Cli;
use clap::Parser;
use anyhow::Result;
use awasmlib::Handle;

use crate::cli::{Commands, DownloadCmd, SeriesCmd};

#[tokio::main]
async fn main() -> Result<()> {
    let mut handle = Handle::new().await?;
    handle.connect().await?;
    handle.load_plugins().await?;

    let cli = Cli::parse();

    match cli.command {
        Commands::Plugins { name } => {
            let plugins = handle.agg.pm.list_plugins();
            if let Some(name) = name {
                println!("Filtering plugins by name: {}", name);
                for plugin_name in plugins.into_iter().filter(|p| p.contains(&name)) {
                    println!("  - {}", plugin_name);
                }
            } else {
                println!("Listing all plugins...");
                for plugin_name in plugins {
                    println!("  - {}", plugin_name);
                }
            }
        }
        Commands::ResolveSeriesId { source, external_id } => {
            // Resolve series ID logic
        }
        Commands::Capabilities { refresh } => {
            let caps = handle.agg.pm.get_all_capabilities(refresh).await?;
            for (name, c) in caps {
                let media: Vec<String> = c.media_types.into_iter().map(|m| format!("{:?}", m)).collect();
                let units: Vec<String> = c.unit_kinds.into_iter().map(|u| format!("{:?}", u)).collect();
                let assets: Vec<String> = c.asset_kinds.into_iter().map(|a| format!("{:?}", a)).collect();
                println!("{}:\n  media:  {}\n  units:  {}\n  assets: {}", name, media.join(", "), units.join(", "), assets.join(", "));
            }
        }
        Commands::AllowedHosts => {
            // Allowed hosts logic
        }
        Commands::Media { query, refresh, json, plugin } => {
            // Media logic
        }
        Commands::Units { media_id, refresh } => {
            // Units logic
        }
        Commands::RefreshCache { prefix } => {
            // Refresh cache logic
        }
        Commands::VacuumDb => {
            // Vacuum database logic
        }
        Commands::Download { cmd } => match cmd {
            DownloadCmd::Chapter { chapter_id, out, cbz, force, mock } => {
                // Download chapter logic
            }
            DownloadCmd::Episode { episode_id, out, index } => {
                // Download episode logic
            }
            DownloadCmd::Series { series_id, out, cbz, force } => {
                // Download series logic
            }
        },
        Commands::Series { cmd } => match cmd {
            SeriesCmd::List { kind } => {
                // List series logic
            }
            SeriesCmd::SetPath { series_id, path } => {
                // Set path logic
            }
            SeriesCmd::Delete { series_id } => {
                // Delete series logic
            }
            SeriesCmd::DeleteChapter { chapter_id } => {
                // Delete chapter logic
            }
            SeriesCmd::DeleteEpisode { episode_id } => {
                // Delete episode logic
            }
        },
    }

    Ok(())
}

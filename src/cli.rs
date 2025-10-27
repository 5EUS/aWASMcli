use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "aWASMcli")]
#[command(about = "A CLI for aWASMlib.", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// List all available plugins
    Plugins {
        /// Filter plugins by name
        #[arg(short, long)]
        name: Option<String>,
    },
    /// Show plugin capabilities (cached by default)
    Capabilities {
        /// Bypass cache and refresh capabilities from plugins
        #[arg(long)]
        refresh: bool,
    },
    /// Show allowed hosts per plugin
    AllowedHosts {
        /// Plugin name to get allowed hosts for
        #[arg(long)]
        plugin: Option<String>,
    },
    /// Search for media
    Media {
        /// Query to search for
        query: String,
        /// Bypass cache and force refresh
        #[arg(long)]
        refresh: bool,
        /// Output JSON for machine readability
        #[arg(long)]
        json: bool,
        /// Optional plugin to restrict search to (by name)
        #[arg(long)]
        plugin: Option<String>,
    },
    /// Get units for a specific media item
    Units {
        /// Media ID to get units for
        media_id: String,
        /// Bypass cache and force refresh
        #[arg(long)]
        refresh: bool,
    },
    /// Refresh cache for a given key prefix (e.g., search) by forcing refresh on next access
    RefreshCache {
        /// Optional key prefix to clear (defaults to all)
        #[arg(long)]
        prefix: Option<String>,
    },
    /// Vacuum/compact the database (SQLite only; no-op for others)
    VacuumDb,
    /// Download helpers
    Download {
        #[command(subcommand)]
        cmd: DownloadCmd,
    },
    /// Manage series stored in the database
    Series {
        #[command(subcommand)]
        cmd: SeriesCmd,
    },
    /// Resolve canonical series id from a plugin source and external media id
    ResolveSeriesId {
        /// Plugin source id (e.g., mangadex_plugin)
        source: String,
        /// External media id as reported by the plugin/search
        external_id: String,
    },
}

#[derive(Subcommand)]
pub enum SeriesCmd {
    /// List series (optionally by kind)
    List {
        /// Filter series by kind (e.g., manga, anime)
        #[arg(long)]
        kind: Option<String>,
    },
    /// Set or clear the download path for a series
    SetPath {
        /// Series ID to set the download path for
        series_id: String,
        /// New download path (leave empty to clear)
        #[arg(long)]
        path: Option<String>,
    },
    /// Delete a series (cascades to chapters/episodes/streams/images)
    Delete {
        /// Series ID to delete
        series_id: String,
    },
    /// Delete a single chapter by id
    DeleteChapter {
        /// Chapter ID to delete
        chapter_id: String,
    },
    /// Delete a single episode by id
    DeleteEpisode {
        /// Episode ID to delete
        episode_id: String,
    },
}

#[derive(Subcommand)]
pub enum DownloadCmd {
    /// Download all images for a chapter to a directory (or a cbz file)
    Chapter {
        /// Chapter ID
        chapter_id: String,
        /// Output directory (created if missing). If --cbz is used, this is the .cbz path. If omitted, use the series download_path and auto-name.
        #[arg(long)]
        out: Option<String>,
        /// Create a .cbz instead of files on disk
        #[arg(long)]
        cbz: bool,
        /// Overwrite existing files
        #[arg(long)]
        force: bool,
        /// Mock mode: generate N dummy images instead of fetching from the network
        #[arg(long, default_value_t = 0)]
        mock: usize,
    },
    /// Download a video stream (HLS/DASH not yet muxed) to a file
    Episode {
        /// Episode ID
        episode_id: String,
        /// Output file. If omitted, use the series download_path and auto-name.
        #[arg(long)]
        out: Option<String>,
        /// Select stream by index (default 0)
        #[arg(long, default_value_t = 0)]
        index: usize,
    },
    /// Download a whole series (all chapters for manga or all episodes for anime)
    Series {
        /// Series ID (canonical)
        series_id: String,
        /// Output directory; if omitted, use the stored series download_path or a folder named from the title
        #[arg(long)]
        out: Option<String>,
        /// For manga, create .cbz files instead of folders with images
        #[arg(long)]
        cbz: bool,
        /// Overwrite existing files
        #[arg(long)]
        force: bool,
    },
}
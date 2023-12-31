use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct RustFlixArgs {
    #[clap(subcommand)]
    pub entity_type: EntityType,
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    /// create, update, delete or show users
    User(UserCommand),
    /// create, update, delete or show videos
    Video(VideoCommand),
    /// create, or show views
    View(ViewCommand),
}

/// user

#[derive(Debug, Args)]
pub struct UserCommand {
    #[clap(subcommand)]
    pub command: UserSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum UserSubcommand {
    /// create a user
    Create(CreateUser),
    /// update a user
    Update(UpdateUser),
    /// delete a user
    Delete(DeleteEntity),
    // show all users
    Show,
}

#[derive(Debug, Args)]
pub struct CreateUser {
    /// name of the user
    pub name: String,
    /// email of the user
    pub email: String,
}

#[derive(Debug, Args)]
pub struct UpdateUser {
    /// The id of the user to update
    pub id: i32,

    /// The name of the user
    pub name: String,

    /// The email of the user
    pub email: String,
}

#[derive(Debug, Args)]
pub struct DeleteEntity {
    /// The id of the entity to delete
    pub id: i32,
}

/// Video
#[derive(Debug, Args)]
pub struct VideoCommand {
    #[clap(subcommand)]
    pub command: VideoSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum VideoSubcommand {
    /// Create a new video
    Create(CreateVideo),

    /// Update an existing video
    Update(UpdateVideo),

    /// Delete a video
    Delete(DeleteEntity),

    /// Show all videos
    Show,
}

#[derive(Debug, Args)]
pub struct CreateVideo {
    /// The title of the video to create
    pub title: String,

    /// The description of the video to create
    pub description: String,
}

#[derive(Debug, Args)]
pub struct UpdateVideo {
    /// The id of the video to update
    pub id: i32,

    /// The title of the video
    pub title: String,

    /// The description of the video
    pub description: String,
}

/// view
#[derive(Debug, Args)]
pub struct ViewCommand {
    #[clap(subcommand)]
    pub command: ViewSubcommands,
}

#[derive(Debug, Subcommand)]
pub enum ViewSubcommands {
    /// Create a new view
    Create(CreateView),

    /// Show all views with id numbers for users and videos
    Show,

    /// Show all views with names for users and videos
    ShowPretty,
}

#[derive(Debug, Args)]
pub struct CreateView {
    /// The id of the user who watched the video
    pub user_id: i32,

    /// The id of the video the user watched
    pub video_id: i32,

    /// The time the user watched the video
    /// the format should be YYYY-MM-DDTHH:MM:SS
    pub watch_start: chrono::NaiveDateTime,

    /// How long the user watched the video for
    pub duration: i32,
}

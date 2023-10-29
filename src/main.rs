#[macro_use]
extern crate diesel;

mod args;
mod db;
mod models;
mod ops;
mod schema;

use args::RustFlixArgs;
use clap::Parser;
use ops::user_ops::handler_user_command;
use ops::video_ops::handler_video_command;
use ops::view_ops::handler_view_command;

// #[derive(Parser)]
// struct Args {
//     #[clap(short = 'n', long = "name")]
//     name: String,
// }

fn main() {
    // let matches = Args::parse();

    // println!("hello {}", matches.name)

    let args = RustFlixArgs::parse();

    match args.entity_type {
        args::EntityType::User(user) => handler_user_command(user),
        args::EntityType::Video(video) => handler_video_command(video),
        args::EntityType::View(view) => handler_view_command(view),
    }
}

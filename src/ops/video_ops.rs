use crate::args::{CreateVideo, DeleteEntity, UpdateVideo, VideoCommand, VideoSubcommand};
use crate::db::establish_connection;
use crate::models::{NewVideo, Video};
use diesel::prelude::*;

pub fn handler_video_command(video: VideoCommand) {
    let command = video.command;

    match command {
        VideoSubcommand::Create(video) => {
            println!("{:?}", video);
            create_video(video)
        }
        VideoSubcommand::Update(video) => {
            println!("{:?}", video);
            update_video(video)
        }
        VideoSubcommand::Delete(delete_entity) => {
            println!("{:?}", delete_entity);
            delete_video(delete_entity)
        }
        VideoSubcommand::Show => {
            println!("showing");
            show_videos()
        }
    }
}
pub fn create_video(video: CreateVideo) {
    use crate::schema::videos::dsl::*;
    let connection = establish_connection();

    let new_video = NewVideo {
        title: &video.title,
        description: &video.description,
        removed: false,
    };

    diesel::insert_into(videos)
        .values(&new_video)
        .execute(&connection)
        .expect("Error saving new video");
}
pub fn delete_video(video: DeleteEntity) {
    println!("Deleting video: {:?}", video);
    use crate::schema::videos::dsl::*;

    let connection = establish_connection();

    diesel::update(videos.filter(id.eq(&video.id)))
        .set(removed.eq(true))
        .execute(&connection)
        .expect("Error Deleting");
}
pub fn update_video(video: UpdateVideo) {
    use crate::schema::videos::dsl::*;

    let connection = establish_connection();

    let db_video = Video {
        description: video.description,
        id: video.id,
        title: video.title,
        removed: false,
    };

    diesel::update(videos.find(video.id))
        .set(&db_video)
        .execute(&connection)
        .expect("Error Updating the Video");
}
pub fn show_videos() {
    println!("Showing videos");
    use crate::schema::videos::dsl::*;

    let connection = establish_connection();

    let results = videos
        .filter(removed.eq(false))
        .load::<Video>(&connection)
        .expect("Error Loading videos");

    println!("Displaying {} videos", results.len());

    for video in results {
        println!("{:?}", video);
    }
}

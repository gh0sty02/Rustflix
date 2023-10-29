use crate::{
    args::{CreateView, ViewCommand, ViewSubcommands},
    db::establish_connection,
    models::{NewView, View as DbView},
};

use diesel::prelude::*;

pub fn handler_view_command(view: ViewCommand) {
    let command = view.command;

    match command {
        ViewSubcommands::Create(view) => {
            println!("creating view");

            create_view(view)
        }

        ViewSubcommands::Show => {
            println!("showing");
            show_views();
        }
        ViewSubcommands::ShowPretty => {
            println!("showing pretty");
            show_views_pretty()
        }
    }
}
pub fn create_view(view: CreateView) {
    use crate::schema::views::dsl::*;

    let connection = establish_connection();

    let new_view = NewView {
        user_id: view.user_id,
        video_id: view.video_id,
        duration: view.duration,
        watch_start: &view.watch_start,
    };

    diesel::insert_into(views)
        .values(new_view)
        .execute(&connection)
        .expect("Error creating view");
}
pub fn show_views() {
    println!("Showing views");

    use crate::schema::views::dsl::*;

    let connection = establish_connection();

    let results = views
        .load::<DbView>(&connection)
        .expect("Error loading views");

    println!("Displaying {} views", results.len());
    for view in results {
        println!("{:?}", view);
    }
}
pub fn show_views_pretty() {
    use crate::schema::users;
    use crate::schema::videos;
    use crate::schema::views;
    let connection = establish_connection();

    let results = views::table
        .inner_join(users::table)
        .inner_join(videos::table)
        .select((
            users::name,
            videos::title,
            views::watch_start,
            views::duration,
        ))
        .load::<(String, String, chrono::NaiveDateTime, i32)>(&connection)
        .expect("error loading views");

    println!("Displaying {} views", results.len());
    for view in results {
        println!("{:?}", view);
    }
}

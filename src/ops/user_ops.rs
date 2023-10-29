use crate::{
    args::{CreateUser, DeleteEntity, UpdateUser, UserCommand, UserSubcommand},
    db::establish_connection,
    models::{NewUser, User},
};
use diesel::prelude::*;

pub fn handler_user_command(user: UserCommand) {
    let command = user.command;

    match command {
        UserSubcommand::Create(user) => {
            println!("Creating User");

            create_user(user);
        }
        UserSubcommand::Update(user) => {
            println!("Updating User");
            update_user(user)
        }
        UserSubcommand::Delete(delete_entity) => {
            println!("Deleting User");
            delete_user(delete_entity)
        }
        UserSubcommand::Show => {
            println!("showing users");
            show_users();
        }
    }
}
pub fn create_user(user: CreateUser) {
    use crate::schema::users::dsl::*;
    let connection = establish_connection();

    let new_user = NewUser {
        name: &user.name,
        email: &user.email,
        removed: false,
    };

    diesel::insert_into(users)
        .values(&new_user)
        .execute(&connection)
        .expect("Error saving new user");

    println!("User Created Successfully");
}
pub fn update_user(user: UpdateUser) {
    use crate::schema::users::dsl::*;
    let connection = establish_connection();

    let db_user = User {
        id: user.id,
        name: user.name,
        email: user.email,
        removed: false,
    };

    diesel::update(users.find(user.id))
        .set(&db_user)
        .execute(&connection)
        .expect("Failed to Update User");
}

pub fn delete_user(user: DeleteEntity) {
    use crate::schema::users::dsl::*;
    let connection = establish_connection();

    diesel::update(users.find(user.id))
        .set(removed.eq(true))
        .execute(&connection)
        .expect("Error Deleting User");
}
pub fn show_users() {
    use crate::schema::users::dsl::*;
    let connection = establish_connection();

    let results = users
        .filter(removed.eq(false))
        .load::<User>(&connection)
        .expect("Error Loading users");

    for user in results {
        println!("{:?}", user)
    }
}

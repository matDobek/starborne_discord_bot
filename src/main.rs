#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use diesel::pg::PgConnection;
use diesel::prelude::*;

use dotenv::dotenv;
use std::env;

use serenity::{
    client::Client,
    framework::standard::{
        macros::{command, group},
        CommandResult, StandardFramework,
    },
    model::{channel::Message, gateway::Ready},
    prelude::*,
    utils::MessageBuilder,
};

use self::models::{NewUser, User};

#[group]
#[commands(claim)]
struct General;
struct Handler;

impl EventHandler for Handler {}

fn main() {
    dotenv().ok();

    let discord_token = &env::var("DISCORD_TOKEN").expect("token");
    let mut client = Client::new(&discord_token, Handler).expect("Error creating client");

    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.prefix("/"))
            .group(&GENERAL_GROUP),
    );

    println!("");
    println!("-------------------------");
    println!("INFO: Server is running!");
    println!("-------------------------");
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
fn claim(ctx: &mut Context, msg: &Message) -> CommandResult {
    println!("INFO: Claim command invoked!");

    let connection = establish_connection();

    let cmd = String::from("/claim");
    let user = create_user(&connection, &msg.author.id.to_string(), &msg.author.name);
    let msg_without_cmd = &msg.content[cmd.len()..msg.content.len()];

    println!("DEBUG: '{:?}'", msg_without_cmd);

    let position: Vec<i32> = msg_without_cmd
        .trim()
        .replace(",", " ")
        .split(" ")
        .filter_map(|x| x.trim().parse().ok())
        .collect();

    println!("DEBUG: {:?}", position);

    let x = position[0];
    let y = position[1];

    let response = MessageBuilder::new()
        .push_bold_safe(user.username)
        .push(" ( ")
        .push_bold_safe(user.discord_id)
        .push(" )")
        .push(" has claimed ")
        .push("/goto ")
        .push(x)
        .push(" ")
        .push(y)
        .build();

    if let Err(why) = msg.channel_id.say(&ctx.http, &response) {
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn create_user<'a>(conn: &PgConnection, discord_id: &'a str, username: &'a str) -> User {
    use schema::users;

    let new_user = NewUser {
        discord_id: discord_id,
        username: username,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn show_users() {
    use self::schema::users::dsl::*;

    let connection = establish_connection();
    let results = users
        .limit(5)
        .load::<User>(&connection)
        .expect("Error loading users");

    println!("\nDisplaying {} users\n", results.len());
    for user in results {
        println!("----------");
        println!("{}", user.discord_id);
        println!("{}", user.username);
    }
    println!("----------");
}

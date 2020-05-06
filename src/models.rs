use super::schema::users;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub discord_id: String,
    pub username: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub discord_id: &'a str,
    pub username: &'a str,
}

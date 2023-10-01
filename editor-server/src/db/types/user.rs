#[derive(sqlx::FromRow)]
pub struct UserData {
    pub id: i32, 
    pub name: String,
    pub password: String,
}

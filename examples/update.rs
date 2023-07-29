use std::error::Error;

use cherrydoor_models::{schema::users, models::User, update::UserUpdate};
use diesel::{QueryDsl, SelectableHelper, ExpressionMethods};
use diesel_async::{AsyncMysqlConnection, AsyncConnection, RunQueryDsl};

const DB_URI :&str = "mysql://root:qazwsx@127.0.0.1:3306/cherrylock";
const PATCH :&str = r#"
    {
        "full_name": "Szymon Kwiręg"
    }
"#;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut db = AsyncMysqlConnection::establish(DB_URI).await?;

    let user :User = users::table
        .select(User::as_select())
        .filter(users::columns::name.eq("simo"))
    .first(&mut db).await?;

    let patch :UserUpdate = serde_json::from_str(PATCH).unwrap();

    diesel::update(&user)
        .set(&patch)
    .execute(&mut db).await?;

    Ok(())
}
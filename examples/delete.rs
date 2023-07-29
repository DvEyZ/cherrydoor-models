use std::error::Error;

use cherrydoor_models::{schema::{users, access_codes, users_permissions}};
use diesel::ExpressionMethods;
use diesel_async::{AsyncMysqlConnection, AsyncConnection, RunQueryDsl};

const USER_ID :i32 = 6;
const DB_URI :&str = "mysql://root:qazwsx@127.0.0.1:3306/cherrylock";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut db = AsyncMysqlConnection::establish(DB_URI).await?;

    diesel::delete(access_codes::table)
        .filter(access_codes::columns::user.eq(USER_ID))
    .execute(&mut db).await?;

    diesel::delete(users_permissions::table)
        .filter(users_permissions::columns::user_id.eq(USER_ID))
    .execute(&mut db).await?;

    diesel::delete(users::table)
        .filter(users::columns::id.eq(USER_ID))
    .execute(&mut db).await?;

    Ok(())
}
use std::error::Error;

use cherrydoor_models::{insert::{UserInsert}, schema::{users}};
use diesel::result;
use diesel_async::{AsyncMysqlConnection, AsyncConnection, RunQueryDsl};

const DB_URI :&str = "mysql://root:qazwsx@127.0.0.1:3306/cherrylock";
const NEW_USER :&str = r#"
    {
        "name": "jkowalska",
        "full_name": "Janina Kowalska",
        "role": "Nic ważnego..."
    }
"#;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut db = AsyncMysqlConnection::establish(DB_URI).await?;
    let new_user :UserInsert = serde_json::from_str(NEW_USER)?;

    if let Err(e) = diesel::insert_into(users::table)
        .values(&new_user)
    .execute(&mut db).await {
        if let result::Error::DatabaseError(result::DatabaseErrorKind::UniqueViolation, _) = e {
            println!("409 CONFLICT")
        } else {
            return Err(Box::new(e) as Box<dyn Error>);
        }
    };

    Ok(())
}
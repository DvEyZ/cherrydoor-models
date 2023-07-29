use std::error::Error;

use cherrydoor_models::{schema::{users, permissions}, models::{User, Permission, UserPermission, AccessCode}, full::UserFull};
use diesel::{QueryDsl, SelectableHelper, BelongingToDsl, GroupedBy};
use diesel_async::{RunQueryDsl, AsyncMysqlConnection, AsyncConnection};

const DB_URI :&str = "mysql://root:qazwsx@127.0.0.1:3306/cherrylock";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut db = AsyncMysqlConnection::establish(DB_URI).await?;

    let users :Vec<User> = users::table
        .select(User::as_select())
    .load(&mut db).await?;

    let codes :Vec<AccessCode> = AccessCode::belonging_to(&users)
        .select(AccessCode::as_select())
    .load(&mut db).await?;

    let permissions :Vec<(UserPermission, Permission)> = UserPermission::belonging_to(&users)
        .inner_join(permissions::table)
        .select((UserPermission::as_select(), Permission::as_select()))
    .load(&mut db).await?;

    let uwc = codes.grouped_by(&users).into_iter();
    let uwp = permissions.grouped_by(&users).into_iter();

    let fin = users.into_iter().zip(uwc).zip(uwp)
        .map(|v| {
            UserFull {
                user: v.0.0, 
                access_codes: v.0.1,
                permissions: v.1.into_iter().map(|v| { v.1 }).collect()
            }
        })
    .collect::<Vec<_>>();

    fin.iter().for_each(|v| {
        println!("{}", serde_json::to_string_pretty(&v).unwrap())
    });

    Ok(())
}
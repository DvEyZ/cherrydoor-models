use std::error::Error;

use cherrydoor_models::{schema::{access_codes, users, permissions, access_profiles}, models::{AccessCode, User, Permission, UserPermission, AccessProfile, AccessProfilePermission}};
use diesel::{QueryDsl, SelectableHelper, ExpressionMethods, BelongingToDsl, OptionalExtension};
use diesel_async::{AsyncConnection, AsyncMysqlConnection, RunQueryDsl};

const DB_URI :&str = "mysql://root:qazwsx@127.0.0.1:3306/cherrylock";
const PROFILE :&str = "lesson";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut db = AsyncMysqlConnection::establish(DB_URI).await?;

    let mut code = String::new();
    std::io::stdin().read_line(&mut code)?;
    code.pop();

    let Some(ac) :Option<AccessCode> = access_codes::table
        .select(AccessCode::as_select())
        .filter(access_codes::columns::code.eq(code))
    .first(&mut db).await.optional()?
    else {
        println!("Access denied.");
        return Ok(());
    };
    
    let user :User = users::table
        .select(User::as_select())
        .filter(users::columns::id.eq(ac.user))
    .first(&mut db).await?;

    let upwp :Vec<(UserPermission, Permission)> = UserPermission::belonging_to(&user)
        .inner_join(permissions::table)
        .select((UserPermission::as_select(), Permission::as_select()))
    .load(&mut db).await?;

    let perms :Vec<Permission> = upwp.into_iter().map(|v| { v.1 }).collect();

    let apwp :Vec<(AccessProfilePermission, AccessProfile)> = AccessProfilePermission::belonging_to(&perms)
        .inner_join(access_profiles::table)
        .select((AccessProfilePermission::as_select(), AccessProfile::as_select()))
    .load(&mut db).await?;

    let aps :Vec<AccessProfile> = apwp.into_iter().map(|v| { v.1 }).collect();

    if aps.iter().any(|prof| {
        prof.name == PROFILE
    }) {
        println!("Access granted for {}.", user.full_name)
    } else {
        println!("Access denied.")
    }

    Ok(())
}
use diesel::prelude::*;
use serde::Deserialize;
use super::schema;

#[derive(Insertable, PartialEq, Debug, Deserialize)]
#[diesel(table_name = schema::web_ui_users)]
pub struct WebUIUserInsert {
    pub name :String,
    pub password_hash :String,
    pub is_admin :bool
}

#[derive(Insertable, PartialEq, Debug, Deserialize)]
#[diesel(table_name = schema::users)]
pub struct UserInsert {
    pub name :String,
    pub full_name :String,
    pub role :String
}

#[derive(Insertable, PartialEq, Debug, Deserialize)]
#[diesel(table_name = schema::permissions)]
pub struct PermissionInsert {
    pub name :String,
    pub description :String
}

#[derive(Insertable, PartialEq, Debug, Deserialize)]
#[diesel(table_name = schema::access_profiles)]
pub struct AccessProfileInsert {
    pub name :String,
    pub description :String,
    pub display_text :String,
    pub color :String
}

#[derive(Insertable, Debug, PartialEq, Deserialize)]
#[diesel(table_name = schema::users_permissions)]
pub struct UserPermissionInsert {
    pub user_id :i32,
    pub permission_id :i32
}

#[derive(Insertable, Debug, PartialEq, Deserialize)]
#[diesel(table_name = schema::access_profiles_permissions)]
pub struct AccessProfilePermissionInsert {
    pub access_profile_id :i32,
    pub permission_id :i32
}

#[derive(Insertable, PartialEq, Debug, Deserialize)]
#[diesel(table_name = schema::access_codes)]
pub struct AccessCodeInsert {
    pub code :String,
    pub user :i32
}
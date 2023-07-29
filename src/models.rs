use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use super::schema;

#[derive(Queryable, Identifiable, Selectable, PartialEq, Debug, Serialize, Deserialize)]
#[diesel(table_name = schema::web_ui_users)]
pub struct WebUIUser {
    pub id :i32,
    pub name :String,
    pub password_hash :String,
    pub is_admin :bool,
}

#[derive(Queryable, Identifiable, Selectable, PartialEq, Debug, Serialize, Deserialize)]
#[diesel(table_name = schema::users)]
pub struct User {
    pub id :i32,
    pub name :String,
    pub full_name :String,
    pub role :String
}

#[derive(Queryable, Identifiable, Selectable, PartialEq, Debug, Serialize, Deserialize)]
#[diesel(table_name = schema::permissions)]
pub struct Permission {
    pub id :i32,
    pub name :String,
    pub description :String
}

#[derive(Queryable, Identifiable, Selectable, PartialEq, Debug, Serialize, Deserialize)]
#[diesel(table_name = schema::access_profiles)]
pub struct AccessProfile {
    pub id :i32,
    pub name :String,
    pub description :String,
    pub display_text :String,
    pub color :String
}

#[derive(Queryable, Identifiable, Selectable, Associations, Debug, PartialEq)]
#[diesel(table_name = schema::users_permissions)]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(belongs_to(Permission, foreign_key = permission_id))]
#[diesel(primary_key(user_id, permission_id))]
pub struct UserPermission {
    pub user_id :i32,
    pub permission_id :i32
}

#[derive(Queryable, Identifiable, Selectable, Associations, Debug, PartialEq)]
#[diesel(table_name = schema::access_profiles_permissions)]
#[diesel(belongs_to(AccessProfile, foreign_key = access_profile_id))]
#[diesel(belongs_to(Permission, foreign_key = permission_id))]    
#[diesel(primary_key(access_profile_id, permission_id))]
pub struct AccessProfilePermission {
    pub access_profile_id :i32,
    pub permission_id :i32
}

#[derive(Queryable, Identifiable, Associations, Selectable, Debug, PartialEq, Serialize, Deserialize)]
#[diesel(table_name = schema::access_codes)]
#[diesel(belongs_to(User, foreign_key = user))]
pub struct AccessCode {
    pub id :i32,
    pub code :String,
    pub user :i32
}
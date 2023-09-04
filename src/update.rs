use diesel::AsChangeset;
use serde::Deserialize;

use crate::schema::AccessProfileAccessMode;

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = super::schema::web_ui_users)]
pub struct WebUIUserUpdate {
    pub password_hash :Option<String>,
    pub is_admin :Option<bool>,
    pub ac_does_not_expire :Option<bool>
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = super::schema::users)]
pub struct UserUpdate {
    pub full_name :Option<String>,
    pub role :Option<String>
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = super::schema::permissions)]
pub struct PermissionUpdate {
    pub description :Option<String>
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = super::schema::access_profiles)]
pub struct AccessProfileUpdate {
    pub description :Option<String>,
    pub display_text :Option<String>,
    pub color :Option<String>,
    pub access_mode :Option<AccessProfileAccessMode>
}
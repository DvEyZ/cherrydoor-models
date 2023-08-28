use diesel_derive_enum::DbEnum;
// @generated automatically by Diesel CLI.
use serde::{Serialize, Deserialize};

#[derive(PartialEq, Debug, Serialize, Deserialize, DbEnum)]
#[DbValueStyle = "PascalCase"]
pub enum AccessProfileAccessMode {
    OpenLock,
    AllowAnyone,
    CheckAccess
}

diesel::table! {
    access_codes (id) {
        id -> Integer,
        #[max_length = 255]
        code -> Varchar,
        user -> Integer,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::AccessProfileAccessModeMapping;

    access_profiles (id) {
        id -> Integer,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        description -> Varchar,
        #[max_length = 255]
        display_text -> Varchar,
        #[max_length = 255]
        color -> Varchar,
        #[max_length = 11]
        access_mode -> AccessProfileAccessModeMapping,
    }
}

diesel::table! {
    access_profiles_permissions (access_profile_id, permission_id) {
        access_profile_id -> Integer,
        permission_id -> Integer,
    }
}

diesel::table! {
    permissions (id) {
        id -> Integer,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        description -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        full_name -> Varchar,
        #[max_length = 255]
        role -> Varchar,
    }
}

diesel::table! {
    users_permissions (user_id, permission_id) {
        user_id -> Integer,
        permission_id -> Integer,
    }
}

diesel::table! {
    web_ui_users (id) {
        id -> Integer,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        password_hash -> Varchar,
        is_admin -> Bool,
    }
}

diesel::joinable!(access_codes -> users (user));
diesel::joinable!(access_profiles_permissions -> access_profiles (access_profile_id));
diesel::joinable!(access_profiles_permissions -> permissions (permission_id));
diesel::joinable!(users_permissions -> permissions (permission_id));
diesel::joinable!(users_permissions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    access_codes,
    access_profiles,
    access_profiles_permissions,
    permissions,
    users,
    users_permissions,
    web_ui_users,
);

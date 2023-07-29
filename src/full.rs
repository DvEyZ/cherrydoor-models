use serde::{Serialize, Deserialize};

use crate::models::{AccessCode, Permission, AccessProfile, User};

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessCodeFull {
    #[serde(flatten)]
    pub access_code :AccessCode,
    pub user :User
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserFull {
    #[serde(flatten)]
    pub user :User,
    pub access_codes :Vec<AccessCode>,
    pub permissions :Vec<Permission>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PermissionFull {
    #[serde(flatten)]
    pub permission :Permission,
    pub users :Vec<User>,
    pub access_profiles :Vec<AccessProfile>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessProfileFull {
    #[serde(flatten)]
    pub access_profile :AccessProfile,
    pub permissions :Vec<Permission>
}
pub struct ScheduledProfile {
    pub id :i32,
    pub description :String,
    pub from :String,       // as crontab expression
    pub to :String,         // as crontab expression
    pub access_profile_id :i32
}

pub struct ScheduledProfileInsert {
    pub description :String,
    pub from :String,       // as crontab expression
    pub to :String,         // as crontab expression
    pub access_profile_id :i32
}

pub struct ScheduledProfileUpdate {
    pub description :Option<String>,
    pub from :Option<String>,       // as crontab expression
    pub to :Option<String>,         // as crontab expression
    pub access_profile_id :Option<i32>
}
use super::user::User;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Course {
    tutor: String,
    students: Vec<User>,
}

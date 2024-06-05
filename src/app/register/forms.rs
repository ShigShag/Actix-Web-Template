use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct RegisterForm {
    pub email: String,
    pub password: String,
    #[serde(rename = "password-confirm")]
    pub password_confirm: String,
}

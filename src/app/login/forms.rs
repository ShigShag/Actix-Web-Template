use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}

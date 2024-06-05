#[macro_export]
macro_rules! get_user_id_from_session {
    ($session:expr) => {{
        $session.get::<i32>("user_id").ok().flatten()
    }};
}

#[macro_export]
macro_rules! authenticate {
    ($req:expr, $ctx:expr) => {
        match $crate::authentication::authentication::get_authenticated_user($req, $ctx).await {
            Ok(user) => user,
            Err(e) => return Ok(e.into()),
        }
    };
}

pub mod authentication;
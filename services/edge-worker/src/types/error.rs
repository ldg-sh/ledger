use worker::Response;

pub enum AuthError {
    MissingToken,
    InvalidToken,
    EnvError(worker::Error),
}

impl From<AuthError> for Response {
    fn from(err: AuthError) -> Self {
        match err {
            AuthError::MissingToken => Response::error("Unauthorized: Missing Token", 401).unwrap(),
            AuthError::InvalidToken => Response::error("Unauthorized: Invalid Token", 401).unwrap(),
            AuthError::EnvError(_) => Response::error("Internal Server Error", 500).unwrap(),
        }
    }
}

impl From<worker::Error> for AuthError {
    fn from(err: worker::Error) -> Self {
        AuthError::EnvError(err)
    }
}
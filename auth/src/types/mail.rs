use serde::Serialize;

#[derive(Serialize)]
pub struct SendEmail {
    pub from: String,
    pub to: Vec<String>,
    pub subject: String,
    pub html: Option<String>,
    pub text: Option<String>,
    pub cc: Option<Vec<String>>,
    pub bcc: Option<Vec<String>>,
    pub reply_to: Option<Vec<String>>,
}

impl Default for SendEmail {
    fn default() -> Self {
        Self {
            from: "noreply@example.com".to_string(),
            to: vec![],
            subject: "".to_string(),
            html: None,
            text: None,
            cc: None,
            bcc: None,
            reply_to: None,
        }
    }
}

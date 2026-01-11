pub mod user;

/*
 Simplified, self-hostable model: users have a name, email, and hashed auth key.
 Teams/invites were removed, so owning a valid token is the only gate to access.
 */

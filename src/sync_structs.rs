use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Root {
    pub local: Option<LocalRoot>,
    ldap: Option<LDAP>,
}

#[derive(Debug, Deserialize)]
pub struct LocalRoot {
    pub encrypted: bool,
    pub users: Vec<User>,
    pub groups: Vec<Group>,
}

#[derive(Debug, Deserialize)]
pub struct LDAP {
    // Unsupported at the moment.
}

#[derive(Debug, Deserialize, Clone)]
pub struct User {
    pub username: String,
    pub uid: Option<u32>,
    pub password: String,
    pub groups: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Group {
    pub name: String,
    pub gid: Option<u32>,
}

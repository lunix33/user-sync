use std::{
    collections::HashMap,
    fs,
    io::{self, BufRead},
};

use crate::{consts, s};

type RawEntry = HashMap<String, String>;

#[derive(Debug, Clone)]
pub struct SystemUser {
    pub username: String, // passwd::0
    pub hash: String,     // shadow::1
    pub uid: u32,         // passwd::2
    pub gid: u32,         // passwd::3
}

impl SystemUser {
    pub fn parse_file() -> io::Result<Vec<Self>> {
        let nobody_username = s!("nobody");

        let raw_users = filter_users(parse_system_file(consts::USER_FILE, consts::USER_FIELDS)?);
        let raw_passwords = parse_system_file(consts::PASSWORD_FILE, consts::PASSWORD_FIELDS)?;

        let mut rst = Vec::<Self>::new();
        for user_entry in raw_users {
            let password_entry_opt = raw_passwords.iter().find(|entry| {
                let username = user_entry
                    .get(consts::NAME_FIELD)
                    .unwrap_or(&nobody_username);
                let password_username = entry.get(consts::NAME_FIELD).unwrap_or(&nobody_username);
                username == password_username && username != &nobody_username
            });

            if let Some(password_entry) = password_entry_opt {
                rst.push(Self {
                    username: user_entry.get(consts::NAME_FIELD).unwrap().clone(),
                    hash: password_entry.get(consts::PASSWORD_FIELD).unwrap().clone(),
                    uid: user_entry
                        .get(consts::UID_FIELD)
                        .unwrap()
                        .clone()
                        .parse()
                        .unwrap(),
                    gid: user_entry
                        .get(consts::GID_FIELD)
                        .unwrap()
                        .clone()
                        .parse()
                        .unwrap(),
                });
            }
        }

        Ok(rst)
    }
}

#[derive(Debug, Clone)]
pub struct SystemGroup {
    pub name: String,       // group::0
    pub gid: u32,           // group::2
    pub users: Vec<String>, // group::3
}

impl SystemGroup {
    pub fn parse_file() -> io::Result<Vec<Self>> {
        let raw_groups =
            filter_groups(parse_system_file(consts::GROUP_FILE, consts::GROUP_FIELDS)?);

        let mut rst = Vec::<Self>::new();
        for group_entry in raw_groups {
            let users: Vec<String> = group_entry
                .get(consts::USER_LIST_FIELD)
                .unwrap()
                .split(',')
                .map(|u| s!(u))
                .collect();

            rst.push(Self {
                name: group_entry.get(consts::NAME_FIELD).unwrap().clone(),
                gid: group_entry
                    .get(consts::GID_FIELD)
                    .unwrap()
                    .clone()
                    .parse()
                    .unwrap(),
                users,
            });
        }

        Ok(rst)
    }
}

/**
Parse a system file where there's an entry per line and column separated fields.

# Parameters
* `path`: A `&str` with the path to the file.
* `fields`: À `&[&str]` with the list of all the fields to parse.
 */
fn parse_system_file(path: &str, fields: &[&str]) -> io::Result<Vec<RawEntry>> {
    // Open the file and reader.
    let file = fs::File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut rst = Vec::<RawEntry>::new();
    for line in reader.lines() {
        // Get the current line content in a parsable way.
        let line_content = line?;
        let line_split: Vec<&str> = line_content.split(':').collect();

        let mut map = HashMap::<String, String>::new();
        for (i, field) in fields.iter().enumerate() {
            // Add the current field value.
            let field_value = *line_split.get(i).unwrap_or(&"");
            map.insert(s!(*field), s!(field_value));
        }

        rst.push(map);
    }

    Ok(rst)
}

/**
Filter the list of user so only real valid users are listed.
To be valid, a user id must be over or equal to the `consts::MIN_UID`,
and must also not be `nobody`.

# Parameters
* users: The `Vec<RawEntry>` coming from `parse_system_file()` for `consts::USER_FILE`.

# Returns
A `Vec<RawEntry>` of all the valid users.
 */
fn filter_users(users: Vec<RawEntry>) -> Vec<RawEntry> {
    let nobody_username = s!("nobody");
    users
        .into_iter()
        .filter(|user| {
            let user_id = user
                .get(consts::UID_FIELD)
                .unwrap_or(&s!("0"))
                .parse::<u32>()
                .unwrap();
            let username = user.get(consts::NAME_FIELD).unwrap_or(&nobody_username);

            user_id >= consts::MIN_UID && username != &nobody_username
        })
        .collect()
}

/**
Filter the list of group so only real valid groups are listed.
To be valid, a group id must be over or equal to the `consts::MIN_GID`,
and must also not be `nogroup`.

# Parameters
* groups: The `Vec<RawEntry>` coming from `parse_system_file()` for `consts::GROUP_FILE`.

# Returns
A `Vec<RawEntry>` of all the valid groups.
 */
fn filter_groups(groups: Vec<RawEntry>) -> Vec<RawEntry> {
    let nogroup_name = s!("nogroup");

    groups
        .into_iter()
        .filter(|group| {
            let group_id = group
                .get(consts::GID_FIELD)
                .unwrap_or(&s!("0"))
                .parse::<u32>()
                .unwrap();
            let username = group.get(consts::NAME_FIELD).unwrap_or(&nogroup_name);

            group_id >= consts::MIN_GID && username != &nogroup_name
        })
        .collect()
}

use std::io::Write;
use std::process::{Command, Stdio};
use std::str;

use crate::consts::{
    ADD_GROUP_CMD, ADD_USER_CMD, REMOVE_GROUP_CMD, REMOVE_USER_CMD, UPDATE_GROUP_CMD,
    UPDATE_PASSWORDS_CMD, UPDATE_USER_CMD,
};
use crate::s;
use crate::sync_structs::{Group, LocalRoot, User};
use crate::{
    differ::Differ,
    system_parser::{SystemGroup, SystemUser},
};

/**
Apply to the system a diff.
Makes sure both the system and sync file are properly synchonized.

# Parameters
* `diff`: The change to apply.
* `system`: The current system state.
* `sync`: The local sync information.

# Panic
If one of the executed commands fail and is unable to terminate it properly.
 */
pub fn apply_diff(diff: &Differ, system: &(Vec<SystemUser>, Vec<SystemGroup>), sync: &LocalRoot) {
    let (add_users, add_groups) = &diff.add;
    let (remove_users, remove_groups) = &diff.remove;
    let (update_users, update_groups) = &diff.update;
    let (system_users, system_groups) = system;

    // Apply group changes
    delete_group(remove_groups);
    update_group(update_groups, system_groups);
    add_group(add_groups);

    // Get an updated group list.
    let system_groups = SystemGroup::parse_file().unwrap_or(system_groups.clone());

    // Apply user changes
    delete_user(remove_users);
    update_user(update_users, system_users, &system_groups);
    add_user(add_users);

    // Apply passwords
    apply_passwords(&sync.users, &sync.encrypted);
}

fn delete_user(remove_users: &Vec<SystemUser>) {
    for user in remove_users {
        print!("Removing user {} ({})...", &user.username, &user.uid);
        run_command(REMOVE_USER_CMD, &[&user.username], None)
    }
}

fn delete_group(remove_groups: &Vec<SystemGroup>) {
    for group in remove_groups {
        print!("Removing group {} ({})...", &group.name, &group.gid);
        run_command(REMOVE_GROUP_CMD, &vec!["-f", &group.name], None);
    }
}

fn add_user(add_users: &Vec<User>) {
    for user in add_users {
        print!("Adding user {}...", &user.username);

        // Skip user if not enough group.
        if user.groups.len() < 1 {
            println!("\tInvalid group list, need at least one group.");
            continue;
        }

        // Compose arguments.
        let mut args: Vec<String> = vec![s!("--create-home"), s!("--no-user-group")];

        // Primary group.
        let primary = &user.groups[0];
        args.push(s!("--gid"));
        args.push(primary.clone());

        // Supplementary groups.
        if user.groups.len() > 1 {
            let groups_str = user.groups[1..].join(",");
            args.push(s!("--groups"));
            args.push(groups_str);
        }

        // UID
        if let Some(uid) = user.uid {
            args.push(s!("--non-unique"));
            args.push(s!("--uid"));
            args.push(uid.to_string());
        }

        args.push(s!(user.username.clone()));

        let args: Vec<&str> = args.iter().map(|a| a.as_ref()).collect();
        run_command(ADD_USER_CMD, &args, None)
    }
}

fn add_group(add_groups: &Vec<Group>) {
    for group in add_groups {
        print!("Adding group {}...", &group.name);

        // Compose options
        let mut args: Vec<String> = vec![s!("--force")];
        if let Some(gid) = group.gid {
            args.push(s!("--non-unique"));
            args.push(s!("--gid"));
            args.push(gid.to_string());
        }

        args.push(group.name.clone());

        let args: Vec<&str> = args.iter().map(|a| a.as_ref()).collect();
        run_command(ADD_GROUP_CMD, &args, None)
    }
}

fn update_user(
    update_users: &Vec<User>,
    system_users: &Vec<SystemUser>,
    system_groups: &Vec<SystemGroup>,
) {
    for user in update_users {
        let system_user = system_users
            .iter()
            .find(|u| user.username == u.username)
            .unwrap();

        println!("Updating user {}...", &system_user.username);
        let mut change = false;

        // Skip user if not enough group.
        if user.groups.len() < 1 {
            println!("\tInvalid group list, need at least one group.");
            continue;
        }

        // Primary group.
        let current_primary = system_groups
            .iter()
            .find(|g| g.gid == system_user.gid)
            .unwrap();
        let sync_primary = &user.groups[0];
        if current_primary.name != *sync_primary {
            print!("\tUpdating primary group...");
            change = true;
            run_command(
                UPDATE_USER_CMD,
                &["--gid", sync_primary, &user.username],
                None,
            );
        }

        // Supplementary groups.
        // Find current groups list name
        let current_supp: Vec<String> = system_groups
            .iter()
            .filter_map(|g| match g.users.contains(&user.username) {
                true => Some(g.name.clone()),
                false => None,
            })
            .collect();
        // Make group strings
        let current_supp_str = current_supp.join(",");
        let sync_supp = user.groups[1..].join(",");
        if sync_supp != current_supp_str {
            print!("\tUpdating supplentary group list...");
            change = true;
            run_command(
                UPDATE_USER_CMD,
                &["--groups", &sync_supp, &user.username],
                None,
            );
        }

        // UID changed
        if let Some(uid) = user.uid {
            if uid != system_user.uid {
                change = true;
                print!("\tUpdating UID {} -> {}...", &system_user.uid, uid);
                run_command(
                    UPDATE_USER_CMD,
                    &["--non-unique", "--uid", &uid.to_string(), &user.username],
                    None,
                );
            }
        }

        // Skip message
        if !change {
            println!("\tSkipping user (no change).");
        }
    }
}

fn update_group(update_groups: &Vec<Group>, system_groups: &Vec<SystemGroup>) {
    for group in update_groups {
        // Find the system user.
        let system_group = system_groups.iter().find(|g| group.name == g.name).unwrap();

        // Print and setup.
        println!("Updating group {}...", &system_group.name);
        let mut change = false;

        // Apply change to GID.
        if let Some(gid) = group.gid {
            if system_group.gid != gid {
                change = true;
                print!("\tUpdating GID {} -> {}...", &system_group.gid, &gid);
                run_command(
                    UPDATE_GROUP_CMD,
                    &vec!["--non-unique", "--gid", &gid.to_string(), &group.name],
                    None,
                )
            }
        }

        // Skip message
        if !change {
            println!("\tSkipping group (no change).");
        }
    }
}

fn apply_passwords(sync: &Vec<User>, encrypted: &bool) {
    print!("Updating password database...");
    let mut stdin_buf = String::new();
    for user in sync {
        stdin_buf.push_str(&format!("{}:{}\n", &user.username, &user.password))
    }

    let mut args: Vec<String> = vec![];
    if *encrypted {
        args.push(s!("--encrypted"));
    }

    let args: Vec<&str> = args.iter().map(|a| a.as_ref()).collect();
    run_command(UPDATE_PASSWORDS_CMD, &args, Some(&stdin_buf));
}

/**
Run the specified command with given arguments and stdin buffer.

# Parameters
* `command`: A string representation of the command to run.
* `args`: A list of arguments to be passed to the command.
* `stdin_buf`: The standard input to be written once the command is spawned.

# Panic
If the command fail and is unable to kill the child process.
 */
fn run_command(command: &str, args: &[&str], stdin_buf: Option<&str>) {
    let mut cmd = Command::new(command);
    cmd.args(args).stdout(Stdio::piped()).stderr(Stdio::piped());

    // Only pipe stdin when needed.
    if stdin_buf.is_some() {
        cmd.stdin(Stdio::piped());
    }

    let child_rst = cmd.spawn();

    match child_rst {
        Ok(mut child) => {
            // Write to stdin when a buffer is present.
            if let Some(stdin_buf) = stdin_buf {
                if let Some(mut stdin) = child.stdin.take() {
                    let write_rst = stdin
                        .write_all(&stdin_buf.as_bytes())
                        .and_then(|_| stdin.flush());

                    if let Err(e) = write_rst {
                        println!("STDIN error; {}", e);
                        child.kill().unwrap();
                    }
                }
            }

            // Wait for command to finish and print result.
            match child.wait_with_output() {
                Ok(output) => {
                    if output.status.success() {
                        println!("Success");
                    } else {
                        println!("Failed");
                        println!(
                            "==========\n{} {:?}\n{}\n==========",
                            command,
                            args,
                            String::from_utf8_lossy(&output.stderr)
                        );
                    }
                }
                Err(err) => println!("Execution error: {}", err),
            }
        }
        Err(err) => println!("Remove command failed: {}", err),
    }
}

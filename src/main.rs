mod consts;
mod differ;
mod runner;
mod sync_structs;
mod system_parser;

use std::{env, fs, path::PathBuf};

use differ::Differ;
use sync_structs::Root;
use system_parser::{SystemGroup, SystemUser};

/**
Main function of the application.

# Positional arguments:
1. Path to the sync file (optional, can use environment variable).

# Environment variables:
* `USER_SYNC`: Path to the sync file. (Will default to `/etc/user-sync.json`).

# Exit codes:
* `0`: Okay
* `1`: Failed to read sync file.
* `2`: Failed to read system user file.
* `3`: Failed to read system group file.
* `4`: Failed to parse sync file.
* `5`: Invalid sync
 */
fn main() {
    let path = get_sync_file_path();
    let sync = get_sync_data(&path);
    let system = get_system_data();

    if let Some(local) = sync.local {
        let differ = Differ::new(&local, &system);
        //println!("{:?}", differ);
        runner::apply_diff(&differ, &system, &local);
    } else {
        println!("Unable to find the local sync configuration.");
        std::process::exit(5);
    }
}

/**
Get the file path from either the argument or the environment variable.

It will tries to first get the path from the first argument of the application.\
If it's not able, it will try to read the `USER_SYNC` environment variable.\
If it's still unable to get a path, it will just default to use `/etc/user-sync.json`.

# Returns
A `PathBuf` with the resolved path.
 */
fn get_sync_file_path() -> PathBuf {
    let file_path_opt = env::args().nth(1);
    PathBuf::from(
        match file_path_opt {
            None => Some(env::var("USER_SYNC").unwrap_or(s!("/etc/user-sync.json"))),
            _ => file_path_opt,
        }
        .unwrap(),
    )
}

/**
Read and parse the synchonisation data from the given file.\
Will exit the program on error.

# Returns
The parsed synchronisation file.
 */
fn get_sync_data(path: &PathBuf) -> Root {
    match fs::read_to_string(path) {
        Err(e) => {
            println!("Sync file read error: {}", e);
            std::process::exit(1);
        }
        Ok(content) => match serde_json::from_str(&content) {
            Err(e) => {
                println!("Sync file parse error: {}", e);
                std::process::exit(4);
            }
            Ok(result) => result,
        },
    }
}

/**
Read and parse system user and group files into usable strucs.\
Will exit the program on error.

# Returns
A tuple with the first element being the list of users (`Vec<SystemUsers>`),\
the second is the list of groups (`Vec<SystemGroup>`).
 */
fn get_system_data() -> (Vec<SystemUser>, Vec<SystemGroup>) {
    let user_list = match SystemUser::parse_file() {
        Ok(list) => list,
        Err(e) => {
            println!("System user parse error: {}", e);
            std::process::exit(2);
        }
    };

    let group_list = match SystemGroup::parse_file() {
        Ok(list) => list,
        Err(e) => {
            println!("System group parse error: {}", e);
            std::process::exit(3);
        }
    };

    (user_list, group_list)
}

/**
Shorten syntax to create a string.

# Parameters
* `$x`: The &str to transform into a `std::string::String`
 */
#[macro_export]
macro_rules! s {
    ($x:expr) => {
        String::from($x)
    };
}

pub const MIN_UID: u32 = 1000;
pub const MIN_GID: u32 = 1000;

pub const USER_FILE: &'static str = "/etc/passwd";
pub const GROUP_FILE: &'static str = "/etc/group";
pub const PASSWORD_FILE: &'static str = "/etc/shadow";

pub const NAME_FIELD: &'static str = "name";
pub const PASSWORD_FIELD: &'static str = "password";
pub const UID_FIELD: &'static str = "uid";
pub const GID_FIELD: &'static str = "gid";
pub const USER_LIST_FIELD: &'static str = "users";

pub const USER_FIELDS: &'static [&'static str] = &[
    NAME_FIELD,
    PASSWORD_FIELD,
    UID_FIELD,
    GID_FIELD,
    "gecos",
    "home",
    "shell",
];
pub const GROUP_FIELDS: &'static [&'static str] =
    &[NAME_FIELD, PASSWORD_FIELD, GID_FIELD, USER_LIST_FIELD];
pub const PASSWORD_FIELDS: &'static [&'static str] = &[
    NAME_FIELD,
    PASSWORD_FIELD,
    "last_changed",
    "minimum",
    "maximum",
    "warn",
    "inactive",
    "expire",
];

pub const ADD_GROUP_CMD: &'static str = "/usr/sbin/groupadd";
pub const REMOVE_GROUP_CMD: &'static str = "/usr/sbin/groupdel";
pub const UPDATE_GROUP_CMD: &'static str = "/usr/sbin/groupmod";

pub const ADD_USER_CMD: &'static str = "/usr/sbin/useradd";
pub const REMOVE_USER_CMD: &'static str = "/usr/sbin/userdel";
pub const UPDATE_USER_CMD: &'static str = "/usr/sbin/usermod";

pub const UPDATE_PASSWORDS_CMD: &'static str = "/usr/sbin/chpasswd";
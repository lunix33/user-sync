use crate::{
    sync_structs::{Group, LocalRoot, User},
    system_parser::{SystemGroup, SystemUser},
};

#[derive(Debug)]
pub struct Differ {
    pub add: (Vec<User>, Vec<Group>),
    pub update: (Vec<User>, Vec<Group>),
    pub remove: (Vec<SystemUser>, Vec<SystemGroup>),
}

impl Differ {
    /**
    Runs a diff between the sync file and local users and groups.

    # Parameters
    * `sync`: A reference to the `LocalRoot` of the sync file.
    * `system`: A reference to the tuple with the list of system users (0) and system groups (1).

    # Returns
    An instance of a `Differ` with the result of a diff.
     */
    pub fn new(sync: &LocalRoot, system: &(Vec<SystemUser>, Vec<SystemGroup>)) -> Self {
        Self {
            add: Self::detect_add(sync, system),
            update: Self::detect_update(sync, system),
            remove: Self::detect_remove(sync, system),
        }
    }

    /**
    Find all the users and groups that needs to be added to the system.

    # Parameters
    * `sync`: A reference to the `LocalRoot` of the sync file.
    * `system`: A reference to the tuple with the list of system users (0) and system groups (1).

    # Returns
    A tuple with the list of users (0) and groups (1) to be added to the system.
     */
    fn detect_add(
        sync: &LocalRoot,
        system: &(Vec<SystemUser>, Vec<SystemGroup>),
    ) -> (Vec<User>, Vec<Group>) {
        let sync_users = &sync.users;
        let sync_groups = &sync.groups;
        let (system_users, system_groups) = system;

        // Look for any user in the `sync_users` list and not in the `system_users` list.
        let add_users: Vec<User> = sync_users
            .iter()
            .filter(|sync_user| {
                system_users
                    .iter()
                    .find(|system_user| system_user.username == sync_user.username)
                    .is_none()
            })
            .map(|user| user.clone())
            .collect();

        // Look for any user in the `sync_groups` list and not in the `system_groups` list.
        let add_groups: Vec<Group> = sync_groups
            .iter()
            .filter(|sync_group| {
                system_groups
                    .iter()
                    .find(|system_group| system_group.name == sync_group.name)
                    .is_none()
            })
            .map(|group| group.clone())
            .collect();

        (add_users, add_groups)
    }

    /**
    Find all the users and groups that needs to be updated to the system.

    # Parameters
    * `sync`: A reference to the `LocalRoot` of the sync file.
    * `system`: A reference to the tuple with the list of system users (0) and system groups (1).

    # Returns
    A tuple with the list of users (0) and groups (1) to be updated to the system.
     */
    fn detect_update(
        sync: &LocalRoot,
        system: &(Vec<SystemUser>, Vec<SystemGroup>),
    ) -> (Vec<User>, Vec<Group>) {
        let sync_users = &sync.users;
        let sync_groups = &sync.groups;
        let (system_users, system_groups) = system;

        // Look for any user in the `sync_users` list and not in the `system_users` list.
        let update_users: Vec<User> = sync_users
            .iter()
            .filter(|sync_user| {
                system_users
                    .iter()
                    .find(|system_user| system_user.username == sync_user.username)
                    .is_some()
            })
            .map(|user| user.clone())
            .collect();

        // Look for any user in the `sync_groups` list and not in the `system_groups` list.
        let update_groups: Vec<Group> = sync_groups
            .iter()
            .filter(|sync_group| {
                system_groups
                    .iter()
                    .find(|system_group| system_group.name == sync_group.name)
                    .is_some()
            })
            .map(|group| group.clone())
            .collect();

        (update_users, update_groups)
    }

    /**
    Find all the users and groups that needs to be removed from the system.

    # Parameters
    * `sync`: A reference to the `LocalRoot` of the sync file.
    * `system`: A reference to the tuple with the list of system users (0) and system groups (1).

    # Returns
    A tuple with the list of users (0) and groups (1) to be removed from the system.
     */
    fn detect_remove(
        sync: &LocalRoot,
        system: &(Vec<SystemUser>, Vec<SystemGroup>),
    ) -> (Vec<SystemUser>, Vec<SystemGroup>) {
        let sync_users = &sync.users;
        let sync_groups = &sync.groups;
        let (system_users, system_groups) = system;

        // Look for any user in the `system_users` list and not in the `sync_users` list.
        let remove_users: Vec<SystemUser> = system_users
            .iter()
            .filter(|system_user| {
                sync_users
                    .iter()
                    .find(|sync_user| system_user.username == sync_user.username)
                    .is_none()
            })
            .map(|user| user.clone())
            .collect();

        // Look for any user in the `system_groups` list and not in the `sync_groups` list.
        let remove_groups: Vec<SystemGroup> = system_groups
            .iter()
            .filter(|system_group| {
                sync_groups
                    .iter()
                    .find(|sync_group| system_group.name == sync_group.name)
                    .is_none()
            })
            .map(|user| user.clone())
            .collect();

        (remove_users, remove_groups)
    }
}

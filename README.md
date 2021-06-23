# User sync

This project synchronize the users and groups of a linux system with the information contained in a synchronization file.

The synchronization file path can be given in one of three ways.
1. The first positional argument of the application.
2. The `USER_SYNC` environment variable.
3. Default location to `/etc/user-sync.json`

It is important to run the application as `root` since the application needs to be able to modify the system user database (`/etc/{passwd, group, shadow}`).

## File structure

```ts
{
  "local": {
    /** True when the passwords are encrypted, otherwise false. */
    "encrypted": boolean,
    /** The list of users. */
    "users": [{
      /** The name of the user. */
      "username": string,
      /**
       * The password of the user.
       * If `local.encrypted` is `true`, this fields should be encrypted.
       * `openssl passwd -6 'userPassword'` can be used to create an encrypted password.
       */
      "password": string,
      /**
       * A list of group the user should be a member of.
       * The first group of the list will be considered their primary group and must be defined.
       */
      "groups": string[]
      /** An optional forced UID for the user. */
      "uid"?: number,
    }],
    /** The list of groups */
    "groups": [{
      /** The name of the group. */
      "name": string,
      /** An optional forced GID for the group. */
      "gid"?: number
    }]
  }
}
```

***Notes:***

* You need to define all the groups even default primary groups.
* The application allows conflicting UID and GID.
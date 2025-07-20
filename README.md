# Command-Line Authentik Authentication Provider

This project is a **command line authentication provider** designed to authenticate users against
an [Authentik](https://goauthentik.io/) identity provider. It's suitable for use with systems that require command-line
authentication, such as [Home Assistant](https://www.home-assistant.io/). It is specifically targeted towards Home
Assistant, but may also work with other command line auth systems.

---

## Features

- Authenticate users via Authentik using a custom flow.
- Map users to Home Assistant groups based on Authentik group membership.
_- Configurable via a simple `config.toml` file.
- Designed for use as a Home_
  Assistant [command line authentication provider](https://www.home-assistant.io/docs/authentication/providers/#command-line).

---

## Table of Contents

- [Installation](#installation)
- [Configuration](#configuration)
- [Setting up Authentik Flow](#setting-up-authentik-flow)
- [Usage](#usage)
- [Integration with Home Assistant](#integration-with-home-assistant)
- [Troubleshooting](#troubleshooting)
- [License](#license)

---

## Installation

1. **Build the project:**

   ```bash
   cargo build --release
   ```

2. **Copy the executable** (from `target/release/`) to your desired location.

---

## Configuration

Create a `config.toml` file in the application directory, based on the following example:
```toml
authentik_base_url = "https://auth.example.com" 
flow_slug = "simple-password" 
admin_group_name = "Home Assistant Admins" 
user_group_name = "Home Assistant Users" 
timeout = 10
``` 

### Configuration Fields

- **authentik_base_url**: The base URL for your Authentik instance.
- **flow_slug**: The slug of the flow used for authentication.
- **admin_group_name** (optional): The Authentik group that corresponds to Home Assistant admins.
- **user_group_name** (optional): The Authentik group that corresponds to regular Home Assistant users.
- **timeout**: Request timeout in seconds.

---

## Setting up Authentik Flow

> ⚠️ Before using this application, you must create a suitable **authentication flow** in Authentik.  
> The flow must accept the user's `username` and `password`, and provide the necessary user information when authentication succeeds.

### Required Fields/Stages for the Flow

**[Please fill in this section with the specific Authentik flow requirements for your deployment, such as required stage types and field names.]**

---

## Usage

You can use the authentication provider directly via the command line:
```
bash ./command-line-auth-provider --username  --password  --config_path config. toml
``` 

- Returns exit code `0` if authentication succeeds, and prints fields required by Home Assistant.
- Returns a non-zero exit code if authentication fails.

Environment variables `username` and `password` can also be used to supply credentials.

---

## Integration with Home Assistant

This project is designed for use with [Home Assistant's command line authentication provider](https://www.home-assistant.io/docs/authentication/providers/#command-line).

### Example Home Assistant Configuration

In your Home Assistant `configuration.yaml`:
```yaml 
homeassistant: 
  auth_providers: 
    - type: command_line
      command: "/your/path/to/command-line-auth-provider --config_path /your/path/to/config.toml" 
      meta: true
``` 

- Home Assistant supplies `username` and `password` as environment variables to the command.

### How the Integration Works

- If the authenticated user is in the configured `admin_group_name`, they are mapped to the Home Assistant admin group (`system-admin`).
- If the authenticated user is in the `user_group_name`, they are mapped to the Home Assistant users group (`system-users`).
- If neither group is configured, or the user is not a member, authentication is denied.

---

## Troubleshooting

- **User not found or not in required group:** Make sure the Authentik flow is correct and the user is a member of the correct group(s).
- **Timeout errors:** Increase the `timeout` parameter in `config.toml` if needed.
- **Flow errors:** Double-check your Authentik flow setup.

---

## License

[MIT License](./LICENSE)

---

### Notes

- Please refer to the Authentik documentation for flow setup and group management.
- If you encounter issues, enable verbose logging by adding `-v` to the command for more debug information.

---

**Fill in the Flow Requirements Below:**

> **Required Authentik Flow Details (to be filled in):**
>
> - Flow slug:
> - Required stages:
> - Required fields:
> - Special notes:
```


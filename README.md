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
### Download a release!

### Build the project by hand
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

### Terraform Example code for the flow

You can use the [terraform provider](https://registry.terraform.io/providers/goauthentik/authentik/latest) provided by authentik to create the flow.
If you don't use that, you do need to create a flow that mimics the behavior of this flow.
It should have 2 stages. The first stage should take a username **and password** and the second stage is just the user_login
stage.

```terraform
data "authentik_source" "inbuilt" {
  managed = "goauthentik.io/sources/inbuilt"
}
data "authentik_stage" "default-authentication-password" {
  name = "default-authentication-password"
}
data "authentik_stage" "default-authentication-login" {
  name = "default-authentication-login"
}

resource "authentik_stage_identification" "api_auth_identification_stage" {
  name = "api_auth_identification"
  user_fields = ["username"]
  sources = [data.authentik_source.inbuilt.uuid]
  password_stage = data.authentik_stage.default-authentication-password.id
}

resource "authentik_flow" "home_assistant_api_password" {
  name = "simple-password"
  title = "Simple Password"
  slug = "simple-password"
  designation = "authentication" //it's for authenticating users
}

resource "authentik_flow_stage_binding" "simple_password_identification_binding" {
  target = authentik_flow.home_assistant_api_password.uuid
  stage  = authentik_stage_identification.api_auth_identification_stage.id
  order  = 0
}
resource "authentik_flow_stage_binding" "simple_password_login_binding" {
  target = authentik_flow.home_assistant_api_password.uuid
  stage  = data.authentik_stage.default-authentication-login.id
  order  = 20
}
```

---

## Usage

You can use the authentication provider directly via the command line:
```bash
./hass-cli-auth-authentik --username username --password password  --config_path config. toml
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
      command: "/your/path/to/hass-cli-auth-authentik"
      # optionally, if it's not in the same directory as the executable:
      #args: ["--config", "/path/to/config.toml"]
      meta: true
``` 

- Home Assistant supplies `username` and `password` as environment variables to the command.

### How the Integration Works

- If the authenticated user is in the configured `admin_group_name`, they are mapped to the Home Assistant admin group (`system-admin`).
- If the authenticated user is in the `user_group_name`, they are mapped to the Home Assistant users group (`system-users`).
- If no groups are configured, and the user is authenticated, they're allowed to log in, but no groups assigned.
- If any groups are configured, and the user is authenticated, but not in any of those groups, they are not allowed to log in.

---

## Troubleshooting

- Turn up verbosity using `-v` or `-vvv`: Note `TRACE` Level will output passwords onto the console, because they're in the environment.
- **User not found or not in required group:** Make sure the Authentik flow is correct and the user is a member of the correct group(s).
- **Timeout errors:** Increase the `timeout` parameter in `config.toml` if needed.
- **Flow errors:** Double-check your Authentik flow setup.

---

## License

[MIT License](./LICENSE)

---

### Notes
- Please refer to the [Authentik documentation](https://docs.goauthentik.io/docs/developer-docs/api/flow-executor)
  for flow setup and group management.

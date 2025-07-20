use crate::config::Config;
use crate::prelude::*;
use anyhow::anyhow;
use authentik_client::apis::configuration::Configuration;
use authentik_client::apis::core_api::core_users_me_retrieve;
use authentik_client::apis::flows_api::{flows_executor_get, flows_executor_solve};
use authentik_client::models::{
    ChallengeTypes, FlowChallengeResponseRequest, IdentificationChallengeResponseRequest,
};
use reqwest_middleware::ClientBuilder;

#[derive(Debug)]
pub struct UserInfo {
    pub display_name: String,
    pub groups: Vec<String>,
}

pub struct Authentik {}

impl Authentik {
    pub async fn authenticate_user(
        config: &Config,
        username: String,
        password: String,
    ) -> Result<Option<UserInfo>> {
        //Using the flow executor api with cookies and reqwest
        let client = reqwest::Client::builder()
            .cookie_store(true)
            .timeout(std::time::Duration::from_secs(config.timeout as u64))
            .build()?;

        let client_with_middleware = ClientBuilder::new(client).build();

        //Now I need a rust-client configuration
        let api_config = Configuration {
            base_path: format!("{}/api/v3", config.authentik_base_url),
            client: client_with_middleware,
            ..Default::default()
        };
        let flow_slug = config.flow_slug.as_str();

        let stage1 = flows_executor_get(&api_config, flow_slug, "").await?;

        let finished = if let ChallengeTypes::AkStageIdentification(identity) = stage1 {
            trace!("IDENTITY CHALLENGE: {:?}", identity);
            if let Some(user_fields) = identity.user_fields {
                if !user_fields.contains(&"username".to_string()) {
                    Err(anyhow!("Username field not found"))
                } else if !identity.password_fields {
                    Err(anyhow!("Password field not found"))
                } else {
                    let identification_response = IdentificationChallengeResponseRequest {
                        component: identity.component,
                        uid_field: username.clone(),
                        password: Some(Some(password.clone())),
                        captcha_token: None,
                    };
                    let stage1_response = FlowChallengeResponseRequest::AkStageIdentification(
                        identification_response,
                    );
                    let stage2 =
                        flows_executor_solve(&api_config, flow_slug, "", Some(stage1_response))
                            .await?;

                    if let ChallengeTypes::XakFlowRedirect(_redirect) = stage2 {
                        //If we made it this far, everything is groovy.
                        //Get user information and collect it
                        let myself = core_users_me_retrieve(&api_config)
                            .await
                            .context("Unable to get user details!")?;

                        let groups = myself
                            .user
                            .groups
                            .iter()
                            .map(|g| g.name.clone())
                            .collect::<Vec<String>>();
                        let display_name = myself.user.name.clone();
                        let is_active = myself.user.is_active;

                        if !is_active {
                            Err(anyhow!("User is not active"))
                        } else {
                            //If for some reason the user isn't active, it's rejected.
                            let user_info = UserInfo {
                                display_name,
                                groups,
                            };
                            trace!("MYSELF: {:?}", myself);
                            Ok(Some(user_info))
                        }
                    } else {
                        Err(anyhow!("Not a successful login stage!: {:?}", stage2))
                    }
                }
            } else {
                Err(anyhow!("No user fields found"))
            }
        } else if let ChallengeTypes::AkStageAccessDenied(access_denied) = stage1 {
            debug!("ACCESS DENIED: {:?}", access_denied);
            Ok(None)
        } else {
            Err(anyhow!("Not an identity challenge: {:?}", stage1))
        };

        trace!("Finished: {:?}", finished);
        finished
    }
}

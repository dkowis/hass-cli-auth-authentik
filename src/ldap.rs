use crate::config::Config;
use crate::prelude::*;
use crate::rfc4511::LdapResultCode;
use anyhow::anyhow;
use futures::stream::StreamExt;
use ldap3::{Ldap, LdapConn, Scope, SearchEntry};
use serde::Deserialize;
use url::Url;

#[derive(Debug, Deserialize)]
pub struct User {
    pub uid: String,
    pub cn: String,
    pub sn: String,
    pub mail: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "memberOf")]
    pub member_of: Vec<String>,
}

pub struct UserChecking {
    ldap_url: String,
    user_base_dn: String,
    bind_dn: String,
    bind_pw: String,
}

impl UserChecking {
    pub fn new(config: &Config) -> Result<Self> {
        let _ldap_url = Url::parse(&config.ldap_url).context("Invalid LDAP URL")?;

        Ok(Self {
            ldap_url: config.ldap_url.clone(),
            user_base_dn: config.user_base_dn.clone(),
            bind_dn: config.bind_dn.clone(),
            bind_pw: config.bind_pw.clone(),
        })
    }

    pub fn get_user(&self, username: &str) -> Result<Option<User>> {
        let mut ldap = LdapConn::new(&self.ldap_url)?;
        ldap.simple_bind(&self.bind_dn, &self.bind_pw)?;

        let (rs, res) = ldap
            .search(
                &self.user_base_dn,
                Scope::Subtree,
                &format!("cn={}", username),
                &["cn", "sn", "uid", "mail", "memberOf", "displayName"],
            )
            .context("Failed to search for user")?
            .success()
            .context("Search was unsuccessful?")?;

        trace!("res: {:?}", res);

        let result = if rs.len() == 0 {
            Ok(None)
        } else if rs.len() > 1 {
            error!("Found more than one user with the same username");
            Ok(None)
        } else {
            //Exactly one entry
            let entry = rs.get(0).unwrap();
            let search_entry = SearchEntry::construct(entry.clone());
            //take apart the search_entry into my User object.
            let transformed_groups = search_entry
                .attrs
                .get("memberOf")
                .unwrap()
                .iter()
                .map(|entry| {
                    //I want the cn= field, but only the value in there
                    let mut split = entry.split(",");
                    split.next().unwrap().to_string().replace("cn=", "")
                })
                .collect::<Vec<String>>();

            let user = User {
                uid: search_entry
                    .attrs
                    .get("uid")
                    .unwrap()
                    .first()
                    .unwrap()
                    .to_string(),
                cn: search_entry
                    .attrs
                    .get("cn")
                    .unwrap()
                    .first()
                    .unwrap()
                    .to_string(),
                sn: search_entry
                    .attrs
                    .get("sn")
                    .unwrap()
                    .first()
                    .unwrap()
                    .to_string(),
                mail: search_entry
                    .attrs
                    .get("mail")
                    .unwrap()
                    .first()
                    .unwrap()
                    .to_string(),
                display_name: search_entry
                    .attrs
                    .get("displayName")
                    .unwrap()
                    .first()
                    .unwrap()
                    .to_string(),
                member_of: transformed_groups,
            };
            trace!("searchEntry: {:?}", search_entry);

            Ok(Some(user))
        };
        ldap.unbind()?;

        result
    }

    pub fn verify_credentials(&self, username: &str, password: &str) -> Result<bool> {
        let mut ldap = LdapConn::new(&self.ldap_url)?;
        //Create the bind_dn
        let bind_dn = format!("cn={},{}", username, self.user_base_dn);
        trace!("Checking authentication bind as : {}", bind_dn);
        let bind_result = ldap
            .simple_bind(&bind_dn, &password)
            .context("Unable to perform LDAP operation")?;
        ldap.unbind().ok();

        trace!("bind_result: {:?}", bind_result);
        let result_code =
            LdapResultCode::from_u32(bind_result.rc).context("Invalid LDAP result code")?;

        trace!("result_code: {}", result_code.description());

        if result_code != LdapResultCode::Success {
            Ok(false)
        } else {
            Ok(true)
        }
    }
}

//! An implementation of an Ephemeral (in-memory) webauthn configuration provider
//! This stores all challenges and credentials in memory - IE they are lost on
//! service restart. It's only really useful for demo-sites, testing and as an
//! example/reference implementation of the WebauthnConfig trait.
use std::collections::BTreeMap;

use crate::proto::{Challenge, Credential, UserId};
use crate::WebauthnConfig;

/// An implementation of an Ephemeral (in-memory) webauthn configuration provider
/// This stores all challenges and credentials in memory - IE they are lost on
/// service restart. It's only really useful for demo-sites, testing and as an
/// example/reference implementation of the WebauthnConfig trait.
#[derive(Debug)]
pub struct WebauthnEphemeralConfig {
    chals: BTreeMap<UserId, Challenge>,
    creds: BTreeMap<UserId, Vec<Credential>>,
    rp_name: String,
    rp_id: String,
    rp_origin: String,
}

impl WebauthnConfig for WebauthnEphemeralConfig {
    /// Returns the relying party name. See the trait documentation for more.
    fn get_relying_party_name(&self) -> String {
        self.rp_name.clone()
    }

    /// Returns the relying party id. See the trait documentation for more.
    fn get_relying_party_id(&self) -> String {
        self.rp_id.clone()
    }

    /// Persist a challenge associated to a userId. See the trait documentation for more.
    fn persist_challenge(&mut self, userid: UserId, challenge: Challenge) -> Result<(), ()> {
        self.chals.insert(userid, challenge);
        Ok(())
    }

    /// Retrieve a challenge associated to a userId. See the trait documentation for more.
    fn retrieve_challenge(&mut self, userid: &UserId) -> Option<Challenge> {
        self.chals.remove(userid)
    }

    /// Assert if a credential related to a userId exists. See the trait documentation for more.
    fn does_exist_credential(&self, userid: &UserId, cred: &Credential) -> Result<bool, ()> {
        match self.creds.get(userid) {
            Some(creds) => Ok(creds.contains(cred)),
            None => Ok(false),
        }
    }

    /// Persist a credential related to a userId. See the trait documentation for more.
    fn persist_credential(&mut self, userid: UserId, credential: Credential) -> Result<(), ()> {
        match self.creds.get_mut(&userid) {
            Some(v) => {
                v.push(credential);
            }
            None => {
                self.creds.insert(userid, vec![credential]);
            }
        };
        Ok(())
    }

    /// Update a credentials counter. See the trait documentation for more.
    fn credential_update_counter(
        &mut self,
        userid: &UserId,
        cred: &Credential,
        counter: u32,
    ) -> Result<(), ()> {
        match self.creds.get_mut(userid) {
            Some(v) => {
                v.remove_item(cred);
                let mut c = cred.clone();
                c.counter = counter;
                v.push(c);
                Ok(())
            }
            None => {
                // Invalid state but not end of world ...
                Err(())
            }
        }
    }

    /// Report an invalid credential counter. See the trait documentation for more.
    fn credential_report_invalid_counter(
        &mut self,
        userid: &UserId,
        cred: &Credential,
        _counter: u32,
    ) -> Result<(), ()> {
        match self.creds.get_mut(userid) {
            Some(v) => {
                v.remove_item(cred);
                Ok(())
            }
            None => {
                // Invalid state but not end of world ...
                Err(())
            }
        }
    }

    /// Retrieve the credentials associated to a userId. See the trait documentation for more.
    fn retrieve_credentials(&self, userid: &UserId) -> Option<&Vec<Credential>> {
        self.creds.get(userid)
    }

    /// Retrieve the relying party origin. See the trait documentation for more.
    fn get_origin(&self) -> &String {
        &self.rp_origin
    }
}

impl WebauthnEphemeralConfig {
    /// Create a new Webauthn Ephemeral instance. This requires a provided relying party
    /// name, origin and id. See the trait documentation for more detail on relying party
    /// name, origin and id.
    pub fn new(rp_name: &str, rp_origin: &str, rp_id: &str) -> Self {
        WebauthnEphemeralConfig {
            chals: BTreeMap::new(),
            creds: BTreeMap::new(),
            rp_name: rp_name.to_string(),
            rp_id: rp_id.to_string(),
            rp_origin: rp_origin.to_string(),
        }
    }
}

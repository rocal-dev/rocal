use keyring::{Entry, Error};

const DEFAULT_KEY: &'static str = "default";

pub struct TokenManager;

#[allow(dead_code)]
impl TokenManager {
    pub fn set_token(kind: Kind, token: &str) -> Result<(), Error> {
        let entry = Entry::new(&kind.to_string(), DEFAULT_KEY)?;
        entry.set_password(token)?;
        Ok(())
    }

    pub fn get_token(kind: Kind) -> Result<String, Error> {
        let entry = Entry::new(&kind.to_string(), DEFAULT_KEY)?;
        let token = entry.get_password()?;
        Ok(token)
    }

    pub fn delete_token(kind: Kind) -> Result<(), Error> {
        let entry = Entry::new(&kind.to_string(), DEFAULT_KEY)?;
        entry.delete_credential()?;
        Ok(())
    }
}

pub enum Kind {
    RocalAccessToken,
    RocalRefreshToken,
}

impl Kind {
    pub fn to_string(&self) -> String {
        match self {
            Kind::RocalAccessToken => {
                format!("{}:rocal_access_token", env!("BUILD_PROFILE"))
            }
            Kind::RocalRefreshToken => {
                format!("{}:rocal_refresh_token", env!("BUILD_PROFILE"))
            }
        }
    }
}

use std::fmt;

/// Authentication configuration.
#[derive(Clone)]
pub enum Auth {
    /// No authentication.
    None,
    /// Alibaba Cloud access key authentication for RPC APIs.
    AccessKey(AccessKey),
}

impl Auth {
    /// Create a `None` authentication configuration.
    pub fn none() -> Self {
        Self::None
    }

    /// Create an access key authentication configuration.
    pub fn access_key(
        access_key_id: impl Into<String>,
        access_key_secret: impl Into<String>,
    ) -> Self {
        Self::AccessKey(AccessKey {
            access_key_id: access_key_id.into(),
            access_key_secret: SecretString::new(access_key_secret),
            security_token: None,
        })
    }

    /// Create an access key authentication configuration with a session token.
    pub fn access_key_with_security_token(
        access_key_id: impl Into<String>,
        access_key_secret: impl Into<String>,
        security_token: impl Into<String>,
    ) -> Self {
        Self::AccessKey(AccessKey {
            access_key_id: access_key_id.into(),
            access_key_secret: SecretString::new(access_key_secret),
            security_token: Some(SecretString::new(security_token)),
        })
    }

    pub(crate) fn as_access_key(&self) -> Option<&AccessKey> {
        match self {
            Auth::AccessKey(access_key) => Some(access_key),
            Auth::None => None,
        }
    }
}

impl Default for Auth {
    fn default() -> Self {
        Self::none()
    }
}

impl fmt::Debug for Auth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Auth::None => f.debug_tuple("Auth::None").finish(),
            Auth::AccessKey(access_key) => {
                f.debug_tuple("Auth::AccessKey").field(access_key).finish()
            }
        }
    }
}

/// Access key credentials.
#[derive(Clone)]
pub struct AccessKey {
    pub(crate) access_key_id: String,
    pub(crate) access_key_secret: SecretString,
    pub(crate) security_token: Option<SecretString>,
}

impl fmt::Debug for AccessKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug = f.debug_struct("AccessKey");
        debug.field("access_key_id", &self.access_key_id);
        debug.field("access_key_secret", &self.access_key_secret);
        debug.field("security_token", &self.security_token);
        debug.finish()
    }
}

/// A redacted string wrapper to reduce accidental secret leakage via logs or errors.
#[derive(Clone)]
pub struct SecretString(String);

impl SecretString {
    pub(crate) fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub(crate) fn expose(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for SecretString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("<redacted>")
    }
}

impl fmt::Display for SecretString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("<redacted>")
    }
}

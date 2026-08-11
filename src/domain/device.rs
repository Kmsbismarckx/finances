use crate::domain::{errors::DomainError, user::UserId};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DeviceId(String);

impl DeviceId {
    pub fn new(raw: String) -> Self {
        DeviceId(raw)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RefreshTokenHash(String);

impl RefreshTokenHash {
    pub fn from_hashed(hash: String) -> Self {
        RefreshTokenHash(hash)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug)]
pub struct Device {
    id: DeviceId,
    user_id: UserId,
    name: String,
    refresh_token_hash: RefreshTokenHash,
    revoked: bool,
}

impl Device {
    pub fn register(
        id: DeviceId,
        user_id: UserId,
        name: String,
        refresh_token_hash: RefreshTokenHash,
    ) -> Result<Self, DomainError> {
        if name.trim().is_empty() {
            return Err(DomainError::EmptyDeviceName);
        }

        Ok(Device {
            id,
            user_id,
            name,
            refresh_token_hash,
            revoked: false,
        })
    }

    pub fn id(&self) -> &DeviceId {
        &self.id
    }

    pub fn user_id(&self) -> &UserId {
        &self.user_id
    }

    pub fn refresh_token_hash(&self) -> &RefreshTokenHash {
        &self.refresh_token_hash
    }

    pub fn is_revoked(&self) -> bool {
        self.revoked
    }

    pub fn revoke(&mut self) {
        self.revoked = true
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::{
        device::{Device, DeviceId, RefreshTokenHash},
        errors::DomainError,
        user::UserId,
    };

    fn sample_device_id() -> DeviceId {
        DeviceId::new("d1".into())
    }

    fn sample_user_id() -> UserId {
        UserId::new("u1".into())
    }

    fn sample_refresh_token_hash() -> RefreshTokenHash {
        RefreshTokenHash::from_hashed("hash".to_string())
    }

    #[test]
    fn registers_not_revoked() {
        let sut = Device::register(
            sample_device_id(),
            sample_user_id(),
            "name".to_string(),
            sample_refresh_token_hash(),
        )
        .unwrap();

        assert!(!sut.is_revoked());
    }

    #[test]
    fn rejects_empty_name() {
        let sut = Device::register(
            sample_device_id(),
            sample_user_id(),
            " ".to_string(),
            sample_refresh_token_hash(),
        );

        assert_eq!(sut.unwrap_err(), DomainError::EmptyDeviceName);
    }

    #[test]
    fn revoke_flips_flag() {
        let mut sut = Device::register(
            sample_device_id(),
            sample_user_id(),
            "name".to_string(),
            sample_refresh_token_hash(),
        )
        .unwrap();

        sut.revoke();
        assert!(sut.is_revoked());
    }
}

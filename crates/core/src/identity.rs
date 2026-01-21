use ed25519_dalek::{SigningKey, VerifyingKey};
use rand::rngs::OsRng;

#[derive(Clone, Debug)]
pub struct Identity {
    pub signing_key: SigningKey,
    pub verifying_key: VerifyingKey,
}

impl Identity {
    pub fn generate() -> Self {
        let signing_key = SigningKey::generate(&mut OsRng);
        let verifying_key = signing_key.verifying_key();
        Self {
            signing_key,
            verifying_key,
        }
    }

    pub fn peer_id(&self) -> String {
        hex::encode(&self.verifying_key.to_bytes())
    }
}

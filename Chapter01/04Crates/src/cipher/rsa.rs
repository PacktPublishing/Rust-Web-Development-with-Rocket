use rand::rngs::OsRng;
use rsa::{PaddingScheme, PublicKey, RsaPrivateKey};
use std::error::Error;

const KEY_SIZE: usize = 2048;

pub struct Rsa {
    data: String,
    private_key: RsaPrivateKey,
}

impl Rsa {
    pub fn new(input: String) -> Result<Self, Box<dyn Error>> {
        let mut rng = OsRng;
        let private_key = RsaPrivateKey::new(&mut rng, KEY_SIZE)?;
        let public_key = private_key.to_public_key();
        let input_bytes = input.as_bytes();
        let encrypted_data =
            public_key.encrypt(&mut rng, PaddingScheme::new_pkcs1v15_encrypt(), input_bytes)?;
        let encoded_data = base64::encode(encrypted_data);
        Ok(Self {
            data: encoded_data,
            private_key,
        })
    }
}

impl super::Cipher for Rsa {
    fn original_string(&self) -> Result<String, Box<dyn Error>> {
        let decoded_data = base64::decode(&self.data)?;
        let decrypted_data = self
            .private_key
            .decrypt(PaddingScheme::new_pkcs1v15_encrypt(), &decoded_data)?;
        Ok(String::from_utf8(decrypted_data)?)
    }

    fn encrypted_string(&self) -> Result<String, Box<dyn Error>> {
        Ok(String::from(&self.data))
    }
}

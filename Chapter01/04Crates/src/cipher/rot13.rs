use std::error::Error;

pub struct Rot13(pub String);

impl super::Cipher for Rot13 {
    fn original_string(&self) -> Result<String, Box<dyn Error>> {
        Ok(String::from(&self.0))
    }

    fn encrypted_string(&self) -> Result<String, Box<dyn Error>> {
        Ok(self
            .0
            .chars()
            .map(|ch| match ch {
                'a'..='m' | 'A'..='M' => (ch as u8 + 13) as char,
                'n'..='z' | 'N'..='Z' => (ch as u8 - 13) as char,
                _ => ch,
            })
            .collect())
    }
}

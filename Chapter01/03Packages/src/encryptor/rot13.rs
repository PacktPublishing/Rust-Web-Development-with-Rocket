pub struct Rot13(pub String);

impl super::Encryptable for Rot13 {
    fn encrypt(&self) -> String {
        self.0
            .chars()
            .map(|ch| match ch {
                'a'..='m' | 'A'..='M' => (ch as u8 + 13) as char,
                'n'..='z' | 'N'..='Z' => (ch as u8 - 13) as char,
                _ => ch,
            })
            .collect()
    }
}

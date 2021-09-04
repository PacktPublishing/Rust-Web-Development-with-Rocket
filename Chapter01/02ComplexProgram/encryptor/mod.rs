pub mod rot13;

pub trait Encryptable {
    fn encrypt(&self) -> String;
}

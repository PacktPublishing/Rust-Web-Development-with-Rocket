pub struct Message {
    pub uuid: String,
    pub orig_filename: String,
    pub dest_filename: String,
}

impl Message {
    pub fn new() -> Self {
        Message {
            uuid: String::new(),
            orig_filename: String::new(),
            dest_filename: String::new(),
        }
    }
}

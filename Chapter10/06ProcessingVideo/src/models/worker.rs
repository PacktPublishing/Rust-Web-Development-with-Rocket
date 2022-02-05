#[derive(PartialEq, Debug)]
enum Type {
    None,
    Bitmap,
    Video,
}

#[derive(Debug)]
pub struct Message {
    m_type: Type,
    pub uuid: String,
    pub orig_filename: String,
    pub dest_filename: String,
}

impl Message {
    pub fn new() -> Self {
        Message {
            m_type: Type::None,
            uuid: String::new(),
            orig_filename: String::new(),
            dest_filename: String::new(),
        }
    }

    pub fn to_bitmap(&mut self) {
        self.m_type = Type::Bitmap
    }

    pub fn to_video(&mut self) {
        self.m_type = Type::Video
    }

    pub fn is_bitmap(&self) -> bool {
        self.m_type == Type::Bitmap
    }

    pub fn is_video(&self) -> bool {
        self.m_type == Type::Video
    }
}

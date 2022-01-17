use super::post::Post;
use crate::traits::DisplayPostContent;

pub struct TextPost(pub Post);

impl DisplayPostContent for TextPost {
    fn raw_html(&self) -> String {
        format!("<p>{}</p>", self.0.content)
    }
}

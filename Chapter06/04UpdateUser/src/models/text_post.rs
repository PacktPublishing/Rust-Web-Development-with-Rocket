use super::post::Post;
use crate::traits::DisplayPostContent;

pub struct TextPost(Post);

impl DisplayPostContent for TextPost {
    fn raw_html() -> String {
        todo!("will implement later")
    }
}

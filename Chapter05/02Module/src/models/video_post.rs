use super::post::Post;
use crate::traits::DisplayPostContent;

pub struct VideoPost(Post);

impl DisplayPostContent for VideoPost {
    fn raw_html() -> String {
        todo!("will implement later")
    }
}

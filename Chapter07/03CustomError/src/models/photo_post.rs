use super::post::Post;
use crate::traits::DisplayPostContent;

pub struct PhotoPost(Post);

impl DisplayPostContent for PhotoPost {
    fn raw_html() -> String {
        todo!("will implement later")
    }
}

use super::post::Post;
use crate::traits::DisplayPostContent;

pub struct TextPost<'r>(Post<'r>);

impl<'r> DisplayPostContent for TextPost<'r> {
    fn raw_html() -> String {
        todo!("will implement later")
    }
}

use super::post::Post;
use crate::traits::DisplayPostContent;

pub struct VideoPost<'r>(Post<'r>);

impl<'r> DisplayPostContent for VideoPost<'r> {
    fn raw_html() -> String {
        todo!("will implement later")
    }
}

use super::post::Post;
use crate::traits::DisplayPostContent;

pub struct PhotoPost<'r>(Post<'r>);
impl<'r> DisplayPostContent for PhotoPost<'r> {
    fn raw_html() -> String {
        todo!("will implement later")
    }
}

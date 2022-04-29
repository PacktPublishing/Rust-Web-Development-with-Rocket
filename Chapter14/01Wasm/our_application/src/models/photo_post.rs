use super::post::Post;
use crate::traits::DisplayPostContent;

pub struct PhotoPost<'a>(&'a Post);

impl<'a> PhotoPost<'a> {
    pub fn new(post: &'a Post) -> Self {
        PhotoPost(post)
    }
}

impl<'a> DisplayPostContent for PhotoPost<'a> {
    fn raw_html(&self) -> String {
        format!(
            r#"<figure><img src="{}" class="section media"/></figure>"#,
            self.0.content
        )
    }
}

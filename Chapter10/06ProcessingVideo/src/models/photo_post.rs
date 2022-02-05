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
        if self.0.content.starts_with("loading") {
            return String::from(
                "<figure><img src=\"/assets/loading.gif\" class=\"section media\"/></figure>",
            );
        }
        format!(
            r#"<figure><img src="{}" class="section media"/></figure>"#,
            self.0.content
        )
    }
}

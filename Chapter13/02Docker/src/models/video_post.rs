use super::post::Post;
use crate::traits::DisplayPostContent;

pub struct VideoPost<'a>(&'a Post);

impl<'a> VideoPost<'a> {
    pub fn new(post: &'a Post) -> Self {
        VideoPost(post)
    }
}

impl<'a> DisplayPostContent for VideoPost<'a> {
    fn raw_html(&self) -> String {
        if self.0.content.starts_with("loading") {
            return String::from(
                "<figure><img src=\"/assets/loading.gif\" class=\"section media\"/></figure>",
            );
        }
        format!(
            r#"<video width="320" height="240" controls>
    <source src="{}" type="video/mp4">
    Your browser does not support the video tag.
</video>"#,
            self.0.content
        )
    }
}

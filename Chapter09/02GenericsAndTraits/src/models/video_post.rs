use super::post::Post;
use crate::traits::DisplayPostContent;

pub struct VideoPost(pub Post);

impl DisplayPostContent for VideoPost {
    fn raw_html(&self) -> String {
        format!(
            r#"<video width="320" height="240" controls>
    <source src="{}" type="video/mp4">
    Your browser does not support the video tag.
</video>"#,
            self.0.content
        )
    }
}

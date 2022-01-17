use super::post::Post;
use crate::traits::DisplayPostContent;

pub struct PhotoPost(pub Post);

impl DisplayPostContent for PhotoPost {
    fn raw_html(&self) -> String {
        format!(
            r#"<figure><img src="{}" class="section media"/></figure>"#,
            self.0.content
        )
    }
}

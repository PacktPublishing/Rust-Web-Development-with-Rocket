use super::post::Post;
use crate::traits::DisplayPostContent;

pub struct TextPost<'a>(&'a Post);

impl<'a> TextPost<'a> {
    pub fn new(post: &'a Post) -> Self {
        TextPost(post)
    }
}

impl<'a> DisplayPostContent for TextPost<'a> {
    fn raw_html(&self) -> String {
        format!("<p>{}</p>", self.0.content)
    }
}

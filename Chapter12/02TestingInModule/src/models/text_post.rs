use crate::models::post::Post;
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

#[cfg(test)]
mod tests {
    use super::TextPost;
    use crate::models::our_date_time::OurDateTime;
    use crate::models::post::Post;
    use crate::models::post_type::PostType;
    use crate::traits::DisplayPostContent;
    use chrono::{offset::Utc, TimeZone};
    use uuid::Uuid;

    #[test]
    fn test_raw_html() {
        let created_at = OurDateTime(Utc.timestamp_nanos(1431648000000000));
        let post = Post {
            uuid: Uuid::new_v4(),
            user_uuid: Uuid::new_v4(),
            post_type: PostType::Text,
            content: String::from("hello"),
            created_at: created_at,
        };
        let text_post = TextPost::new(&post);
        assert!(
            text_post.raw_html() == String::from("<p>hello</p>"),
            "String is not equal, {}, {}",
            text_post.raw_html(),
            String::from("<p>hello</p>")
        );
        assert_eq!(text_post.raw_html(), String::from("<p>hello</p>"));
        assert_ne!(text_post.raw_html(), String::from("<img>hello</img>"));
    }
}

use ammonia::Builder;
use std::collections::hash_set::HashSet;

pub mod bool_wrapper;
pub mod our_date_time;
pub mod pagination;
pub mod photo_post;
pub mod post;
pub mod post_type;
pub mod text_post;
pub mod user;
pub mod user_status;
pub mod video_post;
pub mod worker;

pub fn clean_html(src: &str) -> String {
    Builder::default()
        .tags(HashSet::new())
        .clean(src)
        .to_string()
}

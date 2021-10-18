use super::our_date_time::OurDateTime;
use std::default::Default;

pub const DEFAULT_LIMIT: usize = 1;

#[derive(FromForm, Debug)]
pub struct Pagination {
    pub next: Option<OurDateTime>,
    pub limit: usize,
}

impl Default for Pagination {
    fn default() -> Self {
        Pagination {
            next: None,
            limit: DEFAULT_LIMIT,
        }
    }
}

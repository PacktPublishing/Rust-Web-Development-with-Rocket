use super::our_date_time::OurDateTime;

pub const DEFAULT_LIMIT: usize = 10;

#[derive(FromForm)]
pub struct Pagination {
    pub next: OurDateTime,
    pub limit: usize,
}

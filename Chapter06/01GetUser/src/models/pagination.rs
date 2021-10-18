use super::our_date_time::OurDateTime;

#[derive(FromForm)]
pub struct Pagination {
    pub next: OurDateTime,
    pub limit: usize,
}

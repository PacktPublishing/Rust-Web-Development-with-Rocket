use super::our_date_time::OurDateTime;

#[derive(FromForm)]
pub struct Pagination {
    pub cursor: OurDateTime,
    pub limit: usize,
}

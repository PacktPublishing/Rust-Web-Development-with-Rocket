use super::our_date_time::OurDateTime;

#[derive(FromForm)]
pub struct Pagination {
    cursor: OurDateTime,
    limit: usize,
}

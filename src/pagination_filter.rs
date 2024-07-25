pub struct PaginationFilter<T> {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub order_by: Option<T>,
}

pub fn extract_pagination<T>(o: Option<PaginationFilter<T>>) -> (Option<i64>, Option<i64>, Option<T>) {
    return match o {
        Some(x) => {
            (x.offset, x.limit, x.order_by)
        }
        None => {
            (None, None, None)
        }
    };
}

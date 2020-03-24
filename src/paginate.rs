use crate::errors::ApiError;
use std::cmp::min;

const DEFAULT_PER_PAGE: i64 = 10;

#[derive(Debug, Deserialize)]
pub struct PaginationRequest {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Links {
    base: String,
    first: String,
    last: String,
    prev: Option<String>,
    next: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Pagination {
    pub offset: i64,
    pub page: i64,
    pub per_page: i64,
    pub total: i64,
    pub total_pages: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PaginationResponse<T> {
    pub links: Links,
    pub pagination: Pagination,
    pub data: T,
}

pub fn get_pagination(page: Option<i64>, per_page: Option<i64>, total: i64) -> Pagination {
    let page = page.unwrap_or(1);
    let per_page = per_page.unwrap_or(DEFAULT_PER_PAGE);
    let total_pages = (total as f64 / per_page as f64).ceil() as i64;
    let offset = (page - 1) * per_page;
    Pagination {
        offset,
        page,
        per_page,
        total,
        total_pages,
    }
}

fn get_url(base: &String, page: i64, per_page: i64) -> String {
    format!("{}?page={}&per_page={}", base, page, per_page)
}

fn first_url(pagination: &Pagination, base: &String) -> String {
    get_url(base, 1, pagination.per_page)
}

fn last_url(pagination: &Pagination, base: &String) -> String {
    get_url(base, pagination.total_pages, pagination.per_page)
}

fn prev_url(pagination: &Pagination, base: &String) -> Option<String> {
    if pagination.page > 1 {
        let prev_page = min(pagination.page - 1, pagination.total_pages);
        return Some(get_url(base, prev_page, pagination.per_page));
    }
    None
}

fn next_url(pagination: &Pagination, base: &String) -> Option<String> {
    if pagination.page < pagination.total_pages {
        let next_page = pagination.page + 1;
        return Some(get_url(base, next_page, pagination.per_page));
    }
    None
}

pub fn paginate<T>(
    pagination: Pagination,
    data: T,
    base: String,
) -> Result<PaginationResponse<T>, ApiError> {
    let first = first_url(&pagination, &base);
    let last = last_url(&pagination, &base);
    let prev = prev_url(&pagination, &base);
    let next = next_url(&pagination, &base);
    let response = PaginationResponse {
        links: Links {
            base,
            first,
            last,
            prev,
            next,
        },
        pagination,
        data,
    };

    Ok(response)
}

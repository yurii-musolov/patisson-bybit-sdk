use serde::Deserialize;

use crate::{Timestamp, enums::Category, serde::empty_string_as_none};

/// Raw response envelope of the Bybit REST API.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Resp<T> {
    /// Bybit `retCode`, 0 on success.
    pub ret_code: i64,
    /// Bybit `retMsg`.
    pub ret_msg: String,
    /// Endpoint-specific result.
    pub result: T,
    /// Server time of the response, milliseconds.
    pub time: Option<Timestamp>,
    /// Per-item results of batch endpoints.
    pub ret_ext_info: Option<RetExtInfo>,
}

/// Error part of the response envelope.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct APIErrorResponse {
    /// Bybit `retCode`.
    pub ret_code: i64,
    /// Bybit `retMsg`.
    pub ret_msg: String,
}

/// A successful REST response.
#[derive(Debug, PartialEq)]
pub struct Response<T> {
    /// Endpoint-specific result.
    pub result: T,
    /// Server time of the response, milliseconds.
    pub time: Option<Timestamp>,
    /// Selected response headers (rate limit status, trace id).
    pub headers: Headers,
    /// Per-item results for batch endpoints; `None` for all non-batch calls.
    pub ret_ext_info: Option<RetExtInfo>,
}

/// A page of a cursor-paginated list.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CursorPagination<T> {
    /// Product type, if the endpoint returns it.
    pub category: Option<Category>,
    /// Cursor of the next page; `None` on the last page.
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub next_page_cursor: Option<String>,
    /// Items of this page.
    pub list: Vec<T>,
}

/// A list result without pagination.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct List<T> {
    /// Items.
    pub list: Vec<T>,
}

/// Response headers parsed by the client.
#[derive(Debug, PartialEq)]
pub struct Headers {
    /// `ret_code` header.
    pub ret_code: Option<i32>,
    /// `traceid` header, useful when contacting Bybit support.
    pub trace_id: Option<String>,
    /// `timenow` header: server time, milliseconds.
    pub time_now: Option<Timestamp>,
    /// `X-Bapi-Limit`: the limit of this endpoint.
    pub api_limit: Option<u64>,
    /// `X-Bapi-Limit-Status`: remaining requests in the current window.
    pub api_limit_status: Option<u64>,
    /// `X-Bapi-Limit-Reset-Timestamp`: when the window resets, milliseconds.
    pub api_limit_reset_timestamp: Option<Timestamp>,
}

impl Headers {
    /// `true` if the `ret_code` header is 0.
    pub fn is_ret_code_ok(&self) -> bool {
        match self.ret_code {
            Some(code) => code == 0,
            None => false,
        }
    }
}

/// Result of one item of a batch request.
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct BatchItemResult {
    /// Item `retCode`, 0 on success.
    pub code: i64,
    /// Item `retMsg`.
    pub msg: String,
}

/// Per-item status for batch trade endpoints.
/// For all other endpoints `retExtInfo` is `{}` — the `list` defaults to empty.
#[derive(Debug, Clone, Default, Deserialize, PartialEq)]
pub struct RetExtInfo {
    /// One result per item of the batch, in request order.
    #[serde(default)]
    pub list: Vec<BatchItemResult>,
}

/// Returned by position-management and other void-result endpoints whose
/// `result` field is the empty object `{}`.
#[derive(Debug, Deserialize, PartialEq)]
pub struct EmptyResult {}

//! HTTP Version Filters

use std::convert::Infallible;

use crate::filter::{filter_fn_one, Filter};

/// Creates a `Filter` to get the http version of the connection.
pub fn version() -> impl Filter<Extract = (http::Version,), Error = Infallible> + Copy {
  filter_fn_one(|route| futures_util::future::ok(route.version()))
}

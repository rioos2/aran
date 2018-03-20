use solicit::header::Headers;
use stream_part::HttpPartStream;
use resp::Response;

/// HTTP/2 service interface
///
/// Implemented by `Client` and it is callback provided by user.
pub trait Service: Send + Sync + 'static {
    fn start_request<'a>(&self, headers: Headers, req: HttpPartStream) -> Response;
}

use futures::future;
use futures::future::Future;
use futures::stream;
use futures::stream::Stream;

use bytes::Bytes;

use message::SimpleHttpMessage;
use solicit::header::Headers;
use solicit_async::*;

use error::Error;

use stream_part::*;

/// Convenient wrapper around async HTTP response future/stream
pub struct Response(pub HttpFutureSend<(Headers, HttpPartStream)>);

impl Response {
    // constructors

    pub fn new<F>(future: F) -> Response
    where
        F: Future<Item = (Headers, HttpPartStream), Error = Error> + Send + 'static,
    {
        Response(Box::new(future))
    }

    pub fn headers_and_stream(headers: Headers, stream: HttpPartStream) -> Response {
        Response::new(future::ok((headers, stream)))
    }

    pub fn headers_and_bytes_stream<S>(headers: Headers, content: S) -> Response
    where
        S: Stream<Item = Bytes, Error = Error> + Send + 'static,
    {
        Response::headers_and_stream(headers, HttpPartStream::bytes(content))
    }

    /// Create a response with only headers
    pub fn headers(headers: Headers) -> Response {
        Response::headers_and_bytes_stream(headers, stream::empty())
    }

    /// Create a response with headers and response body
    pub fn headers_and_bytes<B: Into<Bytes>>(header: Headers, content: B) -> Response {
        Response::headers_and_bytes_stream(header, stream::once(Ok(content.into())))
    }

    pub fn message(message: SimpleHttpMessage) -> Response {
        Response::headers_and_bytes(message.headers, message.body)
    }

    pub fn not_found_404() -> Response {
        Response::headers(Headers::not_found_404())
    }

    pub fn redirect_302(location: &str) -> Response {
        let mut headers = Headers::from_status(302);
        headers.add("location", location);
        Response::headers(headers)
    }

    pub fn from_stream<S>(stream: S) -> Response
    where
        S: Stream<Item = HttpStreamPart, Error = Error> + Send + 'static,
    {
        // Check that first frame is HEADERS
        Response::new(stream.into_future().map_err(|(p, _s)| p).and_then(
            |(first, rem)| match first {
                Some(part) => match part.content {
                    HttpStreamPartContent::Headers(headers) => {
                        Ok((headers, HttpPartStream::new(rem)))
                    }
                    HttpStreamPartContent::Data(..) => {
                        Err(Error::InvalidFrame("data before headers".to_owned()))
                    }
                },
                None => Err(Error::InvalidFrame(
                    "empty response, expecting headers".to_owned(),
                )),
            },
        ))
    }

    pub fn err(err: Error) -> Response {
        Response::new(future::err(err))
    }

    // getters

    pub fn into_stream_flag(self) -> HttpFutureStreamSend<HttpStreamPart> {
        Box::new(
            self.0
                .map(|(headers, rem)| {
                    // NOTE: flag may be wrong for first item
                    stream::once(Ok(HttpStreamPart::intermediate_headers(headers))).chain(rem)
                })
                .flatten_stream(),
        )
    }

    pub fn into_stream(self) -> HttpFutureStreamSend<HttpStreamPartContent> {
        Box::new(self.into_stream_flag().map(|c| c.content))
    }

    pub fn into_part_stream(self) -> HttpPartStream {
        HttpPartStream::new(self.into_stream_flag())
    }

    pub fn collect(self) -> HttpFutureSend<SimpleHttpMessage> {
        Box::new(
            self.into_stream()
                .fold(SimpleHttpMessage::new(), |mut c, p| {
                    c.add(p);
                    Ok::<_, Error>(c)
                }),
        )
    }
}

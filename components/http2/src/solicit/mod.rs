//! The module implements the framing layer of HTTP/2 and exposes an API for using it.

use std::u32;
use std::fmt;

use self::frame::settings::HttpSettings;

pub mod frame;
pub mod connection;
pub mod session;
pub mod header;

// 6.5.2 Defined SETTINGS Parameters
pub const DEFAULT_SETTINGS: HttpSettings = HttpSettings {
    header_table_size: 4_096,
    enable_push: true,
    max_concurrent_streams: u32::MAX,
    initial_window_size: 65_535,
    max_frame_size: 16_384,
    max_header_list_size: u32::MAX,
};

/// An alias for the type that represents the ID of an HTTP/2 stream
pub type StreamId = u32;

/// A set of protocol names that the library should use to indicate that HTTP/2
/// is supported during protocol negotiation (NPN or ALPN).
/// We include some of the drafts' protocol names, since there is basically no
/// difference for all intents and purposes (and some servers out there still
/// only officially advertise draft support).
/// TODO: Eventually only use "h2".
pub const ALPN_PROTOCOLS: &'static [&'static [u8]] = &[b"h2", b"h2-16", b"h2-15", b"h2-14"];

/// A sender MUST NOT allow a flow-control window to exceed 231-1 octets. If a sender receives
/// a WINDOW_UPDATE that causes a flow-control window to exceed this maximum,
/// it MUST terminate either the stream or the connection, as appropriate. For streams,
/// the sender sends a RST_STREAM with an error code of FLOW_CONTROL_ERROR; for the connection,
/// a GOAWAY frame with an error code of FLOW_CONTROL_ERROR is sent.
pub const MAX_WINDOW_SIZE: u32 = 0x7fffffff;

// 6.9 WINDOW_UPDATE
/// The payload of a WINDOW_UPDATE frame is one reserved bit plus an unsigned 31-bit integer
/// indicating the number of octets that the sender can transmit in addition to the existing
/// flow-control window. The legal range for the increment to the flow-control window
/// is 1 to 231-1 (2,147,483,647) octets.
pub const MAX_WINDOW_SIZE_INC: u32 = 0x7fffffff;

/// The struct represents the size of a flow control window.
///
/// It exposes methods that allow the manipulation of window sizes, such that they can never
/// overflow the spec-mandated upper bound.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct WindowSize(pub i32);
impl WindowSize {
    /// Tries to increase the window size by the given delta. If the WindowSize would overflow the
    /// maximum allowed value (2^31 - 1), returns an error case. If the increase succeeds, returns
    /// `Ok`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use httpbis::solicit::WindowSize;
    ///
    /// let mut window_size = WindowSize::new(65_535);
    /// assert_eq!(window_size.size(), 65_535);
    /// // An increase within the bounds...
    /// assert!(window_size.try_increase(100).is_ok());
    /// assert_eq!(window_size.size(), 65_635);
    /// // An increase that would overflow
    /// assert!(window_size.try_increase(0x7fffffff).is_err());
    /// assert_eq!(window_size.size(), 65_635);
    /// ```
    pub fn try_increase(&mut self, delta: u32) -> Result<(), ()> {
        // Someone's provided a delta that would definitely overflow the window size.
        if delta > MAX_WINDOW_SIZE_INC || delta == 0 {
            return Err(());
        }
        // Now it is safe to cast the delta to the `i32`.
        match self.0.checked_add(delta as i32) {
            None => {
                // When the add overflows, we will have went over the maximum allowed size of the
                // window size...
                Err(())
            }
            Some(next_val) => {
                // The addition didn't overflow, so the next window size is in the range allowed by
                // the spec.
                self.0 = next_val;
                Ok(())
            }
        }
    }

    /// Tries to decrease the size of the window by the given delta.
    ///
    /// There are situations where the window size should legitimately be allowed to become
    /// negative, so the only situation where the result is an error is if the window size would
    /// underflow, as this would definitely cause the peers to lose sync.
    ///
    /// # Example
    ///
    /// ```rust
    /// use httpbis::solicit::WindowSize;
    ///
    /// let mut window_size = WindowSize::new(65_535);
    /// assert_eq!(window_size.size(), 65_535);
    /// // A decrease...
    /// assert!(window_size.try_decrease(100).is_ok());
    /// assert_eq!(window_size.size(), 65_435);
    /// // A decrease that does not underflow
    /// assert!(window_size.try_decrease(0x7fffffff).is_ok());
    /// assert_eq!(window_size.size(), -2147418212);
    /// // A decrease that *would* underflow
    /// assert!(window_size.try_decrease(0x7fffffff).is_err());
    /// assert_eq!(window_size.size(), -2147418212);
    /// ```
    pub fn try_decrease(&mut self, delta: i32) -> Result<(), ()> {
        match self.0.checked_sub(delta) {
            Some(new) => {
                self.0 = new;
                Ok(())
            }
            None => Err(()),
        }
    }

    pub fn try_decrease_to_positive(&mut self, delta: i32) -> Result<(), ()> {
        match self.0.checked_sub(delta) {
            Some(new) if new >= 0 => {
                self.0 = new;
                Ok(())
            }
            _ => Err(()),
        }
    }

    /// Creates a new `WindowSize` with the given initial size.
    pub fn new(size: i32) -> WindowSize {
        WindowSize(size)
    }
    /// Returns the current size of the window.
    ///
    /// The size is actually allowed to become negative (for instance if the peer changes its
    /// intial window size in the settings); therefore, the return is an `i32`.
    pub fn size(&self) -> i32 {
        self.0
    }
}

impl fmt::Display for WindowSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

/// An enum representing the two possible HTTP schemes.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum HttpScheme {
    /// The variant corresponding to `http://`
    Http,
    /// The variant corresponding to `https://`
    Https,
}

impl HttpScheme {
    /// Returns a byte string representing the scheme.
    #[inline]
    pub fn as_bytes(&self) -> &'static [u8] {
        match *self {
            HttpScheme::Http => b"http",
            HttpScheme::Https => b"https",
        }
    }
}

#[cfg(test)]
pub mod tests;

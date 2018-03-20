use bytes::Bytes;
use bytes::Buf;
use bytes::IntoBuf;
use bytes::BigEndian;

use solicit::StreamId;
use solicit::frame::Frame;
use solicit::frame::FrameIR;
use solicit::frame::RawFrame;
use solicit::frame::FrameHeader;
use solicit::frame::parse_padded_payload;
use solicit::frame::builder::FrameBuilder;

use super::flags::Flag;
use super::flags::Flags;

pub const PUSH_PROMISE_FRAME_TYPE: u8 = 0x5;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PushPromiseFrame {
    /// The set of flags for the frame, packed into a single byte.
    pub flags: Flags<PushPromiseFlag>,
    /// The ID of the stream with which this frame is associated
    pub stream_id: StreamId,
    /// Promised Stream ID
    pub promised_stream_id: StreamId,
    /// The header fragment bytes stored within the frame.
    pub header_fragment: Bytes,
    /// The length of the padding, if any.
    pub padding_len: u8,
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum PushPromiseFlag {
    EndHeaders = 0x4,
    Padded = 0x8,
}

impl Flag for PushPromiseFlag {
    fn bitmask(&self) -> u8 {
        *self as u8
    }

    fn flags() -> &'static [PushPromiseFlag] {
        static FLAGS: &'static [PushPromiseFlag] = &[PushPromiseFlag::EndHeaders, PushPromiseFlag::Padded];
        FLAGS
    }
}

impl PushPromiseFrame {
    /// Returns the length of the payload of the current frame, including any
    /// possible padding in the number of bytes.
    fn payload_len(&self) -> u32 {
        let padding = if self.flags.is_set(PushPromiseFlag::Padded) {
            1 + self.padding_len as u32
        } else {
            0
        };

        let stream_id_len = 4;

        self.header_fragment.len() as u32 + stream_id_len + padding
    }
}

impl Frame for PushPromiseFrame {
    type FlagType = PushPromiseFlag;

    fn from_raw(raw_frame: &RawFrame) -> Option<PushPromiseFrame> {
        // Unpack the header
        let FrameHeader {
            length,
            frame_type,
            flags,
            stream_id,
        } = raw_frame.header();
        // Check that the frame type is correct for this frame implementation
        if frame_type != PUSH_PROMISE_FRAME_TYPE {
            return None;
        }

        // Check that the length given in the header matches the payload
        // length; if not, something went wrong and we do not consider this a
        // valid frame.
        if (length as usize) != raw_frame.payload().len() {
            return None;
        }

        let flags = Flags::new(flags);

        // +---------------+
        // |Pad Length? (8)|
        // +-+-------------+-----------------------------------------------+
        // |R|                  Promised Stream ID (31)                    |
        // +-+-----------------------------+-------------------------------+
        // |                   Header Block Fragment (*)                 ...
        // +---------------------------------------------------------------+
        // |                           Padding (*)                       ...
        // +---------------------------------------------------------------+

        let padded = flags.is_set(PushPromiseFlag::Padded);

        let (payload, pad_len) = match parse_padded_payload(raw_frame.payload(), padded) {
            None => return None,
            Some(t) => t,
        };

        let mut buf = (&payload).into_buf();

        let promised_stream_id = buf.get_u32::<BigEndian>();

        let header_fragment = payload.slice((length as usize) - buf.remaining(), payload.len());

        Some(PushPromiseFrame {
            header_fragment: header_fragment,
            stream_id: stream_id,
            padding_len: pad_len,
            flags: flags,
            promised_stream_id: promised_stream_id,
        })
    }

    fn flags(&self) -> Flags<PushPromiseFlag> {
        self.flags
    }

    fn get_stream_id(&self) -> StreamId {
        self.stream_id
    }

    fn get_header(&self) -> FrameHeader {
        FrameHeader {
            length: self.payload_len(),
            frame_type: PUSH_PROMISE_FRAME_TYPE,
            flags: self.flags.0,
            stream_id: self.stream_id,
        }
    }
}

impl FrameIR for PushPromiseFrame {
    fn serialize_into(self, b: &mut FrameBuilder) {
        b.write_header(self.get_header());
        let padded = self.flags.is_set(PushPromiseFlag::Padded);
        if padded {
            b.write_all(&[self.padding_len]);
        }
        // Now the actual headers fragment
        b.write_all(&self.header_fragment);
        // Finally, add the trailing padding, if required
        if padded {
            b.write_padding(self.padding_len);
        }
    }
}

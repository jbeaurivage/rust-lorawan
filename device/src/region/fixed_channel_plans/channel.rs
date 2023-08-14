/// an enum for channels channels 0-71
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Channel {
    _0,
    _1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
    _10,
    _11,
    _12,
    _13,
    _14,
    _15,
    _16,
    _17,
    _18,
    _19,
    _20,
    _21,
    _22,
    _23,
    _24,
    _25,
    _26,
    _27,
    _28,
    _29,
    _30,
    _31,
    _32,
    _33,
    _34,
    _35,
    _36,
    _37,
    _38,
    _39,
    _40,
    _41,
    _42,
    _43,
    _44,
    _45,
    _46,
    _47,
    _48,
    _49,
    _50,
    _51,
    _52,
    _53,
    _54,
    _55,
    _56,
    _57,
    _58,
    _59,
    _60,
    _61,
    _62,
    _63,
    _64,
    _65,
    _66,
    _67,
    _68,
    _69,
    _70,
    _71,
}

impl From<Channel> for u8 {
    fn from(x: Channel) -> u8 {
        unsafe { core::mem::transmute(x) }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    // we do this impl From<u8> for Channel for testing purposes only
    // if we can avoid ever doing this in the real code, we can avoid the necessary error handling
    fn channel_from_u8(x: u8) -> Channel {
        unsafe { core::mem::transmute(x) }
    }

    #[test]
    fn test_u8_from_channel() {
        for i in 0..71 {
            let channel = channel_from_u8(i);
            // check a few by hand to make sure
            match i {
                0 => assert_eq!(channel, Channel::_0),
                1 => assert_eq!(channel, Channel::_1),
                71 => assert_eq!(channel, Channel::_71),
                // the rest can be verified using the From<Channel> for u8 impl
                _ => assert_eq!(i, u8::from(channel)),
            }
        }
    }
}

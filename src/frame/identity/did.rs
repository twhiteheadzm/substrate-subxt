use crate::frame::utils::StringUtils;
use codec::{
    Decode,
    Encode,
};
// use sp_io;

// use sp_runtime::traits::Printable;
use sp_runtime::RuntimeDebug;

/// Borlaug DID.
/// DID is of the format: "did:bws:<32 Hex characters>".
///
/// A simple example of a Borlaug decentralized identifier (DID)
/// did:bws:123456789abcdefghi
#[derive(
    Encode, Decode, Default, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, RuntimeDebug,
)]
pub struct Did {
    pub id: [u8; 32],
}

// impl Printable for DID<Hash> {
//     fn print(&self) {
//         sp_io::misc::print_utf8("did:bws:".as_bytes());
//         sp_io::misc::print_hex(&self.0);
//     }
// }

impl From<&str> for Did {
    fn from(str: &str) -> Self {
        let mut array: [u8; 32] = Default::default();
        let hex_only = str.substring(2, 64);
        let bytes: &[u8] = &hex::decode(hex_only).unwrap();
        array.copy_from_slice(&bytes[0..32]);
        Did { id: array }
    }
}

use codec::{
    Decode,
    Encode,
};
use sp_core::H256;
// use sp_io;
// use sp_runtime::traits::Hash;
// use sp_runtime::traits::Printable;
// use sp_std::prelude::*;

/// Borlaug DID.
/// DID is of the format: "did:bws:<32 Hex characters>".
///
/// A simple example of a Borlaug decentralized identifier (DID)
/// did:bws:123456789abcdefghi
#[derive(Encode, Decode, Default, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Debug)]
pub struct Did {
    pub id: [u8; 32],
}

// impl Printable for DID<Hash> {
//     fn print(&self) {
//         sp_io::misc::print_utf8("did:bws:".as_bytes());
//         sp_io::misc::print_hex(&self.0);
//     }
// }

impl From<[u8; 32]> for Did {
    fn from(hash: [u8; 32]) -> Self {
        Did { id: hash }
    }
}

impl From<H256> for Did {
    fn from(hash: H256) -> Self {
        Did { id: hash.into() }
    }
}

// impl From<&Vec<u8>> for DID {
//     fn from(string_did: &Vec<u8>) -> Self {
//         let mut array = [0; UUID_LEN];
//         let bytes = &string_did[8..array.len()]; // panics if not enough data
//         array.copy_from_slice(bytes);
//         DID(array)
//     }
// }

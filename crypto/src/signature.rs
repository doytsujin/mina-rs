// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use crate::base58::Base58Encodable;
use serde::{Deserialize, Serialize};
use serde_versions_derive::version;

#[version(1)]
#[derive(Default, Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct CompressedCurvePoint {
    x: [u8; 32],
    is_odd: bool,
}

#[version(1)]
#[derive(Default, Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct PublicKey {
    poly: CompressedCurvePoint,
}

impl Base58Encodable for PublicKey {
    const VERSION_BYTE: u8 = crate::base58::version_bytes::NON_ZERO_CURVE_POINT_COMPRESSED;
}

#[cfg(test)]
pub mod tests {

    use super::*;
    use serde_bin_prot::to_writer;

    #[test]
    fn serialize_empty_keypair() {
        let mut buf = Vec::new();
        to_writer(&mut buf, &PublicKey::default()).unwrap();
        println!("{:?}", buf)
    }

    #[test]
    fn from_base58_roundtrip() {
        let s = "B62qonDZEKYULNkfq7WGu1Z881YBRnMSuBGGX5DhnTv26mUyvN99mpo";
        let k = PublicKey::from_base58(s).unwrap();
        assert_eq!(s, k.to_base58().into_string())
    }
}
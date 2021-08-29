use std::fmt::{Display, Formatter};

use bs58;
use p256::ecdsa::{SigningKey};
use p256::elliptic_curve::sec1::{EncodedPoint, FromEncodedPoint, ToEncodedPoint};
use p256::PublicKey;
use rand_core::OsRng;
use ripemd160::Ripemd160;
use sha2::digest::Output;
use sha2::{Digest, Sha256};

use super::store::WalletDB;

const CHECKSUM_LENGTH: usize = 4;
const VERSION: u8 = 0x00;

pub struct Wallet {
    pub private_key: SigningKey,
    pub public_key: PublicKey,
}

impl Display for Wallet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Wallet [ Address : {} ]",
            String::from_utf8(self.address()).unwrap()
        )
    }
}

fn public_key_hash(pub_key: &PublicKey) -> Vec<u8> {
    let enc_point = pub_key.to_encoded_point(true);
    let pub_hash: Output<Sha256> = Sha256::digest(enc_point.as_bytes());
    let public_ripe_md: Output<Ripemd160> = Ripemd160::digest(&pub_hash);
    public_ripe_md.to_vec()
}

fn checksum(payload: &Vec<u8>) -> Vec<u8> {
    let second_hash = Sha256::digest(&Sha256::digest(payload));
    second_hash.get(0..CHECKSUM_LENGTH).unwrap().to_vec()
}

impl Wallet {
    pub fn new() -> Wallet {
        let private_key: SigningKey = SigningKey::random(&mut OsRng);

        let public_key: PublicKey = private_key.verifying_key().into();
        Wallet {
            private_key,
            public_key,
        }
    }

    pub fn address(&self) -> Vec<u8> {
        let pub_hash = public_key_hash(&self.public_key);
        let versioned_hash: Vec<u8> = vec![vec![VERSION], pub_hash].concat();
        let check_sum = checksum(&versioned_hash);
        let full_hash: Vec<u8> = vec![versioned_hash, check_sum].concat();
        bs58::encode(full_hash).into_vec()
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let private_key_bytes = self.private_key.to_bytes();
        let public_key_bytes = self.public_key.to_encoded_point(false).to_bytes();
        let wallet = WalletDB {
            private_key: private_key_bytes.to_vec(),
            public_key: public_key_bytes.to_vec(),
        };
        bincode::serialize(&wallet).unwrap()
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        let wallet_db: WalletDB = bincode::deserialize(bytes).unwrap();
        let private_key = SigningKey::from_bytes(&wallet_db.private_key).unwrap();
        let public_key = PublicKey::from_encoded_point(
            &EncodedPoint::from_bytes(&wallet_db.public_key).unwrap(),
        )
        .unwrap();
        Wallet {
            private_key,
            public_key,
        }
    }
}

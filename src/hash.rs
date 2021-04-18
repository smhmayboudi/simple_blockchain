use crypto::{digest::Digest, sha3::Sha3};

pub fn sha256(data: &[u8]) -> String {
    let mut hasher = Sha3::sha3_256();
    hasher.input(data);
    hasher.result_str()
}

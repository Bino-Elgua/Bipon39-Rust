use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256, Sha512};
use subtle::ConstantTimeEq;

/// SHA-256 hash of `data`.
pub fn sha256(data: &[u8]) -> [u8; 32] {
    Sha256::digest(data).into()
}

/// HMAC-SHA512 of `data` with `key`.
pub fn hmac_sha512(key: &[u8], data: &[u8]) -> [u8; 64] {
    let mut mac = Hmac::<Sha512>::new_from_slice(key)
        .expect("HMAC accepts keys of any length");
    mac.update(data);
    mac.finalize().into_bytes().into()
}

/// SHA-256 binary Merkle tree over string leaves.
pub fn sha256_merkle_root(leaves: &[&str]) -> [u8; 32] {
    if leaves.is_empty() {
        return sha256(&[]);
    }

    let mut level: Vec<[u8; 32]> = leaves.iter().map(|leaf| sha256(leaf.as_bytes())).collect();

    while level.len() > 1 {
        if level.len() % 2 == 1 {
            let last = level[level.len() - 1];
            level.push(last);
        }

        let mut next = Vec::with_capacity(level.len() / 2);
        for pair in level.chunks_exact(2) {
            let mut joined = [0u8; 64];
            joined[..32].copy_from_slice(&pair[0]);
            joined[32..].copy_from_slice(&pair[1]);
            next.push(sha256(&joined));
        }
        level = next;
    }

    level[0]
}

/// Compute the wordlist Merkle root and return it as lowercase hex.
pub fn compute_wordlist_merkle_root() -> String {
    hex::encode(sha256_merkle_root(crate::wordlist::all_encoding_tokens()))
}

/// Timing-safe byte equality.
pub fn ct_eq(a: &[u8], b: &[u8]) -> bool {
    a.ct_eq(b).into()
}

#[cfg(test)]
mod tests {
    use super::sha256;

    #[test]
    fn sha256_empty_input() {
        assert_eq!(
            hex::encode(sha256(&[])),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }
}

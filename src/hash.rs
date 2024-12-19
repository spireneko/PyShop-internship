use sha2::{Digest, Sha256};

pub fn get_hash(num: usize) -> String {
    let mut hasher = Sha256::new();
    hasher.update(num.to_string());

    format!("{:x}", hasher.finalize())
}

pub fn has_n_trailing_zeros(str: &String, n_zeros: u32) -> bool {
    let mut count = 0;
    for &byte in str.as_bytes().iter().rev() {
        if byte == b'0' {
            count += 1;
        } else if count >= n_zeros {
            return true;
        } else {
            break;
        }
    }

    count >= n_zeros
}

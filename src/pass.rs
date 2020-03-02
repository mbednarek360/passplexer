use blake3;
use zero85::ToZ85;
use super::comm;

// slice to array
fn from_slice(bytes: &[u8]) -> [u8; 16] {
    let mut array = [0; 16];
    let bytes = &bytes[..array.len()];
    array.copy_from_slice(bytes);
    array
}

// salt: map index for service list
pub fn pass_gen(username: String, master: String, salt: u64) -> String {

    // check for empty input
    if username == "" || master == "" {
        comm::error("Invalid Field!".to_string());
        return "".to_string();
    }

    // hash user and master
    let user_hash: [u8; 16] = from_slice(&blake3::hash(username.as_bytes()).as_bytes()[0..16]);
    let master_hash: [u8; 16] = from_slice(&blake3::hash(master.as_bytes()).as_bytes()[0..16]);

    // interlace into single u8 array
    let mut interlace: [u8; 32] = [0; 32];
    for index in 0..user_hash.len() {
        interlace[index * 2] = user_hash[index];
        interlace[index * 2 + 1] = master_hash[index];
    }

    // generate key from salt
    let mut hash_key: [u8; 32] = [0; 32];
    let mut index = 0;
    for byte in &salt.to_le_bytes() {
        hash_key[index] = *byte;
        hash_key[index + 8] = *byte;
        hash_key[index + 16] = *byte;
        hash_key[index + 24] = *byte;
        index += 1;
    }

    // hash and encode into base 85
    let out: String = (*blake3::keyed_hash(&hash_key, &interlace).as_bytes()).to_z85().unwrap();
    comm::info(out.clone());
    out
}

/// Upcase a byte.
///
/// Note: We use the "ascii" case mapping.
pub fn upcase(b: u8) -> u8 {
    if b >= b'a' && b <= b'z' { b & !0x20 }
    else { b }
}

/// Downcase a byte.
///
/// Note: We use the "ascii" case mapping.
pub fn downcase(b: u8) -> u8 {
    if b >= b'A' && b <= b'Z' { b | 0x20 }
    else { b }
}


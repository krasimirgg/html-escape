use alloc::vec::Vec;

#[inline]
pub(crate) fn is_alphanumeric(e: u8) -> bool {
    b'0' <= e && e <= b'9' || b'a' <= e && e <= b'z' || b'A' <= e && e <= b'Z'
}

#[inline]
pub(crate) fn write_hex_to_vec(e: u8, output: &mut Vec<u8>) {
    output.reserve(6);

    let length = output.len();

    unsafe {
        output.set_len(length + 6);
    }

    let output = &mut output[length..];

    output[0] = b'&';
    output[1] = b'#';
    output[2] = b'x';
    output[5] = b';';

    let he = e >> 4;
    let le = e & 0xF;

    output[3] = if he >= 10 {
        b'A' - 10 + he
    } else {
        b'0' + he
    };

    output[4] = if le >= 10 {
        b'A' - 10 + le
    } else {
        b'0' + le
    };
}

#[inline]
pub(crate) fn write_html_entity_to_vec(e: u8, output: &mut Vec<u8>) {
    match e {
        b'&' => output.extend_from_slice(b"&amp;"),
        b'<' => output.extend_from_slice(b"&lt;"),
        b'>' => output.extend_from_slice(b"&gt;"),
        b'"' => output.extend_from_slice(b"&quot;"),
        _ => write_hex_to_vec(e, output),
    }
}
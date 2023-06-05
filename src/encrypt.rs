static base64_chars: &'static [u8] = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".as_bytes();

pub fn u8_base64(source: &[u8]) -> String {
    let mut ret = String::new();
    let (mut i, mut j, mut len) = (0, 0, 0);
    let (mut first_encode, mut second_encode): ([u8; 3], [u8; 4]) = ([0; 3], [0; 4]);
    while len != source.len() {
        first_encode[i] = source[len];
        i += 1; len += 1;
        if i == 3 {
            second_encode[0] = (first_encode[0] & 0xfc).wrapping_shr(2);
            second_encode[1] = (first_encode[0] & 0x03).wrapping_shl(4) + (first_encode[1] & 0xf0).wrapping_shr(4);
            second_encode[2] = (first_encode[1] & 0x0f).wrapping_shl(2) + (first_encode[2] & 0xc0).wrapping_shr(6);
            second_encode[3] = first_encode[2] & 0x3f;
            for k in 0..4 {
                ret.push(base64_chars[second_encode[k] as usize].into());
            }
            i = 0;
        }
    }
    if i != 0 {
        for k in i..3 {
            first_encode[k] = '\0' as u8;
        }
        second_encode[0] = (first_encode[0] & 0xfc).wrapping_shr(2);
        second_encode[1] = (first_encode[0] & 0x03).wrapping_shl(4) + (first_encode[1] & 0xf0).wrapping_shr(4);
        second_encode[2] = (first_encode[1] & 0x0f).wrapping_shl(2) + (first_encode[2] & 0xc0).wrapping_shr(6);
        for k in 0..i + 1 {
            ret.push(base64_chars[second_encode[k] as usize] as char);
        }
    }
    while i != 3 {
        ret.push('=');
        i += 1;
    }
    ret
}
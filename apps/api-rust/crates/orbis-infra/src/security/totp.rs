use async_trait::async_trait;
use orbis_app::ports::security::TotpService;
use uuid::Uuid;

pub struct TotpServiceImpl;

const BASE32_CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";

fn base32_encode(data: &[u8]) -> String {
    let mut result = String::new();
    let mut buffer: u32 = 0;
    let mut bits_left = 0;

    for &byte in data {
        buffer = (buffer << 8) | (byte as u32);
        bits_left += 8;
        while bits_left >= 5 {
            bits_left -= 5;
            let index = ((buffer >> bits_left) & 0x1F) as usize;
            result.push(BASE32_CHARS[index] as char);
        }
    }

    if bits_left > 0 {
        let index = ((buffer << (5 - bits_left)) & 0x1F) as usize;
        result.push(BASE32_CHARS[index] as char);
    }

    result
}

fn base32_decode(input: &str) -> Option<Vec<u8>> {
    let mut result = Vec::new();
    let mut buffer: u32 = 0;
    let mut bits_left = 0;

    for c in input.chars() {
        let val = match c.to_ascii_uppercase() {
            'A'..='Z' => (c as u8 - b'A') as u32,
            '2'..='7' => (c as u8 - b'2' + 26) as u32,
            '=' | ' ' | '-' => continue,
            _ => return None,
        };

        buffer = (buffer << 5) | val;
        bits_left += 5;

        if bits_left >= 8 {
            bits_left -= 8;
            result.push(((buffer >> bits_left) & 0xFF) as u8);
        }
    }

    Some(result)
}

fn sha1(data: &[u8]) -> [u8; 20] {
    let mut h0: u32 = 0x67452301;
    let mut h1: u32 = 0xEFCDAB89;
    let mut h2: u32 = 0x98BADCFE;
    let mut h3: u32 = 0x10325476;
    let mut h4: u32 = 0xC3D2E1F0;

    let msg_len = data.len();
    let mut padded = data.to_vec();
    padded.push(0x80);
    while (padded.len() % 64) != 56 {
        padded.push(0x00);
    }
    let bit_len = (msg_len as u64) * 8;
    padded.extend_from_slice(&bit_len.to_be_bytes());

    for chunk in padded.chunks(64) {
        let mut w = [0u32; 80];
        for i in 0..16 {
            w[i] = u32::from_be_bytes([
                chunk[i * 4],
                chunk[i * 4 + 1],
                chunk[i * 4 + 2],
                chunk[i * 4 + 3],
            ]);
        }
        for i in 16..80 {
            w[i] = (w[i - 3] ^ w[i - 8] ^ w[i - 14] ^ w[i - 16]).rotate_left(1);
        }

        let mut a = h0;
        let mut b = h1;
        let mut c = h2;
        let mut d = h3;
        let mut e = h4;

        for i in 0..80 {
            let (f, k) = match i {
                0..=19 => ((b & c) | ((!b) & d), 0x5A827999),
                20..=39 => (b ^ c ^ d, 0x6ED9EBA1),
                40..=59 => ((b & c) | (b & d) | (c & d), 0x8F1BBCDC),
                _ => (b ^ c ^ d, 0xCA62C1D6),
            };

            let temp = a.rotate_left(5).wrapping_add(f).wrapping_add(e).wrapping_add(k).wrapping_add(w[i]);
            e = d;
            d = c;
            c = b.rotate_left(30);
            b = a;
            a = temp;
        }

        h0 = h0.wrapping_add(a);
        h1 = h1.wrapping_add(b);
        h2 = h2.wrapping_add(c);
        h3 = h3.wrapping_add(d);
        h4 = h4.wrapping_add(e);
    }

    let mut result = [0u8; 20];
    result[0..4].copy_from_slice(&h0.to_be_bytes());
    result[4..8].copy_from_slice(&h1.to_be_bytes());
    result[8..12].copy_from_slice(&h2.to_be_bytes());
    result[12..16].copy_from_slice(&h3.to_be_bytes());
    result[16..20].copy_from_slice(&h4.to_be_bytes());
    result
}

fn hmac_sha1(key: &[u8], message: &[u8]) -> [u8; 20] {
    let mut k = [0u8; 64];
    if key.len() > 64 {
        let hash = sha1(key);
        k[..20].copy_from_slice(&hash);
    } else {
        k[..key.len()].copy_from_slice(key);
    }

    let mut o_key_pad = [0x5cu8; 64];
    let mut i_key_pad = [0x36u8; 64];

    for i in 0..64 {
        o_key_pad[i] ^= k[i];
        i_key_pad[i] ^= k[i];
    }

    let mut inner = i_key_pad.to_vec();
    inner.extend_from_slice(message);
    let inner_hash = sha1(&inner);

    let mut outer = o_key_pad.to_vec();
    outer.extend_from_slice(&inner_hash);
    sha1(&outer)
}

fn generate_totp_code(secret_bytes: &[u8], time_step: u64) -> String {
    let counter_bytes = time_step.to_be_bytes();
    let hash = hmac_sha1(secret_bytes, &counter_bytes);

    let offset = (hash[19] & 0x0f) as usize;
    let binary_code = (((hash[offset] & 0x7f) as u32) << 24)
        | (((hash[offset + 1] as u32)) << 16)
        | (((hash[offset + 2] as u32)) << 8)
        | ((hash[offset + 3] as u32));

    let otp = binary_code % 1_000_000;
    format!("{:06}", otp)
}

fn urlencoding_encode(s: &str) -> String {
    let mut res = String::new();
    for b in s.bytes() {
        match b {
            b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                res.push(b as char);
            }
            _ => {
                res.push_str(&format!("%{:02X}", b));
            }
        }
    }
    res
}

#[async_trait]
impl TotpService for TotpServiceImpl {
    fn generate_secret(&self) -> String {
        let u1 = Uuid::new_v4();
        let u2 = Uuid::new_v4();
        let mut raw = [0u8; 20];
        raw[..16].copy_from_slice(u1.as_bytes());
        raw[16..20].copy_from_slice(&u2.as_bytes()[..4]);
        base32_encode(&raw)
    }

    fn generate_qr_uri(&self, issuer: &str, account_name: &str, secret: &str) -> String {
        format!(
            "otpauth://totp/{}:{}?secret={}&issuer={}&algorithm=SHA1&digits=6&period=30",
            urlencoding_encode(issuer),
            urlencoding_encode(account_name),
            secret,
            urlencoding_encode(issuer)
        )
    }

    fn generate_recovery_codes(&self, count: usize) -> Vec<String> {
        let mut codes = Vec::with_capacity(count);
        for _ in 0..count {
            let u = Uuid::new_v4().simple().to_string();
            let c1 = &u[0..4];
            let c2 = &u[4..8];
            codes.push(format!("{}-{}", c1.to_uppercase(), c2.to_uppercase()));
        }
        codes
    }

    fn verify_code(&self, secret: &str, code: &str) -> bool {
        let secret_bytes = match base32_decode(secret) {
            Some(b) => b,
            None => return false,
        };

        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let current_step = current_time / 30;

        // Allow time drift: current_step - 1, current_step, current_step + 1
        for step_offset in [0i64, -1, 1] {
            let step = (current_step as i64 + step_offset) as u64;
            let expected = generate_totp_code(&secret_bytes, step);
            if expected == code.trim() {
                return true;
            }
        }

        false
    }
}

//! 混合数字，利用随机填充和校验码降低连续编号的可猜测性。
//!
//! 该模块只提供编号混淆，不提供密码学意义上的加密或认证。

use candid::CandidType;
use serde::{Deserialize, Serialize};

const CHECKSUM_LENGTH: usize = 4;
const MAX_INDEX_LENGTH: usize = std::mem::size_of::<u64>();
const MAX_ENCODED_LENGTH: usize = CHECKSUM_LENGTH + MAX_INDEX_LENGTH * 2;

/// 根据指定序号生成一个混淆后的字节串
#[inline]
pub fn encode_index_code(salt: &[u8], index: u64, random: Option<&[u8]>) -> Vec<u8> {
    let trimmed = trim_index(index); // 去除前置 0
    let mix = mix_numbers(&trimmed, random); // 用随机数拓展位数

    use sha2::Digest;
    let mut hasher = sha2::Sha256::new();
    hasher.update(&mix);
    hasher.update(salt); // 加盐
    let digest: [u8; 32] = hasher.finalize().into(); // 取得 hash 结果

    let mut show = Vec::with_capacity(mix.len() + 4);

    show.extend_from_slice(&digest[..CHECKSUM_LENGTH]); // 取前 4 位作为校验

    show.extend_from_slice(&mix); // 补上拓展后的数据

    show
}

/// 编码数字成字符串
#[inline]
pub fn encode_index_code_with_base32(salt: &[u8], index: u64, random: Option<&[u8]>) -> String {
    let show = encode_index_code(salt, index, random);
    base32::encode(base32::Alphabet::Rfc4648 { padding: false }, &show)
}

/// 混淆错误
#[derive(CandidType, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum MixNumberDecodeError {
    /// 长度错误
    WrongLength,
    /// 校验码错误
    WrongChecksum,
    /// 编码错误
    Base32DecodeError,
}
impl std::fmt::Display for MixNumberDecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                MixNumberDecodeError::WrongLength => "Wrong mix number length",
                MixNumberDecodeError::WrongChecksum => "Wrong mix number checksum",
                MixNumberDecodeError::Base32DecodeError => "base32 decode failed",
            }
        )
    }
}
impl std::error::Error for MixNumberDecodeError {}

/// 根据加密结果解析回序号
pub fn decode_index_code(salt: &[u8], show: &[u8]) -> Result<u64, MixNumberDecodeError> {
    if show.len() < CHECKSUM_LENGTH + 2 || MAX_ENCODED_LENGTH < show.len() || !show.len().is_multiple_of(2) {
        return Err(MixNumberDecodeError::WrongLength); // 长度不对
    }
    let mix = &show[CHECKSUM_LENGTH..];

    use sha2::Digest;
    let mut hasher = sha2::Sha256::new();
    hasher.update(mix);
    hasher.update(salt); // 加盐
    let digest: [u8; 32] = hasher.finalize().into(); // 取得 hash 结果

    if show[..CHECKSUM_LENGTH] != digest[..CHECKSUM_LENGTH] {
        return Err(MixNumberDecodeError::WrongChecksum); // 校验失败
    }

    let trimmed = restore_numbers(mix)?;

    let index = restore_index(&trimmed)?;

    Ok(index)
}
/// 根据加密字符串解析回序号
pub fn decode_index_code_by_base32(salt: &[u8], code: &str) -> Result<u64, MixNumberDecodeError> {
    let show = base32::decode(base32::Alphabet::Rfc4648 { padding: false }, code)
        .ok_or(MixNumberDecodeError::Base32DecodeError)?;
    decode_index_code(salt, &show)
}

// ================ 工具方法 ================

// 裁剪数字
// 保留有效位的数字, 最少一个 u8 // ? 也就是说前面太多 0 的情况下会只留下后面有效的
fn trim_index(index: u64) -> Vec<u8> {
    let bytes = index.to_be_bytes();
    let trimmed: Vec<u8> = bytes.into_iter().skip_while(|n| *n == 0).collect();
    if trimmed.is_empty() { vec![0] } else { trimmed }
}

// 恢复数字 大端法 高位在前
#[allow(clippy::identity_op)]
fn restore_index(numbers: &[u8]) -> Result<u64, MixNumberDecodeError> {
    if numbers.is_empty() || MAX_INDEX_LENGTH < numbers.len() {
        return Err(MixNumberDecodeError::WrongLength);
    }

    let mut bytes = [0_u8; 8];

    let len = numbers.len();
    for i in 0..len {
        bytes[8 - len + i] = numbers[i];
    }

    Ok(u64::from_be_bytes(bytes))
}

// 混合数字 // 位数交叉
fn mix_numbers(numbers: &[u8], random: Option<&[u8]>) -> Vec<u8> {
    #[allow(clippy::identity_op)]
    fn mix_single(m: u8, n: u8) -> [u8; 2] {
        [
            0b0000_0000
                | ((m & 0b1000_0000) >> 0)  // x000_0000 奇数位
                | ((m & 0b0100_0000) >> 1)  // 00x0_0000 奇数位
                | ((m & 0b0010_0000) >> 2)  // 0000_x000 奇数位
                | ((m & 0b0001_0000) >> 3)  // 0000_00x0 奇数位
                | ((n & 0b1000_0000) >> 1)  // 0x00_0000 偶数位
                | ((n & 0b0100_0000) >> 2)  // 000x_0000 偶数位
                | ((n & 0b0010_0000) >> 3)  // 0000_0x00 偶数位
                | ((n & 0b0001_0000) >> 4), // 0000_000x 偶数位
            0b0000_0000
                | ((m & 0b0000_1000) << 4)  // x000_0000 奇数位
                | ((m & 0b0000_0100) << 3)  // 00x0_0000 奇数位
                | ((m & 0b0000_0010) << 2)  // 0000_x000 奇数位
                | ((m & 0b0000_0001) << 1)  // 0000_00x0 奇数位
                | ((n & 0b0000_1000) << 3)  // 0x00_0000 偶数位
                | ((n & 0b0000_0100) << 2)  // 000x_0000 偶数位
                | ((n & 0b0000_0010) << 1)  // 0000_0x00 偶数位
                | ((n & 0b0000_0001) << 0), // 0000_000x 偶数位
        ]
    }

    let random = random.unwrap_or_default();

    let mut ns = Vec::with_capacity(numbers.len() * 2);
    for (i, n) in numbers.iter().enumerate() {
        ns.extend_from_slice(&mix_single(random.get(i).copied().unwrap_or(0), *n));
    }
    ns
}

// 恢复数字 // 位数交叉
fn restore_numbers(ns: &[u8]) -> Result<Vec<u8>, MixNumberDecodeError> {
    #[allow(clippy::identity_op)]
    fn restore_single(n1: u8, n2: u8) -> u8 {
        0b0000_0000
            | ((n1 & 0b0100_0000) << 1)
            | ((n1 & 0b0001_0000) << 2)
            | ((n1 & 0b0000_0100) << 3)
            | ((n1 & 0b0000_0001) << 4)
            | ((n2 & 0b0100_0000) >> 3)
            | ((n2 & 0b0001_0000) >> 2)
            | ((n2 & 0b0000_0100) >> 1)
            | ((n2 & 0b0000_0001) >> 0)
    }

    if ns.is_empty() || !ns.len().is_multiple_of(2) || MAX_INDEX_LENGTH * 2 < ns.len() {
        return Err(MixNumberDecodeError::WrongLength);
    }

    let mut numbers = Vec::new();
    for i in 0..(ns.len() / 2) {
        numbers.push(restore_single(ns[i * 2], ns[i * 2 + 1]));
    }
    Ok(numbers)
}

#[cfg(test)]
mod tests {
    use super::{MixNumberDecodeError, decode_index_code, decode_index_code_by_base32, encode_index_code};

    #[test]
    fn round_trips_boundary_values() {
        let salt = b"private-salt";
        for value in [0, 1, 255, 256, u64::MAX] {
            let encoded = encode_index_code(salt, value, Some(b"random!!"));
            assert_eq!(decode_index_code(salt, &encoded), Ok(value));
        }
    }

    #[test]
    fn rejects_invalid_checksum_and_oversized_payload() {
        let salt = b"private-salt";
        let mut encoded = encode_index_code(salt, 42, None);
        encoded[0] ^= 1;
        assert!(matches!(
            decode_index_code(salt, &encoded),
            Err(MixNumberDecodeError::WrongChecksum)
        ));

        assert!(matches!(
            decode_index_code(salt, &[0; 22]),
            Err(MixNumberDecodeError::WrongLength)
        ));
    }

    #[test]
    fn rejects_invalid_base32_without_panicking() {
        assert!(matches!(
            decode_index_code_by_base32(b"salt", "***"),
            Err(MixNumberDecodeError::Base32DecodeError)
        ));
    }
}

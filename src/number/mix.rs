/// 混合数字, 利用随机数进行数字混淆, 不容易猜中原数字

// 根据指定序号生成一个加密字符串
pub fn encode_index_code(salt: &[u8], index: u32, random: Vec<u8>) -> Vec<u8> {
    let trimmed = trim_index(index); // 去除前置 0
    let mix = mix_numbers(trimmed, random); // 用随机数拓展位数

    let mut plain = Vec::from(&mix[..]);
    plain.extend_from_slice(salt); // 加盐

    use sha2::Digest;
    let mut hasher = sha2::Sha256::new();
    hasher.update(&plain[..]);
    let digest: [u8; 32] = hasher.finalize().into(); // 取得 hash 结果
    let mut show = Vec::from(&digest[0..4]); // 取前 4 位作为校验

    show.extend_from_slice(&mix); // 补上拓展后的数据
    show
}

// 编码数字成字符串
pub fn encode_index_code_with_base32(salt: &[u8], index: u32, random: Vec<u8>) -> String {
    let show = encode_index_code(salt, index, random);
    let code = base32::encode(base32::Alphabet::RFC4648 { padding: false }, &show);
    code
}

// 根据加密结果解析回序号
pub fn decode_index_code(salt: &[u8], show: Vec<u8>) -> Result<u32, String> {
    if show.len() <= 4 {
        return Err(format!("wrong length")); // 长度不对
    }
    let mix = &show[4..];

    let mut plain = Vec::from(&mix[..]);
    plain.extend_from_slice(salt); // 加盐

    use sha2::Digest;
    let mut hasher = sha2::Sha256::new();
    hasher.update(&plain[..]);
    let digest: [u8; 32] = hasher.finalize().into(); // 取得 hash 结果
    if &show[0..4] != &digest[0..4] {
        return Err(format!("wrong data")); // 校验失败
    }

    let trimmed = restore_numbers(mix.iter().map(|n| *n).collect());

    let index = restore_index(trimmed);
    Ok(index)
}
// 根据加密字符串解析回序号
pub fn decode_index_code_by_base32(salt: &[u8], code: &str) -> Result<u32, String> {
    let show = base32::decode(base32::Alphabet::RFC4648 { padding: false }, code).unwrap();
    decode_index_code(salt, show)
}

// ================ 工具方法 ================

// 裁剪数字
// 保留有效位的数字, 最少一个 u8 // ? 也就是说前面太多 0 的情况下会只留下后面有效的
fn trim_index(index: u32) -> Vec<u8> {
    let b0 = ((index & 0xFF000000) >> 24) as u8;
    let b1 = ((index & 0x00FF0000) >> 16) as u8;
    let b2 = ((index & 0x0000FF00) >> 8) as u8;
    let b3 = ((index & 0x000000FF) >> 0) as u8;

    let mut numbers = Vec::new();

    if b0 != 0 {
        numbers.push(b0);
    }
    if !numbers.is_empty() || b1 != 0 {
        numbers.push(b1);
    }
    if !numbers.is_empty() || b2 != 0 {
        numbers.push(b2);
    }
    numbers.push(b3);

    numbers
}

// 恢复数字 大端法 高位在前
fn restore_index(numbers: Vec<u8>) -> u32 {
    let mut ns: Vec<u8> = numbers;
    while ns.len() < 4 {
        ns.insert(0, 0); // 不足长度的, 要在前面插入 0 凑足 4 位
    }
    let b0 = ns[0];
    let b1 = ns[1];
    let b2 = ns[2];
    let b3 = ns[3];
    0x0000_0000
        | ((b0 as u32) << 24)
        | ((b1 as u32) << 16)
        | ((b2 as u32) << 8)
        | ((b3 as u32) << 0)
}

// 混合数字 // 位数交叉
fn mix_numbers(numbers: Vec<u8>, random: Vec<u8>) -> Vec<u8> {
    fn mix_single(m: u8, n: u8) -> Vec<u8> {
        vec![
            0b0000_0000
                | ((m & 0b1000_0000) >> 0)
                | ((n & 0b1000_0000) >> 1)
                | ((m & 0b0100_0000) >> 1)
                | ((n & 0b0100_0000) >> 2)
                | ((m & 0b0010_0000) >> 2)
                | ((n & 0b0010_0000) >> 3)
                | ((m & 0b0001_0000) >> 3)
                | ((n & 0b0001_0000) >> 4),
            0b0000_0000
                | ((m & 0b0000_1000) << 4)
                | ((n & 0b0000_1000) << 3)
                | ((m & 0b0000_0100) << 3)
                | ((n & 0b0000_0100) << 2)
                | ((m & 0b0000_0010) << 2)
                | ((n & 0b0000_0010) << 1)
                | ((m & 0b0000_0001) << 1)
                | ((n & 0b0000_0001) << 0),
        ]
    }

    let mut ns = Vec::new();
    for i in 0..numbers.len() {
        ns.extend_from_slice(&mix_single(random[i], numbers[i]));
    }
    ns
}

// 恢复数字 // 位数交叉
fn restore_numbers(ns: Vec<u8>) -> Vec<u8> {
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

    let mut numbers = Vec::new();
    for i in 0..(ns.len() / 2) {
        numbers.push(restore_single(ns[i * 2], ns[i * 2 + 1]));
    }
    numbers
}

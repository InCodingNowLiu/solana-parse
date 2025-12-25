mod types;

use base64::{engine::general_purpose, Engine as _};
use sha2::{Digest, Sha256};
use std::mem::size_of;
use types::PositionV2;

fn anchor_discriminator(name: &str) -> [u8; 8] {
    let mut hasher = Sha256::new();
    hasher.update(format!("account:{}", name));
    let hash = hasher.finalize();
    hash[..8].try_into().unwrap()
}

fn main() {
    let base64_data = r#"
IQsxYrVlsQ1TBywBsASIE0wdAADwSQIArur//1IVAAD0AQAAAAAAANEMAADRDAAAAAAAAAAAAABC9UxpAAAAAAAAAAAAAAAA/lAAAwAAAABQAAAAUwcAAAW+eCTyRUvYaSVlhVatQEpgEsgxe8666vCXz7uBqoyzxvp6877brTo9ZfNqq8l0MbG75MLS9uDkfKYCA0UvXWEtchx2FNZLqapkJLyBWUlyv1hVNWGx70/xsxJrqjcOxSC5VSb+bAqsURupqEh//tjEhyx7QHfZ6n/vg9awV/5mOg2qUgAAAADITU8GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAlaWs4dKZhaNHMVH6SDCBZZp2BM5sWT2L3okgCHRtw5MAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAADABwAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAJDsOIaypV3TWmIf2Sw3Wh7aDgD8vGK+dljU9qruWREaAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABypJDSImBWv/PrvAMm3JqXPugSdtX9G3TaSiIbaimZ2gAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA==
"#;

    let raw = general_purpose::STANDARD
        .decode(base64_data.replace('\n', "").trim())
        .expect("base64 decode failed");

    println!("raw len = {}", raw.len());

    // 1️⃣ 校验 discriminator
    let discm: [u8; 8] = raw[..8].try_into().unwrap();
    let expected = anchor_discriminator("Position");

    println!("discriminator = {:?}", discm);
    println!("expected      = {:?}", expected);

    assert_eq!(discm, expected, "❌ not PositionV2");

    // 2️⃣ size 校验（非常重要）
    let body = &raw[8..];
    assert_eq!(
        body.len(),
        size_of::<PositionV2>(),
        "❌ size mismatch: body={}, struct={}",
        body.len(),
        size_of::<PositionV2>()
    );

    // 3️⃣ unsafe cast（Explorer / Anchor 的做法）
    let position = unsafe { &*(body.as_ptr() as *const PositionV2) };

    // 4️⃣ 打印关键字段
    println!("✅ Parsed PositionV2");
    println!("owner = {}", position.owner);
    println!("lb_pair = {}", position.lb_pair);
    println!("lower_bin_id = {}", position.lower_bin_id);
    println!("upper_bin_id = {}", position.upper_bin_id);
    println!("last_updated_at = {}", position.last_updated_at);
    println!(
        "total_claimed_fee_x = {}",
        position.total_claimed_fee_x_amount
    );
    println!(
        "total_claimed_fee_y = {}",
        position.total_claimed_fee_y_amount
    );
}

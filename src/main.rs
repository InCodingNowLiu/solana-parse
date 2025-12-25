mod types;

use base64::{engine::general_purpose, Engine as _};
use std::mem::size_of;
use std::ptr;
use types::LbPair;

fn main() {
    let base64_data = r#"
IQsxYrVlsQ1TBywBsASIE0wdAADwSQIArur//1IVAAD0AQAAAAAAABAnAAAAAAAAAgAAAAAAAAC5JE1pAAAAAAAAAAAAAAAA/lAAAwEAAABQAAAAUwcAAAW+eCTyRUvYaSVlhVatQEpgEsgxe8666vCXz7uBqoyzxvp6877brTo9ZfNqq8l0MbG75MLS9uDkfKYCA0UvXWEtchx2FNZLqapkJLyBWUlyv1hVNWGx70/xsxJrqjcOxSC5VSb+bAqsURupqEh//tjEhyx7QHfZ6n/vg9awV/5mAASvUgAAAABv/lQGAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAlaWs4dKZhaNHMVH6SDCBZZp2BM5sWT2L3okgCHRtw5MAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAADABwAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAJDsOIaypV3TWmIf2Sw3Wh7aDgD8vGK+dljU9qruWREaAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABypJDSImBWv/PrvAMm3JqXPugSdtX9G3TaSiIbaimZ2gAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA==
"#;

    // 1) Base64 -> bytes
    let raw = general_purpose::STANDARD
        .decode(base64_data.replace('\n', "").trim())
        .expect("base64 decode failed");

    println!("raw len = {}", raw.len());

    // 2) discriminator 校验（IDL 里的 LbPair discriminator）
    let discm: [u8; 8] = raw[..8].try_into().unwrap();
    let lbpair_discm: [u8; 8] = [33, 11, 49, 98, 181, 101, 177, 13];
    println!("discriminator = {:?}", discm);
    assert_eq!(discm, lbpair_discm, "❌ not LbPair");

    // 3) size 校验
    let body = &raw[8..];
    println!("body len = {}", body.len());
    println!("LbPair struct size = {}", size_of::<LbPair>());
    assert_eq!(body.len(), size_of::<LbPair>(), "❌ size mismatch");

    // 4) 关键修复：未对齐读取（LbPair 里含 u128 => 对齐 16，body 不保证对齐）
    let lbpair: LbPair = unsafe { ptr::read_unaligned(body.as_ptr() as *const LbPair) };

    // 5) 输出一些关键字段
    println!("✅ Parsed LbPair");
    println!("token_x_mint = {}", lbpair.token_x_mint);
    println!("token_y_mint = {}", lbpair.token_y_mint);
    println!("reserve_x    = {}", lbpair.reserve_x);
    println!("reserve_y    = {}", lbpair.reserve_y);
    println!("active_id    = {}", lbpair.active_id);
    println!("bin_step     = {}", lbpair.bin_step);
    println!("status       = {}", lbpair.status);
    println!("oracle       = {}", lbpair.oracle);
    println!("last_updated = {}", lbpair.last_updated_at);
    println!(
        "protocol_fee x/y = {}/{}",
        lbpair.protocol_fee.amount_x, lbpair.protocol_fee.amount_y
    );
}

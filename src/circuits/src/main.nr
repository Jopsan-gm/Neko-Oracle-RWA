// Function to sort values in ascending order
fn sort3(a: u64, b: u64, c: u64) -> [u64; 3] {
    let mut x = a;
    let mut y = b;
    let mut z = c;

    if x > y {
        let tmp = x;
        x = y;
        y = tmp;
    }

    if y > z {
        let tmp = y;
        y = z;
        z = tmp;
    }

    if x > y {
        let tmp = x;
        x = y;
        y = tmp;
    }

    [x, y, z]
}

// Two prices are aligned if they are within 7% (hardcoded) of each other.
fn are_close(p: u64, q: u64) -> bool {
    assert(p > 0);
    assert(q > 0);

    let diff = if p > q { p - q } else { q - p };
    let max  = if p > q { p } else { q };

    diff * 7 <= max
}

// Main circuit:
//  - p1, p2, p3: private feeds
//  - asset_id: public (binds proof to an asset, e.g. TSLA)
// Returns: public (final_price, asset_id)
fn main(
    p1: u64,
    p2: u64,
    p3: u64,
    asset_id: pub str<4>,
) -> pub (u64, str<4>) {
    assert(p1 > 0);
    assert(p2 > 0);
    assert(p3 > 0);

    let c12 = are_close(p1, p2);
    let c13 = are_close(p1, p3);
    let c23 = are_close(p2, p3);

    let final_price = if c12 & c13 & c23 {
        // Case 1: all three feeds agree (no outlier)
        let sorted = sort3(p1, p2, p3);
        sorted[1]
    } else if c12 & !c13 & !c23 {
        // Case 2: (p1, p2) aligned, p3 outlier
        (p1 + p2) / 2
    } else if c13 & !c12 & !c23 {
        // Case 3: (p1, p3) aligned, p2 outlier
        (p1 + p3) / 2
    } else if c23 & !c12 & !c13 {
        // Case 4: (p2, p3) aligned, p1 outlier
        (p2 + p3) / 2
    } else {
        // Anything else (e.g. weird inconsistent pattern) -> reject
        // Circuit fails -> no valid proof for this set of feeds.
        assert(false);
        0
    };

    (final_price, asset_id)
}

#[test]
fn test_main() {
    let asset_id: str<4> = "TSLA";
    
    // Test Case 1: All three feeds agree (within 7%)
    let (price, id) = main(100, 105, 102, asset_id);
    assert(price == 102); // median of [100, 102, 105]
    assert(id == asset_id);
    
    // Test Case 2: p1 and p2 aligned, p3 is outlier
    let (price2, id2) = main(100, 105, 200, asset_id);
    assert(price2 == 102); // (100 + 105) / 2 = 102
    assert(id2 == asset_id);
    
    // Test Case 3: p1 and p3 aligned, p2 is outlier
    let (price3, id3) = main(100, 200, 105, asset_id);
    assert(price3 == 102); // (100 + 105) / 2 = 102
    assert(id3 == asset_id);
    
    // Test Case 4: p2 and p3 aligned, p1 is outlier
    let (price4, id4) = main(200, 100, 105, asset_id);
    assert(price4 == 102); // (100 + 105) / 2 = 102
    assert(id4 == asset_id);
    
    // Test with 3-character ticker (padded to 4)
    let asset_id_3: str<4> = "BTC ";
    let (price5, id5) = main(50000, 50100, 50050, asset_id_3);
    assert(price5 == 50050);
    assert(id5 == asset_id_3);
}

#[test]
fn test_are_close() {
    // Within threshold: diff * 7 <= max
    assert(are_close(100, 107));
    assert(are_close(107, 100));
    assert(are_close(100, 100));
    assert(are_close(100, 114)); // 14 * 7 = 98 <= 114
    
    // Outside threshold
    assert(!are_close(100, 118)); // 18 * 7 = 126 > 118
    assert(!are_close(100, 200)); // 100 * 7 = 700 > 200
}

#[test]
fn test_sort3() {
    let sorted = sort3(3, 1, 2);
    assert(sorted[0] == 1);
    assert(sorted[1] == 2);
    assert(sorted[2] == 3);
    
    let sorted2 = sort3(10, 20, 5);
    assert(sorted2[0] == 5);
    assert(sorted2[1] == 10);
    assert(sorted2[2] == 20);
}

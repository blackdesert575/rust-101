use std::collections::HashMap;

fn check_hand(hand: &[u8; 5]) -> String {
    let mut counts = HashMap::new();

    // 計算每個數字的出現次數
    for &card in hand {
        *counts.entry(card).or_insert(0) += 1;
    }

    // 取得出現次數的值（例如 [3, 2] 或 [4, 1]）
    let mut frequencies: Vec<u8> = counts.values().cloned().collect();
    frequencies.sort_unstable(); // 排序讓判斷更簡單

    // 判斷牌型
    if frequencies == [2, 3] {
        "Full House (葫蘆)".to_string()
    } else if frequencies == [1, 4] {
        "Four of a Kind (鐵支)".to_string()
    } else {
        "Neither".to_string()
    }
}

fn main() {
    let hand1 = [3, 3, 3, 5, 5]; // 葫蘆
    let hand2 = [7, 7, 7, 7, 2]; // 鐵支
    let hand3 = [2, 2, 3, 3, 4]; // 不是葫蘆或鐵支

    println!("Hand {:?}: {}", hand1, check_hand(&hand1));
    println!("Hand {:?}: {}", hand2, check_hand(&hand2));
    println!("Hand {:?}: {}", hand3, check_hand(&hand3));
}
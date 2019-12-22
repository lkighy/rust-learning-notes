fn main() {
    // Slice 类型
    // 另一个没有所有权的数据类型
    // slice 允许你引用集合中一段连续的元素序列,而不用引用整个集合

    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // 这清空了字符串, 使其等于 ""
    
    // word 在此处值仍然是 5
    // 因为 s 被清空了, 而 word 记录的 仍然为 5
    // 但是没有字符串让我们可以有效地应用数值 5, word 的值现在完全无效(无用)
}

// 如果里面有
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
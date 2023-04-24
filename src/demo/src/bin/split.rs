fn main() {
    todo!()
}

fn split(s: &str, p: char) -> Option<(&str, &str)> {
    let pos = s.find(p);
    pos.map(|pos| {
        let len = s.len();
        let sep_len = p.len_utf8();
        unsafe {
            (
                s.get_unchecked(0..pos),
                s.get_unchecked((pos + sep_len)..len),
            )
        }
    })
}
///
///
///
///
/// 小提示，你可以把 s 先转换成裸指针，
/// 然后再用 std::slice::from_raw_parts_mut() 通过一个指针和一个长度，
/// 构建出一个 slice（还记得 &[u8] 其实内部就是一个 ptr + len 么？）。
/// 然后，再通过 std::str::from_utf8_unchecked_mut() 构建出 &mut str。

fn split_mut(s: &mut str, sep: char) -> Option<(&mut str, &mut str)> {
    let pos = s.find(sep);

    pos.map(|pos| {
        let len = s.len();
        let sep_len = sep.len_utf8();
        let ptr = s.as_mut_ptr();

        unsafe {
            let prev = std::slice::from_raw_parts_mut(ptr, pos);
            let after = std::slice::from_raw_parts_mut(ptr.add(pos + sep_len), len - pos - sep_len);
            (
                std::str::from_utf8_unchecked_mut(prev),
                std::str::from_utf8_unchecked_mut(after),
            )
        }
    })
}

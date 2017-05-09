fn grabbit(v: &mut Vec<i32>, idx: usize) -> &mut i32 {
    &mut v[idx]
}

fn main() {
    let mut v = vec![1, 2, 3, 4];
    {
        let q = 2;
        let val = grabbit(&mut v, q);
        *val = 1337;
        println!("{}: {}", q, val);
    }
    println!("{:?}", v);
}

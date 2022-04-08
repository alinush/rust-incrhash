use rust_incrhash::ristretto::RistBlakeIncHash;

fn main() {
    let mut h: RistBlakeIncHash = RistBlakeIncHash::default();

    println!("Incremental hash of empty set {{}}: {}", h);

    let s1 = "key1 = val1";
    let b1 = s1.as_bytes();
    let s2 = "key2 = val1";
    let b2 = s2.as_bytes();

    let e1 = RistBlakeIncHash::from(b1);
    let e2 = RistBlakeIncHash::from(b2);

    h += &e1;
    println!("Incremental hash of {{ {} }}: {}", s1, h);

    h += &e2;
    println!("Incremental hash of {{ {}, {} }}: {}", s1, s2, h);

    h -= &e2;
    println!("Incremental hash of {{ {} }}: {}", s1, h);

    h -= &e1;
    println!("Incremental hash of empty set {{}}: {}", h);
}

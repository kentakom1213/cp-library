use cp_library_rs::utils::run_length::{run_length_encode, RunLength};

#[test]
fn test_usize() {
    let arr = vec![0, 1, 1, 3, 3, 3, 2, 2, 1, 5, 9, 0];
    let comp = run_length_encode(&arr);
    let ans = vec![
        (0, 1),
        (1, 2),
        (3, 3),
        (2, 2),
        (1, 1),
        (5, 1),
        (9, 1),
        (0, 1),
    ];

    assert_eq!(comp, ans);
}

#[test]
fn test_string() {
    let strs = vec![
        "Welcome", "to", "Moo", "Moo", "Moo", "nsi", "nsi", "nsi", "nsi", "...", "nside.",
    ];
    let comp = run_length_encode(&strs);
    let ans = vec![
        ("Welcome", 1),
        ("to", 1),
        ("Moo", 3),
        ("nsi", 4),
        ("...", 1),
        ("nside.", 1),
    ];
    // [引用] "Mother2", nintendo, 1989

    assert_eq!(comp, ans);
}

#[test]
fn test_chars() {
    let str = "aaaxbbbbbbccddef";
    let chars: Vec<char> = str.chars().collect();
    let comp = run_length_encode(&chars);
    let ans = vec![
        ('a', 3),
        ('x', 1),
        ('b', 6),
        ('c', 2),
        ('d', 2),
        ('e', 1),
        ('f', 1),
    ];

    assert_eq!(comp, ans);
}

#[test]
fn test_chars_from_iter() {
    let str = "aaaxbbbbbbccddef";
    let comp = str.chars().run_length_encode();
    let ans = vec![
        ('a', 3),
        ('x', 1),
        ('b', 6),
        ('c', 2),
        ('d', 2),
        ('e', 1),
        ('f', 1),
    ];

    println!("{:?}", comp);
    assert_eq!(comp, ans);
}

#[test]
fn test_chars_from_iter_ref() {
    let vals = vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 5];
    let comp = vals.iter().run_length_encode();
    let ans = vec![(&1, 1), (&2, 2), (&3, 3), (&4, 4), (&5, 5)];

    println!("{:?}", comp);
    assert_eq!(comp, ans);
}

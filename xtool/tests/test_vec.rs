#[test]
fn test_chunk() {
    let src: Vec<u8> = vec![1, 2, 3, 4, 5];
    let dst: Vec<&[u8]> = src.chunks(3).collect();
    assert_eq!(dst, vec![vec![1, 2, 3], vec![4, 5]]);

    let dst2: Vec<Vec<u8>> = src.chunks(3).map(|s| s.into()).collect();
    assert_eq!(dst2, vec![vec![1, 2, 3], vec![4, 5]]);
}

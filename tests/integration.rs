use texttools_dedup::dedup_consecutive;

#[test]
fn integration_dedup_example() {
    assert_eq!(dedup_consecutive("a\na\nb\nc\nc"), "a\nb\nc");
}

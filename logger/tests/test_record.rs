use logger::{Level, Metadata, MetadataBuilder, Record, RecordBuilder};

#[test]
fn test_record_builder() {
    let target = "myApp";
    let metadata = MetadataBuilder::new().target(target).build();
    let fmt_args = format_args!("hello");
    let record_test = RecordBuilder::new()
        .args(fmt_args)
        .metadata(metadata)
        .module_path(Some("foo"))
        .file(Some("bar"))
        .line(Some(30))
        .build();
    assert_eq!(record_test.metadata().target(), "myApp");
    assert_eq!(record_test.module_path(), Some("foo"));
    assert_eq!(record_test.file(), Some("bar"));
    assert_eq!(record_test.line(), Some(30));
}

#[test]
fn test_record_convenience_builder() {
    let target = "myApp";
    let metadata = Metadata::builder().target(target).build();
    let fmt_args = format_args!("hello");
    let record_test = Record::builder()
        .args(fmt_args)
        .metadata(metadata)
        .module_path(Some("foo"))
        .file(Some("bar"))
        .line(Some(30))
        .build();
    assert_eq!(record_test.target(), "myApp");
    assert_eq!(record_test.module_path(), Some("foo"));
    assert_eq!(record_test.file(), Some("bar"));
    assert_eq!(record_test.line(), Some(30));
}

#[test]
fn test_record_complete_builder() {
    let target = "myApp";
    let record_test = Record::builder()
        .module_path(Some("foo"))
        .file(Some("bar"))
        .line(Some(30))
        .target(target)
        .level(Level::Error)
        .build();
    assert_eq!(record_test.target(), "myApp");
    assert_eq!(record_test.level(), Level::Error);
    assert_eq!(record_test.module_path(), Some("foo"));
    assert_eq!(record_test.file(), Some("bar"));
    assert_eq!(record_test.line(), Some(30));
}

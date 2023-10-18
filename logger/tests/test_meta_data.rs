use logger::{Level, Metadata, MetadataBuilder};

#[test]
fn test_metadata_builder() {
    let target = "myApp";
    let metadata_test =
        MetadataBuilder::new().level(Level::Debug).target(target).build();
    assert_eq!(metadata_test.level(), Level::Debug);
    assert_eq!(metadata_test.target(), "myApp");

    let target = "myApp";
    let metadata_test =
        MetadataBuilder::default().level(Level::Debug).target(target).build();
    assert_eq!(metadata_test.level(), Level::Debug);
    assert_eq!(metadata_test.target(), "myApp");
}

#[test]
fn test_metadata_convenience_builder() {
    let target = "myApp";
    let metadata_test =
        Metadata::builder().level(Level::Debug).target(target).build();
    assert_eq!(metadata_test.level(), Level::Debug);
    assert_eq!(metadata_test.target(), "myApp");
}

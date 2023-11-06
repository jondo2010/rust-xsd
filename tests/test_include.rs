//! Test the include directive

use xsd::schema::Schema;

#[test]
fn test_include() {
    // Load the content from file "test2.xsd"
    let content = std::fs::read_to_string("tests/test2.xsd").unwrap();
    // Parse the content as Schema
    let schema = yaserde::de::from_str::<Schema>(&content).unwrap();

    dbg!(schema);
}

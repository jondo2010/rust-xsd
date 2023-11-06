use super::*;

#[test]
fn test_schema_basic() {
    let content = r#"<?xml version="1.0" encoding="UTF-8"?>
<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema" elementFormDefault="qualified" attributeFormDefault="unqualified">
</xs:schema>"#;
    let schema = yaserde::de::from_str::<Schema>(content).unwrap();
    assert_eq!(
        schema,
        Schema {
            target_namespace: None,
            element_form_default: Qualification::Qualidified,
            attribute_form_default: Qualification::Unqualified,
            ..Default::default()
        }
    );
}

#[test]
fn test_schema_annotation() {
    let content = r#"<?xml version="1.0" encoding="UTF-8"?>
<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema">
        <xs:annotation><xs:documentation>doc</xs:documentation></xs:annotation>
</xs:schema>"#;
    let schema = yaserde::de::from_str::<Schema>(content).unwrap();
    assert_eq!(schema.annotation.unwrap().documentation, vec!["doc"]);
}

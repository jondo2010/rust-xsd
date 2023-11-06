//! This module contains the data structures and YaSerde derives for XSD.

use yaserde::YaDeserialize;
use yaserde_derive::YaDeserialize;

#[cfg(test)]
mod tests;

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "annotation"
    prefix = "xs",
    namespace = "xs: http://www.w3.org/2001/XMLSchema"
  )]
pub struct Annotation {
    #[yaserde(attribute)]
    pub id: Option<String>,
    #[yaserde(rename = "attribute")]
    pub attributes: Vec<Attribute>,
    #[yaserde(
      rename = "documentation"
      prefix = "xs",
      namespace = "xs: http://www.w3.org/2001/XMLSchema"
    )]
    pub documentation: Vec<String>,
}

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "attribute",
    prefix = "xs",
    namespace = "xs: http://www.w3.org/2001/XMLSchema"
)]
pub struct Attribute {
    #[yaserde(prefix = "xs", attribute)]
    pub name: Option<String>,
    #[yaserde(rename = "type", attribute)]
    pub kind: Option<String>,
    // #[yaserde(attribute)]
    // pub default: Option<String>,
    // #[yaserde(attribute)]
    // pub fixed: Option<String>,
    #[yaserde(rename = "use", attribute)]
    pub required: Required,
    #[yaserde(rename = "ref", attribute)]
    pub reference: Option<String>,
    #[yaserde(rename = "simpleType")]
    pub simple_type: Option<SimpleType>,
}

#[derive(Clone, Debug, PartialEq, YaDeserialize)]
pub enum Required {
    #[yaserde(rename = "optional")]
    Optional,
    #[yaserde(rename = "required")]
    Required,
}

impl Default for Required {
    fn default() -> Self {
        Required::Optional
    }
}

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "attributeGroup",
    prefix = "xs",
    namespace = "xs: http://www.w3.org/2001/XMLSchema"
)]
pub struct AttributeGroup {
    #[yaserde(prefix = "xs", attribute)]
    pub name: Option<String>,
    #[yaserde(rename = "ref", attribute)]
    pub reference: String,
    #[yaserde(rename = "attribute")]
    pub attributes: Vec<Attribute>,
    // #[yaserde(rename = "attributeGroup")]
    // pub attribute_group: Vec<AttributeGroup>,
}

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(prefix = "xs", namespace = "xs: http://www.w3.org/2001/XMLSchema")]
pub struct ComplexContent {
    pub extension: Option<Extension>,
}

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
  rename = "complexType"
  prefix = "xs",
  namespace = "xs: http://www.w3.org/2001/XMLSchema"
)]
pub struct ComplexType {
    #[yaserde(attribute)]
    pub name: String,
    #[yaserde(rename = "attribute")]
    pub attributes: Vec<Attribute>,
    pub sequence: Option<Sequence>,
    #[yaserde(rename = "simpleContent")]
    pub simple_content: Option<SimpleContent>,
    #[yaserde(rename = "complexContent")]
    pub complex_content: Option<ComplexContent>,
    #[yaserde(rename = "annotation")]
    pub annotation: Option<Annotation>,
}

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(prefix = "xs", namespace = "xs: http://www.w3.org/2001/XMLSchema")]
pub struct Element {
    #[yaserde(attribute)]
    pub name: String,
    #[yaserde(rename = "type", attribute)]
    pub kind: Option<String>,
    #[yaserde(rename = "ref", attribute)]
    pub refers: Option<String>,
    #[yaserde(rename = "minOccurs", attribute)]
    pub min_occurences: Option<u64>,
    #[yaserde(rename = "maxOccurs", attribute)]
    pub max_occurences: Option<MaxOccurences>,
    #[yaserde(rename = "complexType")]
    pub complex_type: Option<ComplexType>,
    #[yaserde(rename = "simpleType")]
    pub simple_type: Option<SimpleType>,
    #[yaserde(rename = "annotation")]
    pub annotation: Option<Annotation>,
}

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    root = "extension",
    prefix = "xs",
    namespace = "xs: http://www.w3.org/2001/XMLSchema"
)]
pub struct Extension {
    #[yaserde(attribute)]
    pub base: String,
    #[yaserde(rename = "attribute")]
    pub attributes: Vec<Attribute>,
    #[yaserde(rename = "sequence")]
    pub sequences: Vec<Sequence>,
}

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
  root="schema"
  prefix="xs",
  namespace="xs: http://www.w3.org/2001/XMLSchema",
)]
pub struct Import {
    #[yaserde(attribute)]
    pub id: Option<String>,
    #[yaserde(attribute)]
    pub namespace: Option<String>,
    #[yaserde(rename = "schemaLocation", attribute)]
    pub schema_location: Option<String>,
}

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
  root="schema"
  prefix="xs",
  namespace="xs: http://www.w3.org/2001/XMLSchema",
)]
pub struct Include {
    #[yaserde(attribute)]
    pub id: Option<String>,
    #[yaserde(rename = "schemaLocation", attribute)]
    pub schema_location: Option<String>,
}

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(prefix = "xs", namespace = "xs: http://www.w3.org/2001/XMLSchema")]
pub struct List {
    #[yaserde(rename = "itemType", attribute)]
    pub item_type: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum MaxOccurences {
    Unbounded,
    Number { value: u32 },
}

impl Default for MaxOccurences {
    fn default() -> Self {
        MaxOccurences::Unbounded
    }
}

impl YaDeserialize for MaxOccurences {
    fn deserialize<R: std::io::Read>(
        reader: &mut yaserde::de::Deserializer<R>,
    ) -> Result<Self, String> {
        use xml::reader::XmlEvent;
        if let XmlEvent::StartElement { name, .. } = reader.peek()? {
            if name.local_name != "MaxOccurences" {
                return Err("Unable to parse Max Occurences field".to_string());
            }
            let _start_event = reader.next_event();

            let content = reader.next_event()?;

            match content {
                XmlEvent::Characters(value) => {
                    if value == "unbounded" {
                        Ok(MaxOccurences::Unbounded)
                    } else {
                        let number = value.parse::<u32>().map_err(|e| e.to_string())?;
                        Ok(MaxOccurences::Number { value: number })
                    }
                }
                _ => Err("bad content for Max Occurences field".to_string()),
            }
        } else {
            Err("Missing start event for Max Occurences field".to_string())
        }
    }
}

#[derive(Clone, Debug, PartialEq, YaDeserialize)]
pub enum Qualification {
    #[yaserde(rename = "qualified")]
    Qualidified,
    #[yaserde(rename = "unqualified")]
    Unqualified,
}

impl Default for Qualification {
    fn default() -> Self {
        Qualification::Unqualified
    }
}

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(prefix = "xs", namespace = "xs: http://www.w3.org/2001/XMLSchema")]
pub struct Restriction {
    #[yaserde(rename = "base", attribute)]
    pub base: Option<String>,
}

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
  root="schema"
  prefix="xs",
  namespace="xs: http://www.w3.org/2001/XMLSchema",
)]
pub struct Schema {
    #[yaserde(rename = "targetNamespace", attribute)]
    pub target_namespace: Option<String>,
    #[yaserde(rename = "elementFormDefault", attribute)]
    pub element_form_default: Qualification,
    #[yaserde(rename = "attributeFormDefault", attribute)]
    pub attribute_form_default: Qualification,
    #[yaserde(rename = "import")]
    pub imports: Vec<Import>,
    #[yaserde(rename = "include")]
    pub includes: Vec<Include>,
    #[yaserde(rename = "element")]
    pub elements: Vec<Element>,
    #[yaserde(rename = "simpleType")]
    pub simple_type: Vec<SimpleType>,
    #[yaserde(rename = "complexType")]
    pub complex_type: Vec<ComplexType>,
    #[yaserde(rename = "attribute")]
    pub attributes: Vec<Attribute>,
    #[yaserde(rename = "attributeGroup")]
    pub attribute_group: Vec<AttributeGroup>,
    pub annotation: Option<Annotation>,
}

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(prefix = "xs", namespace = "xs: http://www.w3.org/2001/XMLSchema")]
pub struct Sequence {
    #[yaserde(rename = "element")]
    pub elements: Vec<Element>,
}

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(prefix = "xs", namespace = "xs: http://www.w3.org/2001/XMLSchema")]
pub struct SimpleContent {
    #[yaserde(prefix = "xs", rename = "extension")]
    pub extension: Extension,
}

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(prefix = "xs", namespace = "xs: http://www.w3.org/2001/XMLSchema")]
pub struct SimpleType {
    #[yaserde(attribute)]
    pub name: String,
    pub restriction: Option<Restriction>,
    pub list: Option<List>,
    pub union: Option<Union>,
}

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(prefix = "xs", namespace = "xs: http://www.w3.org/2001/XMLSchema")]
pub struct Union {
    #[yaserde(rename = "memberTypes", attribute)]
    pub member_types: String,
}

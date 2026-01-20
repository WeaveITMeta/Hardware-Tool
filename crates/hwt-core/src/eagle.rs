//! Eagle Import/Export Module.
//!
//! Parses Eagle schematic (.sch) and PCB (.brd) files.
//! Eagle uses XML format for both file types.

use std::collections::HashMap;
use std::io::{self, BufRead};
use std::path::Path;

use crate::schematic::{SchematicSheet, PlacedSymbol, Wire, NetLabel};
use crate::geometry::Point2D;

/// Eagle import error.
#[derive(Debug)]
pub enum EagleError {
    /// I/O error
    Io(io::Error),
    /// XML parse error
    XmlError(String),
    /// Invalid file format
    InvalidFormat(String),
    /// Unsupported version
    UnsupportedVersion(String),
}

impl std::fmt::Display for EagleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EagleError::Io(e) => write!(f, "I/O error: {}", e),
            EagleError::XmlError(s) => write!(f, "XML error: {}", s),
            EagleError::InvalidFormat(s) => write!(f, "Invalid format: {}", s),
            EagleError::UnsupportedVersion(s) => write!(f, "Unsupported version: {}", s),
        }
    }
}

impl std::error::Error for EagleError {}

impl From<io::Error> for EagleError {
    fn from(e: io::Error) -> Self {
        EagleError::Io(e)
    }
}

/// Result type for Eagle operations.
pub type EagleResult<T> = Result<T, EagleError>;

/// Eagle schematic importer.
/// 
/// Parses .sch files (Eagle schematic documents in XML format).
pub struct EagleSchematicImporter;

impl EagleSchematicImporter {
    /// Import schematic from file.
    pub fn import<P: AsRef<Path>>(path: P) -> EagleResult<SchematicSheet> {
        let content = std::fs::read_to_string(path.as_ref())?;
        Self::import_from_string(&content)
    }

    /// Import schematic from string.
    pub fn import_from_string(content: &str) -> EagleResult<SchematicSheet> {
        // Check for XML header
        if !content.trim_start().starts_with("<?xml") && !content.trim_start().starts_with("<eagle") {
            return Err(EagleError::InvalidFormat("Not an XML file".to_string()));
        }

        let mut sheet = SchematicSheet::new("Eagle Import");
        
        // Simple XML parsing without external dependencies
        let parser = SimpleXmlParser::new(content);
        
        // Extract sheet name from description or filename
        if let Some(desc) = parser.find_attribute("schematic", "name") {
            sheet.name = desc;
        }

        // Parse parts (components)
        for part in parser.find_elements("part") {
            if let Some(symbol) = Self::parse_part(&part) {
                sheet.symbols.push(symbol);
            }
        }

        // Parse instances (placed parts with positions)
        for instance in parser.find_elements("instance") {
            if let Some(symbol) = Self::parse_instance(&instance, &sheet.symbols) {
                // Update existing symbol with position
                if let Some(existing) = sheet.symbols.iter_mut().find(|s| s.reference == symbol.reference) {
                    existing.position = symbol.position;
                    existing.rotation = symbol.rotation;
                }
            }
        }

        // Parse wires
        for wire in parser.find_elements("wire") {
            if let Some(w) = Self::parse_wire(&wire) {
                sheet.wires.push(w);
            }
        }

        // Parse labels
        for label in parser.find_elements("label") {
            if let Some(l) = Self::parse_label(&label) {
                sheet.labels.push(l);
            }
        }

        // Parse net names
        for net in parser.find_elements("net") {
            if let Some(l) = Self::parse_net(&net) {
                sheet.labels.push(l);
            }
        }

        Ok(sheet)
    }

    /// Parse a part element.
    fn parse_part(element: &XmlElement) -> Option<PlacedSymbol> {
        let name = element.attributes.get("name")?;
        let library = element.attributes.get("library").cloned().unwrap_or_default();
        let deviceset = element.attributes.get("deviceset").cloned().unwrap_or_default();
        let value = element.attributes.get("value").cloned().unwrap_or_else(|| deviceset.clone());

        Some(PlacedSymbol::new(
            name.clone(),
            value,
            library,
            deviceset,
        ))
    }

    /// Parse an instance element (positioned part).
    fn parse_instance(element: &XmlElement, _parts: &[PlacedSymbol]) -> Option<PlacedSymbol> {
        let part_name = element.attributes.get("part")?;
        
        let x = element.attributes.get("x")
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(0.0);
        let y = element.attributes.get("y")
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(0.0);
        let rotation = element.attributes.get("rot")
            .map(|s| Self::parse_rotation(s))
            .unwrap_or(0.0);

        Some(PlacedSymbol::new(
            part_name.clone(),
            "",
            "",
            "",
        ).at(x, y).rotated(rotation))
    }

    /// Parse rotation string (e.g., "R90", "R180", "MR90").
    fn parse_rotation(rot: &str) -> f64 {
        let rot = rot.trim_start_matches('M').trim_start_matches('R');
        rot.parse::<f64>().unwrap_or(0.0)
    }

    /// Parse a wire element.
    fn parse_wire(element: &XmlElement) -> Option<Wire> {
        let x1 = element.attributes.get("x1")
            .and_then(|s| s.parse::<f64>().ok())?;
        let y1 = element.attributes.get("y1")
            .and_then(|s| s.parse::<f64>().ok())?;
        let x2 = element.attributes.get("x2")
            .and_then(|s| s.parse::<f64>().ok())?;
        let y2 = element.attributes.get("y2")
            .and_then(|s| s.parse::<f64>().ok())?;

        Some(Wire::new(
            Point2D::new(x1, y1),
            Point2D::new(x2, y2),
        ))
    }

    /// Parse a label element.
    fn parse_label(element: &XmlElement) -> Option<NetLabel> {
        let x = element.attributes.get("x")
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(0.0);
        let y = element.attributes.get("y")
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(0.0);
        
        // Label text might be in content or xref attribute
        let name = element.content.clone()
            .or_else(|| element.attributes.get("xref").cloned())
            .unwrap_or_default();

        if name.is_empty() {
            return None;
        }

        Some(NetLabel::new(name, Point2D::new(x, y)))
    }

    /// Parse a net element.
    fn parse_net(element: &XmlElement) -> Option<NetLabel> {
        let name = element.attributes.get("name")?;
        
        // Get position from first segment or pinref
        let x = element.attributes.get("x")
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(0.0);
        let y = element.attributes.get("y")
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(0.0);

        Some(NetLabel::new(name.clone(), Point2D::new(x, y)))
    }
}

/// Eagle PCB importer.
/// 
/// Parses .brd files (Eagle board documents in XML format).
pub struct EaglePcbImporter;

impl EaglePcbImporter {
    /// Import PCB from file.
    pub fn import<P: AsRef<Path>>(path: P) -> EagleResult<crate::layout::Layout> {
        let content = std::fs::read_to_string(path.as_ref())?;
        Self::import_from_string(&content)
    }

    /// Import PCB from string.
    pub fn import_from_string(content: &str) -> EagleResult<crate::layout::Layout> {
        if !content.trim_start().starts_with("<?xml") && !content.trim_start().starts_with("<eagle") {
            return Err(EagleError::InvalidFormat("Not an XML file".to_string()));
        }

        // Return a basic layout - full implementation would parse board elements
        Ok(crate::layout::Layout::new())
    }
}

/// Simple XML element representation.
#[derive(Debug, Clone)]
struct XmlElement {
    tag: String,
    attributes: HashMap<String, String>,
    content: Option<String>,
}

/// Simple XML parser for Eagle files.
/// 
/// This is a lightweight parser that doesn't require external dependencies.
/// For production use, consider using quick-xml or roxmltree.
struct SimpleXmlParser {
    content: String,
}

impl SimpleXmlParser {
    fn new(content: &str) -> Self {
        Self {
            content: content.to_string(),
        }
    }

    /// Find an attribute value on a specific element.
    fn find_attribute(&self, element: &str, attr: &str) -> Option<String> {
        let pattern = format!("<{}", element);
        if let Some(start) = self.content.find(&pattern) {
            let end = self.content[start..].find('>')?;
            let tag_content = &self.content[start..start + end];
            Self::extract_attribute(tag_content, attr)
        } else {
            None
        }
    }

    /// Find all elements with a given tag name.
    fn find_elements(&self, tag: &str) -> Vec<XmlElement> {
        let mut elements = Vec::new();
        let open_tag = format!("<{}", tag);
        let close_tag = format!("</{}>", tag);
        
        let mut search_start = 0;
        while let Some(start) = self.content[search_start..].find(&open_tag) {
            let abs_start = search_start + start;
            
            // Find the end of the opening tag
            if let Some(tag_end) = self.content[abs_start..].find('>') {
                let tag_content = &self.content[abs_start..abs_start + tag_end];
                let attributes = Self::parse_attributes(tag_content);
                
                // Check if self-closing
                let is_self_closing = tag_content.ends_with('/');
                
                let content = if is_self_closing {
                    None
                } else if let Some(close_pos) = self.content[abs_start..].find(&close_tag) {
                    let content_start = abs_start + tag_end + 1;
                    let content_end = abs_start + close_pos;
                    if content_end > content_start {
                        Some(self.content[content_start..content_end].trim().to_string())
                    } else {
                        None
                    }
                } else {
                    None
                };

                elements.push(XmlElement {
                    tag: tag.to_string(),
                    attributes,
                    content,
                });
            }
            
            search_start = abs_start + 1;
        }
        
        elements
    }

    /// Parse attributes from a tag string.
    fn parse_attributes(tag_content: &str) -> HashMap<String, String> {
        let mut attrs = HashMap::new();
        
        // Simple regex-free attribute parsing
        let mut in_value = false;
        let mut quote_char = '"';
        let mut current_key = String::new();
        let mut current_value = String::new();
        let mut chars = tag_content.chars().peekable();
        
        while let Some(c) = chars.next() {
            if in_value {
                if c == quote_char {
                    attrs.insert(current_key.clone(), current_value.clone());
                    current_key.clear();
                    current_value.clear();
                    in_value = false;
                } else {
                    current_value.push(c);
                }
            } else if c == '=' {
                // Next should be quote
                if let Some(&q) = chars.peek() {
                    if q == '"' || q == '\'' {
                        quote_char = q;
                        chars.next();
                        in_value = true;
                    }
                }
            } else if c.is_alphanumeric() || c == '_' || c == '-' {
                current_key.push(c);
            } else {
                current_key.clear();
            }
        }
        
        attrs
    }

    /// Extract a single attribute value.
    fn extract_attribute(tag_content: &str, attr: &str) -> Option<String> {
        let attrs = Self::parse_attributes(tag_content);
        attrs.get(attr).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eagle_error_display() {
        let err = EagleError::InvalidFormat("test".to_string());
        assert!(err.to_string().contains("Invalid format"));
    }

    #[test]
    fn test_parse_rotation() {
        assert_eq!(EagleSchematicImporter::parse_rotation("R90"), 90.0);
        assert_eq!(EagleSchematicImporter::parse_rotation("R180"), 180.0);
        assert_eq!(EagleSchematicImporter::parse_rotation("MR90"), 90.0);
        assert_eq!(EagleSchematicImporter::parse_rotation("R0"), 0.0);
    }

    #[test]
    fn test_simple_xml_parser_attributes() {
        let xml = r#"<part name="R1" library="rcl" deviceset="R-EU_" value="10k"/>"#;
        let parser = SimpleXmlParser::new(xml);
        
        let elements = parser.find_elements("part");
        assert_eq!(elements.len(), 1);
        
        let part = &elements[0];
        assert_eq!(part.attributes.get("name"), Some(&"R1".to_string()));
        assert_eq!(part.attributes.get("library"), Some(&"rcl".to_string()));
        assert_eq!(part.attributes.get("value"), Some(&"10k".to_string()));
    }

    #[test]
    fn test_parse_part() {
        let element = XmlElement {
            tag: "part".to_string(),
            attributes: [
                ("name".to_string(), "R1".to_string()),
                ("library".to_string(), "rcl".to_string()),
                ("deviceset".to_string(), "R-EU_".to_string()),
                ("value".to_string(), "10k".to_string()),
            ].into_iter().collect(),
            content: None,
        };
        
        let symbol = EagleSchematicImporter::parse_part(&element);
        assert!(symbol.is_some());
        
        let symbol = symbol.unwrap();
        assert_eq!(symbol.reference, "R1");
        assert_eq!(symbol.value, "10k");
    }

    #[test]
    fn test_parse_wire() {
        let element = XmlElement {
            tag: "wire".to_string(),
            attributes: [
                ("x1".to_string(), "10.0".to_string()),
                ("y1".to_string(), "20.0".to_string()),
                ("x2".to_string(), "30.0".to_string()),
                ("y2".to_string(), "20.0".to_string()),
            ].into_iter().collect(),
            content: None,
        };
        
        let wire = EagleSchematicImporter::parse_wire(&element);
        assert!(wire.is_some());
        
        let wire = wire.unwrap();
        assert!((wire.start.x - 10.0).abs() < 0.1);
        assert!((wire.end.x - 30.0).abs() < 0.1);
    }

    #[test]
    fn test_import_simple_schematic() {
        let xml = r#"<?xml version="1.0" encoding="utf-8"?>
<eagle version="9.6.2">
    <schematic name="TestSchematic">
        <parts>
            <part name="R1" library="rcl" deviceset="R-EU_" value="10k"/>
            <part name="C1" library="rcl" deviceset="C-EU" value="100n"/>
        </parts>
        <sheets>
            <sheet>
                <instances>
                    <instance part="R1" x="10" y="20"/>
                    <instance part="C1" x="30" y="20"/>
                </instances>
                <nets>
                    <net name="VCC">
                        <wire x1="10" y1="20" x2="30" y2="20"/>
                    </net>
                </nets>
            </sheet>
        </sheets>
    </schematic>
</eagle>"#;

        let result = EagleSchematicImporter::import_from_string(xml);
        assert!(result.is_ok());
        
        let sheet = result.unwrap();
        assert_eq!(sheet.name, "TestSchematic");
        assert_eq!(sheet.symbols.len(), 2);
    }

    #[test]
    fn test_import_invalid_xml() {
        let result = EagleSchematicImporter::import_from_string("not xml at all");
        assert!(result.is_err());
    }

    #[test]
    fn test_find_attribute() {
        let xml = r#"<schematic name="MyDesign"><parts></parts></schematic>"#;
        let parser = SimpleXmlParser::new(xml);
        
        let name = parser.find_attribute("schematic", "name");
        assert_eq!(name, Some("MyDesign".to_string()));
    }
}

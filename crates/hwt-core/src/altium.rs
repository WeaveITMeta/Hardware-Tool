//! Altium Import/Export Module.
//!
//! Parses Altium Designer schematic (.SchDoc) and PCB (.PcbDoc) files.
//! Altium uses OLE compound document format with binary streams.

use std::collections::HashMap;
use std::io::{self, Read, Cursor};
use std::path::Path;

use crate::schematic::{SchematicSheet, PlacedSymbol, Wire, NetLabel, LabelType};
use crate::geometry::Point2D;

/// Altium import error.
#[derive(Debug)]
pub enum AltiumError {
    /// I/O error
    Io(io::Error),
    /// Invalid file format
    InvalidFormat(String),
    /// Unsupported version
    UnsupportedVersion(String),
    /// Parse error
    ParseError(String),
}

impl std::fmt::Display for AltiumError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AltiumError::Io(e) => write!(f, "I/O error: {}", e),
            AltiumError::InvalidFormat(s) => write!(f, "Invalid format: {}", s),
            AltiumError::UnsupportedVersion(s) => write!(f, "Unsupported version: {}", s),
            AltiumError::ParseError(s) => write!(f, "Parse error: {}", s),
        }
    }
}

impl std::error::Error for AltiumError {}

impl From<io::Error> for AltiumError {
    fn from(e: io::Error) -> Self {
        AltiumError::Io(e)
    }
}

/// Result type for Altium operations.
pub type AltiumResult<T> = Result<T, AltiumError>;

/// Altium schematic importer.
/// 
/// Parses .SchDoc files (Altium Designer schematic documents).
/// Note: Full OLE compound document parsing requires additional dependencies.
/// This implementation provides a foundation for parsing the record-based format.
pub struct AltiumSchematicImporter;

impl AltiumSchematicImporter {
    /// Import schematic from file.
    pub fn import<P: AsRef<Path>>(path: P) -> AltiumResult<SchematicSheet> {
        let content = std::fs::read(path.as_ref())?;
        Self::import_from_bytes(&content)
    }

    /// Import schematic from bytes.
    pub fn import_from_bytes(data: &[u8]) -> AltiumResult<SchematicSheet> {
        // Check for OLE compound document signature
        if data.len() < 8 {
            return Err(AltiumError::InvalidFormat("File too small".to_string()));
        }

        // OLE signature: D0 CF 11 E0 A1 B1 1A E1
        let ole_signature = [0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1];
        if data.starts_with(&ole_signature) {
            // This is an OLE compound document
            // For full parsing, we would need to extract the "FileHeader" stream
            // and parse the record-based format within
            return Self::parse_ole_document(data);
        }

        // Try parsing as raw record stream (for extracted data)
        Self::parse_record_stream(data)
    }

    /// Parse OLE compound document.
    fn parse_ole_document(data: &[u8]) -> AltiumResult<SchematicSheet> {
        // OLE compound document parsing is complex
        // For now, return a placeholder with metadata
        let mut sheet = SchematicSheet::new("Altium Import");
        
        // Extract any readable strings from the binary data
        let strings = Self::extract_strings(data);
        
        // Try to find sheet name
        for s in &strings {
            if s.starts_with("SheetName=") {
                let name = s.strip_prefix("SheetName=").unwrap_or("Altium Import");
                sheet.name = name.to_string();
                break;
            }
        }

        Ok(sheet)
    }

    /// Parse Altium record stream format.
    fn parse_record_stream(data: &[u8]) -> AltiumResult<SchematicSheet> {
        let mut sheet = SchematicSheet::new("Altium Import");
        let mut cursor = Cursor::new(data);
        
        // Altium records are pipe-delimited key=value pairs
        // Each record starts with RECORD=N
        
        let mut buffer = Vec::new();
        cursor.read_to_end(&mut buffer)?;
        
        let content = String::from_utf8_lossy(&buffer);
        
        // Accumulate properties until we hit a new RECORD
        let mut current_props: HashMap<String, String> = HashMap::new();
        
        for part in content.split('|') {
            if part.is_empty() {
                continue;
            }
            
            if let Some((key, value)) = part.split_once('=') {
                let key_upper = key.to_uppercase();
                
                // If we hit a new RECORD, process the previous one first
                if key_upper == "RECORD" && !current_props.is_empty() {
                    Self::process_record(&current_props, &mut sheet);
                    current_props.clear();
                }
                
                current_props.insert(key_upper, value.to_string());
            }
        }
        
        // Process the last record
        if !current_props.is_empty() {
            Self::process_record(&current_props, &mut sheet);
        }

        Ok(sheet)
    }

    /// Process a single record based on its type.
    fn process_record(props: &HashMap<String, String>, sheet: &mut SchematicSheet) {
        if let Some(record_type) = props.get("RECORD") {
            match record_type.as_str() {
                "1" => {
                    // Component record
                    if let Some(symbol) = Self::parse_component(props) {
                        sheet.symbols.push(symbol);
                    }
                }
                "27" => {
                    // Wire record
                    if let Some(wire) = Self::parse_wire(props) {
                        sheet.wires.push(wire);
                    }
                }
                "25" => {
                    // Net label record
                    if let Some(label) = Self::parse_net_label(props) {
                        sheet.labels.push(label);
                    }
                }
                "31" => {
                    // Sheet record (metadata)
                    if let Some(name) = props.get("SHEETNAME") {
                        sheet.name = name.clone();
                    }
                }
                _ => {
                    // Other record types (power ports, junctions, etc.)
                }
            }
        }
    }

    /// Parse pipe-delimited properties.
    fn parse_properties(record: &str) -> HashMap<String, String> {
        let mut props = HashMap::new();
        
        for part in record.split('|') {
            if let Some((key, value)) = part.split_once('=') {
                props.insert(key.to_uppercase(), value.to_string());
            }
        }
        
        props
    }

    /// Parse component record.
    fn parse_component(props: &HashMap<String, String>) -> Option<PlacedSymbol> {
        let lib_ref = props.get("LIBREFERENCE").cloned().unwrap_or_default();
        let designator = props.get("DESIGNATOR").cloned().unwrap_or_default();
        
        if designator.is_empty() && lib_ref.is_empty() {
            return None;
        }

        let x = props.get("LOCATION.X")
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(0.0) / 10000.0;  // Altium uses 0.1 mil units
        let y = props.get("LOCATION.Y")
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(0.0) / 10000.0;
        let rotation = props.get("ORIENTATION")
            .and_then(|s| s.parse::<f64>().ok())
            .map(|o| o * 90.0)  // Altium uses 0-3 for 0/90/180/270
            .unwrap_or(0.0);

        Some(PlacedSymbol::new(
            designator,
            lib_ref.clone(),
            "Altium",
            lib_ref,
        ).at(x, y).rotated(rotation))
    }

    /// Parse wire record.
    fn parse_wire(props: &HashMap<String, String>) -> Option<Wire> {
        let x1 = props.get("LOCATION.X")
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(0.0) / 10000.0;
        let y1 = props.get("LOCATION.Y")
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(0.0) / 10000.0;
        let x2 = props.get("CORNER.X")
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(0.0) / 10000.0;
        let y2 = props.get("CORNER.Y")
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(0.0) / 10000.0;

        Some(Wire::new(
            Point2D::new(x1, y1),
            Point2D::new(x2, y2),
        ))
    }

    /// Parse net label record.
    fn parse_net_label(props: &HashMap<String, String>) -> Option<NetLabel> {
        let name = props.get("TEXT").cloned().unwrap_or_default();
        if name.is_empty() {
            return None;
        }

        let x = props.get("LOCATION.X")
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(0.0) / 10000.0;
        let y = props.get("LOCATION.Y")
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(0.0) / 10000.0;

        Some(NetLabel::new(name, Point2D::new(x, y)))
    }

    /// Extract readable ASCII strings from binary data.
    fn extract_strings(data: &[u8]) -> Vec<String> {
        let mut strings = Vec::new();
        let mut current = String::new();
        
        for &byte in data {
            if byte >= 0x20 && byte < 0x7F {
                current.push(byte as char);
            } else if !current.is_empty() {
                if current.len() >= 4 && current.contains('=') {
                    strings.push(current.clone());
                }
                current.clear();
            }
        }
        
        if !current.is_empty() && current.len() >= 4 && current.contains('=') {
            strings.push(current);
        }
        
        strings
    }
}

/// Altium PCB importer.
/// 
/// Parses .PcbDoc files (Altium Designer PCB documents).
pub struct AltiumPcbImporter;

impl AltiumPcbImporter {
    /// Import PCB from file.
    pub fn import<P: AsRef<Path>>(path: P) -> AltiumResult<crate::layout::Layout> {
        let content = std::fs::read(path.as_ref())?;
        Self::import_from_bytes(&content)
    }

    /// Import PCB from bytes.
    pub fn import_from_bytes(data: &[u8]) -> AltiumResult<crate::layout::Layout> {
        // Check for OLE compound document signature
        if data.len() < 8 {
            return Err(AltiumError::InvalidFormat("File too small".to_string()));
        }

        let ole_signature = [0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1];
        if !data.starts_with(&ole_signature) {
            return Err(AltiumError::InvalidFormat("Not an OLE compound document".to_string()));
        }

        // Return a basic layout for now
        // Full PCB parsing would require extracting and parsing multiple streams
        Ok(crate::layout::Layout::new())
    }
}

/// Altium library importer.
/// 
/// Parses .SchLib (schematic library) and .PcbLib (PCB library) files.
pub struct AltiumLibraryImporter;

impl AltiumLibraryImporter {
    /// Import schematic library from file.
    pub fn import_schlib<P: AsRef<Path>>(path: P) -> AltiumResult<Vec<crate::library::LibraryComponent>> {
        let content = std::fs::read(path.as_ref())?;
        Self::import_schlib_from_bytes(&content)
    }

    /// Import schematic library from bytes.
    pub fn import_schlib_from_bytes(data: &[u8]) -> AltiumResult<Vec<crate::library::LibraryComponent>> {
        // Check for OLE signature
        if data.len() < 8 {
            return Err(AltiumError::InvalidFormat("File too small".to_string()));
        }

        let ole_signature = [0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1];
        if !data.starts_with(&ole_signature) {
            return Err(AltiumError::InvalidFormat("Not an OLE compound document".to_string()));
        }

        // Extract component names from binary data
        let strings = AltiumSchematicImporter::extract_strings(data);
        let mut components = Vec::new();
        
        for s in strings {
            if s.starts_with("ComponentName=") {
                let name = s.strip_prefix("ComponentName=").unwrap_or("");
                if !name.is_empty() {
                    components.push(crate::library::LibraryComponent::new(
                        name,
                        crate::library::ComponentType::Symbol,
                    ));
                }
            }
        }

        Ok(components)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_altium_error_display() {
        let err = AltiumError::InvalidFormat("test".to_string());
        assert!(err.to_string().contains("Invalid format"));
    }

    #[test]
    fn test_parse_properties() {
        let record = "RECORD=1|DESIGNATOR=R1|LIBREFERENCE=Resistor|LOCATION.X=10000";
        let props = AltiumSchematicImporter::parse_properties(record);
        
        assert_eq!(props.get("RECORD"), Some(&"1".to_string()));
        assert_eq!(props.get("DESIGNATOR"), Some(&"R1".to_string()));
    }

    #[test]
    fn test_parse_component() {
        let mut props = HashMap::new();
        props.insert("RECORD".to_string(), "1".to_string());
        props.insert("DESIGNATOR".to_string(), "R1".to_string());
        props.insert("LIBREFERENCE".to_string(), "Resistor".to_string());
        props.insert("LOCATION.X".to_string(), "100000".to_string());
        props.insert("LOCATION.Y".to_string(), "200000".to_string());
        
        let symbol = AltiumSchematicImporter::parse_component(&props);
        assert!(symbol.is_some());
        
        let symbol = symbol.unwrap();
        assert_eq!(symbol.reference, "R1");
        assert!((symbol.position.x - 10.0).abs() < 0.1);
    }

    #[test]
    fn test_parse_wire() {
        let mut props = HashMap::new();
        props.insert("RECORD".to_string(), "27".to_string());
        props.insert("LOCATION.X".to_string(), "100000".to_string());
        props.insert("LOCATION.Y".to_string(), "100000".to_string());
        props.insert("CORNER.X".to_string(), "200000".to_string());
        props.insert("CORNER.Y".to_string(), "100000".to_string());
        
        let wire = AltiumSchematicImporter::parse_wire(&props);
        assert!(wire.is_some());
        
        let wire = wire.unwrap();
        assert!((wire.start.x - 10.0).abs() < 0.1);
        assert!((wire.end.x - 20.0).abs() < 0.1);
    }

    #[test]
    fn test_parse_net_label() {
        let mut props = HashMap::new();
        props.insert("RECORD".to_string(), "25".to_string());
        props.insert("TEXT".to_string(), "VCC".to_string());
        props.insert("LOCATION.X".to_string(), "50000".to_string());
        props.insert("LOCATION.Y".to_string(), "50000".to_string());
        
        let label = AltiumSchematicImporter::parse_net_label(&props);
        assert!(label.is_some());
        
        let label = label.unwrap();
        assert_eq!(label.name, "VCC");
    }

    #[test]
    fn test_extract_strings() {
        let data = b"some binary\x00data=value\x00more=stuff\x00end";
        let strings = AltiumSchematicImporter::extract_strings(data);
        
        assert!(strings.iter().any(|s| s.contains("data=value")));
    }

    #[test]
    fn test_import_invalid_data() {
        let result = AltiumSchematicImporter::import_from_bytes(&[0, 1, 2, 3]);
        assert!(result.is_err());
    }

    #[test]
    fn test_import_record_stream() {
        let data = b"RECORD=31|SHEETNAME=TestSheet|RECORD=1|DESIGNATOR=U1|LIBREFERENCE=IC";
        let result = AltiumSchematicImporter::import_from_bytes(data);
        
        assert!(result.is_ok());
        let sheet = result.unwrap();
        assert_eq!(sheet.name, "TestSheet");
        assert_eq!(sheet.symbols.len(), 1);
    }
}

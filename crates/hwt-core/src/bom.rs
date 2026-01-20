//! Bill of Materials (BOM) generation.
//!
//! Generates BOM reports from schematic and layout data in various formats.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::layout::{Layout, PlacedComponent};
use crate::schematic::{SchematicSheet, PlacedSymbol};

/// BOM generation result type.
pub type BomResult<T> = Result<T, BomError>;

/// BOM generation errors.
#[derive(Debug, Clone)]
pub enum BomError {
    /// No components found in design
    NoComponents,
    /// IO error during export
    IoError(String),
    /// Invalid configuration
    InvalidConfig(String),
}

impl std::fmt::Display for BomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BomError::NoComponents => write!(f, "No components found in design"),
            BomError::IoError(msg) => write!(f, "IO error: {}", msg),
            BomError::InvalidConfig(msg) => write!(f, "Invalid configuration: {}", msg),
        }
    }
}

impl std::error::Error for BomError {}

/// BOM output format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum BomFormat {
    #[default]
    Csv,
    Html,
    Json,
}

/// How to group BOM entries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum BomGroupBy {
    /// Group by component value
    #[default]
    Value,
    /// Group by footprint
    Footprint,
    /// Group by value and footprint
    ValueAndFootprint,
    /// No grouping (one line per component)
    None,
}

/// How to sort BOM entries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum BomSortBy {
    /// Sort by reference designator
    #[default]
    Reference,
    /// Sort by value
    Value,
    /// Sort by quantity (descending)
    Quantity,
    /// Sort by footprint
    Footprint,
}

/// BOM column definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BomColumn {
    Reference,
    Quantity,
    Value,
    Footprint,
    Description,
    Manufacturer,
    Mpn,
    Supplier,
    SupplierPn,
    UnitPrice,
    ExtendedPrice,
}

impl BomColumn {
    /// Get the column header name.
    pub fn header(&self) -> &'static str {
        match self {
            BomColumn::Reference => "Reference",
            BomColumn::Quantity => "Qty",
            BomColumn::Value => "Value",
            BomColumn::Footprint => "Footprint",
            BomColumn::Description => "Description",
            BomColumn::Manufacturer => "Manufacturer",
            BomColumn::Mpn => "MPN",
            BomColumn::Supplier => "Supplier",
            BomColumn::SupplierPn => "Supplier PN",
            BomColumn::UnitPrice => "Unit Price",
            BomColumn::ExtendedPrice => "Ext. Price",
        }
    }
}

/// BOM generation configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BomConfig {
    /// Output format
    pub format: BomFormat,
    
    /// How to group entries
    pub group_by: BomGroupBy,
    
    /// How to sort entries
    pub sort_by: BomSortBy,
    
    /// Columns to include
    pub columns: Vec<BomColumn>,
    
    /// Project name for header
    #[serde(default)]
    pub project_name: Option<String>,
    
    /// Include DNP (Do Not Populate) components
    #[serde(default)]
    pub include_dnp: bool,
    
    /// Include virtual components (like net ties)
    #[serde(default)]
    pub include_virtual: bool,
}

impl Default for BomConfig {
    fn default() -> Self {
        Self {
            format: BomFormat::Csv,
            group_by: BomGroupBy::ValueAndFootprint,
            sort_by: BomSortBy::Reference,
            columns: vec![
                BomColumn::Reference,
                BomColumn::Quantity,
                BomColumn::Value,
                BomColumn::Footprint,
                BomColumn::Description,
            ],
            project_name: None,
            include_dnp: false,
            include_virtual: false,
        }
    }
}

/// A single BOM entry (possibly grouped).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BomEntry {
    /// Reference designators (e.g., "R1, R2, R3" or just "R1")
    pub references: Vec<String>,
    
    /// Quantity
    pub quantity: usize,
    
    /// Component value
    pub value: String,
    
    /// Footprint name
    pub footprint: String,
    
    /// Description
    #[serde(default)]
    pub description: Option<String>,
    
    /// Manufacturer name
    #[serde(default)]
    pub manufacturer: Option<String>,
    
    /// Manufacturer part number
    #[serde(default)]
    pub mpn: Option<String>,
    
    /// Supplier name
    #[serde(default)]
    pub supplier: Option<String>,
    
    /// Supplier part number
    #[serde(default)]
    pub supplier_pn: Option<String>,
    
    /// Unit price
    #[serde(default)]
    pub unit_price: Option<f64>,
}

impl BomEntry {
    /// Create a new BOM entry from a single component.
    pub fn from_component(reference: String, value: String, footprint: String) -> Self {
        Self {
            references: vec![reference],
            quantity: 1,
            value,
            footprint,
            description: None,
            manufacturer: None,
            mpn: None,
            supplier: None,
            supplier_pn: None,
            unit_price: None,
        }
    }
    
    /// Get references as a comma-separated string.
    pub fn references_string(&self) -> String {
        self.references.join(", ")
    }
    
    /// Get extended price (unit price * quantity).
    pub fn extended_price(&self) -> Option<f64> {
        self.unit_price.map(|p| p * self.quantity as f64)
    }
    
    /// Merge another entry into this one (for grouping).
    pub fn merge(&mut self, other: &BomEntry) {
        self.references.extend(other.references.clone());
        self.quantity += other.quantity;
        
        // Sort references naturally
        self.references.sort_by(|a, b| natord::compare(a, b));
    }
}

/// Generated BOM report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BomReport {
    /// Project name
    pub project_name: String,
    
    /// BOM entries
    pub entries: Vec<BomEntry>,
    
    /// Total unique parts
    pub unique_parts: usize,
    
    /// Total component count
    pub total_components: usize,
    
    /// Total estimated cost (if pricing available)
    #[serde(default)]
    pub total_cost: Option<f64>,
}

impl BomReport {
    /// Generate a BOM report from layout components.
    pub fn from_layout(layout: &Layout, config: &BomConfig) -> BomResult<Self> {
        let components = &layout.components;
        
        if components.is_empty() {
            return Err(BomError::NoComponents);
        }
        
        let project_name = config.project_name.clone().unwrap_or_else(|| "Untitled".to_string());
        
        // Create initial entries
        let mut entries: Vec<BomEntry> = components
            .iter()
            .map(|c| BomEntry::from_component(
                c.reference.clone(),
                c.value.clone(),
                c.footprint.clone(),
            ))
            .collect();
        
        // Group entries
        entries = Self::group_entries(entries, config.group_by);
        
        // Sort entries
        Self::sort_entries(&mut entries, config.sort_by);
        
        let total_components = entries.iter().map(|e| e.quantity).sum();
        let unique_parts = entries.len();
        let total_cost = Self::calculate_total_cost(&entries);
        
        Ok(Self {
            project_name,
            entries,
            unique_parts,
            total_components,
            total_cost,
        })
    }
    
    /// Generate a BOM report from schematic symbols.
    pub fn from_schematic(sheets: &[SchematicSheet], config: &BomConfig) -> BomResult<Self> {
        let mut all_symbols: Vec<&PlacedSymbol> = Vec::new();
        
        for sheet in sheets {
            all_symbols.extend(sheet.symbols.iter());
        }
        
        if all_symbols.is_empty() {
            return Err(BomError::NoComponents);
        }
        
        let project_name = config.project_name.clone().unwrap_or_else(|| "Untitled".to_string());
        
        // Create initial entries from symbols
        let mut entries: Vec<BomEntry> = all_symbols
            .iter()
            .map(|s| {
                let footprint = s.properties.iter()
                    .find(|p| p.key.to_lowercase() == "footprint")
                    .map(|p| p.value.clone())
                    .unwrap_or_default();
                    
                BomEntry::from_component(
                    s.reference.clone(),
                    s.value.clone(),
                    footprint,
                )
            })
            .collect();
        
        // Group entries
        entries = Self::group_entries(entries, config.group_by);
        
        // Sort entries
        Self::sort_entries(&mut entries, config.sort_by);
        
        let total_components = entries.iter().map(|e| e.quantity).sum();
        let unique_parts = entries.len();
        let total_cost = Self::calculate_total_cost(&entries);
        
        Ok(Self {
            project_name,
            entries,
            unique_parts,
            total_components,
            total_cost,
        })
    }
    
    /// Group entries based on grouping strategy.
    fn group_entries(entries: Vec<BomEntry>, group_by: BomGroupBy) -> Vec<BomEntry> {
        if group_by == BomGroupBy::None {
            return entries;
        }
        
        let mut groups: HashMap<String, BomEntry> = HashMap::new();
        
        for entry in entries {
            let key = match group_by {
                BomGroupBy::Value => entry.value.clone(),
                BomGroupBy::Footprint => entry.footprint.clone(),
                BomGroupBy::ValueAndFootprint => format!("{}|{}", entry.value, entry.footprint),
                BomGroupBy::None => unreachable!(),
            };
            
            groups
                .entry(key)
                .and_modify(|e| e.merge(&entry))
                .or_insert(entry);
        }
        
        groups.into_values().collect()
    }
    
    /// Sort entries based on sort strategy.
    fn sort_entries(entries: &mut [BomEntry], sort_by: BomSortBy) {
        match sort_by {
            BomSortBy::Reference => {
                entries.sort_by(|a, b| {
                    let a_ref = a.references.first().map(|s| s.as_str()).unwrap_or("");
                    let b_ref = b.references.first().map(|s| s.as_str()).unwrap_or("");
                    natord::compare(a_ref, b_ref)
                });
            }
            BomSortBy::Value => {
                entries.sort_by(|a, b| natord::compare(&a.value, &b.value));
            }
            BomSortBy::Quantity => {
                entries.sort_by(|a, b| b.quantity.cmp(&a.quantity));
            }
            BomSortBy::Footprint => {
                entries.sort_by(|a, b| natord::compare(&a.footprint, &b.footprint));
            }
        }
    }
    
    /// Calculate total cost from entries with pricing.
    fn calculate_total_cost(entries: &[BomEntry]) -> Option<f64> {
        let mut total = 0.0;
        let mut has_any_price = false;
        
        for entry in entries {
            if let Some(price) = entry.extended_price() {
                total += price;
                has_any_price = true;
            }
        }
        
        if has_any_price {
            Some(total)
        } else {
            None
        }
    }
    
    /// Export to CSV format.
    pub fn to_csv(&self, config: &BomConfig) -> String {
        let mut output = String::new();
        
        // Header comment
        output.push_str(&format!("# BOM: {}\n", self.project_name));
        output.push_str(&format!("# Unique Parts: {}, Total Components: {}\n", 
            self.unique_parts, self.total_components));
        if let Some(cost) = self.total_cost {
            output.push_str(&format!("# Estimated Cost: ${:.2}\n", cost));
        }
        output.push('\n');
        
        // Column headers
        let headers: Vec<&str> = config.columns.iter().map(|c| c.header()).collect();
        output.push_str(&headers.join(","));
        output.push('\n');
        
        // Data rows
        for entry in &self.entries {
            let row: Vec<String> = config.columns.iter().map(|col| {
                match col {
                    BomColumn::Reference => format!("\"{}\"", entry.references_string()),
                    BomColumn::Quantity => entry.quantity.to_string(),
                    BomColumn::Value => format!("\"{}\"", entry.value),
                    BomColumn::Footprint => format!("\"{}\"", entry.footprint),
                    BomColumn::Description => format!("\"{}\"", entry.description.as_deref().unwrap_or("")),
                    BomColumn::Manufacturer => format!("\"{}\"", entry.manufacturer.as_deref().unwrap_or("")),
                    BomColumn::Mpn => format!("\"{}\"", entry.mpn.as_deref().unwrap_or("")),
                    BomColumn::Supplier => format!("\"{}\"", entry.supplier.as_deref().unwrap_or("")),
                    BomColumn::SupplierPn => format!("\"{}\"", entry.supplier_pn.as_deref().unwrap_or("")),
                    BomColumn::UnitPrice => entry.unit_price.map(|p| format!("{:.4}", p)).unwrap_or_default(),
                    BomColumn::ExtendedPrice => entry.extended_price().map(|p| format!("{:.2}", p)).unwrap_or_default(),
                }
            }).collect();
            output.push_str(&row.join(","));
            output.push('\n');
        }
        
        output
    }
    
    /// Export to HTML format.
    pub fn to_html(&self, config: &BomConfig) -> String {
        let mut html = String::new();
        
        html.push_str("<!DOCTYPE html>\n<html>\n<head>\n");
        html.push_str("<meta charset=\"UTF-8\">\n");
        html.push_str(&format!("<title>BOM: {}</title>\n", self.project_name));
        html.push_str("<style>\n");
        html.push_str("body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; margin: 20px; }\n");
        html.push_str("h1 { color: #333; }\n");
        html.push_str(".summary { background: #f5f5f5; padding: 10px; border-radius: 4px; margin-bottom: 20px; }\n");
        html.push_str("table { border-collapse: collapse; width: 100%; }\n");
        html.push_str("th, td { border: 1px solid #ddd; padding: 8px; text-align: left; }\n");
        html.push_str("th { background: #4a90d9; color: white; }\n");
        html.push_str("tr:nth-child(even) { background: #f9f9f9; }\n");
        html.push_str("tr:hover { background: #e8f4ff; }\n");
        html.push_str(".qty { text-align: center; }\n");
        html.push_str(".price { text-align: right; }\n");
        html.push_str("</style>\n");
        html.push_str("</head>\n<body>\n");
        
        html.push_str(&format!("<h1>Bill of Materials: {}</h1>\n", self.project_name));
        
        html.push_str("<div class=\"summary\">\n");
        html.push_str(&format!("<strong>Unique Parts:</strong> {} | ", self.unique_parts));
        html.push_str(&format!("<strong>Total Components:</strong> {}", self.total_components));
        if let Some(cost) = self.total_cost {
            html.push_str(&format!(" | <strong>Estimated Cost:</strong> ${:.2}", cost));
        }
        html.push_str("\n</div>\n");
        
        html.push_str("<table>\n<thead>\n<tr>\n");
        for col in &config.columns {
            html.push_str(&format!("<th>{}</th>\n", col.header()));
        }
        html.push_str("</tr>\n</thead>\n<tbody>\n");
        
        for entry in &self.entries {
            html.push_str("<tr>\n");
            for col in &config.columns {
                let (class, value) = match col {
                    BomColumn::Reference => ("", entry.references_string()),
                    BomColumn::Quantity => ("qty", entry.quantity.to_string()),
                    BomColumn::Value => ("", entry.value.clone()),
                    BomColumn::Footprint => ("", entry.footprint.clone()),
                    BomColumn::Description => ("", entry.description.clone().unwrap_or_default()),
                    BomColumn::Manufacturer => ("", entry.manufacturer.clone().unwrap_or_default()),
                    BomColumn::Mpn => ("", entry.mpn.clone().unwrap_or_default()),
                    BomColumn::Supplier => ("", entry.supplier.clone().unwrap_or_default()),
                    BomColumn::SupplierPn => ("", entry.supplier_pn.clone().unwrap_or_default()),
                    BomColumn::UnitPrice => ("price", entry.unit_price.map(|p| format!("${:.4}", p)).unwrap_or_default()),
                    BomColumn::ExtendedPrice => ("price", entry.extended_price().map(|p| format!("${:.2}", p)).unwrap_or_default()),
                };
                if class.is_empty() {
                    html.push_str(&format!("<td>{}</td>\n", value));
                } else {
                    html.push_str(&format!("<td class=\"{}\">{}</td>\n", class, value));
                }
            }
            html.push_str("</tr>\n");
        }
        
        html.push_str("</tbody>\n</table>\n");
        html.push_str("<footer><p><em>Generated by Hardware Tool</em></p></footer>\n");
        html.push_str("</body>\n</html>\n");
        
        html
    }
    
    /// Export to JSON format.
    pub fn to_json(&self) -> BomResult<String> {
        serde_json::to_string_pretty(self)
            .map_err(|e| BomError::IoError(e.to_string()))
    }
    
    /// Export to the configured format.
    pub fn export(&self, config: &BomConfig) -> BomResult<String> {
        match config.format {
            BomFormat::Csv => Ok(self.to_csv(config)),
            BomFormat::Html => Ok(self.to_html(config)),
            BomFormat::Json => self.to_json(),
        }
    }
    
    /// Write to a file.
    pub fn write_to_file(&self, path: &std::path::Path, config: &BomConfig) -> BomResult<()> {
        let content = self.export(config)?;
        std::fs::write(path, content)
            .map_err(|e| BomError::IoError(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn create_test_layout() -> Layout {
        let mut layout = Layout::new();
        
        layout.components.push(
            PlacedComponent::new("R1", "10K", "Resistor_SMD:R_0603")
        );
        layout.components.push(
            PlacedComponent::new("R2", "10K", "Resistor_SMD:R_0603")
        );
        layout.components.push(
            PlacedComponent::new("R3", "4.7K", "Resistor_SMD:R_0603")
        );
        layout.components.push(
            PlacedComponent::new("C1", "100nF", "Capacitor_SMD:C_0402")
        );
        layout.components.push(
            PlacedComponent::new("C2", "100nF", "Capacitor_SMD:C_0402")
        );
        layout.components.push(
            PlacedComponent::new("U1", "STM32F407", "Package_QFP:LQFP-100")
        );
        
        layout
    }
    
    #[test]
    fn test_bom_generation() {
        let layout = create_test_layout();
        let config = BomConfig::default();
        
        let bom = BomReport::from_layout(&layout, &config).unwrap();
        
        assert_eq!(bom.total_components, 6);
        assert_eq!(bom.unique_parts, 4); // 10K, 4.7K, 100nF, STM32F407
    }
    
    #[test]
    fn test_bom_grouping_by_value() {
        let layout = create_test_layout();
        let config = BomConfig {
            group_by: BomGroupBy::Value,
            ..Default::default()
        };
        
        let bom = BomReport::from_layout(&layout, &config).unwrap();
        
        // Should group: 10K (2), 4.7K (1), 100nF (2), STM32F407 (1)
        assert_eq!(bom.unique_parts, 4);
        
        let r10k = bom.entries.iter().find(|e| e.value == "10K").unwrap();
        assert_eq!(r10k.quantity, 2);
        assert!(r10k.references.contains(&"R1".to_string()));
        assert!(r10k.references.contains(&"R2".to_string()));
    }
    
    #[test]
    fn test_bom_no_grouping() {
        let layout = create_test_layout();
        let config = BomConfig {
            group_by: BomGroupBy::None,
            ..Default::default()
        };
        
        let bom = BomReport::from_layout(&layout, &config).unwrap();
        
        // No grouping = one entry per component
        assert_eq!(bom.unique_parts, 6);
        assert_eq!(bom.entries.len(), 6);
    }
    
    #[test]
    fn test_bom_csv_export() {
        let layout = create_test_layout();
        let config = BomConfig::default();
        
        let bom = BomReport::from_layout(&layout, &config).unwrap();
        let csv = bom.to_csv(&config);
        
        assert!(csv.contains("Reference,Qty,Value,Footprint,Description"));
        assert!(csv.contains("10K"));
        assert!(csv.contains("STM32F407"));
    }
    
    #[test]
    fn test_bom_html_export() {
        let layout = create_test_layout();
        let config = BomConfig {
            format: BomFormat::Html,
            ..Default::default()
        };
        
        let bom = BomReport::from_layout(&layout, &config).unwrap();
        let html = bom.to_html(&config);
        
        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("<table>"));
        assert!(html.contains("10K"));
        assert!(html.contains("STM32F407"));
    }
    
    #[test]
    fn test_bom_json_export() {
        let layout = create_test_layout();
        let config = BomConfig::default();
        
        let bom = BomReport::from_layout(&layout, &config).unwrap();
        let json = bom.to_json().unwrap();
        
        assert!(json.contains("\"project_name\""));
        assert!(json.contains("\"entries\""));
        assert!(json.contains("10K"));
    }
    
    #[test]
    fn test_bom_sorting() {
        let layout = create_test_layout();
        
        // Sort by quantity (descending)
        let config = BomConfig {
            sort_by: BomSortBy::Quantity,
            ..Default::default()
        };
        
        let bom = BomReport::from_layout(&layout, &config).unwrap();
        
        // First entries should have highest quantity
        assert!(bom.entries[0].quantity >= bom.entries[1].quantity);
    }
    
    #[test]
    fn test_empty_layout_error() {
        let layout = Layout::new();
        let config = BomConfig::default();
        
        let result = BomReport::from_layout(&layout, &config);
        assert!(matches!(result, Err(BomError::NoComponents)));
    }
}

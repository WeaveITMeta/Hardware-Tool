//! Pick-and-Place (PnP) file generation.
//!
//! Generates component placement files for SMT assembly machines.

use serde::{Deserialize, Serialize};

use crate::layout::{Layout, ComponentLayer};

/// PnP generation result type.
pub type PnpResult<T> = Result<T, PnpError>;

/// PnP generation errors.
#[derive(Debug, Clone)]
pub enum PnpError {
    /// No components found
    NoComponents,
    /// IO error during export
    IoError(String),
}

impl std::fmt::Display for PnpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PnpError::NoComponents => write!(f, "No components found in layout"),
            PnpError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for PnpError {}

/// PnP output format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum PnpFormat {
    #[default]
    Csv,
    /// ASCII format (space-separated)
    Ascii,
}

/// Which board side to include.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum PnpSide {
    Top,
    Bottom,
    #[default]
    Both,
}

/// Units for position output.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum PnpUnits {
    #[default]
    Millimeters,
    Inches,
    Mils,
}

impl PnpUnits {
    /// Get the unit suffix string.
    pub fn suffix(&self) -> &'static str {
        match self {
            PnpUnits::Millimeters => "mm",
            PnpUnits::Inches => "in",
            PnpUnits::Mils => "mil",
        }
    }
    
    /// Convert from mm to this unit.
    pub fn from_mm(&self, value: f64) -> f64 {
        match self {
            PnpUnits::Millimeters => value,
            PnpUnits::Inches => value / 25.4,
            PnpUnits::Mils => value / 0.0254,
        }
    }
}

/// PnP generation configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PnpConfig {
    /// Output format
    pub format: PnpFormat,
    
    /// Which side(s) to include
    pub side: PnpSide,
    
    /// Position units
    pub units: PnpUnits,
    
    /// Include SMD components only
    #[serde(default = "default_true")]
    pub smd_only: bool,
    
    /// Include header row
    #[serde(default = "default_true")]
    pub include_header: bool,
    
    /// Decimal precision for coordinates
    #[serde(default = "default_precision")]
    pub precision: usize,
    
    /// Negate Y axis (some machines expect this)
    #[serde(default)]
    pub negate_y: bool,
    
    /// Negate bottom rotation (some machines expect this)
    #[serde(default)]
    pub negate_bottom_rotation: bool,
}

fn default_true() -> bool { true }
fn default_precision() -> usize { 4 }

impl Default for PnpConfig {
    fn default() -> Self {
        Self {
            format: PnpFormat::Csv,
            side: PnpSide::Both,
            units: PnpUnits::Millimeters,
            smd_only: true,
            include_header: true,
            precision: 4,
            negate_y: false,
            negate_bottom_rotation: false,
        }
    }
}

/// A single pick-and-place entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PnpEntry {
    /// Reference designator
    pub reference: String,
    
    /// Component value
    pub value: String,
    
    /// Footprint/package name
    pub footprint: String,
    
    /// X position (in configured units)
    pub x: f64,
    
    /// Y position (in configured units)
    pub y: f64,
    
    /// Rotation in degrees
    pub rotation: f64,
    
    /// Board side
    pub side: ComponentLayer,
}

/// Generated pick-and-place report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PnpReport {
    /// Project name
    pub project_name: String,
    
    /// Units used
    pub units: PnpUnits,
    
    /// All entries
    pub entries: Vec<PnpEntry>,
    
    /// Top side entries
    pub top_count: usize,
    
    /// Bottom side entries
    pub bottom_count: usize,
}

impl PnpReport {
    /// Generate a PnP report from layout.
    pub fn from_layout(layout: &Layout, config: &PnpConfig, project_name: Option<&str>) -> PnpResult<Self> {
        let components = &layout.components;
        
        if components.is_empty() {
            return Err(PnpError::NoComponents);
        }
        
        let project_name = project_name.unwrap_or("Untitled").to_string();
        
        let mut entries: Vec<PnpEntry> = components
            .iter()
            .filter(|c| {
                // Filter by side
                match config.side {
                    PnpSide::Top => c.layer == ComponentLayer::Top,
                    PnpSide::Bottom => c.layer == ComponentLayer::Bottom,
                    PnpSide::Both => true,
                }
            })
            .map(|c| {
                let x = config.units.from_mm(c.position.x);
                let mut y = config.units.from_mm(c.position.y);
                let mut rotation = c.rotation;
                
                if config.negate_y {
                    y = -y;
                }
                
                if config.negate_bottom_rotation && c.layer == ComponentLayer::Bottom {
                    rotation = -rotation;
                }
                
                // Normalize rotation to 0-360
                rotation = rotation.rem_euclid(360.0);
                
                PnpEntry {
                    reference: c.reference.clone(),
                    value: c.value.clone(),
                    footprint: c.footprint.clone(),
                    x,
                    y,
                    rotation,
                    side: c.layer,
                }
            })
            .collect();
        
        // Sort by reference designator naturally
        entries.sort_by(|a, b| natord::compare(&a.reference, &b.reference));
        
        let top_count = entries.iter().filter(|e| e.side == ComponentLayer::Top).count();
        let bottom_count = entries.iter().filter(|e| e.side == ComponentLayer::Bottom).count();
        
        Ok(Self {
            project_name,
            units: config.units,
            entries,
            top_count,
            bottom_count,
        })
    }
    
    /// Get entries for top side only.
    pub fn top_entries(&self) -> Vec<&PnpEntry> {
        self.entries.iter().filter(|e| e.side == ComponentLayer::Top).collect()
    }
    
    /// Get entries for bottom side only.
    pub fn bottom_entries(&self) -> Vec<&PnpEntry> {
        self.entries.iter().filter(|e| e.side == ComponentLayer::Bottom).collect()
    }
    
    /// Export to CSV format.
    pub fn to_csv(&self, config: &PnpConfig) -> String {
        let mut output = String::new();
        let prec = config.precision;
        
        // Header comment
        output.push_str(&format!("# Pick and Place: {}\n", self.project_name));
        output.push_str(&format!("# Units: {}\n", self.units.suffix()));
        output.push_str(&format!("# Top: {}, Bottom: {}\n", self.top_count, self.bottom_count));
        output.push('\n');
        
        if config.include_header {
            output.push_str("Ref,Val,Package,PosX,PosY,Rot,Side\n");
        }
        
        for entry in &self.entries {
            let side_str = match entry.side {
                ComponentLayer::Top => "top",
                ComponentLayer::Bottom => "bottom",
            };
            
            output.push_str(&format!(
                "\"{}\",\"{}\",\"{}\",{:.prec$},{:.prec$},{:.2},{}\n",
                entry.reference,
                entry.value,
                entry.footprint,
                entry.x,
                entry.y,
                entry.rotation,
                side_str,
                prec = prec
            ));
        }
        
        output
    }
    
    /// Export to ASCII format (space-separated, compatible with some machines).
    pub fn to_ascii(&self, config: &PnpConfig) -> String {
        let mut output = String::new();
        let prec = config.precision;
        
        output.push_str(&format!("# Pick and Place: {}\n", self.project_name));
        output.push_str(&format!("# Units: {}\n", self.units.suffix()));
        
        if config.include_header {
            output.push_str("# Ref       Val          Package              PosX       PosY     Rot  Side\n");
        }
        
        for entry in &self.entries {
            let side_str = match entry.side {
                ComponentLayer::Top => "T",
                ComponentLayer::Bottom => "B",
            };
            
            output.push_str(&format!(
                "{:<10} {:<12} {:<20} {:>10.prec$} {:>10.prec$} {:>6.1} {}\n",
                entry.reference,
                entry.value,
                entry.footprint,
                entry.x,
                entry.y,
                entry.rotation,
                side_str,
                prec = prec
            ));
        }
        
        output
    }
    
    /// Export in configured format.
    pub fn export(&self, config: &PnpConfig) -> String {
        match config.format {
            PnpFormat::Csv => self.to_csv(config),
            PnpFormat::Ascii => self.to_ascii(config),
        }
    }
    
    /// Export top side only.
    pub fn export_top(&self, config: &PnpConfig) -> String {
        let top_only = Self {
            project_name: format!("{} (Top)", self.project_name),
            units: self.units,
            entries: self.top_entries().into_iter().cloned().collect(),
            top_count: self.top_count,
            bottom_count: 0,
        };
        top_only.export(config)
    }
    
    /// Export bottom side only.
    pub fn export_bottom(&self, config: &PnpConfig) -> String {
        let bottom_only = Self {
            project_name: format!("{} (Bottom)", self.project_name),
            units: self.units,
            entries: self.bottom_entries().into_iter().cloned().collect(),
            top_count: 0,
            bottom_count: self.bottom_count,
        };
        bottom_only.export(config)
    }
    
    /// Write to file.
    pub fn write_to_file(&self, path: &std::path::Path, config: &PnpConfig) -> PnpResult<()> {
        let content = self.export(config);
        std::fs::write(path, content)
            .map_err(|e| PnpError::IoError(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::PlacedComponent;
    
    fn create_test_layout() -> Layout {
        let mut layout = Layout::new();
        
        layout.components.push(
            PlacedComponent::new("R1", "10K", "R_0603").at(10.0, 20.0).rotated(0.0)
        );
        layout.components.push(
            PlacedComponent::new("R2", "10K", "R_0603").at(15.0, 20.0).rotated(90.0)
        );
        layout.components.push(
            PlacedComponent::new("C1", "100nF", "C_0402").at(10.0, 30.0).rotated(180.0)
        );
        layout.components.push(
            PlacedComponent::new("U1", "STM32F407", "LQFP-100").at(50.0, 50.0).rotated(0.0)
        );
        layout.components.push(
            PlacedComponent::new("U2", "LM1117", "SOT-223").at(20.0, 60.0).rotated(270.0).on_bottom()
        );
        
        layout
    }
    
    #[test]
    fn test_pnp_generation() {
        let layout = create_test_layout();
        let config = PnpConfig::default();
        
        let pnp = PnpReport::from_layout(&layout, &config, Some("Test Board")).unwrap();
        
        assert_eq!(pnp.entries.len(), 5);
        assert_eq!(pnp.top_count, 4);
        assert_eq!(pnp.bottom_count, 1);
    }
    
    #[test]
    fn test_pnp_top_only() {
        let layout = create_test_layout();
        let config = PnpConfig {
            side: PnpSide::Top,
            ..Default::default()
        };
        
        let pnp = PnpReport::from_layout(&layout, &config, None).unwrap();
        
        assert_eq!(pnp.entries.len(), 4);
        assert!(pnp.entries.iter().all(|e| e.side == ComponentLayer::Top));
    }
    
    #[test]
    fn test_pnp_bottom_only() {
        let layout = create_test_layout();
        let config = PnpConfig {
            side: PnpSide::Bottom,
            ..Default::default()
        };
        
        let pnp = PnpReport::from_layout(&layout, &config, None).unwrap();
        
        assert_eq!(pnp.entries.len(), 1);
        assert_eq!(pnp.entries[0].reference, "U2");
    }
    
    #[test]
    fn test_pnp_csv_export() {
        let layout = create_test_layout();
        let config = PnpConfig::default();
        
        let pnp = PnpReport::from_layout(&layout, &config, Some("Test")).unwrap();
        let csv = pnp.to_csv(&config);
        
        assert!(csv.contains("Ref,Val,Package,PosX,PosY,Rot,Side"));
        assert!(csv.contains("R1"));
        assert!(csv.contains("STM32F407"));
        assert!(csv.contains("top"));
        assert!(csv.contains("bottom"));
    }
    
    #[test]
    fn test_pnp_ascii_export() {
        let layout = create_test_layout();
        let config = PnpConfig {
            format: PnpFormat::Ascii,
            ..Default::default()
        };
        
        let pnp = PnpReport::from_layout(&layout, &config, None).unwrap();
        let ascii = pnp.to_ascii(&config);
        
        assert!(ascii.contains("R1"));
        assert!(ascii.contains("U1"));
    }
    
    #[test]
    fn test_pnp_units_conversion() {
        let layout = create_test_layout();
        let config = PnpConfig {
            units: PnpUnits::Inches,
            ..Default::default()
        };
        
        let pnp = PnpReport::from_layout(&layout, &config, None).unwrap();
        
        // R1 is at 10mm, which is ~0.394 inches
        let r1 = pnp.entries.iter().find(|e| e.reference == "R1").unwrap();
        assert!((r1.x - 0.3937).abs() < 0.001);
    }
    
    #[test]
    fn test_pnp_separate_files() {
        let layout = create_test_layout();
        let config = PnpConfig::default();
        
        let pnp = PnpReport::from_layout(&layout, &config, Some("Board")).unwrap();
        
        let top_csv = pnp.export_top(&config);
        let bottom_csv = pnp.export_bottom(&config);
        
        assert!(top_csv.contains("(Top)"));
        assert!(bottom_csv.contains("(Bottom)"));
        assert!(!top_csv.contains("\"U2\"")); // U2 is on bottom
        assert!(bottom_csv.contains("\"U2\""));
    }
    
    #[test]
    fn test_empty_layout_error() {
        let layout = Layout::new();
        let config = PnpConfig::default();
        
        let result = PnpReport::from_layout(&layout, &config, None);
        assert!(matches!(result, Err(PnpError::NoComponents)));
    }
}

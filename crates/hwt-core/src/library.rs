//! Library architecture for component management.
//!
//! Unified library system that works across all hardware domains.
//! Supports symbols, footprints, cells, gates, structures, and dies.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use uuid::Uuid;

use crate::domain::HardwareDomain;

/// A component library.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Library {
    /// Library metadata
    pub metadata: LibraryMetadata,
    
    /// Components in this library
    #[serde(default)]
    pub components: Vec<LibraryComponent>,
    
    /// Dependencies on other libraries
    #[serde(default)]
    pub dependencies: HashMap<String, String>,
    
    /// Quality settings
    #[serde(default)]
    pub quality: QualitySettings,
}

impl Library {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            metadata: LibraryMetadata {
                name: name.into(),
                version: "1.0.0".to_string(),
                author: None,
                license: None,
                description: None,
                domains: vec![HardwareDomain::Pcb],
            },
            components: Vec::new(),
            dependencies: HashMap::new(),
            quality: QualitySettings::default(),
        }
    }
    
    /// Add a component to the library.
    pub fn add_component(&mut self, component: LibraryComponent) {
        self.components.push(component);
    }
    
    /// Find component by name.
    pub fn find_by_name(&self, name: &str) -> Option<&LibraryComponent> {
        self.components.iter().find(|c| c.name == name)
    }
    
    /// Search components.
    pub fn search(&self, query: &str) -> Vec<&LibraryComponent> {
        let query_lower = query.to_lowercase();
        self.components
            .iter()
            .filter(|c| {
                c.name.to_lowercase().contains(&query_lower)
                    || c.description.as_ref().map_or(false, |d| d.to_lowercase().contains(&query_lower))
                    || c.keywords.iter().any(|k| k.to_lowercase().contains(&query_lower))
            })
            .collect()
    }
    
    /// Validate library against quality settings.
    pub fn validate(&self) -> LibraryValidationReport {
        let mut issues = Vec::new();
        
        for component in &self.components {
            if self.quality.require_description && component.description.is_none() {
                issues.push(ValidationIssue {
                    component: component.name.clone(),
                    severity: ValidationSeverity::Warning,
                    message: "Missing description".to_string(),
                });
            }
            
            if self.quality.require_keywords && component.keywords.is_empty() {
                issues.push(ValidationIssue {
                    component: component.name.clone(),
                    severity: ValidationSeverity::Warning,
                    message: "Missing keywords".to_string(),
                });
            }
            
            if self.quality.require_datasheet && component.datasheet.is_none() {
                issues.push(ValidationIssue {
                    component: component.name.clone(),
                    severity: ValidationSeverity::Info,
                    message: "Missing datasheet link".to_string(),
                });
            }
        }
        
        LibraryValidationReport {
            library: self.metadata.name.clone(),
            issues,
        }
    }
    
    /// Load from TOML string.
    pub fn from_toml(toml_str: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(toml_str)
    }
    
    /// Serialize to TOML string.
    pub fn to_toml(&self) -> Result<String, toml::ser::Error> {
        toml::to_string_pretty(self)
    }
}

/// Library metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryMetadata {
    /// Library name
    pub name: String,
    
    /// Version (semver)
    pub version: String,
    
    /// Author
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    
    /// License
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
    
    /// Description
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    
    /// Supported domains
    #[serde(default)]
    pub domains: Vec<HardwareDomain>,
}

/// A component in a library.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryComponent {
    /// Unique identifier
    pub id: Uuid,
    
    /// Component name
    pub name: String,
    
    /// Component type
    pub component_type: ComponentType,
    
    /// Description
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    
    /// Search keywords
    #[serde(default)]
    pub keywords: Vec<String>,
    
    /// Datasheet URL
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub datasheet: Option<String>,
    
    /// Documentation URL
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub documentation: Option<String>,
    
    /// 3D model reference
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model_3d: Option<PathBuf>,
    
    /// Symbol data (for schematic symbols)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<SymbolData>,
    
    /// Footprint data (for physical footprints)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub footprint: Option<FootprintData>,
    
    /// Custom properties
    #[serde(default)]
    pub properties: HashMap<String, PropertyValue>,
    
    /// Creation timestamp
    pub created: chrono::DateTime<chrono::Utc>,
    
    /// Last modified timestamp
    pub modified: chrono::DateTime<chrono::Utc>,
}

impl LibraryComponent {
    pub fn new(name: impl Into<String>, component_type: ComponentType) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            component_type,
            description: None,
            keywords: Vec::new(),
            datasheet: None,
            documentation: None,
            model_3d: None,
            symbol: None,
            footprint: None,
            properties: HashMap::new(),
            created: now,
            modified: now,
        }
    }
    
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
    
    pub fn with_keywords(mut self, keywords: Vec<String>) -> Self {
        self.keywords = keywords;
        self
    }
    
    pub fn with_datasheet(mut self, url: impl Into<String>) -> Self {
        self.datasheet = Some(url.into());
        self
    }
}

/// Component type in library.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ComponentType {
    /// Schematic symbol (PCB, IC, RF)
    Symbol,
    /// Physical footprint (PCB)
    Footprint,
    /// Standard cell (IC)
    Cell,
    /// Quantum gate (Quantum)
    Gate,
    /// MEMS structure (MEMS)
    Structure,
    /// Die IP (Packaging)
    Die,
    /// Combined symbol + footprint
    Component,
}

/// Symbol data for schematic symbols.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolData {
    /// Pin definitions
    pub pins: Vec<SymbolPinDef>,
    
    /// Graphics primitives
    pub graphics: Vec<GraphicPrimitive>,
    
    /// Reference designator prefix (e.g., "R", "C", "U")
    pub reference_prefix: String,
    
    /// Default value
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    
    /// Number of units (for multi-unit symbols)
    #[serde(default = "default_one")]
    pub units: u32,
}

fn default_one() -> u32 {
    1
}

/// Pin definition in a symbol.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolPinDef {
    /// Pin number
    pub number: String,
    
    /// Pin name
    pub name: String,
    
    /// Position relative to symbol origin
    pub x: f64,
    pub y: f64,
    
    /// Pin length
    pub length: f64,
    
    /// Orientation (0, 90, 180, 270)
    pub orientation: f64,
    
    /// Electrical type
    pub electrical_type: PinElectricalType,
    
    /// Pin shape
    #[serde(default)]
    pub shape: PinShape,
}

/// Electrical type of a pin.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum PinElectricalType {
    Input,
    Output,
    Bidirectional,
    TriState,
    #[default]
    Passive,
    Power,
    Ground,
    OpenCollector,
    OpenEmitter,
    NotConnected,
    Unspecified,
}

/// Pin shape.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum PinShape {
    #[default]
    Line,
    Inverted,
    Clock,
    InvertedClock,
    InputLow,
    ClockLow,
    OutputLow,
    EdgeClockHigh,
    NonLogic,
}

/// Graphic primitive for symbol drawing.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum GraphicPrimitive {
    Line { x1: f64, y1: f64, x2: f64, y2: f64, width: f64 },
    Rectangle { x: f64, y: f64, width: f64, height: f64, fill: bool },
    Circle { x: f64, y: f64, radius: f64, fill: bool },
    Arc { x: f64, y: f64, radius: f64, start_angle: f64, end_angle: f64 },
    Polyline { points: Vec<(f64, f64)>, width: f64 },
    Text { x: f64, y: f64, text: String, size: f64 },
}

/// Footprint data for physical footprints.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FootprintData {
    /// Pad definitions
    pub pads: Vec<PadDef>,
    
    /// Silkscreen graphics
    #[serde(default)]
    pub silkscreen: Vec<GraphicPrimitive>,
    
    /// Courtyard outline
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub courtyard: Option<CourtyardDef>,
    
    /// 3D model offset
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model_offset: Option<(f64, f64, f64)>,
    
    /// 3D model rotation
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model_rotation: Option<(f64, f64, f64)>,
}

/// Pad definition in a footprint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PadDef {
    /// Pad number (matches symbol pin)
    pub number: String,
    
    /// Position
    pub x: f64,
    pub y: f64,
    
    /// Pad shape
    pub shape: PadShape,
    
    /// Pad size
    pub width: f64,
    pub height: f64,
    
    /// Drill hole (for through-hole)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub drill: Option<f64>,
    
    /// Layers
    pub layers: Vec<String>,
}

/// Pad shape.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PadShape {
    Circle,
    Rectangle,
    Oval,
    RoundRect,
    Trapezoid,
    Custom,
}

/// Courtyard definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourtyardDef {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

/// Property value (typed).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PropertyValue {
    String(String),
    Number(f64),
    Boolean(bool),
}

/// Quality settings for library validation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualitySettings {
    pub require_description: bool,
    pub require_keywords: bool,
    pub require_datasheet: bool,
    pub require_3d_model: bool,
}

impl Default for QualitySettings {
    fn default() -> Self {
        Self {
            require_description: true,
            require_keywords: false,
            require_datasheet: false,
            require_3d_model: false,
        }
    }
}

/// Library validation report.
#[derive(Debug, Clone)]
pub struct LibraryValidationReport {
    pub library: String,
    pub issues: Vec<ValidationIssue>,
}

impl LibraryValidationReport {
    pub fn is_valid(&self) -> bool {
        !self.issues.iter().any(|i| i.severity == ValidationSeverity::Error)
    }
    
    pub fn error_count(&self) -> usize {
        self.issues.iter().filter(|i| i.severity == ValidationSeverity::Error).count()
    }
    
    pub fn warning_count(&self) -> usize {
        self.issues.iter().filter(|i| i.severity == ValidationSeverity::Warning).count()
    }
}

/// A validation issue.
#[derive(Debug, Clone)]
pub struct ValidationIssue {
    pub component: String,
    pub severity: ValidationSeverity,
    pub message: String,
}

/// Validation severity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationSeverity {
    Error,
    Warning,
    Info,
}

/// Library search query.
#[derive(Debug, Clone, Default)]
pub struct LibrarySearchQuery {
    /// Text search
    pub text: Option<String>,
    
    /// Filter by domain
    pub domain: Option<HardwareDomain>,
    
    /// Filter by component type
    pub component_type: Option<ComponentType>,
    
    /// Has 3D model
    pub has_3d_model: Option<bool>,
    
    /// Has datasheet
    pub has_datasheet: Option<bool>,
    
    /// Maximum results
    pub limit: Option<usize>,
}

/// Library browser for searching and filtering components across multiple libraries.
#[derive(Debug, Default)]
pub struct LibraryBrowser {
    /// Loaded libraries
    libraries: Vec<Library>,
    /// Category index for fast lookup
    category_index: HashMap<String, Vec<(usize, usize)>>,  // category -> [(lib_idx, comp_idx)]
    /// Keyword index for fast search
    keyword_index: HashMap<String, Vec<(usize, usize)>>,
    /// Recent items
    recent: Vec<(usize, usize)>,
    /// Favorites
    favorites: Vec<(usize, usize)>,
}

impl LibraryBrowser {
    /// Create a new empty library browser.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a library to the browser.
    pub fn add_library(&mut self, library: Library) {
        let lib_idx = self.libraries.len();
        
        // Index components
        for (comp_idx, component) in library.components.iter().enumerate() {
            // Index by category (component type)
            let category = format!("{:?}", component.component_type).to_lowercase();
            self.category_index
                .entry(category)
                .or_default()
                .push((lib_idx, comp_idx));
            
            // Index by keywords
            for keyword in &component.keywords {
                self.keyword_index
                    .entry(keyword.to_lowercase())
                    .or_default()
                    .push((lib_idx, comp_idx));
            }
        }
        
        self.libraries.push(library);
    }

    /// Get all loaded libraries.
    pub fn libraries(&self) -> &[Library] {
        &self.libraries
    }

    /// Get library by index.
    pub fn get_library(&self, index: usize) -> Option<&Library> {
        self.libraries.get(index)
    }

    /// Get library by name.
    pub fn find_library(&self, name: &str) -> Option<&Library> {
        self.libraries.iter().find(|lib| lib.metadata.name == name)
    }

    /// Get total component count across all libraries.
    pub fn total_components(&self) -> usize {
        self.libraries.iter().map(|lib| lib.components.len()).sum()
    }

    /// Get all categories.
    pub fn categories(&self) -> Vec<String> {
        self.category_index.keys().cloned().collect()
    }

    /// Get components by category.
    pub fn by_category(&self, category: &str) -> Vec<&LibraryComponent> {
        self.category_index
            .get(&category.to_lowercase())
            .map(|indices| {
                indices.iter()
                    .filter_map(|(lib_idx, comp_idx)| {
                        self.libraries.get(*lib_idx)
                            .and_then(|lib| lib.components.get(*comp_idx))
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Search across all libraries.
    pub fn search(&self, query: &str) -> Vec<BrowserResult> {
        let query_lower = query.to_lowercase();
        let mut results = Vec::new();
        let mut seen = std::collections::HashSet::new();

        // Search by keyword index first (fast path)
        for (keyword, indices) in &self.keyword_index {
            if keyword.contains(&query_lower) {
                for (lib_idx, comp_idx) in indices {
                    if seen.insert((*lib_idx, *comp_idx)) {
                        if let Some(lib) = self.libraries.get(*lib_idx) {
                            if let Some(comp) = lib.components.get(*comp_idx) {
                                results.push(BrowserResult {
                                    library_index: *lib_idx,
                                    library_name: lib.metadata.name.clone(),
                                    component_index: *comp_idx,
                                    component: comp.clone(),
                                    match_score: 100,
                                });
                            }
                        }
                    }
                }
            }
        }

        // Also search by name and description
        for (lib_idx, lib) in self.libraries.iter().enumerate() {
            for (comp_idx, comp) in lib.components.iter().enumerate() {
                if seen.contains(&(lib_idx, comp_idx)) {
                    continue;
                }

                let mut score = 0;
                if comp.name.to_lowercase().contains(&query_lower) {
                    score += 80;
                }
                if comp.description.as_ref().map_or(false, |d| d.to_lowercase().contains(&query_lower)) {
                    score += 40;
                }

                if score > 0 {
                    results.push(BrowserResult {
                        library_index: lib_idx,
                        library_name: lib.metadata.name.clone(),
                        component_index: comp_idx,
                        component: comp.clone(),
                        match_score: score,
                    });
                }
            }
        }

        // Sort by score descending
        results.sort_by(|a, b| b.match_score.cmp(&a.match_score));
        results
    }

    /// Advanced search with filters.
    pub fn search_filtered(&self, filter: &LibrarySearchQuery) -> Vec<BrowserResult> {
        let mut results = if let Some(ref query) = filter.text {
            self.search(query)
        } else {
            // Return all components
            self.libraries.iter().enumerate()
                .flat_map(|(lib_idx, lib)| {
                    lib.components.iter().enumerate().map(move |(comp_idx, comp)| {
                        BrowserResult {
                            library_index: lib_idx,
                            library_name: lib.metadata.name.clone(),
                            component_index: comp_idx,
                            component: comp.clone(),
                            match_score: 50,
                        }
                    })
                })
                .collect()
        };

        // Apply filters
        if let Some(ref comp_type) = filter.component_type {
            results.retain(|r| &r.component.component_type == comp_type);
        }

        if let Some(has_datasheet) = filter.has_datasheet {
            results.retain(|r| r.component.datasheet.is_some() == has_datasheet);
        }

        // Apply limit
        if let Some(limit) = filter.limit {
            results.truncate(limit);
        }

        results
    }

    /// Add to recent items.
    pub fn add_recent(&mut self, lib_idx: usize, comp_idx: usize) {
        // Remove if already exists
        self.recent.retain(|&(l, c)| l != lib_idx || c != comp_idx);
        // Add to front
        self.recent.insert(0, (lib_idx, comp_idx));
        // Keep only last 20
        self.recent.truncate(20);
    }

    /// Get recent items.
    pub fn recent(&self) -> Vec<&LibraryComponent> {
        self.recent.iter()
            .filter_map(|(lib_idx, comp_idx)| {
                self.libraries.get(*lib_idx)
                    .and_then(|lib| lib.components.get(*comp_idx))
            })
            .collect()
    }

    /// Toggle favorite.
    pub fn toggle_favorite(&mut self, lib_idx: usize, comp_idx: usize) -> bool {
        if let Some(pos) = self.favorites.iter().position(|&(l, c)| l == lib_idx && c == comp_idx) {
            self.favorites.remove(pos);
            false
        } else {
            self.favorites.push((lib_idx, comp_idx));
            true
        }
    }

    /// Get favorites.
    pub fn favorites(&self) -> Vec<&LibraryComponent> {
        self.favorites.iter()
            .filter_map(|(lib_idx, comp_idx)| {
                self.libraries.get(*lib_idx)
                    .and_then(|lib| lib.components.get(*comp_idx))
            })
            .collect()
    }

    /// Check if component is favorite.
    pub fn is_favorite(&self, lib_idx: usize, comp_idx: usize) -> bool {
        self.favorites.iter().any(|&(l, c)| l == lib_idx && c == comp_idx)
    }
}

/// Search result from library browser.
#[derive(Debug, Clone)]
pub struct BrowserResult {
    /// Library index
    pub library_index: usize,
    /// Library name
    pub library_name: String,
    /// Component index within library
    pub component_index: usize,
    /// The component
    pub component: LibraryComponent,
    /// Match score (higher = better match)
    pub match_score: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_library_creation() {
        let mut lib = Library::new("My Components");
        
        let resistor = LibraryComponent::new("Resistor_0603", ComponentType::Component)
            .with_description("0603 SMD resistor")
            .with_keywords(vec!["resistor".into(), "smd".into(), "0603".into()]);
        
        lib.add_component(resistor);
        
        assert_eq!(lib.components.len(), 1);
        assert!(lib.find_by_name("Resistor_0603").is_some());
    }

    #[test]
    fn test_library_search() {
        let mut lib = Library::new("Test");
        
        lib.add_component(LibraryComponent::new("Resistor_0603", ComponentType::Component)
            .with_keywords(vec!["resistor".into()]));
        lib.add_component(LibraryComponent::new("Capacitor_0603", ComponentType::Component)
            .with_keywords(vec!["capacitor".into()]));
        lib.add_component(LibraryComponent::new("Resistor_0402", ComponentType::Component)
            .with_keywords(vec!["resistor".into()]));
        
        let results = lib.search("resistor");
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_library_validation() {
        let mut lib = Library::new("Test");
        lib.quality.require_description = true;
        
        lib.add_component(LibraryComponent::new("NoDesc", ComponentType::Symbol));
        
        let report = lib.validate();
        assert_eq!(report.warning_count(), 1);
    }

    #[test]
    fn test_library_browser_new() {
        let browser = LibraryBrowser::new();
        assert_eq!(browser.total_components(), 0);
        assert!(browser.libraries().is_empty());
    }

    #[test]
    fn test_library_browser_add_library() {
        let mut browser = LibraryBrowser::new();
        
        let mut lib = Library::new("Test Library");
        lib.add_component(LibraryComponent::new("Resistor", ComponentType::Component)
            .with_keywords(vec!["resistor".into(), "passive".into()]));
        lib.add_component(LibraryComponent::new("Capacitor", ComponentType::Component)
            .with_keywords(vec!["capacitor".into(), "passive".into()]));
        
        browser.add_library(lib);
        
        assert_eq!(browser.total_components(), 2);
        assert_eq!(browser.libraries().len(), 1);
    }

    #[test]
    fn test_library_browser_search() {
        let mut browser = LibraryBrowser::new();
        
        let mut lib = Library::new("Components");
        lib.add_component(LibraryComponent::new("Resistor_0603", ComponentType::Component)
            .with_keywords(vec!["resistor".into(), "smd".into()]));
        lib.add_component(LibraryComponent::new("Resistor_0402", ComponentType::Component)
            .with_keywords(vec!["resistor".into(), "smd".into()]));
        lib.add_component(LibraryComponent::new("Capacitor_0603", ComponentType::Component)
            .with_keywords(vec!["capacitor".into(), "smd".into()]));
        
        browser.add_library(lib);
        
        let results = browser.search("resistor");
        assert_eq!(results.len(), 2);
        
        let results = browser.search("smd");
        assert_eq!(results.len(), 3);
    }

    #[test]
    fn test_library_browser_categories() {
        let mut browser = LibraryBrowser::new();
        
        let mut lib = Library::new("Test");
        lib.add_component(LibraryComponent::new("Symbol1", ComponentType::Symbol));
        lib.add_component(LibraryComponent::new("Footprint1", ComponentType::Footprint));
        lib.add_component(LibraryComponent::new("Component1", ComponentType::Component));
        
        browser.add_library(lib);
        
        let categories = browser.categories();
        assert!(categories.len() >= 3);
        
        let symbols = browser.by_category("symbol");
        assert_eq!(symbols.len(), 1);
    }

    #[test]
    fn test_library_browser_recent() {
        let mut browser = LibraryBrowser::new();
        
        let mut lib = Library::new("Test");
        lib.add_component(LibraryComponent::new("Comp1", ComponentType::Component));
        lib.add_component(LibraryComponent::new("Comp2", ComponentType::Component));
        
        browser.add_library(lib);
        
        browser.add_recent(0, 0);
        browser.add_recent(0, 1);
        
        let recent = browser.recent();
        assert_eq!(recent.len(), 2);
        assert_eq!(recent[0].name, "Comp2");  // Most recent first
    }

    #[test]
    fn test_library_browser_favorites() {
        let mut browser = LibraryBrowser::new();
        
        let mut lib = Library::new("Test");
        lib.add_component(LibraryComponent::new("Comp1", ComponentType::Component));
        
        browser.add_library(lib);
        
        assert!(!browser.is_favorite(0, 0));
        
        let added = browser.toggle_favorite(0, 0);
        assert!(added);
        assert!(browser.is_favorite(0, 0));
        
        let removed = browser.toggle_favorite(0, 0);
        assert!(!removed);
        assert!(!browser.is_favorite(0, 0));
    }

    #[test]
    fn test_library_browser_filtered_search() {
        let mut browser = LibraryBrowser::new();
        
        let mut lib = Library::new("Test");
        lib.add_component(LibraryComponent::new("Resistor", ComponentType::Component)
            .with_datasheet("http://example.com/datasheet.pdf"));
        lib.add_component(LibraryComponent::new("Capacitor", ComponentType::Component));
        
        browser.add_library(lib);
        
        let filter = LibrarySearchQuery {
            has_datasheet: Some(true),
            limit: Some(10),
            ..Default::default()
        };
        
        let results = browser.search_filtered(&filter);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].component.name, "Resistor");
    }
}

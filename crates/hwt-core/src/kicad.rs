//! KiCAD Import/Export Module.
//!
//! Parses KiCAD schematic (.kicad_sch), PCB (.kicad_pcb), and library files.
//! KiCAD uses S-expression format for all file types.

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use uuid::Uuid;

use crate::component::{Component, Pin, PinType};
use crate::geometry::{Point2D, Position};
use crate::layout::{Layout, Layer, LayerType, PlacedComponent, ComponentLayer, Trace, Via, ViaType, Zone, ZoneFillType, Pad, PadShape, PadType};
use crate::units::LengthUnit;
use crate::schematic::{
    Bus, BusSegment, Junction, LabelType, NetLabel, NoConnect, PlacedSymbol, PowerSymbol,
    PowerSymbolStyle, SchematicSheet, SymbolPin, SymbolProperty, PinElectricalType, Wire,
};

/// KiCAD import error.
#[derive(Debug, Clone)]
pub struct KicadError {
    pub message: String,
    pub line: Option<usize>,
}

impl std::fmt::Display for KicadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(line) = self.line {
            write!(f, "KiCAD error at line {}: {}", line, self.message)
        } else {
            write!(f, "KiCAD error: {}", self.message)
        }
    }
}

impl std::error::Error for KicadError {}

pub type KicadResult<T> = Result<T, KicadError>;

/// S-expression token.
#[derive(Debug, Clone, PartialEq)]
pub enum SExpr {
    /// Atom (string or number)
    Atom(String),
    /// List of expressions
    List(Vec<SExpr>),
}

impl SExpr {
    /// Get as atom string.
    pub fn as_atom(&self) -> Option<&str> {
        match self {
            SExpr::Atom(s) => Some(s),
            _ => None,
        }
    }

    /// Get as list.
    pub fn as_list(&self) -> Option<&[SExpr]> {
        match self {
            SExpr::List(l) => Some(l),
            _ => None,
        }
    }

    /// Get first element (tag) of a list.
    pub fn tag(&self) -> Option<&str> {
        self.as_list()?.first()?.as_atom()
    }

    /// Find child list by tag.
    pub fn find(&self, tag: &str) -> Option<&SExpr> {
        self.as_list()?.iter().find(|e| e.tag() == Some(tag))
    }

    /// Find all child lists by tag.
    pub fn find_all(&self, tag: &str) -> Vec<&SExpr> {
        self.as_list()
            .map(|l| l.iter().filter(|e| e.tag() == Some(tag)).collect())
            .unwrap_or_default()
    }

    /// Get nth element of list.
    pub fn get(&self, index: usize) -> Option<&SExpr> {
        self.as_list()?.get(index)
    }

    /// Get nth element as atom.
    pub fn get_atom(&self, index: usize) -> Option<&str> {
        self.get(index)?.as_atom()
    }

    /// Get nth element as f64.
    pub fn get_f64(&self, index: usize) -> Option<f64> {
        self.get_atom(index)?.parse().ok()
    }

    /// Get property value by key.
    pub fn property(&self, key: &str) -> Option<&str> {
        for expr in self.as_list()? {
            if let Some(list) = expr.as_list() {
                if list.first()?.as_atom()? == key {
                    return list.get(1)?.as_atom();
                }
            }
        }
        None
    }
}

/// S-expression parser.
pub struct SExprParser<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> SExprParser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input, pos: 0 }
    }

    /// Parse the entire input.
    pub fn parse(&mut self) -> KicadResult<SExpr> {
        self.skip_whitespace();
        self.parse_expr()
    }

    fn parse_expr(&mut self) -> KicadResult<SExpr> {
        self.skip_whitespace();

        if self.peek() == Some('(') {
            self.parse_list()
        } else {
            self.parse_atom()
        }
    }

    fn parse_list(&mut self) -> KicadResult<SExpr> {
        self.expect('(')?;
        let mut items = Vec::new();

        loop {
            self.skip_whitespace();
            if self.peek() == Some(')') {
                self.advance();
                break;
            }
            if self.peek().is_none() {
                return Err(KicadError {
                    message: "Unexpected end of input in list".to_string(),
                    line: None,
                });
            }
            items.push(self.parse_expr()?);
        }

        Ok(SExpr::List(items))
    }

    fn parse_atom(&mut self) -> KicadResult<SExpr> {
        self.skip_whitespace();

        if self.peek() == Some('"') {
            self.parse_string()
        } else {
            self.parse_symbol()
        }
    }

    fn parse_string(&mut self) -> KicadResult<SExpr> {
        self.expect('"')?;
        let mut s = String::new();

        while let Some(c) = self.peek() {
            if c == '"' {
                self.advance();
                break;
            }
            if c == '\\' {
                self.advance();
                if let Some(escaped) = self.peek() {
                    s.push(match escaped {
                        'n' => '\n',
                        't' => '\t',
                        'r' => '\r',
                        _ => escaped,
                    });
                    self.advance();
                }
            } else {
                s.push(c);
                self.advance();
            }
        }

        Ok(SExpr::Atom(s))
    }

    fn parse_symbol(&mut self) -> KicadResult<SExpr> {
        let mut s = String::new();

        while let Some(c) = self.peek() {
            if c.is_whitespace() || c == '(' || c == ')' {
                break;
            }
            s.push(c);
            self.advance();
        }

        if s.is_empty() {
            return Err(KicadError {
                message: "Empty symbol".to_string(),
                line: None,
            });
        }

        Ok(SExpr::Atom(s))
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn peek(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    fn advance(&mut self) {
        if let Some(c) = self.peek() {
            self.pos += c.len_utf8();
        }
    }

    fn expect(&mut self, expected: char) -> KicadResult<()> {
        if self.peek() == Some(expected) {
            self.advance();
            Ok(())
        } else {
            Err(KicadError {
                message: format!("Expected '{}', got {:?}", expected, self.peek()),
                line: None,
            })
        }
    }
}

/// KiCAD schematic importer.
pub struct KicadSchematicImporter;

impl KicadSchematicImporter {
    /// Import a KiCAD schematic file.
    pub fn import<P: AsRef<Path>>(path: P) -> KicadResult<SchematicSheet> {
        let content = fs::read_to_string(path.as_ref()).map_err(|e| KicadError {
            message: format!("Failed to read file: {}", e),
            line: None,
        })?;

        Self::import_from_string(&content)
    }

    /// Import from string content.
    pub fn import_from_string(content: &str) -> KicadResult<SchematicSheet> {
        let mut parser = SExprParser::new(content);
        let expr = parser.parse()?;

        if expr.tag() != Some("kicad_sch") {
            return Err(KicadError {
                message: "Not a valid KiCAD schematic file".to_string(),
                line: None,
            });
        }

        let mut sheet = SchematicSheet::new("Imported");

        // Parse UUID if present
        if let Some(uuid_expr) = expr.find("uuid") {
            if let Some(uuid_str) = uuid_expr.get_atom(1) {
                if let Ok(uuid) = Uuid::parse_str(uuid_str) {
                    sheet.id = uuid;
                }
            }
        }

        // Parse symbols
        for symbol_expr in expr.find_all("symbol") {
            if let Ok(symbol) = Self::parse_symbol(symbol_expr) {
                sheet.symbols.push(symbol);
            }
        }

        // Parse wires
        for wire_expr in expr.find_all("wire") {
            if let Ok(wire) = Self::parse_wire(wire_expr) {
                sheet.wires.push(wire);
            }
        }

        // Parse labels
        for label_expr in expr.find_all("label") {
            if let Ok(label) = Self::parse_label(label_expr, LabelType::Local) {
                sheet.labels.push(label);
            }
        }

        for label_expr in expr.find_all("global_label") {
            if let Ok(label) = Self::parse_label(label_expr, LabelType::Global) {
                sheet.labels.push(label);
            }
        }

        for label_expr in expr.find_all("hierarchical_label") {
            if let Ok(label) = Self::parse_label(label_expr, LabelType::Hierarchical) {
                sheet.labels.push(label);
            }
        }

        // Parse junctions
        for junction_expr in expr.find_all("junction") {
            if let Ok(junction) = Self::parse_junction(junction_expr) {
                sheet.junctions.push(junction);
            }
        }

        // Parse no-connects
        for nc_expr in expr.find_all("no_connect") {
            if let Ok(nc) = Self::parse_no_connect(nc_expr) {
                sheet.no_connects.push(nc);
            }
        }

        // Parse power symbols (they're symbols with power property)
        for symbol_expr in expr.find_all("symbol") {
            if Self::is_power_symbol(symbol_expr) {
                if let Ok(power) = Self::parse_power_symbol(symbol_expr) {
                    sheet.power_symbols.push(power);
                }
            }
        }

        // Parse buses
        for bus_expr in expr.find_all("bus") {
            if let Ok(bus) = Self::parse_bus(bus_expr) {
                sheet.buses.push(bus);
            }
        }

        Ok(sheet)
    }

    fn parse_symbol(expr: &SExpr) -> KicadResult<PlacedSymbol> {
        let lib_id = expr
            .find("lib_id")
            .and_then(|e| e.get_atom(1))
            .unwrap_or("unknown:unknown")
            .to_string();

        // Split lib_id into library and symbol_name (format: "Library:Symbol")
        let (library, symbol_name) = if let Some(idx) = lib_id.find(':') {
            (lib_id[..idx].to_string(), lib_id[idx + 1..].to_string())
        } else {
            ("unknown".to_string(), lib_id)
        };

        let uuid = expr
            .find("uuid")
            .and_then(|e| e.get_atom(1))
            .and_then(|s| Uuid::parse_str(s).ok())
            .unwrap_or_else(Uuid::new_v4);

        // Parse position from (at x y angle)
        let (x, y, rotation) = if let Some(at_expr) = expr.find("at") {
            (
                at_expr.get_f64(1).unwrap_or(0.0),
                at_expr.get_f64(2).unwrap_or(0.0),
                at_expr.get_f64(3).unwrap_or(0.0),
            )
        } else {
            (0.0, 0.0, 0.0)
        };

        // Parse mirror from (mirror x) or (mirror y)
        let mirror_x = expr.find("mirror").map(|e| e.get_atom(1) == Some("x")).unwrap_or(false);
        let mirror_y = expr.find("mirror").map(|e| e.get_atom(1) == Some("y")).unwrap_or(false);

        // Parse reference designator from property "Reference"
        let reference = Self::get_property(expr, "Reference").unwrap_or_else(|| "U?".to_string());

        // Parse value from property "Value"
        let value = Self::get_property(expr, "Value").unwrap_or_default();

        // Parse unit number
        let unit = expr
            .find("unit")
            .and_then(|e| e.get_atom(1))
            .and_then(|s| s.parse().ok())
            .unwrap_or(1);

        Ok(PlacedSymbol {
            id: uuid,
            reference,
            value,
            library,
            symbol_name,
            position: Point2D::new(x, y),
            rotation,
            mirror_x,
            mirror_y,
            unit,
            pins: Vec::new(),
            properties: Vec::new(),
        })
    }

    fn get_property(expr: &SExpr, name: &str) -> Option<String> {
        for prop_expr in expr.find_all("property") {
            if prop_expr.get_atom(1) == Some(name) {
                return prop_expr.get_atom(2).map(|s| s.to_string());
            }
        }
        None
    }

    fn parse_wire(expr: &SExpr) -> KicadResult<Wire> {
        let uuid = expr
            .find("uuid")
            .and_then(|e| e.get_atom(1))
            .and_then(|s| Uuid::parse_str(s).ok())
            .unwrap_or_else(Uuid::new_v4);

        let mut start = Point2D::new(0.0, 0.0);
        let mut end = Point2D::new(0.0, 0.0);

        // Parse pts (points) - Wire has just start and end
        if let Some(pts_expr) = expr.find("pts") {
            let xy_exprs: Vec<_> = pts_expr.find_all("xy");
            if xy_exprs.len() >= 2 {
                start = Point2D::new(
                    xy_exprs[0].get_f64(1).unwrap_or(0.0),
                    xy_exprs[0].get_f64(2).unwrap_or(0.0),
                );
                end = Point2D::new(
                    xy_exprs[1].get_f64(1).unwrap_or(0.0),
                    xy_exprs[1].get_f64(2).unwrap_or(0.0),
                );
            }
        }

        Ok(Wire {
            id: uuid,
            start,
            end,
            net_name: None,
        })
    }

    fn parse_label(expr: &SExpr, label_type: LabelType) -> KicadResult<NetLabel> {
        let name = expr.get_atom(1).unwrap_or("").to_string();

        let uuid = expr
            .find("uuid")
            .and_then(|e| e.get_atom(1))
            .and_then(|s| Uuid::parse_str(s).ok())
            .unwrap_or_else(Uuid::new_v4);

        let (x, y, rotation) = if let Some(at_expr) = expr.find("at") {
            (
                at_expr.get_f64(1).unwrap_or(0.0),
                at_expr.get_f64(2).unwrap_or(0.0),
                at_expr.get_f64(3).unwrap_or(0.0),
            )
        } else {
            (0.0, 0.0, 0.0)
        };

        Ok(NetLabel {
            id: uuid,
            name,
            position: Point2D::new(x, y),
            label_type,
            rotation,
        })
    }

    fn parse_junction(expr: &SExpr) -> KicadResult<Junction> {
        let uuid = expr
            .find("uuid")
            .and_then(|e| e.get_atom(1))
            .and_then(|s| Uuid::parse_str(s).ok())
            .unwrap_or_else(Uuid::new_v4);

        let (x, y) = if let Some(at_expr) = expr.find("at") {
            (
                at_expr.get_f64(1).unwrap_or(0.0),
                at_expr.get_f64(2).unwrap_or(0.0),
            )
        } else {
            (0.0, 0.0)
        };

        Ok(Junction {
            id: uuid,
            position: Point2D::new(x, y),
        })
    }

    fn parse_no_connect(expr: &SExpr) -> KicadResult<NoConnect> {
        let uuid = expr
            .find("uuid")
            .and_then(|e| e.get_atom(1))
            .and_then(|s| Uuid::parse_str(s).ok())
            .unwrap_or_else(Uuid::new_v4);

        let (x, y) = if let Some(at_expr) = expr.find("at") {
            (
                at_expr.get_f64(1).unwrap_or(0.0),
                at_expr.get_f64(2).unwrap_or(0.0),
            )
        } else {
            (0.0, 0.0)
        };

        Ok(NoConnect {
            id: uuid,
            position: Point2D::new(x, y),
        })
    }

    fn is_power_symbol(expr: &SExpr) -> bool {
        // Check if symbol has power property or lib_id contains "power"
        if let Some(lib_id) = expr.find("lib_id").and_then(|e| e.get_atom(1)) {
            if lib_id.to_lowercase().contains("power") {
                return true;
            }
        }
        
        // Check for power property
        for prop_expr in expr.find_all("property") {
            if prop_expr.get_atom(1) == Some("power") {
                return true;
            }
        }
        
        false
    }

    fn parse_power_symbol(expr: &SExpr) -> KicadResult<PowerSymbol> {
        let uuid = expr
            .find("uuid")
            .and_then(|e| e.get_atom(1))
            .and_then(|s| Uuid::parse_str(s).ok())
            .unwrap_or_else(Uuid::new_v4);

        let net_name = Self::get_property(expr, "Value").unwrap_or_else(|| "VCC".to_string());

        let (x, y, rotation) = if let Some(at_expr) = expr.find("at") {
            (
                at_expr.get_f64(1).unwrap_or(0.0),
                at_expr.get_f64(2).unwrap_or(0.0),
                at_expr.get_f64(3).unwrap_or(0.0),
            )
        } else {
            (0.0, 0.0, 0.0)
        };

        // Determine style from net name
        let style = if net_name.to_uppercase().contains("GND") {
            PowerSymbolStyle::Ground
        } else if net_name.to_uppercase().contains("EARTH") {
            PowerSymbolStyle::Earth
        } else {
            PowerSymbolStyle::Bar // Default power symbol style
        };

        Ok(PowerSymbol {
            id: uuid,
            net_name,
            position: Point2D::new(x, y),
            rotation,
            style,
        })
    }

    fn parse_bus(expr: &SExpr) -> KicadResult<Bus> {
        let uuid = expr
            .find("uuid")
            .and_then(|e| e.get_atom(1))
            .and_then(|s| Uuid::parse_str(s).ok())
            .unwrap_or_else(Uuid::new_v4);

        let mut segments = Vec::new();

        // Parse pts (points)
        if let Some(pts_expr) = expr.find("pts") {
            let xy_exprs: Vec<_> = pts_expr.find_all("xy");
            for window in xy_exprs.windows(2) {
                let start_x = window[0].get_f64(1).unwrap_or(0.0);
                let start_y = window[0].get_f64(2).unwrap_or(0.0);
                let end_x = window[1].get_f64(1).unwrap_or(0.0);
                let end_y = window[1].get_f64(2).unwrap_or(0.0);

                segments.push(BusSegment {
                    start: Point2D::new(start_x, start_y),
                    end: Point2D::new(end_x, end_y),
                });
            }
        }

        Ok(Bus {
            id: uuid,
            name: String::new(),
            segments,
        })
    }
}

/// KiCAD symbol library importer.
pub struct KicadSymbolLibImporter;

impl KicadSymbolLibImporter {
    /// Import a KiCAD symbol library file.
    pub fn import<P: AsRef<Path>>(path: P) -> KicadResult<Vec<Component>> {
        let content = fs::read_to_string(path.as_ref()).map_err(|e| KicadError {
            message: format!("Failed to read file: {}", e),
            line: None,
        })?;

        Self::import_from_string(&content)
    }

    /// Import from string content.
    pub fn import_from_string(content: &str) -> KicadResult<Vec<Component>> {
        let mut parser = SExprParser::new(content);
        let expr = parser.parse()?;

        if expr.tag() != Some("kicad_symbol_lib") {
            return Err(KicadError {
                message: "Not a valid KiCAD symbol library file".to_string(),
                line: None,
            });
        }

        let mut components = Vec::new();

        for symbol_expr in expr.find_all("symbol") {
            if let Ok(component) = Self::parse_symbol(symbol_expr) {
                components.push(component);
            }
        }

        Ok(components)
    }

    fn parse_symbol(expr: &SExpr) -> KicadResult<Component> {
        let name = expr.get_atom(1).unwrap_or("Unknown").to_string();

        let mut component = Component::new(&name, &name);

        // Parse pins
        for pin_expr in Self::find_all_recursive(expr, "pin") {
            if let Ok(pin) = Self::parse_pin(pin_expr) {
                component.pins.push(pin);
            }
        }

        // Parse properties
        for prop_expr in expr.find_all("property") {
            if let (Some(key), Some(value)) = (prop_expr.get_atom(1), prop_expr.get_atom(2)) {
                component.properties.insert(key.to_string(), value.to_string());
            }
        }

        Ok(component)
    }

    fn find_all_recursive<'a>(expr: &'a SExpr, tag: &str) -> Vec<&'a SExpr> {
        let mut results = Vec::new();
        
        if let Some(list) = expr.as_list() {
            for item in list {
                if item.tag() == Some(tag) {
                    results.push(item);
                }
                results.extend(Self::find_all_recursive(item, tag));
            }
        }
        
        results
    }

    fn parse_pin(expr: &SExpr) -> KicadResult<Pin> {
        // Pin format: (pin type style (at x y angle) (length len) (name "name") (number "num"))
        let pin_type_str = expr.get_atom(1).unwrap_or("passive");
        
        let pin_type = match pin_type_str {
            "input" => PinType::Input,
            "output" => PinType::Output,
            "bidirectional" => PinType::Bidirectional,
            "power_in" => PinType::PowerInput,
            "power_out" => PinType::PowerOutput,
            "passive" => PinType::Passive,
            "no_connect" => PinType::NoConnect,
            "open_collector" => PinType::OpenCollector,
            "open_emitter" => PinType::OpenEmitter,
            "tri_state" => PinType::TriState,
            _ => PinType::Passive,
        };

        let name = expr
            .find("name")
            .and_then(|e| e.get_atom(1))
            .unwrap_or("~")
            .to_string();

        let number = expr
            .find("number")
            .and_then(|e| e.get_atom(1))
            .unwrap_or("1")
            .to_string();

        // Pin struct uses id and name, with pin_type
        Ok(Pin {
            id: number,
            name,
            net: None,
            pin_type,
        })
    }
}

/// KiCAD PCB importer.
pub struct KicadPcbImporter;

impl KicadPcbImporter {
    /// Import a KiCAD PCB file.
    pub fn import<P: AsRef<Path>>(path: P) -> KicadResult<Layout> {
        let content = fs::read_to_string(path.as_ref()).map_err(|e| KicadError {
            message: format!("Failed to read file: {}", e),
            line: None,
        })?;

        Self::import_from_string(&content)
    }

    /// Import from string content.
    pub fn import_from_string(content: &str) -> KicadResult<Layout> {
        let mut parser = SExprParser::new(content);
        let expr = parser.parse()?;

        if expr.tag() != Some("kicad_pcb") {
            return Err(KicadError {
                message: "Not a valid KiCAD PCB file".to_string(),
                line: None,
            });
        }

        let mut layout = Layout::new();

        // Parse layers
        if let Some(layers_expr) = expr.find("layers") {
            layout.layers = Self::parse_layers(layers_expr)?;
        } else {
            layout.layers = Layout::default_pcb_layers();
        }

        // Parse general info for board size
        if let Some(general_expr) = expr.find("general") {
            Self::parse_general(&mut layout, general_expr);
        }

        // Parse footprints (components)
        for fp_expr in expr.find_all("footprint") {
            if let Ok(component) = Self::parse_footprint(fp_expr) {
                layout.components.push(component);
            }
        }

        // Parse segments (traces)
        for segment_expr in expr.find_all("segment") {
            if let Ok(trace) = Self::parse_segment(segment_expr) {
                layout.traces.push(trace);
            }
        }

        // Parse vias
        for via_expr in expr.find_all("via") {
            if let Ok(via) = Self::parse_via(via_expr) {
                layout.vias.push(via);
            }
        }

        // Parse zones
        for zone_expr in expr.find_all("zone") {
            if let Ok(zone) = Self::parse_zone(zone_expr) {
                layout.zones.push(zone);
            }
        }

        Ok(layout)
    }

    /// Parse layer definitions.
    fn parse_layers(expr: &SExpr) -> KicadResult<Vec<Layer>> {
        let mut layers = Vec::new();
        
        if let Some(list) = expr.as_list() {
            for item in list.iter().skip(1) {
                if let Some(layer_list) = item.as_list() {
                    if layer_list.len() >= 3 {
                        let name = layer_list.get(1)
                            .and_then(|e| e.as_atom())
                            .unwrap_or("Unknown")
                            .to_string();
                        
                        let type_str = layer_list.get(2)
                            .and_then(|e| e.as_atom())
                            .unwrap_or("signal");
                        
                        let layer_type = match type_str {
                            "signal" | "power" => LayerType::Copper,
                            "user" => LayerType::Fabrication,
                            _ => LayerType::Fabrication,
                        };
                        
                        layers.push(Layer::new(&name, layer_type));
                    }
                }
            }
        }
        
        // Add standard non-copper layers if not present
        let standard_layers = [
            ("F.SilkS", LayerType::Silkscreen),
            ("B.SilkS", LayerType::Silkscreen),
            ("F.Mask", LayerType::SolderMask),
            ("B.Mask", LayerType::SolderMask),
            ("Edge.Cuts", LayerType::Fabrication),
        ];
        
        for (name, layer_type) in standard_layers {
            if !layers.iter().any(|l| l.name == name) {
                layers.push(Layer::new(name, layer_type));
            }
        }
        
        Ok(layers)
    }

    /// Parse general board info.
    fn parse_general(layout: &mut Layout, _expr: &SExpr) {
        // Board outline is typically in gr_rect or gr_poly on Edge.Cuts layer
        // For now, we'll leave outline as None and let it be set from edge cuts
        layout.outline = None;
    }

    /// Parse a footprint (placed component).
    fn parse_footprint(expr: &SExpr) -> KicadResult<PlacedComponent> {
        let footprint_name = expr.get_atom(1)
            .unwrap_or("Unknown")
            .to_string();

        // Parse position
        let (x, y, rotation) = if let Some(at_expr) = expr.find("at") {
            let x = at_expr.get_f64(1).unwrap_or(0.0);
            let y = at_expr.get_f64(2).unwrap_or(0.0);
            let rot = at_expr.get_f64(3).unwrap_or(0.0);
            (x, y, rot)
        } else {
            (0.0, 0.0, 0.0)
        };

        // Parse layer
        let layer = expr.find("layer")
            .and_then(|e| e.get_atom(1))
            .unwrap_or("F.Cu")
            .to_string();

        let component_layer = if layer.starts_with("B.") {
            ComponentLayer::Bottom
        } else {
            ComponentLayer::Top
        };

        // Parse reference
        let reference = expr.find_all("fp_text")
            .iter()
            .find(|e| e.get_atom(1) == Some("reference"))
            .and_then(|e| e.get_atom(2))
            .unwrap_or("U?")
            .to_string();

        // Parse value
        let value = expr.find_all("fp_text")
            .iter()
            .find(|e| e.get_atom(1) == Some("value"))
            .and_then(|e| e.get_atom(2))
            .unwrap_or("")
            .to_string();

        // Parse pads
        let mut pads = Vec::new();
        for pad_expr in expr.find_all("pad") {
            if let Ok(pad) = Self::parse_pad(pad_expr) {
                pads.push(pad);
            }
        }

        Ok(PlacedComponent {
            id: Uuid::new_v4(),
            reference,
            value,
            footprint: footprint_name,
            position: Position {
                x,
                y,
                z: None,
                unit: LengthUnit::Mm,
            },
            rotation,
            layer: component_layer,
            pads,
            locked: false,
        })
    }

    /// Parse a pad.
    fn parse_pad(expr: &SExpr) -> KicadResult<Pad> {
        let number = expr.get_atom(1).unwrap_or("1").to_string();
        let pad_type_str = expr.get_atom(2).unwrap_or("smd");
        let shape_str = expr.get_atom(3).unwrap_or("rect");

        let pad_type = match pad_type_str {
            "thru_hole" => PadType::ThruHole,
            "smd" => PadType::Smd,
            "np_thru_hole" => PadType::Npth,
            "connect" => PadType::Connect,
            _ => PadType::Smd,
        };

        let shape = match shape_str {
            "circle" => PadShape::Circle,
            "rect" => PadShape::Rect,
            "oval" => PadShape::Oval,
            "roundrect" => PadShape::RoundRect,
            "trapezoid" => PadShape::Trapezoid,
            "custom" => PadShape::Custom,
            _ => PadShape::Rect,
        };

        // Parse position
        let (x, y) = if let Some(at_expr) = expr.find("at") {
            (at_expr.get_f64(1).unwrap_or(0.0), at_expr.get_f64(2).unwrap_or(0.0))
        } else {
            (0.0, 0.0)
        };

        // Parse size
        let (width, height) = if let Some(size_expr) = expr.find("size") {
            (size_expr.get_f64(1).unwrap_or(1.0), size_expr.get_f64(2).unwrap_or(1.0))
        } else {
            (1.0, 1.0)
        };

        // Parse drill
        let drill = expr.find("drill")
            .and_then(|e| e.get_f64(1))
            .unwrap_or(0.0);

        // Parse net
        let net = expr.find("net")
            .and_then(|e| e.get_atom(2))
            .map(|s| s.to_string());

        Ok(Pad {
            number,
            name: None,
            pad_type,
            shape,
            position: Point2D::new(x, y),
            size: (width, height),
            drill,
            net,
            layers: Vec::new(),
        })
    }

    /// Parse a trace segment.
    fn parse_segment(expr: &SExpr) -> KicadResult<Trace> {
        let start = if let Some(start_expr) = expr.find("start") {
            Position {
                x: start_expr.get_f64(1).unwrap_or(0.0),
                y: start_expr.get_f64(2).unwrap_or(0.0),
                z: None,
                unit: LengthUnit::Mm,
            }
        } else {
            return Err(KicadError {
                message: "Segment missing start point".to_string(),
                line: None,
            });
        };

        let end = if let Some(end_expr) = expr.find("end") {
            Position {
                x: end_expr.get_f64(1).unwrap_or(0.0),
                y: end_expr.get_f64(2).unwrap_or(0.0),
                z: None,
                unit: LengthUnit::Mm,
            }
        } else {
            return Err(KicadError {
                message: "Segment missing end point".to_string(),
                line: None,
            });
        };

        let width = expr.find("width")
            .and_then(|e| e.get_f64(1))
            .unwrap_or(0.25);

        let layer = expr.find("layer")
            .and_then(|e| e.get_atom(1))
            .unwrap_or("F.Cu")
            .to_string();

        let net = expr.find("net")
            .and_then(|e| e.get_atom(1))
            .unwrap_or("")
            .to_string();

        Ok(Trace {
            net,
            layer,
            start,
            end,
            width,
            unit: LengthUnit::Mm,
        })
    }

    /// Parse a via.
    fn parse_via(expr: &SExpr) -> KicadResult<Via> {
        let position = if let Some(at_expr) = expr.find("at") {
            Position {
                x: at_expr.get_f64(1).unwrap_or(0.0),
                y: at_expr.get_f64(2).unwrap_or(0.0),
                z: None,
                unit: LengthUnit::Mm,
            }
        } else {
            return Err(KicadError {
                message: "Via missing position".to_string(),
                line: None,
            });
        };

        let size = expr.find("size")
            .and_then(|e| e.get_f64(1))
            .unwrap_or(0.6);

        let drill = expr.find("drill")
            .and_then(|e| e.get_f64(1))
            .unwrap_or(0.3);

        let net = expr.find("net")
            .and_then(|e| e.get_atom(1))
            .unwrap_or("")
            .to_string();

        // Parse layers for blind/buried vias
        let layers: Vec<String> = expr.find("layers")
            .and_then(|e| e.as_list())
            .map(|l| l.iter().skip(1).filter_map(|e| e.as_atom().map(|s| s.to_string())).collect())
            .unwrap_or_default();

        let (via_type, start_layer, end_layer) = if layers.len() >= 2 {
            let start = layers.first().cloned();
            let end = layers.last().cloned();
            
            // Determine via type based on layers
            let vt = if start.as_deref() == Some("F.Cu") && end.as_deref() == Some("B.Cu") {
                ViaType::Through
            } else {
                ViaType::Blind
            };
            (vt, start, end)
        } else {
            (ViaType::Through, None, None)
        };

        Ok(Via {
            net,
            position,
            via_type,
            drill,
            pad: size,
            start_layer,
            end_layer,
            unit: LengthUnit::Mm,
        })
    }

    /// Parse a copper zone.
    fn parse_zone(expr: &SExpr) -> KicadResult<Zone> {
        let net = expr.find("net_name")
            .and_then(|e| e.get_atom(1))
            .unwrap_or("")
            .to_string();

        let layer = expr.find("layer")
            .and_then(|e| e.get_atom(1))
            .unwrap_or("F.Cu")
            .to_string();

        // Parse polygon points
        let mut points = Vec::new();
        if let Some(polygon_expr) = expr.find("polygon") {
            if let Some(pts_expr) = polygon_expr.find("pts") {
                for xy_expr in pts_expr.find_all("xy") {
                    let x = xy_expr.get_f64(1).unwrap_or(0.0);
                    let y = xy_expr.get_f64(2).unwrap_or(0.0);
                    points.push(Point2D::new(x, y));
                }
            }
        }

        // Parse fill settings
        let fill_type = if let Some(fill_expr) = expr.find("fill") {
            if fill_expr.get_atom(1) == Some("yes") {
                ZoneFillType::Solid
            } else {
                ZoneFillType::None
            }
        } else {
            ZoneFillType::Solid
        };

        let clearance = expr.find("clearance")
            .and_then(|e| e.get_f64(1));

        let min_thickness = expr.find("min_thickness")
            .and_then(|e| e.get_f64(1));

        Ok(Zone {
            net,
            layer,
            points,
            fill_type,
            clearance,
            min_width: min_thickness,
            unit: LengthUnit::Mm,
        })
    }
}

/// A footprint definition (library footprint, not placed on board).
#[derive(Debug, Clone)]
pub struct Footprint {
    /// Footprint name
    pub name: String,
    /// Library name (if known)
    pub library: Option<String>,
    /// Description
    pub description: Option<String>,
    /// Keywords for searching
    pub keywords: Option<String>,
    /// Pads
    pub pads: Vec<Pad>,
    /// Courtyard width
    pub courtyard_width: Option<f64>,
    /// Courtyard height
    pub courtyard_height: Option<f64>,
    /// 3D model path
    pub model_3d: Option<String>,
}

impl Footprint {
    /// Create a new empty footprint.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            library: None,
            description: None,
            keywords: None,
            pads: Vec::new(),
            courtyard_width: None,
            courtyard_height: None,
            model_3d: None,
        }
    }
}

/// KiCAD footprint importer (.kicad_mod files).
pub struct KicadFootprintImporter;

impl KicadFootprintImporter {
    /// Import a KiCAD footprint file.
    pub fn import<P: AsRef<Path>>(path: P) -> KicadResult<Footprint> {
        let content = fs::read_to_string(path.as_ref()).map_err(|e| KicadError {
            message: format!("Failed to read file: {}", e),
            line: None,
        })?;

        Self::import_from_string(&content)
    }

    /// Import from string content.
    pub fn import_from_string(content: &str) -> KicadResult<Footprint> {
        let mut parser = SExprParser::new(content);
        let expr = parser.parse()?;

        // Footprint files can start with "module" (older) or "footprint" (newer)
        let tag = expr.tag();
        if tag != Some("footprint") && tag != Some("module") {
            return Err(KicadError {
                message: "Not a valid KiCAD footprint file".to_string(),
                line: None,
            });
        }

        let name = expr.get_atom(1).unwrap_or("Unknown").to_string();
        let mut footprint = Footprint::new(name);

        // Parse description
        footprint.description = expr.find("descr")
            .and_then(|e| e.get_atom(1))
            .map(|s| s.to_string());

        // Parse keywords
        footprint.keywords = expr.find("tags")
            .and_then(|e| e.get_atom(1))
            .map(|s| s.to_string());

        // Parse pads
        for pad_expr in expr.find_all("pad") {
            if let Ok(pad) = KicadPcbImporter::parse_pad(pad_expr) {
                footprint.pads.push(pad);
            }
        }

        // Parse courtyard from fp_rect or fp_poly on F.CrtYd layer
        for rect_expr in expr.find_all("fp_rect") {
            if let Some(layer) = rect_expr.find("layer").and_then(|e| e.get_atom(1)) {
                if layer.contains("CrtYd") {
                    if let (Some(start), Some(end)) = (rect_expr.find("start"), rect_expr.find("end")) {
                        let x1 = start.get_f64(1).unwrap_or(0.0);
                        let y1 = start.get_f64(2).unwrap_or(0.0);
                        let x2 = end.get_f64(1).unwrap_or(0.0);
                        let y2 = end.get_f64(2).unwrap_or(0.0);
                        footprint.courtyard_width = Some((x2 - x1).abs());
                        footprint.courtyard_height = Some((y2 - y1).abs());
                    }
                }
            }
        }

        // Parse 3D model
        footprint.model_3d = expr.find("model")
            .and_then(|e| e.get_atom(1))
            .map(|s| s.to_string());

        Ok(footprint)
    }

    /// Import multiple footprints from a library directory.
    pub fn import_library<P: AsRef<Path>>(path: P) -> KicadResult<Vec<Footprint>> {
        let path = path.as_ref();
        let mut footprints = Vec::new();

        if path.is_dir() {
            for entry in fs::read_dir(path).map_err(|e| KicadError {
                message: format!("Failed to read directory: {}", e),
                line: None,
            })? {
                let entry = entry.map_err(|e| KicadError {
                    message: format!("Failed to read entry: {}", e),
                    line: None,
                })?;
                
                let file_path = entry.path();
                if file_path.extension().map_or(false, |ext| ext == "kicad_mod") {
                    if let Ok(fp) = Self::import(&file_path) {
                        footprints.push(fp);
                    }
                }
            }
        }

        Ok(footprints)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sexpr_parser_atom() {
        let mut parser = SExprParser::new("hello");
        let result = parser.parse().unwrap();
        assert_eq!(result, SExpr::Atom("hello".to_string()));
    }

    #[test]
    fn test_sexpr_parser_string() {
        let mut parser = SExprParser::new("\"hello world\"");
        let result = parser.parse().unwrap();
        assert_eq!(result, SExpr::Atom("hello world".to_string()));
    }

    #[test]
    fn test_sexpr_parser_list() {
        let mut parser = SExprParser::new("(kicad_sch version 1)");
        let result = parser.parse().unwrap();
        assert_eq!(result.tag(), Some("kicad_sch"));
        assert_eq!(result.get_atom(1), Some("version"));
        assert_eq!(result.get_atom(2), Some("1"));
    }

    #[test]
    fn test_sexpr_parser_nested() {
        let mut parser = SExprParser::new("(symbol (at 10 20 0) (lib_id \"Device:R\"))");
        let result = parser.parse().unwrap();
        
        assert_eq!(result.tag(), Some("symbol"));
        
        let at = result.find("at").unwrap();
        assert_eq!(at.get_f64(1), Some(10.0));
        assert_eq!(at.get_f64(2), Some(20.0));
        
        let lib_id = result.find("lib_id").unwrap();
        assert_eq!(lib_id.get_atom(1), Some("Device:R"));
    }

    #[test]
    fn test_import_simple_schematic() {
        let content = r#"
(kicad_sch
  (version 20230121)
  (generator "eeschema")
  (uuid "12345678-1234-1234-1234-123456789abc")
  
  (symbol
    (lib_id "Device:R")
    (at 100 50 0)
    (uuid "aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa")
    (property "Reference" "R1")
    (property "Value" "10k")
  )
  
  (wire
    (pts (xy 90 50) (xy 100 50))
    (uuid "bbbbbbbb-bbbb-bbbb-bbbb-bbbbbbbbbbbb")
  )
  
  (label "NET1"
    (at 95 50 0)
    (uuid "cccccccc-cccc-cccc-cccc-cccccccccccc")
  )
  
  (junction
    (at 100 50)
    (uuid "dddddddd-dddd-dddd-dddd-dddddddddddd")
  )
)
"#;

        let sheet = KicadSchematicImporter::import_from_string(content).unwrap();
        
        assert_eq!(sheet.symbols.len(), 1);
        assert_eq!(sheet.symbols[0].reference, "R1");
        assert_eq!(sheet.symbols[0].value, "10k");
        
        assert_eq!(sheet.wires.len(), 1);
        assert_eq!(sheet.labels.len(), 1);
        assert_eq!(sheet.labels[0].name, "NET1");
        assert_eq!(sheet.junctions.len(), 1);
    }

    #[test]
    fn test_import_symbol_library() {
        let content = r#"
(kicad_symbol_lib
  (version 20230121)
  (generator "kicad_symbol_editor")
  
  (symbol "R"
    (property "Reference" "R")
    (property "Value" "R")
    (symbol "R_0_1"
      (pin passive line
        (at 0 3.81 270)
        (length 1.27)
        (name "~")
        (number "1")
      )
      (pin passive line
        (at 0 -3.81 90)
        (length 1.27)
        (name "~")
        (number "2")
      )
    )
  )
)
"#;

        let components = KicadSymbolLibImporter::import_from_string(content).unwrap();
        
        assert_eq!(components.len(), 1);
        assert_eq!(components[0].component_type, "R");
        assert_eq!(components[0].pins.len(), 2);
        assert_eq!(components[0].pins[0].id, "1");
        assert_eq!(components[0].pins[1].id, "2");
    }

    #[test]
    fn test_import_simple_pcb() {
        let content = r#"
(kicad_pcb
  (version 20230121)
  (generator "pcbnew")
  
  (layers
    (0 "F.Cu" signal)
    (31 "B.Cu" signal)
  )
  
  (footprint "Resistor_SMD:R_0603"
    (layer "F.Cu")
    (at 100 50 0)
    (fp_text reference "R1" (at 0 -1.5))
    (fp_text value "10k" (at 0 1.5))
    (pad "1" smd rect (at -0.8 0) (size 0.8 0.9) (net 1 "VCC"))
    (pad "2" smd rect (at 0.8 0) (size 0.8 0.9) (net 2 "GND"))
  )
  
  (segment
    (start 90 50)
    (end 99.2 50)
    (width 0.25)
    (layer "F.Cu")
    (net 1)
  )
  
  (via
    (at 95 55)
    (size 0.6)
    (drill 0.3)
    (layers "F.Cu" "B.Cu")
    (net 1)
  )
)
"#;

        let layout = KicadPcbImporter::import_from_string(content).unwrap();
        
        assert_eq!(layout.components.len(), 1);
        assert_eq!(layout.components[0].reference, "R1");
        assert_eq!(layout.components[0].value, "10k");
        assert_eq!(layout.components[0].pads.len(), 2);
        
        assert_eq!(layout.traces.len(), 1);
        assert!((layout.traces[0].width - 0.25).abs() < 0.001);
        
        assert_eq!(layout.vias.len(), 1);
        assert!((layout.vias[0].pad - 0.6).abs() < 0.001);
        assert!((layout.vias[0].drill - 0.3).abs() < 0.001);
    }

    #[test]
    fn test_import_pcb_with_zone() {
        let content = r#"
(kicad_pcb
  (version 20230121)
  (generator "pcbnew")
  
  (zone
    (net_name "GND")
    (layer "F.Cu")
    (fill yes)
    (clearance 0.3)
    (min_thickness 0.2)
    (polygon
      (pts
        (xy 0 0)
        (xy 100 0)
        (xy 100 80)
        (xy 0 80)
      )
    )
  )
)
"#;

        let layout = KicadPcbImporter::import_from_string(content).unwrap();
        
        assert_eq!(layout.zones.len(), 1);
        assert_eq!(layout.zones[0].net, "GND");
        assert_eq!(layout.zones[0].layer, "F.Cu");
        assert_eq!(layout.zones[0].points.len(), 4);
    }

    #[test]
    fn test_import_pcb_invalid_file() {
        let content = "(kicad_sch (version 1))";
        let result = KicadPcbImporter::import_from_string(content);
        assert!(result.is_err());
    }

    #[test]
    fn test_import_footprint_smd() {
        let content = r#"
(footprint "Resistor_SMD:R_0603_1608Metric"
  (version 20230121)
  (generator "pcbnew")
  (descr "Resistor SMD 0603 (1608 Metric)")
  (tags "resistor 0603")
  
  (pad "1" smd rect (at -0.8 0) (size 0.8 0.9) (layers "F.Cu" "F.Paste" "F.Mask"))
  (pad "2" smd rect (at 0.8 0) (size 0.8 0.9) (layers "F.Cu" "F.Paste" "F.Mask"))
  
  (fp_rect (start -1.5 -0.8) (end 1.5 0.8) (layer "F.CrtYd") (stroke (width 0.05)))
  
  (model "${KICAD7_3DMODEL_DIR}/Resistor_SMD.3dshapes/R_0603_1608Metric.wrl")
)
"#;

        let footprint = KicadFootprintImporter::import_from_string(content).unwrap();
        
        assert_eq!(footprint.name, "Resistor_SMD:R_0603_1608Metric");
        assert_eq!(footprint.description, Some("Resistor SMD 0603 (1608 Metric)".to_string()));
        assert_eq!(footprint.keywords, Some("resistor 0603".to_string()));
        assert_eq!(footprint.pads.len(), 2);
        assert_eq!(footprint.pads[0].number, "1");
        assert_eq!(footprint.pads[1].number, "2");
        assert!(footprint.courtyard_width.is_some());
        assert!(footprint.model_3d.is_some());
    }

    #[test]
    fn test_import_footprint_thru_hole() {
        let content = r#"
(footprint "Package_DIP:DIP-8_W7.62mm"
  (version 20230121)
  (generator "pcbnew")
  (descr "8-lead through-hole DIP package")
  (tags "DIP 8 through-hole")
  
  (pad "1" thru_hole rect (at -3.81 -3.81) (size 1.6 1.6) (drill 0.8))
  (pad "2" thru_hole circle (at -3.81 -1.27) (size 1.6 1.6) (drill 0.8))
  (pad "3" thru_hole circle (at -3.81 1.27) (size 1.6 1.6) (drill 0.8))
  (pad "4" thru_hole circle (at -3.81 3.81) (size 1.6 1.6) (drill 0.8))
  (pad "5" thru_hole circle (at 3.81 3.81) (size 1.6 1.6) (drill 0.8))
  (pad "6" thru_hole circle (at 3.81 1.27) (size 1.6 1.6) (drill 0.8))
  (pad "7" thru_hole circle (at 3.81 -1.27) (size 1.6 1.6) (drill 0.8))
  (pad "8" thru_hole circle (at 3.81 -3.81) (size 1.6 1.6) (drill 0.8))
)
"#;

        let footprint = KicadFootprintImporter::import_from_string(content).unwrap();
        
        assert_eq!(footprint.name, "Package_DIP:DIP-8_W7.62mm");
        assert_eq!(footprint.pads.len(), 8);
        assert_eq!(footprint.pads[0].pad_type, PadType::ThruHole);
        assert!((footprint.pads[0].drill - 0.8).abs() < 0.001);
    }

    #[test]
    fn test_import_footprint_invalid_file() {
        let content = "(kicad_pcb (version 1))";
        let result = KicadFootprintImporter::import_from_string(content);
        assert!(result.is_err());
    }
}

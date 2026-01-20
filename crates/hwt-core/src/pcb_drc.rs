//! PCB-specific Design Rule Check implementation.
//!
//! Implements DRC rules for PCB layouts including clearance, width, and via checks.

use crate::drc::{DrcConfig, DrcReport, DrcRule, DrcSeverity, DrcViolation};
use crate::geometry::{Point2D, Position};
use crate::layout::{Layout, Trace, Via};
use crate::units::LengthUnit;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// PCB design rules configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PcbDesignRules {
    /// Minimum track-to-track clearance (mm)
    pub min_track_clearance: f64,
    
    /// Minimum track-to-pad clearance (mm)
    pub min_track_to_pad_clearance: f64,
    
    /// Minimum track-to-via clearance (mm)
    pub min_track_to_via_clearance: f64,
    
    /// Minimum via-to-via clearance (mm)
    pub min_via_clearance: f64,
    
    /// Minimum track width (mm)
    pub min_track_width: f64,
    
    /// Minimum via diameter (mm)
    pub min_via_diameter: f64,
    
    /// Minimum via drill (mm)
    pub min_via_drill: f64,
    
    /// Minimum annular ring (mm)
    pub min_annular_ring: f64,
    
    /// Minimum hole-to-hole clearance (mm)
    pub min_hole_clearance: f64,
    
    /// Minimum copper-to-edge clearance (mm)
    pub min_edge_clearance: f64,
    
    /// Minimum silkscreen line width (mm)
    pub min_silk_width: f64,
    
    /// Minimum silkscreen text height (mm)
    pub min_silk_text_height: f64,
    
    /// Check silkscreen over pads
    pub check_silk_over_pads: bool,
    
    /// Minimum courtyard clearance (mm)
    pub min_courtyard_clearance: f64,
}

impl Default for PcbDesignRules {
    fn default() -> Self {
        Self {
            min_track_clearance: 0.2,
            min_track_to_pad_clearance: 0.2,
            min_track_to_via_clearance: 0.2,
            min_via_clearance: 0.3,
            min_track_width: 0.15,
            min_via_diameter: 0.6,
            min_via_drill: 0.3,
            min_annular_ring: 0.15,
            min_hole_clearance: 0.5,
            min_edge_clearance: 0.3,
            min_silk_width: 0.15,
            min_silk_text_height: 0.8,
            check_silk_over_pads: true,
            min_courtyard_clearance: 0.25,
        }
    }
}

impl PcbDesignRules {
    /// Create rules for JLCPCB manufacturing.
    pub fn jlcpcb() -> Self {
        Self {
            min_track_clearance: 0.127,      // 5mil
            min_track_to_pad_clearance: 0.127,
            min_track_to_via_clearance: 0.127,
            min_via_clearance: 0.254,
            min_track_width: 0.127,          // 5mil
            min_via_diameter: 0.45,
            min_via_drill: 0.2,
            min_annular_ring: 0.125,
            min_hole_clearance: 0.5,
            min_edge_clearance: 0.3,
            min_silk_width: 0.15,
            min_silk_text_height: 1.0,
            check_silk_over_pads: true,
            min_courtyard_clearance: 0.25,
        }
    }
    
    /// Create rules for OSH Park manufacturing.
    pub fn osh_park() -> Self {
        Self {
            min_track_clearance: 0.152,      // 6mil
            min_track_to_pad_clearance: 0.152,
            min_track_to_via_clearance: 0.152,
            min_via_clearance: 0.254,
            min_track_width: 0.152,          // 6mil
            min_via_diameter: 0.5,
            min_via_drill: 0.254,            // 10mil
            min_annular_ring: 0.127,
            min_hole_clearance: 0.381,
            min_edge_clearance: 0.381,
            min_silk_width: 0.152,
            min_silk_text_height: 0.8,
            check_silk_over_pads: true,
            min_courtyard_clearance: 0.25,
        }
    }
}

/// PCB DRC checker.
pub struct PcbDrcChecker<'a> {
    layout: &'a Layout,
    rules: PcbDesignRules,
}

impl<'a> PcbDrcChecker<'a> {
    /// Create a new PCB DRC checker.
    pub fn new(layout: &'a Layout, rules: PcbDesignRules) -> Self {
        Self { layout, rules }
    }
    
    /// Run all PCB DRC checks.
    pub fn check_all(&self) -> DrcReport {
        let mut report = DrcReport::new("PCB Layout", "pcb");
        
        self.check_track_widths(&mut report);
        self.check_track_clearances(&mut report);
        self.check_via_rules(&mut report);
        self.check_edge_clearances(&mut report);
        self.check_courtyard_overlaps(&mut report);
        
        report
    }
    
    /// Check minimum track widths.
    fn check_track_widths(&self, report: &mut DrcReport) {
        for trace in &self.layout.traces {
            if trace.width < self.rules.min_track_width {
                let midpoint = trace_midpoint(trace);
                report.violations.push(
                    DrcViolation::new(
                        "width.track",
                        format!("Track width {:.3}mm is below minimum {:.3}mm", 
                            trace.width, self.rules.min_track_width),
                        midpoint,
                    )
                    .with_severity(DrcSeverity::Error)
                    .with_values(trace.width, self.rules.min_track_width, "mm")
                    .with_fix(format!("Increase track width to at least {:.3}mm", 
                        self.rules.min_track_width))
                );
            }
        }
    }
    
    /// Check track-to-track clearances.
    fn check_track_clearances(&self, report: &mut DrcReport) {
        let traces = &self.layout.traces;
        
        for i in 0..traces.len() {
            for j in (i + 1)..traces.len() {
                let t1 = &traces[i];
                let t2 = &traces[j];
                
                // Skip if on different layers
                if t1.layer != t2.layer {
                    continue;
                }
                
                // Skip if same net
                if t1.net == t2.net {
                    continue;
                }
                
                // Calculate minimum distance between traces
                if let Some(clearance) = min_trace_distance(t1, t2) {
                    if clearance < self.rules.min_track_clearance {
                        let midpoint = trace_midpoint(t1);
                        report.violations.push(
                            DrcViolation::new(
                                "clearance.track_to_track",
                                format!("Track clearance {:.3}mm is below minimum {:.3}mm",
                                    clearance, self.rules.min_track_clearance),
                                midpoint,
                            )
                            .with_severity(DrcSeverity::Error)
                            .with_values(clearance, self.rules.min_track_clearance, "mm")
                            .with_fix("Increase spacing between tracks")
                        );
                    }
                }
            }
        }
    }
    
    /// Check via rules (diameter, drill, annular ring).
    fn check_via_rules(&self, report: &mut DrcReport) {
        for via in &self.layout.vias {
            // Check via diameter
            if via.pad < self.rules.min_via_diameter {
                report.violations.push(
                    DrcViolation::new(
                        "size.via_diameter",
                        format!("Via diameter {:.3}mm is below minimum {:.3}mm",
                            via.pad, self.rules.min_via_diameter),
                        position_to_point(&via.position),
                    )
                    .with_severity(DrcSeverity::Error)
                    .with_values(via.pad, self.rules.min_via_diameter, "mm")
                );
            }
            
            // Check via drill
            if via.drill < self.rules.min_via_drill {
                report.violations.push(
                    DrcViolation::new(
                        "size.via_drill",
                        format!("Via drill {:.3}mm is below minimum {:.3}mm",
                            via.drill, self.rules.min_via_drill),
                        position_to_point(&via.position),
                    )
                    .with_severity(DrcSeverity::Error)
                    .with_values(via.drill, self.rules.min_via_drill, "mm")
                );
            }
            
            // Check annular ring
            let annular_ring = (via.pad - via.drill) / 2.0;
            if annular_ring < self.rules.min_annular_ring {
                report.violations.push(
                    DrcViolation::new(
                        "size.annular_ring",
                        format!("Annular ring {:.3}mm is below minimum {:.3}mm",
                            annular_ring, self.rules.min_annular_ring),
                        position_to_point(&via.position),
                    )
                    .with_severity(DrcSeverity::Error)
                    .with_values(annular_ring, self.rules.min_annular_ring, "mm")
                    .with_fix("Increase via diameter or decrease drill size")
                );
            }
        }
        
        // Check via-to-via clearance
        let vias = &self.layout.vias;
        for i in 0..vias.len() {
            for j in (i + 1)..vias.len() {
                let v1 = &vias[i];
                let v2 = &vias[j];
                
                // Skip if same net
                if v1.net == v2.net {
                    continue;
                }
                
                let distance = position_distance(&v1.position, &v2.position);
                let edge_distance = distance - (v1.pad + v2.pad) / 2.0;
                
                if edge_distance < self.rules.min_via_clearance {
                    report.violations.push(
                        DrcViolation::new(
                            "clearance.via_to_via",
                            format!("Via clearance {:.3}mm is below minimum {:.3}mm",
                                edge_distance, self.rules.min_via_clearance),
                            position_to_point(&v1.position),
                        )
                        .with_severity(DrcSeverity::Error)
                        .with_values(edge_distance, self.rules.min_via_clearance, "mm")
                    );
                }
            }
        }
    }
    
    /// Check copper-to-edge clearances.
    fn check_edge_clearances(&self, report: &mut DrcReport) {
        if let Some(outline) = &self.layout.outline {
            if let (Some(width), Some(height)) = (outline.width, outline.height) {
                // Check traces near edges
                for trace in &self.layout.traces {
                    let start = &trace.start;
                    let end = &trace.end;
                    
                    // Check distance to each edge
                    let half_width = trace.width / 2.0;
                    
                    // Left edge
                    let left_clearance = f64::min(start.x, end.x) - half_width;
                    if left_clearance < self.rules.min_edge_clearance {
                        report.violations.push(
                            DrcViolation::new(
                                "clearance.edge",
                                format!("Track too close to board edge ({:.3}mm < {:.3}mm)",
                                    left_clearance, self.rules.min_edge_clearance),
                                trace_midpoint(trace),
                            )
                            .with_severity(DrcSeverity::Error)
                            .with_values(left_clearance, self.rules.min_edge_clearance, "mm")
                        );
                    }
                    
                    // Right edge
                    let right_clearance = width - f64::max(start.x, end.x) - half_width;
                    if right_clearance < self.rules.min_edge_clearance {
                        report.violations.push(
                            DrcViolation::new(
                                "clearance.edge",
                                format!("Track too close to board edge ({:.3}mm < {:.3}mm)",
                                    right_clearance, self.rules.min_edge_clearance),
                                trace_midpoint(trace),
                            )
                            .with_severity(DrcSeverity::Error)
                            .with_values(right_clearance, self.rules.min_edge_clearance, "mm")
                        );
                    }
                    
                    // Bottom edge
                    let bottom_clearance = f64::min(start.y, end.y) - half_width;
                    if bottom_clearance < self.rules.min_edge_clearance {
                        report.violations.push(
                            DrcViolation::new(
                                "clearance.edge",
                                format!("Track too close to board edge ({:.3}mm < {:.3}mm)",
                                    bottom_clearance, self.rules.min_edge_clearance),
                                trace_midpoint(trace),
                            )
                            .with_severity(DrcSeverity::Error)
                            .with_values(bottom_clearance, self.rules.min_edge_clearance, "mm")
                        );
                    }
                    
                    // Top edge
                    let top_clearance = height - f64::max(start.y, end.y) - half_width;
                    if top_clearance < self.rules.min_edge_clearance {
                        report.violations.push(
                            DrcViolation::new(
                                "clearance.edge",
                                format!("Track too close to board edge ({:.3}mm < {:.3}mm)",
                                    top_clearance, self.rules.min_edge_clearance),
                                trace_midpoint(trace),
                            )
                            .with_severity(DrcSeverity::Error)
                            .with_values(top_clearance, self.rules.min_edge_clearance, "mm")
                        );
                    }
                }
            }
        }
    }
    
    /// Check component courtyard overlaps.
    fn check_courtyard_overlaps(&self, report: &mut DrcReport) {
        let components = &self.layout.components;
        
        for i in 0..components.len() {
            for j in (i + 1)..components.len() {
                let c1 = &components[i];
                let c2 = &components[j];
                
                // Only check components on the same layer
                if c1.layer != c2.layer {
                    continue;
                }
                
                // Simple bounding box overlap check
                // In a real implementation, this would use actual courtyard geometry
                let distance = position_distance(&c1.position, &c2.position);
                
                // Rough check - components closer than 2mm should be verified
                if distance < self.rules.min_courtyard_clearance {
                    report.violations.push(
                        DrcViolation::new(
                            "clearance.courtyard",
                            format!("Components {} and {} may overlap (distance: {:.3}mm)",
                                c1.reference, c2.reference, distance),
                            position_to_point(&c1.position),
                        )
                        .with_severity(DrcSeverity::Warning)
                        .with_values(distance, self.rules.min_courtyard_clearance, "mm")
                    );
                }
            }
        }
    }
    
    /// Get all available PCB DRC rules.
    pub fn available_rules() -> Vec<DrcRule> {
        vec![
            DrcRule {
                id: "clearance.track_to_track".to_string(),
                name: "Track-to-Track Clearance".to_string(),
                description: "Minimum spacing between copper tracks".to_string(),
                category: "Clearance".to_string(),
                default_severity: DrcSeverity::Error,
                can_disable: false,
            },
            DrcRule {
                id: "clearance.track_to_pad".to_string(),
                name: "Track-to-Pad Clearance".to_string(),
                description: "Minimum spacing between tracks and pads".to_string(),
                category: "Clearance".to_string(),
                default_severity: DrcSeverity::Error,
                can_disable: false,
            },
            DrcRule {
                id: "clearance.via_to_via".to_string(),
                name: "Via-to-Via Clearance".to_string(),
                description: "Minimum spacing between vias".to_string(),
                category: "Clearance".to_string(),
                default_severity: DrcSeverity::Error,
                can_disable: false,
            },
            DrcRule {
                id: "clearance.edge".to_string(),
                name: "Edge Clearance".to_string(),
                description: "Minimum copper distance from board edge".to_string(),
                category: "Clearance".to_string(),
                default_severity: DrcSeverity::Error,
                can_disable: false,
            },
            DrcRule {
                id: "clearance.courtyard".to_string(),
                name: "Courtyard Clearance".to_string(),
                description: "Minimum spacing between component courtyards".to_string(),
                category: "Clearance".to_string(),
                default_severity: DrcSeverity::Warning,
                can_disable: true,
            },
            DrcRule {
                id: "width.track".to_string(),
                name: "Minimum Track Width".to_string(),
                description: "Tracks must meet minimum width requirement".to_string(),
                category: "Size".to_string(),
                default_severity: DrcSeverity::Error,
                can_disable: false,
            },
            DrcRule {
                id: "size.via_diameter".to_string(),
                name: "Minimum Via Diameter".to_string(),
                description: "Vias must meet minimum diameter requirement".to_string(),
                category: "Size".to_string(),
                default_severity: DrcSeverity::Error,
                can_disable: false,
            },
            DrcRule {
                id: "size.via_drill".to_string(),
                name: "Minimum Via Drill".to_string(),
                description: "Via drill holes must meet minimum size".to_string(),
                category: "Size".to_string(),
                default_severity: DrcSeverity::Error,
                can_disable: false,
            },
            DrcRule {
                id: "size.annular_ring".to_string(),
                name: "Minimum Annular Ring".to_string(),
                description: "Copper ring around drill holes must meet minimum".to_string(),
                category: "Size".to_string(),
                default_severity: DrcSeverity::Error,
                can_disable: false,
            },
            DrcRule {
                id: "silk.over_pads".to_string(),
                name: "Silkscreen Over Pads".to_string(),
                description: "Silkscreen should not overlap solder pads".to_string(),
                category: "Silkscreen".to_string(),
                default_severity: DrcSeverity::Warning,
                can_disable: true,
            },
        ]
    }
}

/// Convert Position to Point2D.
fn position_to_point(pos: &Position) -> Point2D {
    Point2D::new(pos.x, pos.y)
}

/// Calculate the midpoint of a trace.
fn trace_midpoint(trace: &Trace) -> Point2D {
    Point2D::new(
        (trace.start.x + trace.end.x) / 2.0,
        (trace.start.y + trace.end.y) / 2.0,
    )
}

/// Calculate distance between two positions.
fn position_distance(p1: &Position, p2: &Position) -> f64 {
    ((p2.x - p1.x).powi(2) + (p2.y - p1.y).powi(2)).sqrt()
}

/// Calculate distance between two points.
fn point_distance(p1: &Point2D, p2: &Point2D) -> f64 {
    ((p2.x - p1.x).powi(2) + (p2.y - p1.y).powi(2)).sqrt()
}

/// Calculate minimum distance between two traces (simplified).
fn min_trace_distance(t1: &Trace, t2: &Trace) -> Option<f64> {
    // Simplified: calculate distance between midpoints minus half widths
    let mid1 = trace_midpoint(t1);
    let mid2 = trace_midpoint(t2);
    let center_distance = point_distance(&mid1, &mid2);
    let edge_distance = center_distance - (t1.width + t2.width) / 2.0;
    
    if edge_distance > 0.0 {
        Some(edge_distance)
    } else {
        Some(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::ViaType;
    
    fn make_position(x: f64, y: f64) -> Position {
        Position { x, y, z: None, unit: LengthUnit::Mm }
    }
    
    fn create_test_layout() -> Layout {
        let mut layout = Layout::with_board_size(100.0, 80.0, LengthUnit::Mm);
        
        // Add some traces
        layout.traces.push(Trace {
            net: "VCC".to_string(),
            layer: "F.Cu".to_string(),
            start: make_position(10.0, 10.0),
            end: make_position(50.0, 10.0),
            width: 0.2,
            unit: LengthUnit::Mm,
        });
        
        layout.traces.push(Trace {
            net: "GND".to_string(),
            layer: "F.Cu".to_string(),
            start: make_position(10.0, 12.0),
            end: make_position(50.0, 12.0),
            width: 0.2,
            unit: LengthUnit::Mm,
        });
        
        // Add a via
        layout.vias.push(Via {
            net: "VCC".to_string(),
            position: make_position(30.0, 30.0),
            via_type: ViaType::Through,
            drill: 0.3,
            pad: 0.6,
            start_layer: Some("F.Cu".to_string()),
            end_layer: Some("B.Cu".to_string()),
            unit: LengthUnit::Mm,
        });
        
        layout
    }
    
    #[test]
    fn test_pcb_drc_clean_design() {
        let layout = create_test_layout();
        let rules = PcbDesignRules::default();
        let checker = PcbDrcChecker::new(&layout, rules);
        
        let report = checker.check_all();
        
        // Should have some clearance warnings due to close traces
        println!("Violations: {:?}", report.violations.len());
    }
    
    #[test]
    fn test_pcb_drc_track_width_violation() {
        let mut layout = Layout::new();
        
        // Add a trace that's too thin
        layout.traces.push(Trace {
            net: "SIG".to_string(),
            layer: "F.Cu".to_string(),
            start: make_position(10.0, 10.0),
            end: make_position(50.0, 10.0),
            width: 0.1, // Below minimum
            unit: LengthUnit::Mm,
        });
        
        let rules = PcbDesignRules::default();
        let checker = PcbDrcChecker::new(&layout, rules);
        let report = checker.check_all();
        
        assert!(report.violations.iter().any(|v| v.rule == "width.track"));
    }
    
    #[test]
    fn test_pcb_drc_via_violations() {
        let mut layout = Layout::new();
        
        // Add a via with small annular ring
        layout.vias.push(Via {
            net: "SIG".to_string(),
            position: make_position(30.0, 30.0),
            via_type: ViaType::Through,
            drill: 0.35,    // Large drill = small annular ring
            pad: 0.4,       // Small pad
            start_layer: Some("F.Cu".to_string()),
            end_layer: Some("B.Cu".to_string()),
            unit: LengthUnit::Mm,
        });
        
        let rules = PcbDesignRules::default();
        let checker = PcbDrcChecker::new(&layout, rules);
        let report = checker.check_all();
        
        // Should have via diameter and annular ring violations
        assert!(report.violations.iter().any(|v| v.rule == "size.via_diameter"));
        assert!(report.violations.iter().any(|v| v.rule == "size.annular_ring"));
    }
    
    #[test]
    fn test_pcb_drc_clearance_violation() {
        let mut layout = Layout::new();
        
        // Add two traces very close together
        layout.traces.push(Trace {
            net: "NET1".to_string(),
            layer: "F.Cu".to_string(),
            start: make_position(10.0, 10.0),
            end: make_position(50.0, 10.0),
            width: 0.2,
            unit: LengthUnit::Mm,
        });
        
        layout.traces.push(Trace {
            net: "NET2".to_string(),
            layer: "F.Cu".to_string(),
            start: make_position(10.0, 10.3), // Only 0.3mm apart, minus widths = 0.1mm clearance
            end: make_position(50.0, 10.3),
            width: 0.2,
            unit: LengthUnit::Mm,
        });
        
        let rules = PcbDesignRules::default();
        let checker = PcbDrcChecker::new(&layout, rules);
        let report = checker.check_all();
        
        assert!(report.violations.iter().any(|v| v.rule == "clearance.track_to_track"));
    }
    
    #[test]
    fn test_pcb_drc_jlcpcb_rules() {
        let rules = PcbDesignRules::jlcpcb();
        
        assert_eq!(rules.min_track_width, 0.127);
        assert_eq!(rules.min_via_drill, 0.2);
    }
    
    #[test]
    fn test_available_rules() {
        let rules = PcbDrcChecker::available_rules();
        
        assert!(rules.len() >= 9);
        assert!(rules.iter().any(|r| r.id == "clearance.track_to_track"));
        assert!(rules.iter().any(|r| r.id == "width.track"));
        assert!(rules.iter().any(|r| r.id == "size.annular_ring"));
    }
}

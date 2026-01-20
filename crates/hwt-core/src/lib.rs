//! # hwt-core
//!
//! Core data model and Circuit JSON IR for Hardware Tool.
//!
//! This crate provides the universal data structures for representing
//! hardware designs across all domains: PCB, IC, Quantum, MEMS, RF, and Packaging.

pub mod bom;
pub mod circuit;
pub mod pnp;
pub mod command;
pub mod component;
pub mod constraint;
pub mod domain;
pub mod drc;
pub mod erc;
pub mod pcb_drc;
pub mod geometry;
pub mod gerber;
pub mod io;
pub mod kicad;
pub mod layout;
pub mod library;
pub mod net;
pub mod netclass;
pub mod programmatic;
pub mod project;
pub mod routing;
pub mod spice;
pub mod schematic;
pub mod sync;
pub mod units;

pub use circuit::CircuitJson;
pub use component::Component;
pub use constraint::Constraint;
pub use domain::HardwareDomain;
pub use geometry::{BoundingBox, Point2D, Point3D, Position};
pub use io::{load_file, load_pcb, load_project, load_schematic, save_pcb, save_project, save_schematic, FileContent, IoError, IoResult, RecentFiles};
pub use layout::Layout;
pub use net::Net;
pub use project::Project;
pub use units::{AngleUnit, FrequencyUnit, LengthUnit};

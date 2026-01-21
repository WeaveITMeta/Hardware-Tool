//! Hardware Tool UI Application
//!
//! Main entry point for the Hardware Tool desktop application.

use hwt_core::HardwareDomain;
use slint::Model;
use std::cell::RefCell;
use std::rc::Rc;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    tracing::info!("Starting Hardware Tool...");

    // Create the main window
    let main_window = MainWindow::new()?;

    // Set initial state
    main_window.set_project_name("Untitled".into());
    main_window.set_current_domain(0); // PCB
    main_window.set_current_view(0); // Schematic

    // Schematic editor state
    let symbol_counter = Rc::new(RefCell::new(1u32));
    let wire_counter = Rc::new(RefCell::new(1u32));

    // Connect domain/view callbacks
    main_window.on_domain_changed(move |domain| {
        let domain_enum = match domain {
            0 => HardwareDomain::Pcb,
            1 => HardwareDomain::Ic,
            2 => HardwareDomain::Quantum,
            3 => HardwareDomain::Mems,
            4 => HardwareDomain::Rf,
            5 => HardwareDomain::Packaging,
            _ => HardwareDomain::Pcb,
        };
        tracing::info!("Domain changed to: {}", domain_enum.display_name());
    });

    main_window.on_view_changed(move |view| {
        let view_name = match view {
            0 => "Schematic",
            1 => "Layout",
            2 => "3D",
            3 => "Code",
            _ => "Unknown",
        };
        tracing::info!("View changed to: {}", view_name);
    });

    main_window.on_save_requested(|| {
        tracing::info!("Save requested");
    });

    main_window.on_undo_requested(|| {
        tracing::info!("Undo requested");
    });

    main_window.on_redo_requested(|| {
        tracing::info!("Redo requested");
    });

    main_window.on_search_requested(|| {
        tracing::info!("Search requested (Ctrl+K)");
    });

    // Schematic editor callbacks
    main_window.on_schematic_tool_changed(|tool| {
        tracing::info!("Schematic tool changed to: {:?}", tool);
    });

    let window_weak = main_window.as_weak();
    let counter = symbol_counter.clone();
    main_window.on_symbol_placed(move |x, y, rotation| {
        tracing::info!("Symbol placed at ({}, {}) rotation: {}", x, y, rotation);
        
        if let Some(window) = window_weak.upgrade() {
            let mut symbols: Vec<PlacedSymbolView> = window.get_placed_symbols().iter().collect();
            let num = *counter.borrow();
            *counter.borrow_mut() += 1;
            
            symbols.push(PlacedSymbolView {
                id: format!("sym_{}", num).into(),
                reference: format!("U{}", num).into(),
                value: "Component".into(),
                x,
                y,
                rotation: rotation,
                mirror_x: false,
                mirror_y: false,
                selected: false,
            });
            
            let model = Rc::new(slint::VecModel::from(symbols));
            window.set_placed_symbols(model.into());
        }
    });

    main_window.on_wire_started(|x, y| {
        tracing::info!("Wire started at ({}, {})", x, y);
    });

    let window_weak = main_window.as_weak();
    let counter = wire_counter.clone();
    main_window.on_wire_segment_added(move |sx, sy, ex, ey| {
        tracing::info!("Wire segment: ({}, {}) -> ({}, {})", sx, sy, ex, ey);
        
        if let Some(window) = window_weak.upgrade() {
            let mut wires: Vec<WireSegmentView> = window.get_placed_wires().iter().collect();
            let num = *counter.borrow();
            *counter.borrow_mut() += 1;
            
            wires.push(WireSegmentView {
                id: format!("wire_{}", num).into(),
                start_x: sx,
                start_y: sy,
                end_x: ex,
                end_y: ey,
                selected: false,
            });
            
            let model = Rc::new(slint::VecModel::from(wires));
            window.set_placed_wires(model.into());
        }
    });

    main_window.on_wire_completed(|| {
        tracing::info!("Wire completed");
    });

    let window_weak = main_window.as_weak();
    main_window.on_junction_placed(move |x, y| {
        tracing::info!("Junction placed at ({}, {})", x, y);
        
        if let Some(window) = window_weak.upgrade() {
            let mut junctions: Vec<JunctionView> = window.get_placed_junctions().iter().collect();
            junctions.push(JunctionView {
                id: format!("junc_{}", junctions.len() + 1).into(),
                x,
                y,
                selected: false,
            });
            
            let model = Rc::new(slint::VecModel::from(junctions));
            window.set_placed_junctions(model.into());
        }
    });

    let window_weak = main_window.as_weak();
    main_window.on_label_placed(move |x, y| {
        tracing::info!("Label placed at ({}, {})", x, y);
        
        if let Some(window) = window_weak.upgrade() {
            let mut labels: Vec<NetLabelView> = window.get_placed_labels().iter().collect();
            labels.push(NetLabelView {
                id: format!("label_{}", labels.len() + 1).into(),
                name: format!("NET{}", labels.len() + 1).into(),
                x,
                y,
                rotation: 0,
                selected: false,
            });
            
            let model = Rc::new(slint::VecModel::from(labels));
            window.set_placed_labels(model.into());
        }
    });

    main_window.on_no_connect_placed(|x, y| {
        tracing::info!("No-connect placed at ({}, {})", x, y);
    });

    main_window.on_element_selected(|id| {
        tracing::info!("Element selected: {}", id);
    });

    main_window.on_selection_cleared(|| {
        tracing::debug!("Selection cleared");
    });

    // Quick-add symbol callbacks
    let window_weak = main_window.as_weak();
    main_window.on_quick_add_resistor(move || {
        tracing::info!("Quick-add: Resistor (R)");
        if let Some(window) = window_weak.upgrade() {
            window.set_placing_symbol(SymbolData {
                library: "Device".into(),
                name: "R".into(),
                reference_prefix: "R".into(),
                value: "10k".into(),
            });
            window.set_is_placing_symbol(true);
            window.set_schematic_tool(EditorTool::PlaceSymbol);
        }
    });

    let window_weak = main_window.as_weak();
    main_window.on_quick_add_capacitor(move || {
        tracing::info!("Quick-add: Capacitor (C)");
        if let Some(window) = window_weak.upgrade() {
            window.set_placing_symbol(SymbolData {
                library: "Device".into(),
                name: "C".into(),
                reference_prefix: "C".into(),
                value: "100nF".into(),
            });
            window.set_is_placing_symbol(true);
            window.set_schematic_tool(EditorTool::PlaceSymbol);
        }
    });

    let window_weak = main_window.as_weak();
    main_window.on_quick_add_inductor(move || {
        tracing::info!("Quick-add: Inductor (L)");
        if let Some(window) = window_weak.upgrade() {
            window.set_placing_symbol(SymbolData {
                library: "Device".into(),
                name: "L".into(),
                reference_prefix: "L".into(),
                value: "10uH".into(),
            });
            window.set_is_placing_symbol(true);
            window.set_schematic_tool(EditorTool::PlaceSymbol);
        }
    });

    let window_weak = main_window.as_weak();
    main_window.on_quick_add_ground(move || {
        tracing::info!("Quick-add: Ground (G)");
        if let Some(window) = window_weak.upgrade() {
            window.set_placing_symbol(SymbolData {
                library: "Power".into(),
                name: "GND".into(),
                reference_prefix: "#PWR".into(),
                value: "GND".into(),
            });
            window.set_is_placing_symbol(true);
            window.set_schematic_tool(EditorTool::PlaceSymbol);
        }
    });

    let window_weak = main_window.as_weak();
    main_window.on_quick_add_vcc(move || {
        tracing::info!("Quick-add: VCC (V)");
        if let Some(window) = window_weak.upgrade() {
            window.set_placing_symbol(SymbolData {
                library: "Power".into(),
                name: "VCC".into(),
                reference_prefix: "#PWR".into(),
                value: "VCC".into(),
            });
            window.set_is_placing_symbol(true);
            window.set_schematic_tool(EditorTool::PlaceSymbol);
        }
    });

    main_window.on_rotate_placement(|| {
        tracing::info!("Rotate placement 90°");
    });

    main_window.on_delete_selected(|| {
        tracing::info!("Delete selected elements");
    });

    tracing::info!("Hardware Tool ready!");

    // Run the event loop
    main_window.run()
}

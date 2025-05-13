use disassembler::disassembler::disassemble;
use glium::Display;
use glutin::surface::WindowSurface;
use imgui::Ui;
use imgui_glium_renderer::Renderer;

use ps::cpu::{
    instruction::{self, JTypeOperation, Operation},
    mipsr3000,
    utils::register_name,
};

use crate::emulator::Emulator;

pub struct DebuggerUI {
    breakpoint_address_input: String,
    cpu_window: bool,
    code_window: bool,
    memory_address_input: String,
    memory_base_address: u32,
    memory_view_size: usize,
    watch_window: bool,
    watch_entries: Vec<WatchEntry>,
    new_watch_address: String,
    new_watch_name: String,
    selected_format: usize,
}

impl DebuggerUI {
    pub fn new(renderer: &mut Renderer, display: &mut Display<WindowSurface>) -> Self {
        // Create default watch entries for IRQ registers
        let mut watch_entries = Vec::new();

        // Add default IRQ registers
        watch_entries.push(WatchEntry {
            address: 0x1f801070,
            name: String::from("I_STAT"),
            format: WatchFormat::Hex,
        });

        watch_entries.push(WatchEntry {
            address: 0x1f801074,
            name: String::from("I_MASK"),
            format: WatchFormat::Hex,
        });

        Self {
            cpu_window: true,
            code_window: false,
            breakpoint_address_input: String::with_capacity(32),
            memory_address_input: String::with_capacity(32),
            memory_base_address: 0x0000_0000,
            memory_view_size: 256,
            watch_window: true,
            watch_entries,
            new_watch_address: String::with_capacity(32),
            new_watch_name: String::with_capacity(32),
            selected_format: 0, // Default to Hex
        }
    }

    pub fn render_ui(&mut self, emu: &mut Emulator, ui: &Ui, renderer: &mut Renderer) {
        self.cpu_self_window(ui, emu);
        self.control_window(ui, emu);
        self.code_window(ui, emu);
        self.memory_window(ui, emu);
        //self.watch_window(ui, emu);
    }
    fn control_window(&self, ui: &Ui, emu: &mut Emulator) {
        ui.window("Controls")
            .size([300.0, 200.0], imgui::Condition::FirstUseEver)
            .position([10.0, 10.0], imgui::Condition::FirstUseEver)
            .build(|| {
                let ps = &mut emu.ps;
                if ui.button("Step") {
                    mipsr3000::run_instruction(ps);
                }
                if ui.button(if emu.running { "Pause" } else { "Run" }) {
                    emu.running = !emu.running;
                }

                if ui.button("Reset") {
                    emu.ps.cpu.pc = 0xbfc00000;
                    emu.ps.cpu.next_pc = 0xbfc00004
                    //TODO move the reset function to ps. with cop0 handling and shit
                }
            });
    }

    fn cpu_self_window(&self, ui: &Ui, emu: &mut Emulator) {
        let style = ui.push_style_var(imgui::StyleVar::ItemInnerSpacing([0.0, 0.0]));

        let register_label = |value: u32, name: &str| {
            ui.text(&format!("{:<10}{}", format!("{}:", name), format!("{:#010X}", value)));
        };

        ui.window("CPU")
            .size([360.0, 400.0], imgui::Condition::FirstUseEver)
            .position([1590.0, 10.0], imgui::Condition::FirstUseEver)
            .build(|| {
                let cpu = &emu.ps.cpu;

                register_label(cpu.pc, "PC");
                ui.separator();
                for i in 0..32 {
                    register_label(cpu.registers[i], &register_name(i as u8));
                    if i % 2 == 0 {
                        ui.same_line();
                    }
                }
                ui.separator();
                register_label(cpu.hi, "hi");
                ui.same_line();
                register_label(cpu.lo, "lo");
            });
    }

    fn toggle_breakpoint(&self, address: u32, emu: &mut Emulator) {
        // Check if the address is already a breakpoint using Emulator methods
        if emu.breakpoints.contains(&address) {
            emu.remove_breakpoint(address); // Remove breakpoint if it exists
            println!("Breakpoint removed at: {:#08X}", address);
        } else {
            emu.add_breakpoint(address); // Add breakpoint if it doesn't exist
            println!("Breakpoint added at: {:#08X}", address);
        }
    }

    pub fn code_window(&mut self, ui: &Ui, emu: &mut Emulator) {
        let code_line = |pc: u32, ins_bin: u32, ins_disassembled: &str| {
            format!(
                "{}{:^20}{}",
                format!("{:#010X}", pc),
                format!("{:#010X}", ins_bin),
                format!("{}", ins_disassembled)
            )
        };

        ui.window("Code")
            .size([500.0, 350.0], imgui::Condition::FirstUseEver)
            .position_pivot([0.5, 0.5])
            .build(|| {
                // Add breakpoint button and modal with unique ID
                if ui.button("Add Breakpoint##code_window_add_breakpoint_button") {
                    ui.open_popup("Add Breakpoint##code_window_add_breakpoint_popup");
                }

                // Modal popup for adding breakpoints with unique ID
                ui.modal_popup_config("Add Breakpoint##code_window_add_breakpoint_popup").build(
                    || {
                        ui.text("Enter breakpoint address (in hex):");

                        // Input field with unique identifier
                        ui.input_text(
                            "Breakpoint Address##code_window_breakpoint_input",
                            &mut self.breakpoint_address_input,
                        )
                        .chars_hexadecimal(true) // Limit input to valid hex characters
                        .build();

                        if ui.button("Add##code_window_add_button") {
                            let parsed_input = self.breakpoint_address_input.trim();
                            if let Ok(address) = u32::from_str_radix(parsed_input, 16) {
                                emu.add_breakpoint(address); // Use Emulator's add_breakpoint method
                                println!("Breakpoint added at: {:#08X}", address);
                            } else {
                                println!("Invalid address: {}", parsed_input);
                            }

                            self.breakpoint_address_input.clear(); // Clear input after adding
                            ui.close_current_popup();
                        }

                        if ui.button("Cancel##code_window_cancel_button") {
                            self.breakpoint_address_input.clear(); // Clear input on cancel
                            ui.close_current_popup();
                        }
                    },
                );

                // Display the code lines with click-to-toggle breakpoints
                for i in 0..31 {
                    let pc = emu.ps.cpu.pc + i * 4;
                    let ins = instruction::Instruction(emu.ps.read32(pc));
                    let disassembled = disassemble(&ins);

                    // Check if there is already a breakpoint at this address
                    let is_breakpoint = emu.breakpoints.contains(&pc);

                    // Push a unique ID for each selectable item
                    let _id_token = ui.push_id(pc.to_string());
                    // Now, within this scope, the ID is pushed

                    if ui.selectable(&code_line(pc, ins.0, &disassembled)) {
                        // Toggle the breakpoint when the line is clicked
                        self.toggle_breakpoint(pc, emu);
                    }

                    // Highlight the line if it has a breakpoint
                    if is_breakpoint {
                        ui.same_line();
                        ui.text_colored([1.0, 0.0, 0.0, 1.0], " (Breakpoint)");
                    }
                    // _id_token goes out of scope here, and the ID is popped
                }
            });
    }

    fn memory_window(&mut self, ui: &Ui, emu: &mut Emulator) {
        ui.window("Memory").size([600.0, 400.0], imgui::Condition::FirstUseEver).build(|| {
            ui.text("Memory Viewer");

            // Address input field with a unique identifier
            ui.input_text("Address##memory", &mut self.memory_address_input)
                .chars_hexadecimal(true)
                .auto_select_all(true)
                .build();

            ui.same_line();
            if ui.button("Go##memory_go") {
                if let Ok(address) =
                    u32::from_str_radix(self.memory_address_input.trim_start_matches("0x"), 16)
                {
                    self.memory_base_address = address;
                } else {
                    println!("Invalid address: {}", self.memory_address_input);
                }
            }

            // Navigation buttons with unique identifiers
            if ui.button("<<##memory_prev") {
                self.memory_base_address = self.memory_base_address.saturating_sub(256);
            }
            ui.same_line();
            if ui.button(">>##memory_next") {
                self.memory_base_address = self.memory_base_address.saturating_add(256);
            }

            ui.separator();

            // Memory display area
            ui.child_window("MemoryView").size([0.0, 0.0]).horizontal_scrollbar(true).build(|| {
                self.display_memory(ui, emu);
            });
        });
    }

    fn display_memory(&self, ui: &Ui, emu: &Emulator) {
        let base_addr = self.memory_base_address;
        let num_columns = 16;
        let num_rows = self.memory_view_size / num_columns;

        for row in 0..num_rows {
            let addr = base_addr.wrapping_add((row * num_columns) as u32);
            let mut bytes = [0u8; 16];

            for i in 0..num_columns {
                bytes[i] = emu.ps.read8(addr.wrapping_add(i as u32));
            }

            // Address
            ui.text(format!("{:08X}:", addr));
            ui.same_line();

            // Hex bytes
            for &byte in &bytes {
                ui.text(format!("{:02X} ", byte));
                ui.same_line();
            }

            // ASCII representation
            let ascii: String = bytes
                .iter()
                .map(|&b| if b.is_ascii_graphic() || b == b' ' { b as char } else { '.' })
                .collect();
            ui.text(format!("|{}|", ascii));
        }
    }

  /*  fn watch_window(&mut self, ui: &Ui, emu: &mut Emulator) {
        ui.window("Watch Window")
            .size([400.0, 300.0], imgui::Condition::FirstUseEver)
            .position([1190.0, 420.0], imgui::Condition::FirstUseEver)
            .build(|| {
                // Format options
                let formats = ["Hex", "Decimal", "Binary"];

                // Add new watch section
                ui.text("Add new watch:");
                ui.input_text("Address##watch_addr", &mut self.new_watch_address)
                    .chars_hexadecimal(true)
                    .build();

                ui.same_line();
                ui.input_text("Name##watch_name", &mut self.new_watch_name).build();

                ui.same_line();
                ui.combo(
                    "Format##watch_format",
                    &mut self.selected_format,
                    &formats,
                    |s| std::borrow::Cow::from(*s)
                );

                ui.same_line();
                if ui.button("Add##watch_add") {
                    if let Ok(address) = u32::from_str_radix(
                        self.new_watch_address.trim_start_matches("0x"), 16
                    ) {
                        let name = if self.new_watch_name.is_empty() {
                            format!("Watch {:#010X}", address)
                        } else {
                            self.new_watch_name.clone()
                        };

                        let format = match self.selected_format {
                            0 => WatchFormat::Hex,
                            1 => WatchFormat::Decimal,
                            2 => WatchFormat::Binary,
                            _ => WatchFormat::Hex,
                        };

                        self.watch_entries.push(WatchEntry {
                            address,
                            name,
                            format,
                        });

                        self.new_watch_address.clear();
                        self.new_watch_name.clear();
                    }
                }

                ui.separator();

                // Display current watches
                if ui.button("Refresh All") {
                    // Just a visual refresh button, actual refresh happens every frame
                }

                // Table header
                ui.columns(3, "watch_table", true);
                ui.text("Name");
                ui.next_column();
                ui.text("Address");
                ui.next_column();
                ui.text("Value");
                ui.next_column();

                // Table entries with unique IDs
                let mut entry_to_remove = None;

                for (i, entry) in self.watch_entries.iter_mut().enumerate() {
                    // Push a unique ID for this row
                    let _id_token = ui.push_id(i.to_string());

                    ui.text(&entry.name);
                    ui.next_column();

                    ui.text(format!("{:#010X}", entry.address));
                    ui.next_column();

                    // Read the current value
                    let value = emu.ps.read_word(entry.address);
                    ui.text(entry.format.format_value(value));

                    ui.same_line();
                    if ui.small_button("×") {
                        entry_to_remove = Some(i);
                    }

                    ui.next_column();
                }

                // Remove entry if needed (outside of iteration)
                if let Some(index) = entry_to_remove {
                    self.watch_entries.remove(index);
                }

                ui.columns(1, "", false);

                // Additional contextual information for IRQ registers
                for entry in &self.watch_entries {
                    if entry.name == "I_STAT" || entry.name == "I_MASK" {
                        let value = emu.ps.read_word(entry.address);
                        self.display_irq_info(ui, entry.name.as_str(), value);
                    }
                }
            });
    }*/

    fn display_irq_info(&self, ui: &Ui, register_name: &str, value: u32) {
        if register_name == "I_STAT" || register_name == "I_MASK" {
            ui.separator();
            ui.text(format!("{} Details:", register_name));

            // PlayStation interrupt names
            let interrupt_names = [
                "VBLANK", "GPU", "CDROM", "DMA",
                "TMR0", "TMR1", "TMR2", "Controller/Mem Card",
                "SIO", "SPU", "PIO", "Reserved"
            ];

            // Only show active bits
            for i in 0..interrupt_names.len() {
                if (value & (1 << i)) != 0 {
                    let color = [0.0, 1.0, 0.0, 1.0]; // Green for active bits
                    ui.text_colored(color, format!("Bit {}: {}", i, interrupt_names[i]));
                }
            }
        }
    }

}



struct WatchEntry {
    address: u32,
    name: String,
    format: WatchFormat,
}

// 2. Define watch value format
enum WatchFormat {
    Hex,
    Decimal,
    Binary,
}

impl WatchFormat {
    fn format_value(&self, value: u32) -> String {
        match self {
            WatchFormat::Hex => format!("{:#010X}", value),
            WatchFormat::Decimal => format!("{}", value),
            WatchFormat::Binary => format!("{:#034b}", value),
        }
    }
}

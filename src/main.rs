use std::fs::File;
use std::io::prelude::*;
use std::io::Read;

extern crate svd_parser as svd;

fn get_methods(access: Option<svd::Access>) -> (bool, bool) {
    let mut r = true;
    let mut w = true;

    if let Some(access) = access {
        match access {
            svd::Access::ReadOnly => { w = false; },
            svd::Access::ReadWrite => { },
            svd::Access::ReadWriteOnce => { },
            svd::Access::WriteOnce => { r = false; },
            svd::Access::WriteOnly => { r = false; },
        }
    }
    (r, w)
}

fn generate(peripheral: &Vec<svd::peripheral::Peripheral>) {
    let mut intterupts_list: Vec<svd::interrupt::Interrupt> = Vec::new();

    for periph in peripheral {
        let mut p = periph.clone();

        for interrupt in &p.interrupt {
            intterupts_list.push(interrupt.clone()); 
        }

        if let Some(derived_name) = &periph.derived_from {
            println!("Peripheral name: {}, derived from {}", periph.name, derived_name);

            for new_p in peripheral {
                if &new_p.name == derived_name {
                    p = periph.derive_from(new_p);
                    break;
                }
            }

        } else {
            println!("Peripheral name: {}", p.name);
        }

        let mut content = String::new();

        if let Some(registers) = &p.registers {
            for reg in registers {
                if let svd::RegisterCluster::Register(r) = reg {
                    let reg_name = r.name.to_lowercase();
                    content += (format!("pub mod {} {{\r\n", reg_name)).as_str();

                    if let Some(fields) = &r.fields {
                        for f in fields {
                            let field_name = f.name.to_lowercase();
                            content += (format!("    pub mod {} {{\r\n", field_name)).as_str();
                            
                            let (rd, wr) = get_methods(f.access);
                            let reg_addr = p.base_address + r.address_offset;
                            let mask = ((1u64 << f.bit_range.width) - 1) as u32;

                            if rd == true {
                                // GET
                                content +=          "        pub fn get() -> u32 {\r\n";
                                content +=          "            unsafe {\r\n";
                                
                                let read_str = format!("core::ptr::read_volatile(0x{:X}u32 as *const u32)", reg_addr);

                                //content += (format!("                let mut reg = core::ptr::read_volatile(0x{:X}u32 as *const u32);\r\n", reg_addr)).as_str();

                                if f.bit_range.offset == 0 {
                                    content += (format!("                {} & 0x{:X}\r\n", read_str, mask)).as_str();
                                } else {
                                    content += (format!("                ({} >> {}) & 0x{:X}\r\n", read_str, f.bit_range.offset, mask)).as_str();
                                }

                                content +=          "            }\r\n";
                                content +=          "        }\r\n\r\n";
                            }

                            if wr == true {
                                // SET
                                content +=          "        pub fn set(val: u32) {\r\n";
                                content +=          "            unsafe {\r\n";

                                content += (format!("                let mut reg = core::ptr::read_volatile(0x{:X}u32 as *const u32);\r\n", reg_addr)).as_str();
                                content += (format!("                reg &= 0x{:X}u32;\r\n", !mask)).as_str();

                                if f.bit_range.offset == 0 {
                                    content += (format!("                reg |= val & 0x{:X};\r\n", mask)).as_str();
                                } else {
                                    content += (format!("                reg |= (val & 0x{:X}) << {};\r\n", mask, f.bit_range.offset)).as_str();
                                }

                                content += (format!("                core::ptr::write_volatile(0x{:X}u32 as *mut u32, reg);\r\n", reg_addr)).as_str();
                                content +=          "            }\r\n";
                                content +=          "        }\r\n";
                            }

                            content += "    }\r\n";
                        }
                    }

                    content += "}\r\n\r\n";
                }
            }
        }

        let path = p.name.to_lowercase() + ".rs";
        let mut file = File::create(path).unwrap();
        file.write(content.as_bytes()).unwrap();
    }

    let mut file = File::create("startup.rs").unwrap();

    let mut max_interrupt_number = 0u32;

    for intterupt in &intterupts_list {
        if intterupt.value > max_interrupt_number {
            max_interrupt_number = intterupt.value;
        }
    }

    for i in 0..max_interrupt_number {
        let mut not_found = true;

        for intterupt in &intterupts_list {
            if intterupt.value == i {
                writeln!(&mut file, "    {} {}", intterupt.name, intterupt.value).unwrap();
                not_found = false;
                break;
            }
        }

        if not_found == true {
            writeln!(&mut file, "    - {}", i).unwrap();
        }
    }
}

fn main() {
    let xml = &mut String::new();
    File::open("C:/stay-on-main/svd/STM32F103.svd").unwrap().read_to_string(xml).unwrap();

    let f = svd::parse(xml).unwrap();
    generate(&f.peripherals);
    println!("name: {}", f.name);
}

// cortex-m0 or cortex-m0+
// 0 Initial SP value
// 1 Reset
// 2 NMI
// 3 HardFault
// 4 reserved
// 5 reserved
// 6 reserved
// 7 reserved
// 8 reserved
// 9 reserved
// 10 reserved
// 11 SVCall
// 12 reserved
// 13 reserved
// 14 PendSV
// 15 SysTick, if implemented
// ... Device specific ...


// cortex-m3 or cortex-m4
// 0 Initial SP value
// 1 Reset
// 2 NMI
// 3 HardFault
// 4 Memory managment fault
// 5 Bus fault
// 6 Usage fault
// 7 reserved
// 8 reserved
// 9 reserved
// 10 reserved
// 11 SVCall
// 12 Reserved for Debug
// 13 reserved
// 14 PendSV
// 15 SysTick
// ... Device specific ...

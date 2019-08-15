use std::fs::File;
use std::io::prelude::*;
use xmltree::Element;

#[derive(Clone)]
struct Field
{
    name: String,
    description: String,
    bit_offset: String,
    bit_width: String,
}

#[derive(Clone)]
struct Register
{
    name: String,
    description: String,
    address_offset: String,
    //access: String,
    reset_value: String,
    //reset_mask: String,
    fields: Vec<Field>,
}

#[derive(Clone)]
struct Perepheral
{
    name: String,
    description: String,
    group_name: String,
    base_address: String,
    registers: Vec<Register>,
}

fn fields_get(mut field: Element) -> Field
{
    let f = Field {
        name: field.take_child("name").unwrap().text.unwrap(),
        description: field.take_child("description").unwrap().text.unwrap(),
        bit_offset: field.take_child("bitOffset").unwrap().text.unwrap(),
        bit_width: field.take_child("bitWidth").unwrap().text.unwrap(),
    };

    //println!("Field: {}", f.name);

    f
}

fn registers_get(mut register: Element) -> Register
{
    let mut r = Register {
        name: register.take_child("name").unwrap().text.unwrap(),
        description: register.take_child("description").unwrap().text.unwrap(),
        address_offset: register.take_child("addressOffset").unwrap().text.unwrap(),
        //access: register.take_child("access").unwrap().text.unwrap(),
        reset_value: register.take_child("resetValue").unwrap().text.unwrap(),
        //reset_mask: register.take_child("resetMask").unwrap().text.unwrap(),
        fields: Vec::new(),
    };

    //println!("Register: {}", r.name);

    match register.take_child("fields") {
        Some(mut fields) => {
            loop {
                match fields.take_child("field") {
                    Some(field) => {
                        r.fields.push(fields_get(field));
                    },
                    None => { break },
                }
            }
        },
        None => { },
    };

    r
}

fn name_from_file(word: &String) -> String {
    let (first, last) = word.split_at(1);
    first.to_string() + last.to_string().to_lowercase().as_str()
}

fn generate(p: &Perepheral) {
    let path = p.name.to_lowercase() + ".rs";
    let mut file = File::create(path).unwrap();

    for r in &p.registers {
        writeln!(&mut file, "pub struct {} {{", name_from_file(&r.name)).unwrap();
        writeln!(&mut file, "   raw: u32,").unwrap();
        writeln!(&mut file, "}}").unwrap();
        writeln!(&mut file, "").unwrap();
        
        if r.fields.len() > 0 {
            writeln!(&mut file, "impl {} {{", name_from_file(&r.name)).unwrap();

            for f in &r.fields {
                writeln!(&mut file, "    #[inline(always)]").unwrap();
                writeln!(&mut file, "    pub fn {}_get(&self) -> u32 {{", f.name.to_lowercase()).unwrap();
                writeln!(&mut file, "        (self.raw >> {}) & ((1 << {}) - 1)", f.bit_offset, f.bit_width).unwrap();
                writeln!(&mut file, "    }}", ).unwrap();
                writeln!(&mut file, "", ).unwrap();

                writeln!(&mut file, "    #[inline(always)]").unwrap();
                writeln!(&mut file, "    pub fn {}_set(&mut self, val: u32) {{", f.name.to_lowercase()).unwrap();
                writeln!(&mut file, "        self.raw = (self.raw & !(((1 << {}) - 1) << {})) | ((val & ((1 << {}) - 1)) << {})", f.bit_width, f.bit_offset, f.bit_width, f.bit_offset).unwrap();
                writeln!(&mut file, "    }}", ).unwrap();
                writeln!(&mut file, "", ).unwrap();
            }

            writeln!(&mut file, "}}").unwrap();
            writeln!(&mut file, "").unwrap();
        } 

        writeln!(&mut file, "pub mod {} {{", r.name.to_lowercase()).unwrap();
        writeln!(&mut file, "    #[inline(always)]").unwrap();
        writeln!(&mut file, "    pub fn read() -> super::{} {{", name_from_file(&r.name)).unwrap();
        writeln!(&mut file, "        super::{} {{", name_from_file(&r.name)).unwrap();
        writeln!(&mut file, "            raw: unsafe {{ *(({} + {}) as *const u32) }}", p.base_address, r.address_offset).unwrap();
        writeln!(&mut file, "        }}").unwrap();
        writeln!(&mut file, "    }}").unwrap();
        writeln!(&mut file, "").unwrap();
        writeln!(&mut file, "    #[inline(always)]").unwrap();
        writeln!(&mut file, "    pub fn write(val: & super::{}) {{", name_from_file(&r.name)).unwrap();
        writeln!(&mut file, "       unsafe {{ *(({} + {}) as *mut u32) = val.raw; }}", p.base_address, r.address_offset).unwrap();
        writeln!(&mut file, "    }}").unwrap();
        writeln!(&mut file, "}}").unwrap();
        writeln!(&mut file, "").unwrap();
    }
}

fn peripheral_get(mut peripheral: Element) -> Perepheral
{
    let mut p = Perepheral {
        name: peripheral.take_child("name").unwrap().text.unwrap(),
        description: peripheral.take_child("description").unwrap().text.unwrap(),
        group_name: peripheral.take_child("groupName").unwrap().text.unwrap(),
        base_address: peripheral.take_child("baseAddress").unwrap().text.unwrap(),
        registers: Vec::new(),
    };

    println!("Perepheral: {}", p.name);

    let mut registers = peripheral.take_child("registers").expect("Can't find name element");

    loop {
        match registers.take_child("register") {
            Some(register) => {
                p.registers.push(registers_get(register));
            },
            None => { break },
        }
    }

    p
}


fn main() {
    let mut file = File::open("D:/stay-on-main/svd/STM32F103.svd").unwrap();
    let mut contents = vec![];
    file.read_to_end(&mut contents).unwrap();

    let mut names_element = Element::parse(contents.as_slice()).unwrap();

    let mut peripherals = names_element.take_child("peripherals").expect("Can't find name element");

    let mut ph: Vec<Perepheral> = Vec::new();

    loop {
        match peripherals.take_child("peripheral") {
            Some(mut peripheral) => {
                let mut derived = false;

                for (attribute, value) in &peripheral.attributes {
                    if *attribute == "derivedFrom".to_string() {
                        //print!("derivedFrom: {}, ", value);
                        
                        for p in &ph {
                            if &p.name == value {
                                let mut new_peripheral = p.clone();
                                new_peripheral.name = peripheral.take_child("name").unwrap().text.unwrap();
                                new_peripheral.base_address = peripheral.take_child("baseAddress").unwrap().text.unwrap();
                                println!("Peripheral: {}, derivedFrom: {}", new_peripheral.name, &p.name);
                                ph.push(new_peripheral);
                                break;
                            }
                        }

                        derived = true;
                        break;
                    }
                }
                
                if derived == false {
                    ph.push(peripheral_get(peripheral));
                }
            },
            None => { break },
        }
    }

    for p in ph {
        generate(&p);
    }
}

// Type your code here, or load an example.
/*
struct Conset
{
    raw: u32,
}

impl Conset {
    #[inline(always)]
    fn new() -> Conset {
        let c = Conset { raw: 0 };
        return c;
    }
    
    #[inline(always)]
    fn en(&mut self, data: u8) -> &mut Conset {
        self.raw |= (data << 3) as u32;
        self
    }

    #[inline(always)]
    fn clk(&mut self, data: u8)  -> &mut Conset {
        self.raw |= (data << 5) as u32;
        self
    }

    #[inline(always)]
    fn raw(self)  -> u32{
        self.raw
    }
}

pub fn square() -> u32 {
    let r = Conset::new().en(3).clk(5);
    r.raw()
}
*/
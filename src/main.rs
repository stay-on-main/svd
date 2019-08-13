/*
struct Register {
    name: String,
    display_name: String,
    description: String,
    address_offset: usize,
    size: usize,
    access: u8,
    reset_value: u32,

}


struct Field {
    name: String,
    description: String,
    bit_offset: usize,
    bit_width: usize,
}
*/
/*
extern crate xml;

use std::fs::File;
use std::io::BufReader;

use xml::reader::{EventReader, XmlEvent};

fn indent(size: usize) -> String {
    const INDENT: &'static str = "    ";
    (0..size).map(|_| INDENT)
             .fold(String::with_capacity(size*INDENT.len()), |r, s| r + s)
}

fn main() {
    let file = File::open("C:/Users/zahar.kravtsov/Documents/svd/STM32F103.svd").unwrap();
    let file = BufReader::new(file);

    let parser = EventReader::new(file);
    //let mut depth = 0;
    let mut tag_name: String = "".to_string();

    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => {
                println!("{}+{}", indent(0), name);
                tag_name = name.local_name;
                //depth += 1;
            }
            Ok(XmlEvent::EndElement { name }) => {
                tag_name = "".to_string();
                //depth -= 1;
                //println!("{}-{}", indent(depth), name);
            }
            Ok(XmlEvent::Characters(text)) => {
                match tag_name.as_str() {
                    "name" => {
                        println!("{}", text);
                        break;
                    },
                    _ => {

                    }
                }
                //depth -= 1;
                //println!("{}-{}", indent(depth), name);
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }
}
*/
/*
struct Xml {
    tag: String,
    content: String,
}

impl Xml {
    fn new(data: String) -> Xml {

    }
}

//fn xml_parse(s: &str) ->

fn main()
{

    let xml = "<field>
              <name>WAITCFG</name>
              <description>WAITCFG</description>
              <bitOffset>11</bitOffset>
              <bitWidth>1</bitWidth>
            </field>";
    

}
*/
/*
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("C:/Users/zahar.kravtsov/Documents/svd/STM32F103.svd")?;
    let mut contents = vec![];
    file.read_to_end(&mut contents)?;

    for i in 0..10 {
        println!("{:2x}", contents[i]);
    }

    //assert_eq!(contents, "Hello, world!");
    Ok(())
}
*/
use std::fs::File;
use std::io::prelude::*;
use xmltree::Element;

struct Field
{
    name: String,
    description: String,
    bit_range: String,
}

struct Register
{
    name: String,
    description: String,
    address_offset: String,
    access: String,
    reset_value: String,
    reset_mask: String,
    fields: Vec<Field>,
}

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
    Field {
        name: field.take_child("name").unwrap().text.unwrap(),
        description: field.take_child("description").unwrap().text.unwrap(),
        bit_range: field.take_child("bitRange").unwrap().text.unwrap(),
    }
}

fn registers_get(mut register: Element) -> Register
{
    let mut r = Register {
        name: register.take_child("name").unwrap().text.unwrap(),
        description: register.take_child("description").unwrap().text.unwrap(),
        address_offset: register.take_child("addressOffset").unwrap().text.unwrap(),
        access: register.take_child("access").unwrap().text.unwrap(),
        reset_value: register.take_child("resetValue").unwrap().text.unwrap(),
        reset_mask: register.take_child("resetMask").unwrap().text.unwrap(),
        fields: Vec::new(),
    };

    let mut fields = register.take_child("fields").expect("Can't find name element");

    loop {
        match fields.take_child("field") {
            Some(field) => {
                r.fields.push(fields_get(field));
            },
            None => { break },
        }
    }

    r
}

fn ph(mut peripheral: Element)
{
    let mut p = Perepheral {
        name: peripheral.take_child("name").unwrap().text.unwrap(),
        description: peripheral.take_child("description").unwrap().text.unwrap(),
        group_name: peripheral.take_child("groupName").unwrap().text.unwrap(),
        base_address: peripheral.take_child("baseAddress").unwrap().text.unwrap(),
        registers: Vec::new(),
    };

    let mut registers = peripheral.take_child("registers").expect("Can't find name element");

    loop {
        match registers.take_child("register") {
            Some(register) => {
                p.registers.push(registers_get(register));
            },
            None => { break },
        }
    }
    //println!("{}", name.text.unwrap());
}

fn main() {
    let mut file = File::open("C:/Users/stay-on-main/Desktop/svd/STM32F103.svd").unwrap();
    let mut contents = vec![];
    file.read_to_end(&mut contents).unwrap();


    let mut names_element = Element::parse(contents.as_slice()).unwrap();

    let mut peripherals = names_element.take_child("peripherals").expect("Can't find name element");

    loop {
        match peripherals.take_child("peripheral") {
            Some(peripheral) => {
                ph(peripheral);
            },
            None => { break },
        }
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
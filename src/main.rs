mod access_flags;
mod constant_pool;

use std::{
    fs::File,
    io::{self, Read},
};

use crate::constant_pool::Methodref;

struct JavaClassFile {
    minor: u16,
    major: u16,
    constant_pool: Vec<constant_pool::Constant>,
    access_flags: u16,
    this_class: u16,
}

impl JavaClassFile {
    pub fn try_from_file(f: &mut File) -> Result<JavaClassFile, io::Error> {
        let mut magic = [0; 4];
        f.read(&mut magic).unwrap();

        if magic != [0xCA, 0xFE, 0xBA, 0xBE] {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid magic number",
            ));
        }

        let mut minor = [0; 2];
        f.read(&mut minor).unwrap();

        let mut major = [0; 2];
        f.read(&mut major).unwrap();

        let mut constant_pool_count = [0; 2];
        f.read(&mut constant_pool_count).unwrap();

        let count = u16::from_be_bytes(constant_pool_count) - 1;

        println!("Count: {}", count);

        let mut constant_pool: Vec<constant_pool::Constant> = Vec::new();
        for i in 0..count {
            let mut tag = [0; 1];
            f.read(&mut tag).unwrap();

            match tag[0] {
                constant_pool::METHODREF => {
                    let mut class_index = [0; 2];
                    f.read(&mut class_index).unwrap();

                    let mut name_and_type_index = [0; 2];
                    f.read(&mut name_and_type_index).unwrap();

                    let t = Methodref {
                        class_index: u16::from_be_bytes(class_index),
                        name_and_type_index: u16::from_be_bytes(name_and_type_index),
                    };

                    println!(
                        "  {}: Methodref:\n    Class Index: {}\n    Name and Type Index: {}",
                        i, t.class_index, t.name_and_type_index
                    );

                    constant_pool.push(constant_pool::Constant::Methodref(t));
                }
                constant_pool::CLASS => {
                    let mut name_index = [0; 2];
                    f.read(&mut name_index).unwrap();

                    let t = constant_pool::Class {
                        name_index: u16::from_be_bytes(name_index),
                    };

                    println!("  {}: Class:\n    Name Index: {}", i, t.name_index);

                    constant_pool.push(constant_pool::Constant::Class(t));
                }
                constant_pool::NAME_AND_TYPE => {
                    let mut name_index = [0; 2];
                    f.read(&mut name_index).unwrap();

                    let mut descriptor_index = [0; 2];
                    f.read(&mut descriptor_index).unwrap();

                    let t = constant_pool::NameAndType {
                        name_index: u16::from_be_bytes(name_index),
                        descriptor_index: u16::from_be_bytes(descriptor_index),
                    };

                    println!(
                        "  {}: Name and Type:\n    Name Index: {}\n    Descriptor Index: {}",
                        i, t.name_index, t.descriptor_index
                    );

                    constant_pool.push(constant_pool::Constant::NameAndType(t));
                }
                constant_pool::UTF8 => {
                    let mut length = [0; 2];
                    f.read(&mut length).unwrap();

                    let mut bytes = vec![0; u16::from_be_bytes(length) as usize];
                    f.read(&mut bytes).unwrap();

                    let t = constant_pool::Utf8 {
                        length: u16::from_be_bytes(length),
                        bytes,
                    };

                    let b = &t.bytes;

                    println!(
                        "  {}: Utf8:\n    Length: {}\n    Bytes: {}",
                        i,
                        t.length,
                        String::from_utf8(b).unwrap()
                    );

                    constant_pool.push(constant_pool::Constant::Utf8(t));
                }
                constant_pool::FIELDREF => {
                    let mut class_index = [0; 2];
                    f.read(&mut class_index).unwrap();

                    let mut name_and_type_index = [0; 2];
                    f.read(&mut name_and_type_index).unwrap();

                    let t = constant_pool::Fieldref {
                        class_index: u16::from_be_bytes(class_index),
                        name_and_type_index: u16::from_be_bytes(name_and_type_index),
                    };

                    println!(
                        "  {}: Fieldref:\n    Class Index: {}\n    Name and Type Index: {}",
                        i, t.class_index, t.name_and_type_index
                    );

                    constant_pool.push(constant_pool::Constant::Fieldref(t));
                }
                constant_pool::STRING => {
                    let mut string_index = [0; 2];
                    f.read(&mut string_index).unwrap();

                    let t = constant_pool::String {
                        string_index: u16::from_be_bytes(string_index),
                    };

                    println!("  {}: String:\n    String Index: {}", i, t.string_index);

                    constant_pool.push(constant_pool::Constant::String(t));
                }
                _ => unimplemented!("Unknown tag: {}", tag[0]),
            }
        }

        let mut access_flags = [0; 2];
        f.read(&mut access_flags).unwrap();

        let mut this_class = [0; 2];
        f.read(&mut this_class).unwrap();

        Ok(JavaClassFile {
            minor: u16::from_be_bytes(minor),
            major: u16::from_be_bytes(major),
            constant_pool,
            access_flags: u16::from_be_bytes(access_flags),
            this_class: u16::from_be_bytes(this_class),
        })
    }
}

fn main() {
    let mut f = File::open("./Main.class").unwrap();

    let class_file = JavaClassFile::try_from_file(&mut f).unwrap();

    println!("Minor version: {}", class_file.minor);
    println!("Major version: {}", class_file.major);
    println!("Access flags: {}", class_file.access_flags);
    println!("This class: {}", class_file.this_class);
}

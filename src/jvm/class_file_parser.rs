use crate::error::RuntimeError;
use super::jvm_types::{ConstantPool, ConstantPoolEntry, JvmInstruction};
use std::io::{Cursor, Read};

pub struct ClassFile {
    pub constant_pool: ConstantPool,
    pub main_method_bytecode: Vec<JvmInstruction>,
    pub max_locals: usize,
    pub max_stack: usize,
}

pub struct ClassFileParser;

impl ClassFileParser {
    pub fn parse(data: &[u8]) -> Result<ClassFile, RuntimeError> {
        let mut cursor = Cursor::new(data);

        // Parse class file header
        let magic = read_u32(&mut cursor)?;
        if magic != 0xCAFEBABE {
            return Err(RuntimeError::InvalidStackState); // Invalid class file
        }

        let _minor_version = read_u16(&mut cursor)?;
        let _major_version = read_u16(&mut cursor)?;

        // Parse constant pool
        let constant_pool_count = read_u16(&mut cursor)?;
        let mut constant_pool = ConstantPool::new();

        // Parse constant pool entries (1-based indexing, skip index 0)
        for i in 1..constant_pool_count {
            let tag = read_u8(&mut cursor)?;
            match tag {
                1 => {
                    // CONSTANT_Utf8
                    let length = read_u16(&mut cursor)?;
                    let mut bytes = vec![0u8; length as usize];
                    cursor
                        .read_exact(&mut bytes)
                        .map_err(|_| RuntimeError::InvalidStackState)?;
                    let utf8_string =
                        String::from_utf8(bytes).map_err(|_| RuntimeError::InvalidStackState)?;
                    constant_pool.add_utf8(utf8_string);
                }
                3 => {
                    // CONSTANT_Integer
                    let value = read_i32(&mut cursor)?;
                    constant_pool.add_integer(value);
                }
                4 => {
                    // CONSTANT_Float
                    let value = read_f32(&mut cursor)?;
                    constant_pool.add_float(value);
                }
                5 => {
                    // CONSTANT_Long
                    let value = read_i64(&mut cursor)?;
                    constant_pool.add_long(value);
                }
                6 => {
                    // CONSTANT_Double
                    let value = read_f64(&mut cursor)?;
                    constant_pool.add_double(value);
                }
                7 => {
                    // CONSTANT_Class
                    let name_index = read_u16(&mut cursor)?;
                    constant_pool.add_class(name_index);
                }
                8 => {
                    // CONSTANT_String
                    let string_index = read_u16(&mut cursor)?;
                    constant_pool.add_string(string_index);
                }
                9 => {
                    // CONSTANT_Fieldref
                    let class_index = read_u16(&mut cursor)?;
                    let name_and_type_index = read_u16(&mut cursor)?;
                    constant_pool.add_fieldref(class_index, name_and_type_index);
                }
                10 => {
                    // CONSTANT_Methodref
                    let class_index = read_u16(&mut cursor)?;
                    let name_and_type_index = read_u16(&mut cursor)?;
                    constant_pool.add_methodref(class_index, name_and_type_index);
                }
                12 => {
                    // CONSTANT_NameAndType
                    let name_index = read_u16(&mut cursor)?;
                    let descriptor_index = read_u16(&mut cursor)?;
                    constant_pool.add_name_and_type(name_index, descriptor_index);
                }
                _ => {
                    // Skip unknown constant types for now
                    eprintln!("Warning: Unknown constant pool tag {} at index {}", tag, i);
                    return Err(RuntimeError::InvalidStackState);
                }
            }
        }

        // Skip access flags, this_class, super_class
        let _access_flags = read_u16(&mut cursor)?;
        let _this_class = read_u16(&mut cursor)?;
        let _super_class = read_u16(&mut cursor)?;

        // Skip interfaces
        let interfaces_count = read_u16(&mut cursor)?;
        for _ in 0..interfaces_count {
            let _interface = read_u16(&mut cursor)?;
        }

        // Skip fields
        let fields_count = read_u16(&mut cursor)?;
        for _ in 0..fields_count {
            let _access_flags = read_u16(&mut cursor)?;
            let _name_index = read_u16(&mut cursor)?;
            let _descriptor_index = read_u16(&mut cursor)?;
            let attributes_count = read_u16(&mut cursor)?;
            for _ in 0..attributes_count {
                let _attribute_name_index = read_u16(&mut cursor)?;
                let attribute_length = read_u32(&mut cursor)?;
                // Skip attribute data
                for _ in 0..attribute_length {
                    read_u8(&mut cursor)?;
                }
            }
        }

        // Parse methods to find main method
        let methods_count = read_u16(&mut cursor)?;
        let mut main_method_bytecode = Vec::new();
        let mut max_locals = 0;
        let mut max_stack = 0;

        for _ in 0..methods_count {
            let _access_flags = read_u16(&mut cursor)?;
            let name_index = read_u16(&mut cursor)?;
            let descriptor_index = read_u16(&mut cursor)?;
            let attributes_count = read_u16(&mut cursor)?;

            // Check if this is the main method
            let is_main_method = check_is_main_method(&constant_pool, name_index, descriptor_index);

            for _ in 0..attributes_count {
                let attribute_name_index = read_u16(&mut cursor)?;
                let attribute_length = read_u32(&mut cursor)?;

                // Check if this is a Code attribute and we're in the main method
                if is_main_method && check_is_code_attribute(&constant_pool, attribute_name_index) {
                    max_stack = read_u16(&mut cursor)? as usize;
                    max_locals = read_u16(&mut cursor)? as usize;
                    let code_length = read_u32(&mut cursor)?;

                    // Parse bytecode
                    let mut bytecode = vec![0u8; code_length as usize];
                    cursor
                        .read_exact(&mut bytecode)
                        .map_err(|_| RuntimeError::InvalidStackState)?;
                    main_method_bytecode = parse_bytecode(&bytecode)?;

                    // Skip exception table
                    let exception_table_length = read_u16(&mut cursor)?;
                    for _ in 0..exception_table_length {
                        let _start_pc = read_u16(&mut cursor)?;
                        let _end_pc = read_u16(&mut cursor)?;
                        let _handler_pc = read_u16(&mut cursor)?;
                        let _catch_type = read_u16(&mut cursor)?;
                    }

                    // Skip code attributes
                    let code_attributes_count = read_u16(&mut cursor)?;
                    for _ in 0..code_attributes_count {
                        let _code_attribute_name_index = read_u16(&mut cursor)?;
                        let code_attribute_length = read_u32(&mut cursor)?;
                        for _ in 0..code_attribute_length {
                            read_u8(&mut cursor)?;
                        }
                    }
                } else {
                    // Skip other attributes
                    for _ in 0..attribute_length {
                        read_u8(&mut cursor)?;
                    }
                }
            }
        }

        Ok(ClassFile {
            constant_pool,
            main_method_bytecode,
            max_locals,
            max_stack,
        })
    }
}

fn parse_bytecode(bytecode: &[u8]) -> Result<Vec<JvmInstruction>, RuntimeError> {
    let mut instructions = Vec::new();
    let mut i = 0;

    while i < bytecode.len() {
        let opcode = bytecode[i];
        i += 1;

        match opcode {
            0x02 => instructions.push(JvmInstruction::IconstM1),
            0x03 => instructions.push(JvmInstruction::Iconst0),
            0x04 => instructions.push(JvmInstruction::Iconst1),
            0x05 => instructions.push(JvmInstruction::Iconst2),
            0x06 => instructions.push(JvmInstruction::Iconst3),
            0x07 => instructions.push(JvmInstruction::Iconst4),
            0x08 => instructions.push(JvmInstruction::Iconst5),
            0x10 => {
                // bipush
                if i >= bytecode.len() {
                    return Err(RuntimeError::InvalidStackState);
                }
                let value = bytecode[i] as i8;
                instructions.push(JvmInstruction::Bipush(value));
                i += 1;
            }
            0x11 => {
                // sipush
                if i + 1 >= bytecode.len() {
                    return Err(RuntimeError::InvalidStackState);
                }
                let value = ((bytecode[i] as u16) << 8) | (bytecode[i + 1] as u16);
                instructions.push(JvmInstruction::Sipush(value as i16));
                i += 2;
            }
            0x12 => {
                // ldc
                if i >= bytecode.len() {
                    return Err(RuntimeError::InvalidStackState);
                }
                let index = bytecode[i] as u16;
                instructions.push(JvmInstruction::Ldc(index));
                i += 1;
            }
            0x57 => instructions.push(JvmInstruction::Pop),
            0x59 => instructions.push(JvmInstruction::Dup),
            0x5F => instructions.push(JvmInstruction::Swap),
            0x60 => instructions.push(JvmInstruction::Iadd),
            0x64 => instructions.push(JvmInstruction::Isub),
            0x68 => instructions.push(JvmInstruction::Imul),
            0x6C => instructions.push(JvmInstruction::Idiv),
            0x70 => instructions.push(JvmInstruction::Irem),
            0x63 => instructions.push(JvmInstruction::Dadd),
            0x67 => instructions.push(JvmInstruction::Dsub),
            0x6B => instructions.push(JvmInstruction::Dmul),
            0x6F => instructions.push(JvmInstruction::Ddiv),
            0x87 => instructions.push(JvmInstruction::I2d),
            0x8E => instructions.push(JvmInstruction::D2i),
            0xA7 => {
                // goto
                if i + 1 >= bytecode.len() {
                    return Err(RuntimeError::InvalidStackState);
                }
                let offset = ((bytecode[i] as u16) << 8) | (bytecode[i + 1] as u16);
                instructions.push(JvmInstruction::Goto(offset));
                i += 2;
            }
            0x99 => {
                // ifeq
                if i + 1 >= bytecode.len() {
                    return Err(RuntimeError::InvalidStackState);
                }
                let offset = ((bytecode[i] as u16) << 8) | (bytecode[i + 1] as u16);
                instructions.push(JvmInstruction::Ifeq(offset));
                i += 2;
            }
            0x9A => {
                // ifne
                if i + 1 >= bytecode.len() {
                    return Err(RuntimeError::InvalidStackState);
                }
                let offset = ((bytecode[i] as u16) << 8) | (bytecode[i + 1] as u16);
                instructions.push(JvmInstruction::Ifne(offset));
                i += 2;
            }
            0x9B => {
                // iflt
                if i + 1 >= bytecode.len() {
                    return Err(RuntimeError::InvalidStackState);
                }
                let offset = ((bytecode[i] as u16) << 8) | (bytecode[i + 1] as u16);
                instructions.push(JvmInstruction::Iflt(offset));
                i += 2;
            }
            0x9C => {
                // ifge
                if i + 1 >= bytecode.len() {
                    return Err(RuntimeError::InvalidStackState);
                }
                let offset = ((bytecode[i] as u16) << 8) | (bytecode[i + 1] as u16);
                instructions.push(JvmInstruction::Ifge(offset));
                i += 2;
            }
            0x9D => {
                // ifgt
                if i + 1 >= bytecode.len() {
                    return Err(RuntimeError::InvalidStackState);
                }
                let offset = ((bytecode[i] as u16) << 8) | (bytecode[i + 1] as u16);
                instructions.push(JvmInstruction::Ifgt(offset));
                i += 2;
            }
            0x9E => {
                // ifle
                if i + 1 >= bytecode.len() {
                    return Err(RuntimeError::InvalidStackState);
                }
                let offset = ((bytecode[i] as u16) << 8) | (bytecode[i + 1] as u16);
                instructions.push(JvmInstruction::Ifle(offset));
                i += 2;
            }
            0xB1 => instructions.push(JvmInstruction::Return),
            0xAC => instructions.push(JvmInstruction::Ireturn),
            0xB2 => {
                // getstatic
                if i + 1 >= bytecode.len() {
                    return Err(RuntimeError::InvalidStackState);
                }
                let index = ((bytecode[i] as u16) << 8) | (bytecode[i + 1] as u16);
                instructions.push(JvmInstruction::Getstatic(index));
                i += 2;
            }
            0xB6 => {
                // invokevirtual
                if i + 1 >= bytecode.len() {
                    return Err(RuntimeError::InvalidStackState);
                }
                let index = ((bytecode[i] as u16) << 8) | (bytecode[i + 1] as u16);
                instructions.push(JvmInstruction::Invokevirtual(index));
                i += 2;
            }
            0xB8 => {
                // invokestatic
                if i + 1 >= bytecode.len() {
                    return Err(RuntimeError::InvalidStackState);
                }
                let index = ((bytecode[i] as u16) << 8) | (bytecode[i + 1] as u16);
                instructions.push(JvmInstruction::Invokestatic(index));
                i += 2;
            }
            0x0E => instructions.push(JvmInstruction::Dconst0),
            0x0F => instructions.push(JvmInstruction::Dconst1),
            _ => {
                // Unknown opcode, skip for now
                eprintln!(
                    "Warning: Unknown opcode 0x{:02X} at position {}",
                    opcode,
                    i - 1
                );
            }
        }
    }

    Ok(instructions)
}

fn check_is_main_method(
    constant_pool: &ConstantPool,
    name_index: u16,
    descriptor_index: u16,
) -> bool {
    let entries = constant_pool.entries();

    // Check method name
    if let Some(ConstantPoolEntry::Utf8(name)) = entries.get((name_index - 1) as usize) {
        if name != "main" {
            return false;
        }
    } else {
        return false;
    }

    // Check method descriptor
    if let Some(ConstantPoolEntry::Utf8(descriptor)) = entries.get((descriptor_index - 1) as usize)
    {
        descriptor == "([Ljava/lang/String;)V"
    } else {
        false
    }
}

fn check_is_code_attribute(constant_pool: &ConstantPool, attribute_name_index: u16) -> bool {
    let entries = constant_pool.entries();

    if let Some(ConstantPoolEntry::Utf8(attr_name)) =
        entries.get((attribute_name_index - 1) as usize)
    {
        attr_name == "Code"
    } else {
        false
    }
}

fn read_u8(cursor: &mut Cursor<&[u8]>) -> Result<u8, RuntimeError> {
    let mut buf = [0u8; 1];
    cursor
        .read_exact(&mut buf)
        .map_err(|_| RuntimeError::InvalidStackState)?;
    Ok(buf[0])
}

fn read_u16(cursor: &mut Cursor<&[u8]>) -> Result<u16, RuntimeError> {
    let mut buf = [0u8; 2];
    cursor
        .read_exact(&mut buf)
        .map_err(|_| RuntimeError::InvalidStackState)?;
    Ok(u16::from_be_bytes(buf))
}

fn read_u32(cursor: &mut Cursor<&[u8]>) -> Result<u32, RuntimeError> {
    let mut buf = [0u8; 4];
    cursor
        .read_exact(&mut buf)
        .map_err(|_| RuntimeError::InvalidStackState)?;
    Ok(u32::from_be_bytes(buf))
}

fn read_i32(cursor: &mut Cursor<&[u8]>) -> Result<i32, RuntimeError> {
    let mut buf = [0u8; 4];
    cursor
        .read_exact(&mut buf)
        .map_err(|_| RuntimeError::InvalidStackState)?;
    Ok(i32::from_be_bytes(buf))
}

fn read_f32(cursor: &mut Cursor<&[u8]>) -> Result<f32, RuntimeError> {
    let mut buf = [0u8; 4];
    cursor
        .read_exact(&mut buf)
        .map_err(|_| RuntimeError::InvalidStackState)?;
    Ok(f32::from_be_bytes(buf))
}

fn read_i64(cursor: &mut Cursor<&[u8]>) -> Result<i64, RuntimeError> {
    let mut buf = [0u8; 8];
    cursor
        .read_exact(&mut buf)
        .map_err(|_| RuntimeError::InvalidStackState)?;
    Ok(i64::from_be_bytes(buf))
}

fn read_f64(cursor: &mut Cursor<&[u8]>) -> Result<f64, RuntimeError> {
    let mut buf = [0u8; 8];
    cursor
        .read_exact(&mut buf)
        .map_err(|_| RuntimeError::InvalidStackState)?;
    Ok(f64::from_be_bytes(buf))
}

use super::jvm_types::{ConstantPool, ConstantPoolEntry, JvmInstruction};
use crate::analyzer::SemanticAnalyzer;
/// Java class file generator
use std::fs;

/// Complete Java class file generator
pub struct JavaClassGenerator {
    constant_pool: ConstantPool,
    class_name: String,
}

impl JavaClassGenerator {
    pub fn new(class_name: String) -> Self {
        Self {
            constant_pool: ConstantPool::new(),
            class_name,
        }
    }

    /// Generate Java class file from Dice expression
    pub fn generate_dice_class(
        &mut self,
        expression: &str,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // AST analysis
        let mut analyzer = SemanticAnalyzer::new(expression)?;
        let ast = analyzer.analyze()?;

        if let Some(stmt) = ast.statement {
            let crate::ast::StatementKind::Expression { expr } = stmt.kind;
            let crate::ast::ExpressionKind::Dice { count, faces } = expr.kind;

            self.setup_constant_pool();
            let bytecode = self.generate_dice_bytecode(count, faces)?;
            return self.generate_class_file(bytecode);
        }

        Err("Invalid expression".into())
    }

    /// Generate JVM instruction sequence from Dice expression (for VM execution)
    pub fn generate_dice_instructions(
        &mut self,
        expression: &str,
    ) -> Result<Vec<JvmInstruction>, Box<dyn std::error::Error>> {
        let mut analyzer = SemanticAnalyzer::new(expression)?;
        let ast = analyzer.analyze()?;

        if let Some(stmt) = ast.statement {
            let crate::ast::StatementKind::Expression { expr } = stmt.kind;
            let crate::ast::ExpressionKind::Dice { count, faces } = expr.kind;

            return self.generate_dice_bytecode(count, faces);
        }

        Err("Invalid expression".into())
    }

    /// Setup constant pool
    fn setup_constant_pool(&mut self) {
        let class_name_index = self.constant_pool.add_utf8(self.class_name.clone()); // UTF8 - class name
        let object_class_index = self.constant_pool.add_utf8("java/lang/Object".to_string()); // UTF8 - "java/lang/Object"
        let main_method_index = self.constant_pool.add_utf8("main".to_string()); // UTF8 - "main"
        let main_descriptor_index = self.constant_pool.add_utf8("([Ljava/lang/String;)V".to_string()); // UTF8 - "([Ljava/lang/String;)V"
        let code_index = self.constant_pool.add_utf8("Code".to_string()); // UTF8 - "Code"
        let system_class_index = self.constant_pool.add_utf8("java/lang/System".to_string()); // UTF8 - "java/lang/System"
        let out_field_index = self.constant_pool.add_utf8("out".to_string()); // UTF8 - "out"
        let err_field_index = self.constant_pool.add_utf8("err".to_string()); // UTF8 - "err"
        let print_stream_descriptor_index = self.constant_pool.add_utf8("Ljava/io/PrintStream;".to_string()); // UTF8 - "Ljava/io/PrintStream;"
        let print_stream_class_index = self.constant_pool.add_utf8("java/io/PrintStream".to_string()); // UTF8 - "java/io/PrintStream"
        let println_method_index = self.constant_pool.add_utf8("println".to_string()); // UTF8 - "println"
        let println_descriptor_index = self.constant_pool.add_utf8("(I)V".to_string()); // UTF8 - "(I)V"
        let math_class_index = self.constant_pool.add_utf8("java/lang/Math".to_string()); // UTF8 - "java/lang/Math"
        let random_method_index = self.constant_pool.add_utf8("random".to_string()); // UTF8 - "random"
        // 15: UTF8 - "()D"
        self.constant_pool.add_utf8("()D".to_string());
        // 16: UTF8 - "Total: "
        self.constant_pool.add_utf8("Total: ".to_string());
        // 17: UTF8 - "print"
        self.constant_pool.add_utf8("print".to_string());
        // 18: UTF8 - "(Ljava/lang/String;)V"
        self.constant_pool
            .add_utf8("(Ljava/lang/String;)V".to_string());

        // Classes
        // 19: Class - this class
        self.constant_pool.add_class(1);
        // 20: Class - java/lang/Object
        self.constant_pool.add_class(2);
        // 21: Class - java/lang/System
        self.constant_pool.add_class(6);
        // 22: Class - java/io/PrintStream
        self.constant_pool.add_class(10);
        // 23: Class - java/lang/Math
        self.constant_pool.add_class(13);

        // String constants
        // 24: String - "Total: "
        self.constant_pool.add_string(16);

        // NameAndType
        // 25: NameAndType - main method
        self.constant_pool.add_name_and_type(3, 4);
        // 26: NameAndType - out field
        self.constant_pool.add_name_and_type(7, 9);
        // 27: NameAndType - err field
        self.constant_pool.add_name_and_type(8, 9);
        // 28: NameAndType - println method
        self.constant_pool.add_name_and_type(11, 12);
        // 29: NameAndType - print method
        self.constant_pool.add_name_and_type(17, 18);
        // 30: NameAndType - random method
        self.constant_pool.add_name_and_type(14, 15);

        // Field and method references
        // 31: Fieldref - System.out
        self.constant_pool.add_fieldref(21, 26);
        // 32: Fieldref - System.err
        self.constant_pool.add_fieldref(21, 27);
        // 33: Methodref - println
        self.constant_pool.add_methodref(22, 28);
        // 34: Methodref - print
        self.constant_pool.add_methodref(22, 29);
        // 35: Methodref - Math.random
        self.constant_pool.add_methodref(23, 30);
    }

    /// Generate bytecode for Dice
    fn generate_dice_bytecode(
        &self,
        count: u32,
        faces: u32,
    ) -> Result<Vec<JvmInstruction>, Box<dyn std::error::Error>> {
        let mut instructions = Vec::new();

        if count == 1 {
            // Single dice - don't display Total
            self.generate_single_dice(&mut instructions, faces);
        } else {
            // Multiple dice - display each result and Total
            self.generate_multiple_dice(&mut instructions, count, faces);
        }

        instructions.push(JvmInstruction::Return);
        Ok(instructions)
    }

    /// Generate bytecode for single dice
    fn generate_single_dice(&self, instructions: &mut Vec<JvmInstruction>, faces: u32) {
        // Math.random() * faces + 1
        instructions.push(JvmInstruction::Invokestatic(35)); // Math.random()
        self.push_double_constant(instructions, faces as f64);
        instructions.push(JvmInstruction::Dmul);
        instructions.push(JvmInstruction::Dconst1);
        instructions.push(JvmInstruction::Dadd);
        instructions.push(JvmInstruction::D2i);

        // Output result to System.out
        instructions.push(JvmInstruction::Getstatic(31)); // System.out
        instructions.push(JvmInstruction::Swap);
        instructions.push(JvmInstruction::Invokevirtual(33)); // println(I)V
    }

    /// Generate bytecode for multiple dice
    fn generate_multiple_dice(
        &self,
        instructions: &mut Vec<JvmInstruction>,
        count: u32,
        faces: u32,
    ) {
        instructions.push(JvmInstruction::Iconst0); // total = 0

        // Roll each dice
        for _ in 0..count {
            // Math.random() * faces + 1
            instructions.push(JvmInstruction::Invokestatic(35)); // Math.random()
            self.push_double_constant(instructions, faces as f64);
            instructions.push(JvmInstruction::Dmul);
            instructions.push(JvmInstruction::Dconst1);
            instructions.push(JvmInstruction::Dadd);
            instructions.push(JvmInstruction::D2i);

            // Duplicate result (one for display, one for total)
            instructions.push(JvmInstruction::Dup);

            // Output individual result to System.out
            instructions.push(JvmInstruction::Getstatic(31)); // System.out
            instructions.push(JvmInstruction::Swap);
            instructions.push(JvmInstruction::Invokevirtual(33)); // println(I)V

            // Add to total
            instructions.push(JvmInstruction::Iadd);
        }

        // Output "Total: " to System.err
        instructions.push(JvmInstruction::Dup); // Duplicate total
        instructions.push(JvmInstruction::Getstatic(32)); // System.err
        instructions.push(JvmInstruction::Ldc(24)); // "Total: "
        instructions.push(JvmInstruction::Invokevirtual(34)); // print(String)V

        // Output total to System.err
        instructions.push(JvmInstruction::Getstatic(32)); // System.err
        instructions.push(JvmInstruction::Swap);
        instructions.push(JvmInstruction::Invokevirtual(33)); // println(I)V
        instructions.push(JvmInstruction::Pop); // Remove remaining value from stack
    }

    /// Push double constant to stack
    fn push_double_constant(&self, instructions: &mut Vec<JvmInstruction>, value: f64) {
        if value == 0.0 {
            instructions.push(JvmInstruction::Dconst0);
        } else if value == 1.0 {
            instructions.push(JvmInstruction::Dconst1);
        } else {
            // For more complex constants, use integer conversion
            let int_val = value as i32;
            self.push_int_constant(instructions, int_val);
            instructions.push(JvmInstruction::I2d);
        }
    }

    /// Push int constant to stack
    fn push_int_constant(&self, instructions: &mut Vec<JvmInstruction>, value: i32) {
        match value {
            -1 => instructions.push(JvmInstruction::IconstM1),
            0 => instructions.push(JvmInstruction::Iconst0),
            1 => instructions.push(JvmInstruction::Iconst1),
            2 => instructions.push(JvmInstruction::Iconst2),
            3 => instructions.push(JvmInstruction::Iconst3),
            4 => instructions.push(JvmInstruction::Iconst4),
            5 => instructions.push(JvmInstruction::Iconst5),
            _ if (-128..=127).contains(&value) => {
                instructions.push(JvmInstruction::Bipush(value as i8));
            }
            _ if (-32768..=32767).contains(&value) => {
                instructions.push(JvmInstruction::Sipush(value as i16));
            }
            _ => {
                // Handle unsupported values explicitly
                return Err(format!("Value {} is outside the supported range for Sipush (-32768 to 32767)", value));
            }
        }
        Ok(())
    }

    /// Generate Java class file
    fn generate_class_file(
        &self,
        bytecode_instructions: Vec<JvmInstruction>,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut bytes = Vec::new();

        // Magic number
        bytes.extend_from_slice(&0xCAFEBABEu32.to_be_bytes());

        // Version (Java 8 = 52)
        bytes.extend_from_slice(&0u16.to_be_bytes()); // Minor version
        bytes.extend_from_slice(&52u16.to_be_bytes()); // Major version

        // Constant pool count (non-placeholder entries + 1)
        let non_placeholder_count = self
            .constant_pool
            .entries()
            .iter()
            .filter(|entry| !matches!(entry, ConstantPoolEntry::Placeholder))
            .count();
        bytes.extend_from_slice(&(non_placeholder_count as u16 + 1).to_be_bytes());

        // Constant pool entries
        self.write_constant_pool(&mut bytes);

        // Access flags (public class)
        bytes.extend_from_slice(&0x0021u16.to_be_bytes());

        // This class (index 19)
        bytes.extend_from_slice(&19u16.to_be_bytes());

        // Super class (index 20)
        bytes.extend_from_slice(&20u16.to_be_bytes());

        // Interfaces count
        bytes.extend_from_slice(&0u16.to_be_bytes());

        // Fields count
        bytes.extend_from_slice(&0u16.to_be_bytes());

        // Methods count
        bytes.extend_from_slice(&1u16.to_be_bytes());

        // Main method
        self.write_main_method(&mut bytes, bytecode_instructions);

        // Class attributes count
        bytes.extend_from_slice(&0u16.to_be_bytes());

        Ok(bytes)
    }

    /// Write constant pool in binary format
    fn write_constant_pool(&self, bytes: &mut Vec<u8>) {
        for entry in self.constant_pool.entries() {
            match entry {
                ConstantPoolEntry::Utf8(s) => {
                    bytes.push(1); // CONSTANT_Utf8
                    bytes.extend_from_slice(&(s.len() as u16).to_be_bytes());
                    bytes.extend_from_slice(s.as_bytes());
                }
                ConstantPoolEntry::Class(name_index) => {
                    bytes.push(7); // CONSTANT_Class
                    bytes.extend_from_slice(&name_index.to_be_bytes());
                }
                ConstantPoolEntry::String(utf8_index) => {
                    bytes.push(8); // CONSTANT_String
                    bytes.extend_from_slice(&utf8_index.to_be_bytes());
                }
                ConstantPoolEntry::Fieldref(class_index, name_and_type_index) => {
                    bytes.push(9); // CONSTANT_Fieldref
                    bytes.extend_from_slice(&class_index.to_be_bytes());
                    bytes.extend_from_slice(&name_and_type_index.to_be_bytes());
                }
                ConstantPoolEntry::Methodref(class_index, name_and_type_index) => {
                    bytes.push(10); // CONSTANT_Methodref
                    bytes.extend_from_slice(&class_index.to_be_bytes());
                    bytes.extend_from_slice(&name_and_type_index.to_be_bytes());
                }
                ConstantPoolEntry::NameAndType(name_index, descriptor_index) => {
                    bytes.push(12); // CONSTANT_NameAndType
                    bytes.extend_from_slice(&name_index.to_be_bytes());
                    bytes.extend_from_slice(&descriptor_index.to_be_bytes());
                }
                ConstantPoolEntry::Integer(i) => {
                    bytes.push(3); // CONSTANT_Integer
                    bytes.extend_from_slice(&i.to_be_bytes());
                }
                ConstantPoolEntry::Float(f) => {
                    bytes.push(4); // CONSTANT_Float
                    bytes.extend_from_slice(&f.to_be_bytes());
                }
                ConstantPoolEntry::Long(l) => {
                    bytes.push(5); // CONSTANT_Long
                    bytes.extend_from_slice(&l.to_be_bytes());
                }
                ConstantPoolEntry::Double(d) => {
                    bytes.push(6); // CONSTANT_Double
                    bytes.extend_from_slice(&d.to_be_bytes());
                }
                ConstantPoolEntry::Placeholder => {
                    // Skip placeholder entries - they should not be written to the class file
                    // as they represent the second slot of 8-byte constants (Long/Double)
                    // which are automatically handled by the JVM
                    continue;
                }
            }
        }
    }

    /// Write main method
    fn write_main_method(&self, bytes: &mut Vec<u8>, instructions: Vec<JvmInstruction>) {
        // Access flags (public static)
        bytes.extend_from_slice(&0x0009u16.to_be_bytes());

        // Name index (3 = "main")
        bytes.extend_from_slice(&3u16.to_be_bytes());

        // Descriptor index (4 = "([Ljava/lang/String;)V")
        bytes.extend_from_slice(&4u16.to_be_bytes());

        // Attributes count
        bytes.extend_from_slice(&1u16.to_be_bytes());

        // Code attribute index (5 = "Code")
        bytes.extend_from_slice(&5u16.to_be_bytes());

        // Code attribute
        let code_bytes = self.instructions_to_bytes(instructions);
        let attribute_length = code_bytes.len() as u32 + 12;

        bytes.extend_from_slice(&attribute_length.to_be_bytes());
        bytes.extend_from_slice(&5u16.to_be_bytes()); // max_stack
        bytes.extend_from_slice(&2u16.to_be_bytes()); // max_locals
        bytes.extend_from_slice(&(code_bytes.len() as u32).to_be_bytes()); // code_length
        bytes.extend_from_slice(&code_bytes); // actual bytecode
        bytes.extend_from_slice(&0u16.to_be_bytes()); // exception_table_length
        bytes.extend_from_slice(&0u16.to_be_bytes()); // attributes_count
    }

    /// Convert JVM instructions to byte array
    fn instructions_to_bytes(&self, instructions: Vec<JvmInstruction>) -> Vec<u8> {
        let mut bytes = Vec::new();

        for instruction in instructions {
            match instruction {
                JvmInstruction::Iconst0 => bytes.push(0x03),
                JvmInstruction::Iconst1 => bytes.push(0x04),
                JvmInstruction::Iconst2 => bytes.push(0x05),
                JvmInstruction::Iconst3 => bytes.push(0x06),
                JvmInstruction::Iconst4 => bytes.push(0x07),
                JvmInstruction::Iconst5 => bytes.push(0x08),
                JvmInstruction::IconstM1 => bytes.push(0x02),
                JvmInstruction::Bipush(value) => {
                    bytes.push(0x10);
                    bytes.push(value as u8);
                }
                JvmInstruction::Sipush(value) => {
                    bytes.push(0x11);
                    bytes.extend_from_slice(&(value as u16).to_be_bytes());
                }
                JvmInstruction::Ldc(index) => {
                    bytes.push(0x12);
                    bytes.push(index as u8);
                }
                JvmInstruction::Dup => bytes.push(0x59),
                JvmInstruction::Pop => bytes.push(0x57),
                JvmInstruction::Swap => bytes.push(0x5F),
                JvmInstruction::Iadd => bytes.push(0x60),
                JvmInstruction::Isub => bytes.push(0x64),
                JvmInstruction::Imul => bytes.push(0x68),
                JvmInstruction::Idiv => bytes.push(0x6C),
                JvmInstruction::Irem => bytes.push(0x70),
                JvmInstruction::Dadd => bytes.push(0x63),
                JvmInstruction::Dsub => bytes.push(0x67),
                JvmInstruction::Dmul => bytes.push(0x6B),
                JvmInstruction::Ddiv => bytes.push(0x6F),
                JvmInstruction::I2d => bytes.push(0x87),
                JvmInstruction::D2i => bytes.push(0x8E),
                JvmInstruction::Dconst0 => bytes.push(0x0E),
                JvmInstruction::Dconst1 => bytes.push(0x0F),
                JvmInstruction::Getstatic(index) => {
                    bytes.push(0xB2);
                    bytes.extend_from_slice(&index.to_be_bytes());
                }
                JvmInstruction::Invokestatic(index) => {
                    bytes.push(0xB8);
                    bytes.extend_from_slice(&index.to_be_bytes());
                }
                JvmInstruction::Invokevirtual(index) => {
                    bytes.push(0xB6);
                    bytes.extend_from_slice(&index.to_be_bytes());
                }
                JvmInstruction::Return => bytes.push(0xB1),
                JvmInstruction::Ireturn => bytes.push(0xAC),
                JvmInstruction::Nop => bytes.push(0x00),
                _ => {
                    // Other instructions not implemented for this use case
                    // Use nop as fallback
                    bytes.push(0x00); // nop
                }
            }
        }

        bytes
    }

    pub fn constant_pool(&self) -> &ConstantPool {
        &self.constant_pool
    }
}

/// Unified JVM system - Java class file generation
pub fn generate_java_class(
    expression: &str,
    class_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut generator = JavaClassGenerator::new(class_name.to_string());
    let class_bytes = generator.generate_dice_class(expression)?;
    let filename = format!("{class_name}.class");
    fs::write(&filename, &class_bytes)?;

    println!("Generated: {filename}");
    println!("Run with: java {class_name}");
    println!("View bytecode with: javap -c {class_name}.class");

    Ok(())
}

/// Generate JVM instructions for VM execution
pub fn generate_vm_instructions(
    expression: &str,
) -> Result<(Vec<JvmInstruction>, ConstantPool), Box<dyn std::error::Error>> {
    let mut generator = JavaClassGenerator::new("DiceRoll".to_string());
    generator.setup_constant_pool();
    let instructions = generator.generate_dice_instructions(expression)?;
    Ok((instructions, generator.constant_pool.clone()))
}

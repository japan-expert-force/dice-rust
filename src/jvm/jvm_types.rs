/// JVM bytecode instructions and data type definitions
/// JVM bytecode instructions
#[derive(Debug, Clone)]
pub enum JvmInstruction {
    // Constant pool operations
    Ldc(u16),    // Load constant from pool
    Ldc2W(u16),  // Load 2-word constant from pool (long/double)
    IconstM1,    // Load -1
    Iconst0,     // Load 0
    Iconst1,     // Load 1
    Iconst2,     // Load 2
    Iconst3,     // Load 3
    Iconst4,     // Load 4
    Iconst5,     // Load 5
    Lconst0,     // Load long 0
    Lconst1,     // Load long 1
    Bipush(i8),  // Push byte value
    Sipush(i16), // Push short value

    // Stack operations
    Pop,  // Pop top value
    Dup,  // Duplicate top value
    Swap, // Swap top two values

    // Arithmetic operations
    Iadd, // Add two ints
    Isub, // Subtract two ints
    Imul, // Multiply two ints
    Idiv, // Divide two ints
    Irem, // Remainder of two ints

    // Floating-point operations
    Dadd, // Add two doubles
    Dsub, // Subtract two doubles
    Dmul, // Multiply two doubles
    Ddiv, // Divide two doubles

    // Type conversion
    I2d, // Convert int to double
    D2i, // Convert double to int

    // Control flow
    Ifeq(u16), // Branch if int equals zero
    Ifne(u16), // Branch if int not equals zero
    Iflt(u16), // Branch if int less than zero
    Ifge(u16), // Branch if int greater or equal zero
    Ifgt(u16), // Branch if int greater than zero
    Ifle(u16), // Branch if int less or equal zero
    Goto(u16), // Unconditional branch

    // Local variable operations
    Iload(u8),  // Load int from local variable
    Iload0,     // Load int from local variable 0
    Iload1,     // Load int from local variable 1
    Iload2,     // Load int from local variable 2
    Iload3,     // Load int from local variable 3
    Istore(u8), // Store int to local variable
    Istore0,    // Store int to local variable 0
    Istore1,    // Store int to local variable 1
    Istore2,    // Store int to local variable 2
    Istore3,    // Store int to local variable 3

    Aload(u8),  // Load reference from local variable
    Aload0,     // Load reference from local variable 0
    Aload1,     // Load reference from local variable 1
    Aload2,     // Load reference from local variable 2
    Aload3,     // Load reference from local variable 3
    Astore(u8), // Store reference to local variable
    Astore0,    // Store reference to local variable 0
    Astore1,    // Store reference to local variable 1
    Astore2,    // Store reference to local variable 2
    Astore3,    // Store reference to local variable 3

    Dload(u8),  // Load double from local variable
    Dload0,     // Load double from local variable 0
    Dload1,     // Load double from local variable 1
    Dload2,     // Load double from local variable 2
    Dload3,     // Load double from local variable 3
    Dstore(u8), // Store double to local variable
    Dstore0,    // Store double to local variable 0
    Dstore1,    // Store double to local variable 1
    Dstore2,    // Store double to local variable 2
    Dstore3,    // Store double to local variable 3

    Lload(u8),  // Load long from local variable
    Lload0,     // Load long from local variable 0
    Lload1,     // Load long from local variable 1
    Lload2,     // Load long from local variable 2
    Lload3,     // Load long from local variable 3
    Lstore(u8), // Store long to local variable
    Lstore0,    // Store long to local variable 0
    Lstore1,    // Store long to local variable 1
    Lstore2,    // Store long to local variable 2
    Lstore3,    // Store long to local variable 3

    // Method invocation
    Invokevirtual(u16), // Invoke virtual method
    Invokestatic(u16),  // Invoke static method
    Invokespecial(u16), // Invoke special method (constructors, private methods)
    Invokedynamic(u16), // Invoke dynamic method (for lambda and string concatenation)

    // Object operations
    New(u16), // Create new object

    // Return instructions
    Return,  // Return void
    Ireturn, // Return int

    // Field access
    Getstatic(u16), // Get static field

    // Constants
    Dconst0, // Push double 0.0
    Dconst1, // Push double 1.0

    // Miscellaneous
    Nop, // No operation
}

/// Constant pool entry
#[derive(Debug, Clone)]
pub enum ConstantPoolEntry {
    Utf8(String),
    Class(u16),
    String(u16),
    Fieldref(u16, u16),
    Methodref(u16, u16),
    NameAndType(u16, u16),
    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    Placeholder, // Used for the second slot of 8-byte constants
}

/// Constant pool
#[derive(Debug, Clone)]
pub struct ConstantPool {
    entries: Vec<ConstantPoolEntry>,
}

impl Default for ConstantPool {
    fn default() -> Self {
        Self::new()
    }
}

impl ConstantPool {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn add_utf8(&mut self, value: String) -> u16 {
        let index = self.entries.len();
        if index >= u16::MAX as usize {
            panic!(
                "Constant pool size exceeds the maximum limit of {}",
                u16::MAX
            );
        }
        self.entries.push(ConstantPoolEntry::Utf8(value));
        index as u16 + 1
    }

    pub fn add_class(&mut self, name_index: u16) -> u16 {
        let index = self.entries.len();
        if index >= u16::MAX as usize {
            panic!(
                "Constant pool size exceeds the maximum limit of {}",
                u16::MAX
            );
        }
        self.entries.push(ConstantPoolEntry::Class(name_index));
        index as u16 + 1
    }

    pub fn add_string(&mut self, utf8_index: u16) -> u16 {
        let index = self.entries.len();
        if index >= u16::MAX as usize {
            panic!(
                "Constant pool size exceeds the maximum limit of {}",
                u16::MAX
            );
        }
        self.entries.push(ConstantPoolEntry::String(utf8_index));
        index as u16 + 1
    }

    pub fn add_fieldref(&mut self, class_index: u16, name_and_type_index: u16) -> u16 {
        let index = self.entries.len();
        if index >= u16::MAX as usize {
            panic!(
                "Constant pool size exceeds the maximum limit of {}",
                u16::MAX
            );
        }
        self.entries.push(ConstantPoolEntry::Fieldref(
            class_index,
            name_and_type_index,
        ));
        index as u16 + 1
    }

    pub fn add_methodref(&mut self, class_index: u16, name_and_type_index: u16) -> u16 {
        let index = self.entries.len();
        self.entries.push(ConstantPoolEntry::Methodref(
            class_index,
            name_and_type_index,
        ));
        index as u16 + 1
    }

    pub fn add_name_and_type(&mut self, name_index: u16, descriptor_index: u16) -> u16 {
        let index = self.entries.len();
        self.entries
            .push(ConstantPoolEntry::NameAndType(name_index, descriptor_index));
        index as u16 + 1
    }

    pub fn add_integer(&mut self, value: i32) -> u16 {
        let index = self.entries.len();
        self.entries.push(ConstantPoolEntry::Integer(value));
        index as u16 + 1
    }

    pub fn add_float(&mut self, value: f32) -> u16 {
        let index = self.entries.len();
        self.entries.push(ConstantPoolEntry::Float(value));
        index as u16 + 1
    }

    pub fn add_long(&mut self, value: i64) -> u16 {
        let index = self.entries.len();
        self.entries.push(ConstantPoolEntry::Long(value));
        // Long takes 2 slots in the constant pool, add placeholder for the second slot
        self.entries.push(ConstantPoolEntry::Placeholder);
        index as u16 + 1
    }

    pub fn add_double(&mut self, value: f64) -> u16 {
        let index = self.entries.len();
        self.entries.push(ConstantPoolEntry::Double(value));
        // Double takes 2 slots in the constant pool, add placeholder for the second slot
        self.entries.push(ConstantPoolEntry::Placeholder);
        index as u16 + 1
    }

    pub fn add_placeholder(&mut self) -> u16 {
        let index = self.entries.len();
        self.entries.push(ConstantPoolEntry::Placeholder);
        index as u16 + 1
    }

    pub fn entries(&self) -> &Vec<ConstantPoolEntry> {
        &self.entries
    }
}

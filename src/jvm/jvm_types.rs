/// JVMバイトコード命令とデータ型定義

/// JVMバイトコード命令
#[derive(Debug, Clone)]
pub enum JvmInstruction {
    // 定数プール操作
    Ldc(u16),    // Load constant from pool
    IconstM1,    // Load -1
    Iconst0,     // Load 0
    Iconst1,     // Load 1
    Iconst2,     // Load 2
    Iconst3,     // Load 3
    Iconst4,     // Load 4
    Iconst5,     // Load 5
    Bipush(i8),  // Push byte value
    Sipush(i16), // Push short value

    // スタック操作
    Pop,  // Pop top value
    Dup,  // Duplicate top value
    Swap, // Swap top two values

    // 算術演算
    Iadd, // Add two ints
    Isub, // Subtract two ints
    Imul, // Multiply two ints
    Idiv, // Divide two ints
    Irem, // Remainder of two ints

    // 浮動小数点演算
    Dadd, // Add two doubles
    Dsub, // Subtract two doubles
    Dmul, // Multiply two doubles
    Ddiv, // Divide two doubles

    // 型変換
    I2d, // Convert int to double
    D2i, // Convert double to int

    // 制御フロー
    Ifeq(u16), // Branch if int equals zero
    Ifne(u16), // Branch if int not equals zero
    Iflt(u16), // Branch if int less than zero
    Ifge(u16), // Branch if int greater or equal zero
    Ifgt(u16), // Branch if int greater than zero
    Ifle(u16), // Branch if int less or equal zero
    Goto(u16), // Unconditional branch

    // メソッド呼び出し
    Invokevirtual(u16), // Invoke virtual method
    Invokestatic(u16),  // Invoke static method

    // リターン
    Return,  // Return void
    Ireturn, // Return int

    // フィールドアクセス
    Getstatic(u16), // Get static field

    // 定数
    Dconst0, // Push double 0.0
    Dconst1, // Push double 1.0
}

/// 定数プールエントリ
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
}

/// 定数プール
#[derive(Debug, Clone)]
pub struct ConstantPool {
    entries: Vec<ConstantPoolEntry>,
}

impl ConstantPool {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn add_utf8(&mut self, value: String) -> u16 {
        let index = self.entries.len();
        self.entries.push(ConstantPoolEntry::Utf8(value));
        index as u16 + 1
    }

    pub fn add_class(&mut self, name_index: u16) -> u16 {
        let index = self.entries.len();
        self.entries.push(ConstantPoolEntry::Class(name_index));
        index as u16 + 1
    }

    pub fn add_string(&mut self, utf8_index: u16) -> u16 {
        let index = self.entries.len();
        self.entries.push(ConstantPoolEntry::String(utf8_index));
        index as u16 + 1
    }

    pub fn add_fieldref(&mut self, class_index: u16, name_and_type_index: u16) -> u16 {
        let index = self.entries.len();
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
        index as u16 + 1
    }

    pub fn add_double(&mut self, value: f64) -> u16 {
        let index = self.entries.len();
        self.entries.push(ConstantPoolEntry::Double(value));
        index as u16 + 1
    }

    pub fn entries(&self) -> &Vec<ConstantPoolEntry> {
        &self.entries
    }
}

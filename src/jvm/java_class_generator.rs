use crate::analyzer::SemanticAnalyzer;
use super::jvm_types::{ConstantPool, ConstantPoolEntry, JvmInstruction};
/// Javaクラスファイル生成器
use std::fs;

/// 完全なJavaクラスファイル生成器
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

    /// Dice式からJavaクラスファイルを生成
    pub fn generate_dice_class(
        &mut self,
        expression: &str,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // AST解析
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

    /// Dice式からJVM命令シーケンスを生成（VM実行用）
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

    /// 定数プールをセットアップ
    fn setup_constant_pool(&mut self) {
        // 1: UTF8 - class name
        self.constant_pool.add_utf8(self.class_name.clone());
        // 2: UTF8 - "java/lang/Object"
        self.constant_pool.add_utf8("java/lang/Object".to_string());
        // 3: UTF8 - "main"
        self.constant_pool.add_utf8("main".to_string());
        // 4: UTF8 - "([Ljava/lang/String;)V"
        self.constant_pool
            .add_utf8("([Ljava/lang/String;)V".to_string());
        // 5: UTF8 - "Code"
        self.constant_pool.add_utf8("Code".to_string());
        // 6: UTF8 - "java/lang/System"
        self.constant_pool.add_utf8("java/lang/System".to_string());
        // 7: UTF8 - "out"
        self.constant_pool.add_utf8("out".to_string());
        // 8: UTF8 - "err"
        self.constant_pool.add_utf8("err".to_string());
        // 9: UTF8 - "Ljava/io/PrintStream;"
        self.constant_pool
            .add_utf8("Ljava/io/PrintStream;".to_string());
        // 10: UTF8 - "java/io/PrintStream"
        self.constant_pool
            .add_utf8("java/io/PrintStream".to_string());
        // 11: UTF8 - "println"
        self.constant_pool.add_utf8("println".to_string());
        // 12: UTF8 - "(I)V"
        self.constant_pool.add_utf8("(I)V".to_string());
        // 13: UTF8 - "java/lang/Math"
        self.constant_pool.add_utf8("java/lang/Math".to_string());
        // 14: UTF8 - "random"
        self.constant_pool.add_utf8("random".to_string());
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

    /// Dice用のバイトコードを生成
    fn generate_dice_bytecode(
        &self,
        count: u32,
        faces: u32,
    ) -> Result<Vec<JvmInstruction>, Box<dyn std::error::Error>> {
        let mut instructions = Vec::new();

        if count == 1 {
            // 単一のダイス - Totalを表示しない
            self.generate_single_dice(&mut instructions, faces);
        } else {
            // 複数のダイス - 各結果とTotalを表示
            self.generate_multiple_dice(&mut instructions, count, faces);
        }

        instructions.push(JvmInstruction::Return);
        Ok(instructions)
    }

    /// 単一ダイスのバイトコード生成
    fn generate_single_dice(&self, instructions: &mut Vec<JvmInstruction>, faces: u32) {
        // Math.random() * faces + 1
        instructions.push(JvmInstruction::Invokestatic(35)); // Math.random()
        self.push_double_constant(instructions, faces as f64);
        instructions.push(JvmInstruction::Dmul);
        instructions.push(JvmInstruction::Dconst1);
        instructions.push(JvmInstruction::Dadd);
        instructions.push(JvmInstruction::D2i);

        // System.outに結果を出力
        instructions.push(JvmInstruction::Getstatic(31)); // System.out
        instructions.push(JvmInstruction::Swap);
        instructions.push(JvmInstruction::Invokevirtual(33)); // println(I)V
    }

    /// 複数ダイスのバイトコード生成
    fn generate_multiple_dice(
        &self,
        instructions: &mut Vec<JvmInstruction>,
        count: u32,
        faces: u32,
    ) {
        instructions.push(JvmInstruction::Iconst0); // total = 0

        // 各ダイスを振る
        for _ in 0..count {
            // Math.random() * faces + 1
            instructions.push(JvmInstruction::Invokestatic(35)); // Math.random()
            self.push_double_constant(instructions, faces as f64);
            instructions.push(JvmInstruction::Dmul);
            instructions.push(JvmInstruction::Dconst1);
            instructions.push(JvmInstruction::Dadd);
            instructions.push(JvmInstruction::D2i);

            // 結果を複製（1つは表示用、1つは合計用）
            instructions.push(JvmInstruction::Dup);

            // System.outに個別結果を出力
            instructions.push(JvmInstruction::Getstatic(31)); // System.out
            instructions.push(JvmInstruction::Swap);
            instructions.push(JvmInstruction::Invokevirtual(33)); // println(I)V

            // 合計に加算
            instructions.push(JvmInstruction::Iadd);
        }

        // System.errに"Total: "を出力
        instructions.push(JvmInstruction::Dup); // 合計を複製
        instructions.push(JvmInstruction::Getstatic(32)); // System.err
        instructions.push(JvmInstruction::Ldc(24)); // "Total: "
        instructions.push(JvmInstruction::Invokevirtual(34)); // print(String)V

        // System.errに合計を出力
        instructions.push(JvmInstruction::Getstatic(32)); // System.err
        instructions.push(JvmInstruction::Swap);
        instructions.push(JvmInstruction::Invokevirtual(33)); // println(I)V
        instructions.push(JvmInstruction::Pop); // スタックから残った値を削除
    }

    /// double定数をスタックにプッシュ
    fn push_double_constant(&self, instructions: &mut Vec<JvmInstruction>, value: f64) {
        if value == 0.0 {
            instructions.push(JvmInstruction::Dconst0);
        } else if value == 1.0 {
            instructions.push(JvmInstruction::Dconst1);
        } else {
            // より複雑な定数の場合、整数変換を使用
            let int_val = value as i32;
            self.push_int_constant(instructions, int_val);
            instructions.push(JvmInstruction::I2d);
        }
    }

    /// int定数をスタックにプッシュ
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
                // 大きな値は簡略化
                instructions.push(JvmInstruction::Sipush((value % 32767) as i16));
            }
        }
    }

    /// Javaクラスファイルを生成
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

        // Constant pool count (entries + 1)
        bytes.extend_from_slice(&(self.constant_pool.entries().len() as u16 + 1).to_be_bytes());

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

    /// 定数プールをバイナリ形式で書き込み
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
                    // Placeholder entries for second slot of 8-byte constants
                    // These should not be written to the actual class file
                    // as they don't exist in the JVM spec
                }
            }
        }
    }

    /// mainメソッドを書き込み
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

    /// JVM命令をバイト配列に変換
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
                _ => {
                    // その他の命令は今回は未実装
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

/// 統一されたJVMシステム - Java class file生成
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

/// VM実行用のJVM命令を生成
pub fn generate_vm_instructions(
    expression: &str,
) -> Result<(Vec<JvmInstruction>, ConstantPool), Box<dyn std::error::Error>> {
    let mut generator = JavaClassGenerator::new("DiceRoll".to_string());
    generator.setup_constant_pool();
    let instructions = generator.generate_dice_instructions(expression)?;
    Ok((instructions, generator.constant_pool.clone()))
}

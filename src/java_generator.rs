/// Java class file generation module
use crate::analyzer::SemanticAnalyzer;
use std::fs;

/// Generate Java class file from dice expression
pub fn generate_java_class(
    expression: &str,
    class_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut analyzer = SemanticAnalyzer::new(expression)?;
    let _ast = analyzer.analyze()?;

    let class_bytes = generate_complete_java_class(class_name, expression)?;
    let filename = format!("{class_name}.class");
    fs::write(&filename, &class_bytes)?;

    Ok(())
}

/// Generate complete Java class file
fn generate_complete_java_class(
    class_name: &str,
    expression: &str,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut bytes = Vec::new();

    // Magic number
    bytes.extend_from_slice(&0xCAFEBABEu32.to_be_bytes());

    // Version (Java 8 = 52)
    bytes.extend_from_slice(&0u16.to_be_bytes()); // Minor version
    bytes.extend_from_slice(&52u16.to_be_bytes()); // Major version

    // Constant pool count
    bytes.extend_from_slice(&38u16.to_be_bytes());

    // Constant pool entries
    add_constant_pool(&mut bytes, class_name);

    // Access flags (public class)
    bytes.extend_from_slice(&0x0021u16.to_be_bytes());

    // This class
    bytes.extend_from_slice(&19u16.to_be_bytes());

    // Super class
    bytes.extend_from_slice(&20u16.to_be_bytes());

    // Interfaces count
    bytes.extend_from_slice(&0u16.to_be_bytes());

    bytes.extend_from_slice(&0u16.to_be_bytes());

    bytes.extend_from_slice(&1u16.to_be_bytes());

    bytes.extend_from_slice(&0x0009u16.to_be_bytes());
    bytes.extend_from_slice(&25u16.to_be_bytes());
    bytes.extend_from_slice(&4u16.to_be_bytes());
    bytes.extend_from_slice(&1u16.to_be_bytes());

    bytes.extend_from_slice(&5u16.to_be_bytes());

    let code_data = generate_random_dice_bytecode(expression);
    bytes.extend_from_slice(&(code_data.len() as u32 + 12).to_be_bytes());
    bytes.extend_from_slice(&5u16.to_be_bytes());
    bytes.extend_from_slice(&2u16.to_be_bytes());
    bytes.extend_from_slice(&(code_data.len() as u32).to_be_bytes()); // code_length
    bytes.extend_from_slice(&code_data); // actual bytecode
    bytes.extend_from_slice(&0u16.to_be_bytes()); // exception_table_length
    bytes.extend_from_slice(&0u16.to_be_bytes()); // attributes_count

    // Class attributes count
    bytes.extend_from_slice(&0u16.to_be_bytes());

    Ok(bytes)
}

/// Add constant pool entries
fn add_constant_pool(bytes: &mut Vec<u8>, class_name: &str) {
    // 1: UTF8 - class name
    bytes.push(1); // CONSTANT_Utf8
    bytes.extend_from_slice(&(class_name.len() as u16).to_be_bytes());
    bytes.extend_from_slice(class_name.as_bytes());

    // 2: UTF8 - "java/lang/Object"
    let object_name = "java/lang/Object";
    bytes.push(1);
    bytes.extend_from_slice(&(object_name.len() as u16).to_be_bytes());
    bytes.extend_from_slice(object_name.as_bytes());

    // 3: UTF8 - "main"
    let main_name = "main";
    bytes.push(1);
    bytes.extend_from_slice(&(main_name.len() as u16).to_be_bytes());
    bytes.extend_from_slice(main_name.as_bytes());

    // 4: UTF8 - "([Ljava/lang/String;)V"
    let main_desc = "([Ljava/lang/String;)V";
    bytes.push(1);
    bytes.extend_from_slice(&(main_desc.len() as u16).to_be_bytes());
    bytes.extend_from_slice(main_desc.as_bytes());

    // 5: UTF8 - "Code"
    let code_name = "Code";
    bytes.push(1);
    bytes.extend_from_slice(&(code_name.len() as u16).to_be_bytes());
    bytes.extend_from_slice(code_name.as_bytes());

    // 6: UTF8 - "java/lang/System"
    let system_name = "java/lang/System";
    bytes.push(1);
    bytes.extend_from_slice(&(system_name.len() as u16).to_be_bytes());
    bytes.extend_from_slice(system_name.as_bytes());

    // 7: UTF8 - "out"
    let out_name = "out";
    bytes.push(1);
    bytes.extend_from_slice(&(out_name.len() as u16).to_be_bytes());
    bytes.extend_from_slice(out_name.as_bytes());

    // 8: UTF8 - "err"
    let err_name = "err";
    bytes.push(1);
    bytes.extend_from_slice(&(err_name.len() as u16).to_be_bytes());
    bytes.extend_from_slice(err_name.as_bytes());

    // 9: UTF8 - "Ljava/io/PrintStream;"
    let printstream_desc = "Ljava/io/PrintStream;";
    bytes.push(1);
    bytes.extend_from_slice(&(printstream_desc.len() as u16).to_be_bytes());
    bytes.extend_from_slice(printstream_desc.as_bytes());

    // 10: UTF8 - "java/io/PrintStream"
    let printstream_name = "java/io/PrintStream";
    bytes.push(1);
    bytes.extend_from_slice(&(printstream_name.len() as u16).to_be_bytes());
    bytes.extend_from_slice(printstream_name.as_bytes());

    // 11: UTF8 - "println"
    let println_name = "println";
    bytes.push(1);
    bytes.extend_from_slice(&(println_name.len() as u16).to_be_bytes());
    bytes.extend_from_slice(println_name.as_bytes());

    // 12: UTF8 - "(I)V"
    let println_desc = "(I)V";
    bytes.push(1);
    bytes.extend_from_slice(&(println_desc.len() as u16).to_be_bytes());
    bytes.extend_from_slice(println_desc.as_bytes());

    // 13: UTF8 - "java/lang/Math"
    let math_name = "java/lang/Math";
    bytes.push(1);
    bytes.extend_from_slice(&(math_name.len() as u16).to_be_bytes());
    bytes.extend_from_slice(math_name.as_bytes());

    // 14: UTF8 - "random"
    let random_name = "random";
    bytes.push(1);
    bytes.extend_from_slice(&(random_name.len() as u16).to_be_bytes());
    bytes.extend_from_slice(random_name.as_bytes());

    // 15: UTF8 - "()D"
    let random_desc = "()D";
    bytes.push(1);
    bytes.extend_from_slice(&(random_desc.len() as u16).to_be_bytes());
    bytes.extend_from_slice(random_desc.as_bytes());

    // 16: UTF8 - "Total: "
    let total_str = "Total: ";
    bytes.push(1);
    bytes.extend_from_slice(&(total_str.len() as u16).to_be_bytes());
    bytes.extend_from_slice(total_str.as_bytes());

    // 17: UTF8 - "print"
    let print_name = "print";
    bytes.push(1);
    bytes.extend_from_slice(&(print_name.len() as u16).to_be_bytes());
    bytes.extend_from_slice(print_name.as_bytes());

    // 18: UTF8 - "(Ljava/lang/String;)V"
    let print_desc = "(Ljava/lang/String;)V";
    bytes.push(1);
    bytes.extend_from_slice(&(print_desc.len() as u16).to_be_bytes());
    bytes.extend_from_slice(print_desc.as_bytes());

    // 19: Class - this class
    bytes.push(7); // CONSTANT_Class
    bytes.extend_from_slice(&1u16.to_be_bytes());

    // 20: Class - java/lang/Object
    bytes.push(7);
    bytes.extend_from_slice(&2u16.to_be_bytes());

    // 21: Class - java/lang/System
    bytes.push(7);
    bytes.extend_from_slice(&6u16.to_be_bytes());

    // 22: Class - java/io/PrintStream
    bytes.push(7);
    bytes.extend_from_slice(&10u16.to_be_bytes());

    // 23: Class - java/lang/Math
    bytes.push(7);
    bytes.extend_from_slice(&13u16.to_be_bytes());

    // 24: String - "Total: "
    bytes.push(8); // CONSTANT_String
    bytes.extend_from_slice(&16u16.to_be_bytes());

    // 25: NameAndType - main method
    bytes.push(12); // CONSTANT_NameAndType
    bytes.extend_from_slice(&3u16.to_be_bytes());
    bytes.extend_from_slice(&4u16.to_be_bytes());

    // 26: NameAndType - out field
    bytes.push(12);
    bytes.extend_from_slice(&7u16.to_be_bytes());
    bytes.extend_from_slice(&9u16.to_be_bytes());

    // 27: NameAndType - err field
    bytes.push(12);
    bytes.extend_from_slice(&8u16.to_be_bytes());
    bytes.extend_from_slice(&9u16.to_be_bytes());

    // 28: NameAndType - println method
    bytes.push(12);
    bytes.extend_from_slice(&11u16.to_be_bytes());
    bytes.extend_from_slice(&12u16.to_be_bytes());

    // 29: NameAndType - print method
    bytes.push(12);
    bytes.extend_from_slice(&17u16.to_be_bytes());
    bytes.extend_from_slice(&18u16.to_be_bytes());

    // 30: NameAndType - random method
    bytes.push(12);
    bytes.extend_from_slice(&14u16.to_be_bytes());
    bytes.extend_from_slice(&15u16.to_be_bytes());

    // 31: Fieldref - System.out
    bytes.push(9); // CONSTANT_Fieldref
    bytes.extend_from_slice(&21u16.to_be_bytes());
    bytes.extend_from_slice(&26u16.to_be_bytes());

    // 32: Fieldref - System.err
    bytes.push(9); // CONSTANT_Fieldref
    bytes.extend_from_slice(&21u16.to_be_bytes());
    bytes.extend_from_slice(&27u16.to_be_bytes());

    // 33: Methodref - println
    bytes.push(10); // CONSTANT_Methodref
    bytes.extend_from_slice(&22u16.to_be_bytes());
    bytes.extend_from_slice(&28u16.to_be_bytes());

    // 34: Methodref - print
    bytes.push(10); // CONSTANT_Methodref
    bytes.extend_from_slice(&22u16.to_be_bytes());
    bytes.extend_from_slice(&29u16.to_be_bytes());

    // 35: Methodref - Math.random
    bytes.push(10); // CONSTANT_Methodref
    bytes.extend_from_slice(&23u16.to_be_bytes());
    bytes.extend_from_slice(&30u16.to_be_bytes());

    // 36-37: Padding entries
    for _ in 36..38 {
        bytes.push(1);
        bytes.extend_from_slice(&0u16.to_be_bytes());
    }
}

/// Generate bytecode for random dice rolling
fn generate_random_dice_bytecode(expression: &str) -> Vec<u8> {
    let mut code = Vec::new();

    // Parse dice expression
    if let Ok(mut analyzer) = SemanticAnalyzer::new(expression) {
        if let Ok(ast) = analyzer.analyze() {
            if let Some(stmt) = ast.statement {
                let crate::ast::StatementKind::Expression { expr } = stmt.kind;
                let crate::ast::ExpressionKind::Dice { count, faces } = expr.kind;

                if count == 1 {
                    generate_single_dice_bytecode(&mut code, faces);
                } else {
                    generate_multiple_dice_bytecode(&mut code, count, faces);
                }

                code.push(0xB1); // return
                return code;
            }
        }
    }

    // Default case: return 6
    code.push(0x08); // iconst_5
    code.push(0x04); // iconst_1
    code.push(0x60); // iadd -> 6
    code.push(0xB2); // getstatic System.out
    code.extend_from_slice(&29u16.to_be_bytes());
    code.push(0x5F); // swap
    code.push(0xB6); // invokevirtual println
    code.extend_from_slice(&30u16.to_be_bytes());
    code.push(0xB1); // return
    code
}

/// Generate bytecode for single dice
fn generate_single_dice_bytecode(code: &mut Vec<u8>, faces: u32) {
    // Math.random() * faces + 1
    code.push(0xB8); // invokestatic Math.random
    code.extend_from_slice(&32u16.to_be_bytes());

    push_double_constant(code, faces as f64);
    code.push(0x6B); // dmul
    code.push(0x0F); // dconst_1
    code.push(0x63); // dadd
    code.push(0x8E); // d2i

    // Print dice result
    code.push(0xB2); // getstatic System.out
    code.extend_from_slice(&31u16.to_be_bytes());
    code.push(0x5F); // swap
    code.push(0xB6); // invokevirtual println
    code.extend_from_slice(&33u16.to_be_bytes());
}

/// Generate bytecode for multiple dice
fn generate_multiple_dice_bytecode(code: &mut Vec<u8>, count: u32, faces: u32) {
    code.push(0x03); // iconst_0 (total = 0)

    // Roll each dice
    for _ in 0..count {
        // Math.random() * faces + 1
        code.push(0xB8); // invokestatic Math.random
        code.extend_from_slice(&32u16.to_be_bytes());

        push_double_constant(code, faces as f64);
        code.push(0x6B); // dmul
        code.push(0x0F); // dconst_1
        code.push(0x63); // dadd
        code.push(0x8E); // d2i

        // Print individual dice result
        code.push(0x59); // dup
        code.push(0xB2); // getstatic System.out
        code.extend_from_slice(&31u16.to_be_bytes());
        code.push(0x5F); // swap
        code.push(0xB6); // invokevirtual println
        code.extend_from_slice(&33u16.to_be_bytes());

        // Add to total
        code.push(0x60); // iadd
    }

    // Print "Total: " to stderr
    code.push(0x59); // dup total
    code.push(0xB2); // getstatic System.err
    code.extend_from_slice(&32u16.to_be_bytes());
    code.push(0x12); // ldc "Total: "
    code.push(24u8);
    code.push(0xB6); // invokevirtual print
    code.extend_from_slice(&34u16.to_be_bytes());

    // Print total to stderr
    code.push(0xB2); // getstatic System.err
    code.extend_from_slice(&32u16.to_be_bytes());
    code.push(0x5F); // swap
    code.push(0xB6); // invokevirtual println
    code.extend_from_slice(&33u16.to_be_bytes());
}

/// Push double constant onto stack
fn push_double_constant(code: &mut Vec<u8>, value: f64) {
    if value == 0.0 {
        code.push(0x0E); // dconst_0
    } else if value == 1.0 {
        code.push(0x0F); // dconst_1
    } else if value < 10.0 {
        let int_val = value as i32;
        if int_val <= 5 {
            match int_val {
                0 => code.push(0x0E), // dconst_0
                1 => code.push(0x0F), // dconst_1
                _ => {
                    code.push(0x0F); // dconst_1
                    for _ in 1..int_val {
                        code.push(0x0F); // dconst_1
                        code.push(0x63); // dadd
                    }
                }
            }
        } else {
            push_int_constant(code, int_val);
            code.push(0x87); // i2d
        }
    } else {
        push_int_constant(code, value as i32);
        code.push(0x87); // i2d
    }
}

/// Push integer constant onto stack
fn push_int_constant(code: &mut Vec<u8>, value: i32) {
    match value {
        -1 => code.push(0x02), // iconst_m1
        0 => code.push(0x03),  // iconst_0
        1 => code.push(0x04),  // iconst_1
        2 => code.push(0x05),  // iconst_2
        3 => code.push(0x06),  // iconst_3
        4 => code.push(0x07),  // iconst_4
        5 => code.push(0x08),  // iconst_5
        _ if (-128..=127).contains(&value) => {
            code.push(0x10); // bipush
            code.push(value as u8);
        }
        _ if (-32768..=32767).contains(&value) => {
            code.push(0x11); // sipush
            code.extend_from_slice(&(value as u16).to_be_bytes());
        }
        _ => {
            code.push(0x11); // sipush (clamped)
            code.extend_from_slice(&((value % 32767) as u16).to_be_bytes());
        }
    }
}

use super::class_file_parser::ClassFileParser;
use crate::error::RuntimeError;
use super::jvm_types::{ConstantPool, ConstantPoolEntry, JvmInstruction};
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone, PartialEq)]
pub enum JvmValue {
    Int(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    Reference(Option<usize>),
    ReturnAddress(usize),
}

#[derive(Debug, Clone)]
enum ResolvedMethod {
    PrintStreamPrintln,       // println(I)V
    PrintStreamPrint,         // print(Ljava/lang/String;)V
    PrintStreamPrintlnString, // println(Ljava/lang/String;)V
    MathRandom,               // Math.random()D
    Unknown,
}

impl JvmValue {
    pub fn as_int(&self) -> Result<i32, RuntimeError> {
        match self {
            JvmValue::Int(i) => Ok(*i),
            JvmValue::Float(f) => Ok(*f as i32),
            _ => Err(RuntimeError::InvalidStackState),
        }
    }

    pub fn as_float(&self) -> Result<f32, RuntimeError> {
        match self {
            JvmValue::Int(i) => Ok(*i as f32),
            JvmValue::Float(f) => Ok(*f),
            _ => Err(RuntimeError::InvalidStackState),
        }
    }

    pub fn is_null(&self) -> bool {
        matches!(self, JvmValue::Reference(None))
    }
}

#[derive(Debug, Clone)]
pub struct MethodFrame {
    pub locals: Vec<JvmValue>,
    pub operand_stack: Vec<JvmValue>,
    pub constant_pool: ConstantPool,
    pub pc: usize,
    pub bytecode: Vec<JvmInstruction>,
}

pub struct JvmCompatibleVm {
    frames: Vec<MethodFrame>,
    heap: HashMap<usize, JvmObject>,
    string_data: HashMap<usize, String>,
    next_object_id: usize,
    max_steps: usize,
    steps: usize,
}

#[derive(Debug, Clone)]
pub struct JvmObject {
    pub class_name: String,
    pub fields: HashMap<String, JvmValue>,
}

impl JvmCompatibleVm {
    pub fn new() -> Self {
        Self {
            frames: Vec::new(),
            heap: HashMap::new(),
            string_data: HashMap::new(),
            next_object_id: 1,
            max_steps: 100_000,
            steps: 0,
        }
    }

    fn create_string_object(&mut self, value: String) -> usize {
        let object_id = self.next_object_id;
        self.next_object_id += 1;

        let mut fields = HashMap::new();
        fields.insert("length".to_string(), JvmValue::Int(value.len() as i32));

        let string_object = JvmObject {
            class_name: "java/lang/String".to_string(),
            fields,
        };

        self.heap.insert(object_id, string_object);
        // Store the actual string value
        self.string_data.insert(object_id, value);

        object_id
    }

    pub fn execute_method(
        &mut self,
        bytecode: Vec<JvmInstruction>,
        constant_pool: ConstantPool,
        max_locals: usize,
    ) -> Result<Option<JvmValue>, RuntimeError> {
        let frame = MethodFrame {
            locals: vec![JvmValue::Int(0); max_locals],
            operand_stack: Vec::new(),
            constant_pool,
            pc: 0,
            bytecode,
        };

        self.frames.push(frame);
        self.steps = 0;

        while !self.frames.is_empty() {
            if self.steps >= self.max_steps {
                return Err(RuntimeError::InvalidStackState);
            }

            let result = self.execute_single_instruction()?;
            self.steps += 1;

            if let Some(return_value) = result {
                return Ok(Some(return_value));
            }
        }

        Ok(None)
    }

    /// Execute a Java class file (.class) by parsing it and running the main method
    pub fn execute_class_file(
        &mut self,
        class_file_path: &str,
    ) -> Result<Option<JvmValue>, RuntimeError> {
        // Read the class file
        let class_data = fs::read(class_file_path).map_err(|_| RuntimeError::InvalidStackState)?;

        // Parse the class file
        let class_file = ClassFileParser::parse(&class_data)?;

        // Execute the main method
        self.execute_method(
            class_file.main_method_bytecode,
            class_file.constant_pool,
            class_file.max_locals,
        )
    }

    fn execute_single_instruction(&mut self) -> Result<Option<JvmValue>, RuntimeError> {
        let frame = self
            .frames
            .last_mut()
            .ok_or(RuntimeError::CallStackUnderflow)?;

        if frame.pc >= frame.bytecode.len() {
            self.frames.pop();
            return Ok(None);
        }

        let instruction = frame.bytecode[frame.pc].clone();

        match instruction {
            JvmInstruction::IconstM1 => {
                frame.operand_stack.push(JvmValue::Int(-1));
                frame.pc += 1;
            }
            JvmInstruction::Iconst0 => {
                frame.operand_stack.push(JvmValue::Int(0));
                frame.pc += 1;
            }
            JvmInstruction::Iconst1 => {
                frame.operand_stack.push(JvmValue::Int(1));
                frame.pc += 1;
            }
            JvmInstruction::Iconst2 => {
                frame.operand_stack.push(JvmValue::Int(2));
                frame.pc += 1;
            }
            JvmInstruction::Iconst3 => {
                frame.operand_stack.push(JvmValue::Int(3));
                frame.pc += 1;
            }
            JvmInstruction::Iconst4 => {
                frame.operand_stack.push(JvmValue::Int(4));
                frame.pc += 1;
            }
            JvmInstruction::Iconst5 => {
                frame.operand_stack.push(JvmValue::Int(5));
                frame.pc += 1;
            }
            JvmInstruction::Bipush(value) => {
                frame.operand_stack.push(JvmValue::Int(value as i32));
                frame.pc += 1;
            }
            JvmInstruction::Sipush(value) => {
                frame.operand_stack.push(JvmValue::Int(value as i32));
                frame.pc += 1;
            }
            JvmInstruction::Ldc(index) => {
                let value = self.load_constant_from_pool(index)?;
                let frame = self
                    .frames
                    .last_mut()
                    .ok_or(RuntimeError::CallStackUnderflow)?;
                frame.operand_stack.push(value);
                frame.pc += 1;
            }

            JvmInstruction::Pop => {
                frame
                    .operand_stack
                    .pop()
                    .ok_or(RuntimeError::StackUnderflow)?;
                frame.pc += 1;
            }
            JvmInstruction::Dup => {
                let value = frame
                    .operand_stack
                    .last()
                    .ok_or(RuntimeError::StackUnderflow)?
                    .clone();
                frame.operand_stack.push(value);
                frame.pc += 1;
            }
            JvmInstruction::Swap => {
                let len = frame.operand_stack.len();
                if len < 2 {
                    return Err(RuntimeError::StackUnderflow);
                }
                frame.operand_stack.swap(len - 1, len - 2);
                frame.pc += 1;
            }

            JvmInstruction::Iadd => {
                let b = frame
                    .operand_stack
                    .pop()
                    .ok_or(RuntimeError::StackUnderflow)?
                    .as_int()?;
                let a = frame
                    .operand_stack
                    .pop()
                    .ok_or(RuntimeError::StackUnderflow)?
                    .as_int()?;
                frame.operand_stack.push(JvmValue::Int(a + b));
                frame.pc += 1;
            }
            JvmInstruction::Isub => {
                let b = frame
                    .operand_stack
                    .pop()
                    .ok_or(RuntimeError::StackUnderflow)?
                    .as_int()?;
                let a = frame
                    .operand_stack
                    .pop()
                    .ok_or(RuntimeError::StackUnderflow)?
                    .as_int()?;
                frame.operand_stack.push(JvmValue::Int(a - b));
                frame.pc += 1;
            }
            JvmInstruction::Imul => {
                let b = frame
                    .operand_stack
                    .pop()
                    .ok_or(RuntimeError::StackUnderflow)?
                    .as_int()?;
                let a = frame
                    .operand_stack
                    .pop()
                    .ok_or(RuntimeError::StackUnderflow)?
                    .as_int()?;
                frame.operand_stack.push(JvmValue::Int(a * b));
                frame.pc += 1;
            }
            JvmInstruction::Idiv => {
                let b = frame
                    .operand_stack
                    .pop()
                    .ok_or(RuntimeError::StackUnderflow)?
                    .as_int()?;
                let a = frame
                    .operand_stack
                    .pop()
                    .ok_or(RuntimeError::StackUnderflow)?
                    .as_int()?;
                if b == 0 {
                    return Err(RuntimeError::DivisionByZero);
                }
                frame.operand_stack.push(JvmValue::Int(a / b));
                frame.pc += 1;
            }
            JvmInstruction::Irem => {
                let b = frame
                    .operand_stack
                    .pop()
                    .ok_or(RuntimeError::StackUnderflow)?
                    .as_int()?;
                let a = frame
                    .operand_stack
                    .pop()
                    .ok_or(RuntimeError::StackUnderflow)?
                    .as_int()?;
                if b == 0 {
                    return Err(RuntimeError::DivisionByZero);
                }
                frame.operand_stack.push(JvmValue::Int(a % b));
                frame.pc += 1;
            }

            JvmInstruction::Dadd => {
                let b = frame
                    .operand_stack
                    .pop()
                    .ok_or(RuntimeError::StackUnderflow)?;
                let a = frame
                    .operand_stack
                    .pop()
                    .ok_or(RuntimeError::StackUnderflow)?;

                if let (JvmValue::Double(a_val), JvmValue::Double(b_val)) = (a, b) {
                    frame.operand_stack.push(JvmValue::Double(a_val + b_val));
                } else {
                    return Err(RuntimeError::InvalidStackState);
                }
                frame.pc += 1;
            }
            JvmInstruction::Dsub => {
                let b = frame
                    .operand_stack
                    .pop()
                    .ok_or(RuntimeError::StackUnderflow)?;
                let a = frame
                    .operand_stack
                    .pop()
                    .ok_or(RuntimeError::StackUnderflow)?;

                if let (JvmValue::Double(a_val), JvmValue::Double(b_val)) = (a, b) {
                    frame.operand_stack.push(JvmValue::Double(a_val - b_val));
                } else {
                    return Err(RuntimeError::InvalidStackState);
                }
                frame.pc += 1;
            }
            JvmInstruction::Dmul => {
                let b = frame
                    .operand_stack
                    .pop()
                    .ok_or(RuntimeError::StackUnderflow)?;
                let a = frame
                    .operand_stack
                    .pop()
                    .ok_or(RuntimeError::StackUnderflow)?;

                if let (JvmValue::Double(a_val), JvmValue::Double(b_val)) = (a, b) {
                    frame.operand_stack.push(JvmValue::Double(a_val * b_val));
                } else {
                    return Err(RuntimeError::InvalidStackState);
                }
                frame.pc += 1;
            }
            JvmInstruction::Ddiv => {
                let b = frame
                    .operand_stack
                    .pop()
                    .ok_or(RuntimeError::StackUnderflow)?;
                let a = frame
                    .operand_stack
                    .pop()
                    .ok_or(RuntimeError::StackUnderflow)?;

                if let (JvmValue::Double(a_val), JvmValue::Double(b_val)) = (a, b) {
                    if b_val == 0.0 {
                        return Err(RuntimeError::DivisionByZero);
                    }
                    frame.operand_stack.push(JvmValue::Double(a_val / b_val));
                } else {
                    return Err(RuntimeError::InvalidStackState);
                }
                frame.pc += 1;
            }

            JvmInstruction::I2d => {
                let value = frame
                    .operand_stack
                    .pop()
                    .ok_or(RuntimeError::StackUnderflow)?
                    .as_int()?;
                frame.operand_stack.push(JvmValue::Double(value as f64));
                frame.pc += 1;
            }
            JvmInstruction::D2i => {
                let value = frame
                    .operand_stack
                    .pop()
                    .ok_or(RuntimeError::StackUnderflow)?;

                if let JvmValue::Double(d_val) = value {
                    frame.operand_stack.push(JvmValue::Int(d_val as i32));
                } else {
                    return Err(RuntimeError::InvalidStackState);
                }
                frame.pc += 1;
            }

            JvmInstruction::Goto(offset) => {
                frame.pc = offset as usize;
            }
            JvmInstruction::Ifeq(offset) => {
                let value = frame
                    .operand_stack
                    .pop()
                    .ok_or(RuntimeError::StackUnderflow)?
                    .as_int()?;
                if value == 0 {
                    frame.pc = offset as usize;
                } else {
                    frame.pc += 1;
                }
            }
            JvmInstruction::Ifne(offset) => {
                let value = frame
                    .operand_stack
                    .pop()
                    .ok_or(RuntimeError::StackUnderflow)?
                    .as_int()?;
                if value != 0 {
                    frame.pc = offset as usize;
                } else {
                    frame.pc += 1;
                }
            }
            JvmInstruction::Iflt(offset) => {
                let value = frame
                    .operand_stack
                    .pop()
                    .ok_or(RuntimeError::StackUnderflow)?
                    .as_int()?;
                if value < 0 {
                    frame.pc = offset as usize;
                } else {
                    frame.pc += 1;
                }
            }
            JvmInstruction::Ifge(offset) => {
                let value = frame
                    .operand_stack
                    .pop()
                    .ok_or(RuntimeError::StackUnderflow)?
                    .as_int()?;
                if value >= 0 {
                    frame.pc = offset as usize;
                } else {
                    frame.pc += 1;
                }
            }
            JvmInstruction::Ifgt(offset) => {
                let value = frame
                    .operand_stack
                    .pop()
                    .ok_or(RuntimeError::StackUnderflow)?
                    .as_int()?;
                if value > 0 {
                    frame.pc = offset as usize;
                } else {
                    frame.pc += 1;
                }
            }
            JvmInstruction::Ifle(offset) => {
                let value = frame
                    .operand_stack
                    .pop()
                    .ok_or(RuntimeError::StackUnderflow)?
                    .as_int()?;
                if value <= 0 {
                    frame.pc = offset as usize;
                } else {
                    frame.pc += 1;
                }
            }

            JvmInstruction::Return => {
                self.frames.pop();
            }
            JvmInstruction::Ireturn => {
                let return_value = frame
                    .operand_stack
                    .pop()
                    .ok_or(RuntimeError::StackUnderflow)?;
                self.frames.pop();
                return Ok(Some(return_value));
            }

            JvmInstruction::Getstatic(field_ref) => {
                // Handle System.out and System.err field access
                let field_value = self.resolve_static_field(field_ref)?;
                let frame = self
                    .frames
                    .last_mut()
                    .ok_or(RuntimeError::CallStackUnderflow)?;
                frame.operand_stack.push(field_value);
                frame.pc += 1;
            }

            JvmInstruction::Invokevirtual(method_ref) => {
                // Handle PrintStream.println and PrintStream.print
                self.invoke_virtual_method(method_ref)?;
                let frame = self
                    .frames
                    .last_mut()
                    .ok_or(RuntimeError::CallStackUnderflow)?;
                frame.pc += 1;
            }

            JvmInstruction::Invokestatic(method_ref) => {
                // Handle Math.random and other static methods
                self.invoke_static_method(method_ref)?;
                let frame = self
                    .frames
                    .last_mut()
                    .ok_or(RuntimeError::CallStackUnderflow)?;
                frame.pc += 1;
            }

            JvmInstruction::Dconst0 => {
                frame.operand_stack.push(JvmValue::Double(0.0));
                frame.pc += 1;
            }
            JvmInstruction::Dconst1 => {
                frame.operand_stack.push(JvmValue::Double(1.0));
                frame.pc += 1;
            }
        }

        Ok(None)
    }

    fn resolve_method_reference(&self, method_ref: u16) -> Result<ResolvedMethod, RuntimeError> {
        let frame = self.frames.last().ok_or(RuntimeError::CallStackUnderflow)?;
        let entries = frame.constant_pool.entries();

        // JVM constant pool is 1-based, but our array is 0-based
        let actual_index = (method_ref - 1) as usize;
        if actual_index >= entries.len() {
            return Ok(ResolvedMethod::Unknown);
        }

        match &entries[actual_index] {
            ConstantPoolEntry::Methodref(class_index, name_and_type_index) => {
                // Get class name
                let class_actual_index = (*class_index - 1) as usize;
                let class_name =
                    if let ConstantPoolEntry::Class(name_index) = &entries[class_actual_index] {
                        let name_actual_index = (*name_index - 1) as usize;
                        if let ConstantPoolEntry::Utf8(name) = &entries[name_actual_index] {
                            name
                        } else {
                            return Ok(ResolvedMethod::Unknown);
                        }
                    } else {
                        return Ok(ResolvedMethod::Unknown);
                    };

                // Get method name and descriptor
                let name_and_type_actual_index = (*name_and_type_index - 1) as usize;
                let (method_name, descriptor) =
                    if let ConstantPoolEntry::NameAndType(name_index, desc_index) =
                        &entries[name_and_type_actual_index]
                    {
                        let name_actual_index = (*name_index - 1) as usize;
                        let desc_actual_index = (*desc_index - 1) as usize;
                        let name =
                            if let ConstantPoolEntry::Utf8(name) = &entries[name_actual_index] {
                                name
                            } else {
                                return Ok(ResolvedMethod::Unknown);
                            };
                        let desc =
                            if let ConstantPoolEntry::Utf8(desc) = &entries[desc_actual_index] {
                                desc
                            } else {
                                return Ok(ResolvedMethod::Unknown);
                            };
                        (name, desc)
                    } else {
                        return Ok(ResolvedMethod::Unknown);
                    };

                // Resolve based on class, method name, and descriptor
                match (
                    class_name.as_str(),
                    method_name.as_str(),
                    descriptor.as_str(),
                ) {
                    ("java/io/PrintStream", "println", "(I)V") => {
                        Ok(ResolvedMethod::PrintStreamPrintln)
                    }
                    ("java/io/PrintStream", "print", "(Ljava/lang/String;)V") => {
                        Ok(ResolvedMethod::PrintStreamPrint)
                    }
                    ("java/io/PrintStream", "println", "(Ljava/lang/String;)V") => {
                        Ok(ResolvedMethod::PrintStreamPrintlnString)
                    }
                    ("java/lang/Math", "random", "()D") => Ok(ResolvedMethod::MathRandom),
                    _ => Ok(ResolvedMethod::Unknown),
                }
            }
            _ => Ok(ResolvedMethod::Unknown),
        }
    }

    fn load_constant_from_pool(&mut self, index: u16) -> Result<JvmValue, RuntimeError> {
        let frame = self.frames.last().ok_or(RuntimeError::CallStackUnderflow)?;
        let entries = frame.constant_pool.entries();

        // JVM constant pool is 1-based, but our array is 0-based
        let actual_index = (index - 1) as usize;
        if actual_index >= entries.len() {
            return Err(RuntimeError::InvalidStackState);
        }

        match &entries[actual_index] {
            ConstantPoolEntry::Integer(i) => Ok(JvmValue::Int(*i)),
            ConstantPoolEntry::Float(f) => Ok(JvmValue::Float(*f)),
            ConstantPoolEntry::Long(l) => Ok(JvmValue::Long(*l)),
            ConstantPoolEntry::Double(d) => Ok(JvmValue::Double(*d)),
            ConstantPoolEntry::String(utf8_index) => {
                let utf8_actual_index = (*utf8_index - 1) as usize;
                if let ConstantPoolEntry::Utf8(s) = &entries[utf8_actual_index] {
                    let object_id = self.create_string_object(s.clone());
                    Ok(JvmValue::Reference(Some(object_id)))
                } else {
                    Err(RuntimeError::InvalidStackState)
                }
            }
            _ => Err(RuntimeError::InvalidStackState),
        }
    }

    pub fn debug_state(&self) {
        eprintln!("=== JVM Compatible VM State ===");
        eprintln!("Steps: {}", self.steps);
        eprintln!("Frames: {}", self.frames.len());

        if let Some(frame) = self.frames.last() {
            eprintln!("Current Frame:");
            eprintln!("  PC: {}", frame.pc);
            eprintln!("  Operand Stack: {:?}", frame.operand_stack);
            eprintln!("  Locals: {:?}", frame.locals);
        }

        eprintln!("Heap Objects: {}", self.heap.len());
        eprintln!("==============================");
    }

    fn resolve_static_field(&mut self, field_ref: u16) -> Result<JvmValue, RuntimeError> {
        let frame = self.frames.last().ok_or(RuntimeError::CallStackUnderflow)?;
        let entries = frame.constant_pool.entries();

        // JVM constant pool is 1-based, but our array is 0-based
        let actual_index = (field_ref - 1) as usize;
        if actual_index >= entries.len() {
            // Fallback to old numeric resolution
            return self.resolve_static_field_numeric(field_ref);
        }

        match &entries[actual_index] {
            ConstantPoolEntry::Fieldref(class_index, name_and_type_index) => {
                // Get class name
                let class_actual_index = (*class_index - 1) as usize;
                let class_name =
                    if let ConstantPoolEntry::Class(name_index) = &entries[class_actual_index] {
                        let name_actual_index = (*name_index - 1) as usize;
                        if let ConstantPoolEntry::Utf8(name) = &entries[name_actual_index] {
                            name
                        } else {
                            return self.resolve_static_field_numeric(field_ref);
                        }
                    } else {
                        return self.resolve_static_field_numeric(field_ref);
                    };

                // Get field name
                let name_and_type_actual_index = (*name_and_type_index - 1) as usize;
                let field_name = if let ConstantPoolEntry::NameAndType(name_index, _desc_index) =
                    &entries[name_and_type_actual_index]
                {
                    let name_actual_index = (*name_index - 1) as usize;
                    if let ConstantPoolEntry::Utf8(name) = &entries[name_actual_index] {
                        name
                    } else {
                        return self.resolve_static_field_numeric(field_ref);
                    }
                } else {
                    return self.resolve_static_field_numeric(field_ref);
                };

                // Resolve based on class and field name
                match (class_name.as_str(), field_name.as_str()) {
                    ("java/lang/System", "out") => {
                        let stdout_id = self.create_printstream_object("stdout".to_string());
                        Ok(JvmValue::Reference(Some(stdout_id)))
                    }
                    ("java/lang/System", "err") => {
                        let stderr_id = self.create_printstream_object("stderr".to_string());
                        Ok(JvmValue::Reference(Some(stderr_id)))
                    }
                    _ => self.resolve_static_field_numeric(field_ref),
                }
            }
            _ => self.resolve_static_field_numeric(field_ref),
        }
    }

    fn resolve_static_field_numeric(&mut self, field_ref: u16) -> Result<JvmValue, RuntimeError> {
        // Fallback for numeric field references (for backward compatibility)
        match field_ref {
            31 => {
                // System.out - create a PrintStream object reference
                let stdout_id = self.create_printstream_object("stdout".to_string());
                Ok(JvmValue::Reference(Some(stdout_id)))
            }
            32 => {
                // System.err - create a PrintStream object reference
                let stderr_id = self.create_printstream_object("stderr".to_string());
                Ok(JvmValue::Reference(Some(stderr_id)))
            }
            _ => Err(RuntimeError::InvalidStackState),
        }
    }

    fn create_printstream_object(&mut self, stream_type: String) -> usize {
        let object_id = self.next_object_id;
        self.next_object_id += 1;

        let mut fields = HashMap::new();
        fields.insert("type".to_string(), JvmValue::Reference(None));

        let printstream_object = JvmObject {
            class_name: "java/io/PrintStream".to_string(),
            fields,
        };

        self.heap.insert(object_id, printstream_object);

        // Store the stream type separately for our implementation
        if stream_type == "stderr" {
            self.heap
                .get_mut(&object_id)
                .unwrap()
                .fields
                .insert("is_stderr".to_string(), JvmValue::Int(1));
        } else {
            self.heap
                .get_mut(&object_id)
                .unwrap()
                .fields
                .insert("is_stderr".to_string(), JvmValue::Int(0));
        }

        object_id
    }

    fn invoke_virtual_method(&mut self, method_ref: u16) -> Result<(), RuntimeError> {
        // First try to resolve the method from the constant pool
        let method_info = self.resolve_method_reference(method_ref)?;

        let frame = self
            .frames
            .last_mut()
            .ok_or(RuntimeError::CallStackUnderflow)?;

        match method_info {
            ResolvedMethod::PrintStreamPrintln => {
                // println(I)V
                let value = frame
                    .operand_stack
                    .pop()
                    .ok_or(RuntimeError::StackUnderflow)?
                    .as_int()?;
                let printstream_ref = frame
                    .operand_stack
                    .pop()
                    .ok_or(RuntimeError::StackUnderflow)?;

                if let JvmValue::Reference(Some(obj_id)) = printstream_ref {
                    if let Some(obj) = self.heap.get(&obj_id) {
                        if let Some(JvmValue::Int(is_stderr)) = obj.fields.get("is_stderr") {
                            if *is_stderr == 1 {
                                eprintln!("{}", value);
                            } else {
                                println!("{}", value);
                            }
                        }
                    }
                }
            }
            ResolvedMethod::PrintStreamPrint => {
                // print(Ljava/lang/String;)V
                let string_ref = frame
                    .operand_stack
                    .pop()
                    .ok_or(RuntimeError::StackUnderflow)?;
                let printstream_ref = frame
                    .operand_stack
                    .pop()
                    .ok_or(RuntimeError::StackUnderflow)?;

                if let (
                    JvmValue::Reference(Some(string_id)),
                    JvmValue::Reference(Some(stream_id)),
                ) = (string_ref, printstream_ref)
                {
                    // Get the actual string value from our string data storage
                    if let Some(string_value) = self.string_data.get(&string_id) {
                        if let Some(stream_obj) = self.heap.get(&stream_id) {
                            if let Some(JvmValue::Int(is_stderr)) =
                                stream_obj.fields.get("is_stderr")
                            {
                                if *is_stderr == 1 {
                                    eprint!("{}", string_value);
                                } else {
                                    print!("{}", string_value);
                                }
                            }
                        }
                    }
                }
            }
            ResolvedMethod::PrintStreamPrintlnString => {
                // println(Ljava/lang/String;)V
                let string_ref = frame
                    .operand_stack
                    .pop()
                    .ok_or(RuntimeError::StackUnderflow)?;
                let printstream_ref = frame
                    .operand_stack
                    .pop()
                    .ok_or(RuntimeError::StackUnderflow)?;

                if let (
                    JvmValue::Reference(Some(string_id)),
                    JvmValue::Reference(Some(stream_id)),
                ) = (string_ref, printstream_ref)
                {
                    // Get the actual string value from our string data storage
                    if let Some(string_value) = self.string_data.get(&string_id) {
                        if let Some(stream_obj) = self.heap.get(&stream_id) {
                            if let Some(JvmValue::Int(is_stderr)) =
                                stream_obj.fields.get("is_stderr")
                            {
                                if *is_stderr == 1 {
                                    eprintln!("{}", string_value);
                                } else {
                                    println!("{}", string_value);
                                }
                            }
                        }
                    }
                }
            }
            ResolvedMethod::MathRandom => {
                // Math.random()D - This shouldn't be called in invoke_virtual_method
                // but we need to handle it for completeness
                return Err(RuntimeError::InvalidStackState);
            }
            ResolvedMethod::Unknown => {
                // Fallback for unknown methods - attempt old numeric resolution
                match method_ref {
                    33 => {
                        // println(I)V
                        let value = frame
                            .operand_stack
                            .pop()
                            .ok_or(RuntimeError::StackUnderflow)?
                            .as_int()?;
                        let printstream_ref = frame
                            .operand_stack
                            .pop()
                            .ok_or(RuntimeError::StackUnderflow)?;

                        if let JvmValue::Reference(Some(obj_id)) = printstream_ref {
                            if let Some(obj) = self.heap.get(&obj_id) {
                                if let Some(JvmValue::Int(is_stderr)) = obj.fields.get("is_stderr")
                                {
                                    if *is_stderr == 1 {
                                        eprintln!("{}", value);
                                    } else {
                                        println!("{}", value);
                                    }
                                }
                            }
                        }
                    }
                    34 => {
                        // print(Ljava/lang/String;)V
                        let string_ref = frame
                            .operand_stack
                            .pop()
                            .ok_or(RuntimeError::StackUnderflow)?;
                        let printstream_ref = frame
                            .operand_stack
                            .pop()
                            .ok_or(RuntimeError::StackUnderflow)?;

                        if let (
                            JvmValue::Reference(Some(string_id)),
                            JvmValue::Reference(Some(stream_id)),
                        ) = (string_ref, printstream_ref)
                        {
                            if let Some(string_value) = self.string_data.get(&string_id) {
                                if let Some(stream_obj) = self.heap.get(&stream_id) {
                                    if let Some(JvmValue::Int(is_stderr)) =
                                        stream_obj.fields.get("is_stderr")
                                    {
                                        if *is_stderr == 1 {
                                            eprint!("{}", string_value);
                                        } else {
                                            print!("{}", string_value);
                                        }
                                    }
                                }
                            }
                        }
                    }
                    35 => {
                        // println(Ljava/lang/String;)V
                        let string_ref = frame
                            .operand_stack
                            .pop()
                            .ok_or(RuntimeError::StackUnderflow)?;
                        let printstream_ref = frame
                            .operand_stack
                            .pop()
                            .ok_or(RuntimeError::StackUnderflow)?;

                        if let (
                            JvmValue::Reference(Some(string_id)),
                            JvmValue::Reference(Some(stream_id)),
                        ) = (string_ref, printstream_ref)
                        {
                            if let Some(string_value) = self.string_data.get(&string_id) {
                                if let Some(stream_obj) = self.heap.get(&stream_id) {
                                    if let Some(JvmValue::Int(is_stderr)) =
                                        stream_obj.fields.get("is_stderr")
                                    {
                                        if *is_stderr == 1 {
                                            eprintln!("{}", string_value);
                                        } else {
                                            println!("{}", string_value);
                                        }
                                    }
                                }
                            }
                        }
                    }
                    _ => return Err(RuntimeError::InvalidStackState),
                }
            }
        }

        Ok(())
    }

    fn invoke_static_method(&mut self, method_ref: u16) -> Result<(), RuntimeError> {
        // First try to resolve the method from the constant pool
        let method_info = self.resolve_method_reference(method_ref)?;

        let frame = self
            .frames
            .last_mut()
            .ok_or(RuntimeError::CallStackUnderflow)?;

        match method_info {
            ResolvedMethod::MathRandom => {
                // Math.random()D
                use rand::Rng;
                let mut rng = rand::rng();
                let random_value = rng.random::<f64>();
                frame.operand_stack.push(JvmValue::Double(random_value));
            }
            ResolvedMethod::Unknown => {
                // Fallback for unknown methods - attempt old numeric resolution
                match method_ref {
                    36 => {
                        // Math.random()D
                        use rand::Rng;
                        let mut rng = rand::rng();
                        let random_value = rng.random::<f64>();
                        frame.operand_stack.push(JvmValue::Double(random_value));
                    }
                    _ => return Err(RuntimeError::InvalidStackState),
                }
            }
            _ => {
                // Other resolved methods shouldn't be called via invokestatic
                return Err(RuntimeError::InvalidStackState);
            }
        }

        Ok(())
    }
}

impl Default for JvmCompatibleVm {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_arithmetic() {
        let mut vm = JvmCompatibleVm::new();
        let bytecode = vec![
            JvmInstruction::Iconst2, // Push 2
            JvmInstruction::Iconst3, // Push 3
            JvmInstruction::Iadd,    // Add
            JvmInstruction::Ireturn, // Return result
        ];

        let constant_pool = ConstantPool::new();
        let result = vm.execute_method(bytecode, constant_pool, 0).unwrap();

        assert_eq!(result, Some(JvmValue::Int(5)));
    }

    #[test]
    fn test_conditional_jump() {
        let mut vm = JvmCompatibleVm::new();
        let bytecode = vec![
            JvmInstruction::Iconst0, // Push 0
            JvmInstruction::Ifne(5), // Jump to index 5 if not zero (won't jump)
            JvmInstruction::Iconst1, // Push 1
            JvmInstruction::Ireturn, // Return 1
            JvmInstruction::Iconst2, // Push 2 (unreachable)
            JvmInstruction::Ireturn, // Return 2 (unreachable)
        ];

        let constant_pool = ConstantPool::new();
        let result = vm.execute_method(bytecode, constant_pool, 0).unwrap();

        assert_eq!(result, Some(JvmValue::Int(1)));
    }

    #[test]
    fn test_jvm_compatible_hello_world() {
        let mut vm = JvmCompatibleVm::new();
        let mut constant_pool = ConstantPool::new();

        // Set up constant pool for proper JVM-compatible Hello World
        let hello_utf8 = constant_pool.add_utf8("Hello, World!".to_string());
        let hello_string = constant_pool.add_string(hello_utf8);

        let system_utf8 = constant_pool.add_utf8("java/lang/System".to_string());
        let system_class = constant_pool.add_class(system_utf8);

        let out_utf8 = constant_pool.add_utf8("out".to_string());
        let printstream_desc_utf8 = constant_pool.add_utf8("Ljava/io/PrintStream;".to_string());
        let out_name_and_type = constant_pool.add_name_and_type(out_utf8, printstream_desc_utf8);
        let system_out_field = constant_pool.add_fieldref(system_class, out_name_and_type);

        let printstream_utf8 = constant_pool.add_utf8("java/io/PrintStream".to_string());
        let printstream_class = constant_pool.add_class(printstream_utf8);

        let println_utf8 = constant_pool.add_utf8("println".to_string());
        let println_desc_utf8 = constant_pool.add_utf8("(Ljava/lang/String;)V".to_string());
        let println_name_and_type =
            constant_pool.add_name_and_type(println_utf8, println_desc_utf8);
        let println_method = constant_pool.add_methodref(printstream_class, println_name_and_type);

        let bytecode = vec![
            JvmInstruction::Getstatic(system_out_field), // Get System.out
            JvmInstruction::Ldc(hello_string),           // Load "Hello, World!" string
            JvmInstruction::Invokevirtual(println_method), // Call println
            JvmInstruction::Return,                      // Return
        ];

        // This should execute without errors and print "Hello, World!"
        let result = vm.execute_method(bytecode, constant_pool, 0);
        match &result {
            Ok(_) => {}
            Err(e) => eprintln!("Error: {:?}", e),
        }
        assert!(result.is_ok());
    }

    #[test]
    fn test_math_random_compatibility() {
        let mut vm = JvmCompatibleVm::new();
        let mut constant_pool = ConstantPool::new();

        // Set up constant pool for Math.random()
        let math_utf8 = constant_pool.add_utf8("java/lang/Math".to_string());
        let math_class = constant_pool.add_class(math_utf8);

        let random_utf8 = constant_pool.add_utf8("random".to_string());
        let random_desc_utf8 = constant_pool.add_utf8("()D".to_string());
        let random_name_and_type = constant_pool.add_name_and_type(random_utf8, random_desc_utf8);
        let random_method = constant_pool.add_methodref(math_class, random_name_and_type);

        let bytecode = vec![
            JvmInstruction::Invokestatic(random_method), // Call Math.random()
            JvmInstruction::Return,                      // Return (void method for test)
        ];

        let result = vm.execute_method(bytecode, constant_pool, 0);
        assert!(result.is_ok());

        // Check that there's a double value on the stack (but we return void, so won't get it)
        // The fact that it executes without error means the method resolution worked
    }
}

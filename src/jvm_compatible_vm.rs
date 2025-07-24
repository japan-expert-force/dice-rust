use crate::error::RuntimeError;
use crate::unified_jvm::{ConstantPool, ConstantPoolEntry, JvmInstruction};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum JvmValue {
    Int(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    Reference(Option<usize>),
    ReturnAddress(usize),
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

    fn load_constant_from_pool(&mut self, index: u16) -> Result<JvmValue, RuntimeError> {
        let frame = self.frames.last().ok_or(RuntimeError::CallStackUnderflow)?;
        let entries = frame.constant_pool.entries();

        if index as usize >= entries.len() {
            return Err(RuntimeError::InvalidStackState);
        }

        match &entries[index as usize] {
            ConstantPoolEntry::Integer(i) => Ok(JvmValue::Int(*i)),
            ConstantPoolEntry::Float(f) => Ok(JvmValue::Float(*f)),
            ConstantPoolEntry::Long(l) => Ok(JvmValue::Long(*l)),
            ConstantPoolEntry::Double(d) => Ok(JvmValue::Double(*d)),
            ConstantPoolEntry::String(utf8_index) => {
                if let ConstantPoolEntry::Utf8(s) = &entries[*utf8_index as usize] {
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
        // For demo purposes, assume field_ref 31 = System.out, 32 = System.err
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
        let frame = self
            .frames
            .last_mut()
            .ok_or(RuntimeError::CallStackUnderflow)?;

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
                    JvmValue::Reference(Some(_string_id)),
                    JvmValue::Reference(Some(stream_id)),
                ) = (string_ref, printstream_ref)
                {
                    // Get the string from constant pool or heap
                    if method_ref == 34 {
                        // Assuming this is "Total: " string
                        if let Some(stream_obj) = self.heap.get(&stream_id) {
                            if let Some(JvmValue::Int(is_stderr)) =
                                stream_obj.fields.get("is_stderr")
                            {
                                if *is_stderr == 1 {
                                    eprint!("Total: ");
                                } else {
                                    print!("Total: ");
                                }
                            }
                        }
                    }
                }
            }
            _ => return Err(RuntimeError::InvalidStackState),
        }

        Ok(())
    }

    fn invoke_static_method(&mut self, method_ref: u16) -> Result<(), RuntimeError> {
        let frame = self
            .frames
            .last_mut()
            .ok_or(RuntimeError::CallStackUnderflow)?;

        match method_ref {
            35 => {
                // Math.random()D
                use rand::Rng;
                let mut rng = rand::rng();
                let random_value = rng.random::<f64>();
                frame.operand_stack.push(JvmValue::Double(random_value));
            }
            _ => return Err(RuntimeError::InvalidStackState),
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
}

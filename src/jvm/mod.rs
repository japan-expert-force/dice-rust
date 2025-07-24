/// JVM関連のモジュール
pub mod class_file_parser;
pub mod java_class_generator;
pub mod jvm_compatible_vm;
pub mod jvm_types;

// 公開API
pub use class_file_parser::ClassFileParser;
pub use java_class_generator::{generate_java_class, generate_vm_instructions};
pub use jvm_compatible_vm::JvmCompatibleVm;
pub use jvm_types::{ConstantPool, ConstantPoolEntry, JvmInstruction};

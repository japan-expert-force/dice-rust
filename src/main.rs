use clap::{Parser, Subcommand};
use dice_rust::{jvm, stack_vm::StackVm};

fn generate_and_execute_jvm_bytecode(
    expression: &str,
    verbose: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    // Get instructions and constant pool from the JVM class generator
    let (instructions, constant_pool) = jvm::generate_vm_instructions(expression)?;

    let mut vm = jvm::JvmCompatibleVm::new();
    vm.set_verbose(verbose);
    vm.execute_method(instructions, constant_pool, 10)?;

    Ok(())
}

#[derive(Parser)]
#[command(name = "dice-rust")]
#[command(about = "A dice rolling language interpreter with multi-VM support")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Run dice expressions using stack VM or JVM-compatible VM")]
    Run {
        #[arg(value_name = "EXPRESSION")]
        expression: String,
        #[arg(
            long,
            help = "Use JVM-compatible virtual machine instead of the default stack VM"
        )]
        jvm: bool,
        #[arg(short, long, help = "Enable verbose output for debugging")]
        verbose: bool,
    },
    #[command(about = "Compile dice expressions to Java class files")]
    Compile {
        #[arg(value_name = "EXPRESSION")]
        expression: String,
        #[arg(short, long, default_value = "DiceRoll")]
        output: String,
        #[arg(short, long, help = "Enable verbose output for debugging")]
        verbose: bool,
    },
    #[command(about = "Execute compiled Java class files")]
    Execute {
        #[arg(value_name = "CLASS_FILE")]
        class_file: String,
        #[arg(short, long, help = "Enable verbose output for debugging")]
        verbose: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run {
            expression,
            jvm,
            verbose,
        } => {
            if jvm {
                // Generate bytecode and execute on JVM-compatible VM
                match generate_and_execute_jvm_bytecode(&expression, verbose) {
                    Ok(()) => (),
                    Err(e) => eprintln!("JVM VM Error: {e}"),
                }
            } else {
                let mut stack_vm = StackVm::new();
                match stack_vm.execute(&expression) {
                    Ok(()) => (),
                    Err(e) => eprintln!("Error occurred: {e}"),
                }
            }
        }
        Commands::Compile {
            expression,
            output,
        } => {
            if let Err(e) = jvm::generate_java_class(&expression, &output) {
                eprintln!("Java class generation error: {e}");
            }
        }
        Commands::Execute {
            class_file,
            verbose,
        } => {
            let mut vm = jvm::JvmCompatibleVm::new();
            vm.set_verbose(verbose);
            match vm.execute_class_file(&class_file) {
                Ok(_) => (),
                Err(e) => eprintln!("JVM execution error: {e:?}"),
            }
        }
    }
}

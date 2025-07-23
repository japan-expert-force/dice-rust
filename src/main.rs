use clap::{Parser, Subcommand};
use dice_rust::stack_vm::StackVm;

#[derive(Parser)]
#[command(name = "dice-rust")]
#[command(about = "A dice rolling language interpreter")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run dice expression using stack VM
    Run {
        /// Dice expression to evaluate
        #[arg(value_name = "EXPRESSION", default_value = "2d100")]
        expression: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { expression } => {
            // VM mode: interpret using stack VM
            let mut stack_vm = StackVm::new();
            match stack_vm.execute(&expression) {
                Ok(()) => (),
                Err(e) => eprintln!("Error occurred: {e}"),
            }
        }
    }
}

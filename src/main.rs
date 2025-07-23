use dice_rust::vm::StackVm;

fn main() {
    let input = "2d100";
    let mut stack_vm = StackVm::new();
    match stack_vm.execute(&input) {
        Ok(()) => (),
        Err(e) => eprintln!("Error occurred: {}", e),
    }
}

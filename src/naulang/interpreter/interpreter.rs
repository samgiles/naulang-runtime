use naulang::interpreter::frame::Frame;
use naulang::interpreter::bytecode::Bytecode;
use naulang::interpreter::task::Task;


pub fn interpreter_step(task: &mut Task) -> bool {
    match task.top_frame {
        Some(frame) => run_interpreter_step(frame, task),
        None => false
    }
}

fn run_interpreter_step(frame: &Frame, task: &mut Task) -> bool {
    let bytecode = frame.method.get_bytecode(frame.pc);

    match bytecode {
        Bytecode::HALT => task.restore_previous_frame_or_halt(),
        _ => false // Not implemented
    }
}

#[cfg(test)]
mod tests {
    use super::interpreter_step;
    use naulang::objectspace::method::MethodObject;
    use naulang::interpreter::frame::{Frame, FrameInfo};
    use naulang::interpreter::task::{Task, TaskState};
    use naulang::interpreter::bytecode::Bytecode;

    #[test]
    fn test_interepreter_step_task_halt() {
        let halting_method = MethodObject::new(vec!(Bytecode::HALT));
        let stack_root = Frame::new(FrameInfo {
            stack_depth: 0,
            local_count: 0,
            literal_count: 0,
            method: &halting_method
        });

        let mut task = Task::new_withframe(&stack_root, Option::None);

        let step_result = interpreter_step(&mut task);

        assert!(!step_result);
        assert!(task.state == TaskState::Halt);

    }
}

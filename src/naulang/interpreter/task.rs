use naulang::interpreter::frame::Frame;

#[derive(PartialEq)]
pub enum TaskState {
    Continue,
    Halt,
    Yield,
    Suspend,
}

/// Represents a independently running function and its call stack
pub struct Task<'task> {
    /// The current state of this task as TaskState
    pub state:       TaskState,

    /// Represents the top of the current stack this task is running.
    pub top_frame:   Option<&'task Frame<'task>>,

    /// The task that spawned this task
    parent_task: Option<Box<Task<'task>>>,
}

impl<'task> Task<'task> {
    pub fn new() -> Task<'task> {
        Task {
            state: TaskState::Continue,
            top_frame: Option::None,
            parent_task: Option::None,
        }
    }

    pub fn new_withframe(frame: &'task Box<Frame>, parent_task: Option<Box<Task<'task>>>) -> Task<'task> {
        Task {
            state: TaskState::Continue,
            top_frame: Some(frame),
            parent_task: parent_task
        }
    }

    pub fn restore_previous_frame_or_halt(&mut self) -> bool {
        let task_still_running = match self.top_frame {
            Some(top_frame) => {
                match top_frame.previous_frame {
                    Some(ref previous_frame) => {
                        self.top_frame = Some(previous_frame);
                        true
                    }
                    None => false
                }
            },
            None => false
        };

        self.state = TaskState::Halt;
        return task_still_running
    }

    pub fn set_parent_task(&mut self, task: Task<'task>) -> () {
        self.parent_task =  Some(Box::new(task));
    }
}


#[cfg(test)]
mod tests {
    use super::{Task, TaskState};
    use naulang::interpreter::frame::{Frame, FrameInfo};
    use naulang::objectspace::method::{MethodObject};
    use std::ops::Deref;

    #[test]
    fn test_new_task_state() {
        assert!(Task::new().state == TaskState::Continue);
    }

    #[test]
    fn test_restore_previous_frame() {
        let method = MethodObject::new_stub();

        let stack_root = Frame::new(FrameInfo {
            stack_depth: 1,
            local_count: 1,
            literal_count: 1,
            method: &method,
        });

        let mut next_stack = Frame::new(FrameInfo {
            stack_depth: 1,
            local_count: 1,
            literal_count: 1,
            method: &method,
        });

        next_stack.previous_frame = Some(stack_root);

        {
            let top_frame = next_stack;
            let mut task = Task::new_withframe(&top_frame, None);
            let frame_restored = task.restore_previous_frame_or_halt();
            assert!(frame_restored);
            let frame_restored = task.restore_previous_frame_or_halt();
            assert!(!frame_restored);
            assert!(task.state == TaskState::Halt);
        }
    }

    #[test]
    fn test_set_parent_task() {
        let mut root_task = Task::new();

        // Create new Task in short lived scope
        {
            // TODO: Proper initialisation, here we're mutating, and then later
            // moving immutable
            let mut next_task = Task::new();
            (*&mut next_task.state) = TaskState::Halt;

            let next_task = next_task;
            root_task.set_parent_task(next_task);
        }

        let task_is_set = match root_task.parent_task {
            Some(rc_task) => rc_task.deref().state == TaskState::Halt,
            None => false
        };

        assert!(task_is_set);
    }
}

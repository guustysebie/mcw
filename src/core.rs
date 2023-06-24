use std::cell::RefCell;

pub struct McwContext {
    pub base_path: String,
    pub repositories:RefCell<Vec<String>>,
    pub mcw_command: Option<Box<dyn McwCommand>>,
    pub verbose: bool,
}

pub trait McwCommand {
    fn execute(&self, context: &McwContext);
}
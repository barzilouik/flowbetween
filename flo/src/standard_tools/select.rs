use super::super::tools::*;

use ui::*;
use animation::*;

use std::sync::*;

///
/// The Select tool (Selects control points of existing objects)
/// 
pub struct Select { }

impl Select {
    ///
    /// Creates a new instance of the Select tool
    /// 
    pub fn new() -> Select {
        Select {}
    }
}

impl<Anim: Animation> Tool<Anim> for Select {
    fn tool_name(&self) -> String { "Select".to_string() }

    fn image_name(&self) -> String { "select".to_string() }

    fn paint<'a>(&self, _model: &ToolModel<'a, Anim>, _device: &PaintDevice, _actions: &Vec<Painting>) {
        
    }
}

impl<Anim: Animation> Tool2<(), Anim> for Select {
    fn tool_name(&self) -> String { "Select".to_string() }

    fn image_name(&self) -> String { "select".to_string() }

    fn actions_for_input<'a>(&self, _data: Option<Arc<()>>, _input: Box<'a+Iterator<Item=ToolInput<()>>>) -> Box<Iterator<Item=ToolAction<()>>> {
        Box::new(vec![].into_iter())
    }
}
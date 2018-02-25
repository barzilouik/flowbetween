use super::super::tools::*;

use ui::*;
use animation::*;

use std::sync::*;

///
/// The Pan tool (Pans control points of existing objects)
/// 
pub struct Pan { }

impl Pan {
    ///
    /// Creates a new instance of the Pan tool
    /// 
    pub fn new() -> Pan {
        Pan {}
    }
}

impl<Anim: Animation> Tool<Anim> for Pan {
    fn tool_name(&self) -> String { "Pan".to_string() }

    fn image_name(&self) -> String { "pan".to_string() }

    fn paint<'a>(&self, _model: &ToolModel<'a, Anim>, _device: &PaintDevice, _actions: &Vec<Painting>) {
        
    }
}

impl<Anim: Animation> Tool2<(), Anim> for Pan {
    fn tool_name(&self) -> String { "Pan".to_string() }

    fn image_name(&self) -> String { "pan".to_string() }

    fn actions_for_input<'a>(&'a self, _data: Option<Arc<()>>, _input: Box<'a+Iterator<Item=ToolInput<()>>>) -> Box<Iterator<Item=ToolAction<()>>> {
        Box::new(vec![].into_iter())
    }
}
use super::tool_input::*;
use super::tool_action::*;
use super::generic_tool::*;
use super::super::model::*;

use animation::*;

use futures::*;
use futures::executor;
use futures::executor::{Spawn, Notify, NotifyHandle};
use std::sync::*;

///
/// Runs the actions for a particular tool
/// 
pub struct ToolRunner<Anim: Animation> {
    /// The view model that is passed to the tools
    view_model: Arc<FloModel<Anim>>,

    /// The currently active tool
    current_tool: Option<Arc<FloTool<Anim>>>,

    /// Most recent tool data from the current tool
    tool_data: Option<Arc<GenericToolData>>,

    /// The model actions specified by the current tool
    model_actions: Option<Spawn<Box<Stream<Item=ToolAction<GenericToolData>, Error=()>+Send>>>
}

impl<Anim: Animation> ToolRunner<Anim> {
    ///
    /// Creates a new tool runner
    /// 
    pub fn new(view_model: &FloModel<Anim>) -> ToolRunner<Anim> {
        let view_model = Arc::new(view_model.clone());

        ToolRunner {
            view_model:     view_model,
            current_tool:   None,
            tool_data:      None,
            model_actions:  None
        }
    }

    ///
    /// Sets the tool that this will use to run its actions on
    /// 
    pub fn set_tool(&mut self, new_tool: &Arc<FloTool<Anim>>, tool_model: &GenericToolModel) {
        // Free the data for the current tool
        self.tool_data      = None;
        self.model_actions  = None;

        // Set the new tool
        let model_actions   = new_tool.actions_for_model(Arc::clone(&self.view_model), tool_model);
        self.current_tool   = Some(Arc::clone(&new_tool));

        // Start the actions running for the new tool
        self.model_actions  = Some(executor::spawn(model_actions));
    }

    ///
    /// Updates the data that will be used for the tool
    /// 
    pub fn set_tool_data(&mut self, new_data: GenericToolData) {
        self.tool_data = Some(Arc::new(new_data));
    }

    ///
    /// Returns the pending model actions for this object
    /// 
    pub fn model_actions(&mut self) -> Box<Iterator<Item=ToolAction<GenericToolData>>> {
        // Flush any pending actions from the model actions stream
        let mut flushed_actions = vec![];

        if let Some(ref mut model_actions) = self.model_actions {
            // TODO: close the stream if this returns None (existing tools generate infinite streams so this doesn't happen)
            while let Ok(Async::Ready(Some(action))) = model_actions.poll_stream_notify(&NotifyHandle::from(&NotifyNothing), 0) {
                flushed_actions.push(action);
            }
        }

        // Pass the remaining actions to the caller
        Box::new(flushed_actions.into_iter())
    }

    ///
    /// Given a set of tool inputs, returns an iterator that specifies the resulting tool actions
    /// 
    /// If there are any actions resulting from a change in model state, these are also returned here
    /// 
    pub fn actions_for_input<Iter: Iterator<Item=ToolInput<GenericToolData>>>(&mut self, input: Iter) -> Box<Iterator<Item=ToolAction<GenericToolData>>> {
        // Create a place to store the updated tool data for this request
        let mut new_tool_data = None;

        // Before processing the input actions, generate the list of model actions
        let model_actions = self.model_actions();

        // Process any data updates generated by the model actions
        let mut after_processing_data   = vec![];
        let mut data_input              = vec![];
        for action in model_actions {
            match action {
                ToolAction::Data(new_data)  => {
                    // Data changes caused by model changes cause the 'Data' event to be generated
                    let new_data = Arc::new(new_data);
                    data_input.push(ToolInput::Data(Arc::clone(&new_data)));
                    self.tool_data = Some(new_data);
                },
                action                      => after_processing_data.push(action)
            }
        }

        if let Some(ref tool) = self.current_tool {
            // Chain the data (after model actions) with the supplied input
            let input = data_input.into_iter().chain(input);
            let input = Box::new(input);

            // Call the tool to get the actions
            let tool_actions = tool.actions_for_input(self.tool_data.clone(), input);

            // Process any data actions and return the remainder
            for action in tool_actions {
                match action {
                    ToolAction::Data(new_data)  => new_tool_data = Some(Arc::new(new_data)),
                    action                      => after_processing_data.push(action)
                }
            }
        }

        // Update the tool data stored in this object
        if let Some(new_tool_data) = new_tool_data {
            // TODO: next set of actions should probably get the 'Data' event as a result of this
            self.tool_data = Some(new_tool_data);
        }

        // The 'after processing' vec forms the result
        Box::new(after_processing_data.into_iter())
    }
}

///
/// Notification object that doesn't actually notify anything
/// 
#[derive(Clone)]
struct NotifyNothing;

impl Notify for NotifyNothing {
    fn notify(&self, _id: usize) { }
}
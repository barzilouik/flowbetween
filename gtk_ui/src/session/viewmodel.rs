use super::super::gtk_action::*;

use flo_ui::*;

use std::collections::HashMap;
use std::collections::hash_map::Entry;

///
/// Tracks and generates events for viewmodel changes in GTK
/// 
pub struct GtkSessionViewModel {
    /// Values set in the viewmodel (controller path to properties)
    values: HashMap<Vec<String>, HashMap<String, PropertyValue>>
}

impl GtkSessionViewModel {
    ///
    /// Creates a new GTK sesion viewmodel, which will send events to the specified sink
    /// 
    pub fn new() -> GtkSessionViewModel {
        GtkSessionViewModel {
            values:         HashMap::new()
        }
    }

    ///
    /// Binds a property to an action to be performed every time it's changed
    /// 
    pub fn bind(&mut self, widget_id: WidgetId, controller_path: &Vec<String>, property: &Property, action_fn: Box<Fn(PropertyValue) -> GtkWidgetAction>) -> Vec<GtkWidgetAction> {
        println!("Bind {:?} -> {:?}", controller_path, property);

        vec![ GtkWidgetAction::Content(WidgetContent::SetText("Not implemented".to_string())) ]
    }

    ///
    /// Update the viewmodel with values from some updates
    /// 
    pub fn update(&mut self, updates: Vec<ViewModelUpdate>) -> Vec<GtkAction> {
        for controller_update in updates {
            // Each update is a set of changes to a particular controller
            let mut property_values = self.values.entry(controller_update.controller_path().clone()).or_insert_with(|| HashMap::new());

            // Process each update in turn
            for &(ref property_name, ref property_value) in controller_update.updates() {
                // Update the property in the model
                let property_changed = {
                    match property_values.entry(property_name.clone()) {
                        Entry::Occupied(mut occupied)  => {
                            if occupied.get() == property_value {
                                // Entry exists but is unchanged
                                false
                            } else {
                                // Entry exists and is changed
                                *occupied.get_mut() = property_value.clone();
                                true
                            }
                        },

                        Entry::Vacant(vacant) => {
                            // Create a new entry
                            vacant.insert(property_value.clone());
                            true
                        }
                    }
                };

                // If the property is changed, generate the events to send to the GTK sink
            }
        }

        // Result is the actions generated by the property change
        vec![]
    }
}

use binding::*;
use binding::binding_context::*;

use super::property::*;
use super::viewmodel::*;

use std::sync::*;
use std::collections::HashMap;

///
/// The dynamic viewmodel lets us define arbitrary properties as bound or
/// computed values. A particular key can only be bound or computed: if it
/// is set as both, the computed version 'wins'. 
///
pub struct DynamicViewModel {
    /// Maps bindings in this viewmodel to their values
    bindings: Mutex<HashMap<String, Arc<Binding<PropertyValue>>>>,

    /// Maps computed bindings to their values (we ignore these when setting)
    computed: Mutex<HashMap<String, BindRef<PropertyValue>>>,

    /// Used for properties that don't exist in this model
    nothing: BindRef<PropertyValue>
}

impl DynamicViewModel {
    ///
    /// Creates a new dynamic viewmodel
    /// 
    pub fn new() -> DynamicViewModel {
        DynamicViewModel { 
            bindings:   Mutex::new(HashMap::new()), 
            computed:   Mutex::new(HashMap::new()),
            nothing:    BindRef::from(bind(PropertyValue::Nothing)) }
    }

    ///
    /// Attempts to retrieve the set binding with a particular name
    ///
    fn get_binding(&self, property_name: &str) -> Option<Arc<Binding<PropertyValue>>> {
        let bindings = self.bindings.lock().unwrap();

        bindings.get(&String::from(property_name)).map(|arc| arc.clone())
    }

    ///
    /// Attempts to retrieve the computed binding with a paritcular name
    /// 
    fn get_computed(&self, property_name: &str) -> Option<BindRef<PropertyValue>> {
        let computed = self.computed.lock().unwrap();

        computed.get(&String::from(property_name)).map(|arc| arc.clone())
    }

    ///
    /// Sets a binding to a computed value 
    ///
    pub fn set_computed<TFn>(&self, property_name: &str, calculate_value: TFn)
    where TFn: 'static+Send+Sync+Fn() -> PropertyValue {
        // If this is done while computing the UI, we don't want our computed to attach to the current context
        BindingContext::out_of_context(move || {
            let new_binding = BindRef::from(computed(calculate_value));

            let mut computed = self.computed.lock().unwrap();
            computed.insert(String::from(property_name), new_binding);
        });
    }
}

impl ViewModel for DynamicViewModel {
    fn get_property(&self, property_name: &str) -> BindRef<PropertyValue> {
        if let Some(result) = self.get_computed(property_name) {
            // Computed values are returned first, so these bindings cannot be set
            result
        } else if let Some(result) = self.get_binding(property_name) {
            // 'Set' bindings are returned next
            BindRef::from_arc(result)
        } else {
            // If an invalid name is requested, we return something bound to nothing
            self.nothing.clone()
        }
    }

    fn set_property(&self, property_name: &str, new_value: PropertyValue) { 
        let mut bindings = self.bindings.lock().unwrap();

        if let Some(value) = bindings.get(&String::from(property_name)) {
            // Trick here is that while the bindings aren't mutable, their clones can be (and refer to the same place)
            (**value).clone().set(new_value);

            // Awkward return because rust keeps the borrow in the else clause even though nothing can reference it
            return;
        }

        // Property does not exist in this viewmodel: create a new one
        let new_binding = bind(new_value);
        bindings.insert(String::from(property_name), Arc::new(new_binding));
    }

    fn get_property_names(&self) -> Vec<String> {
        // The keys for items with 'set' bindings
        let mut binding_keys: Vec<_> = {
            let bindings = self.bindings.lock().unwrap();
            bindings
                .keys()
                .map(|key| key.clone())
                .collect()
        };

        // Keys for items with computed bindings
        let mut computed_keys: Vec<_> = {
            let computed = self.computed.lock().unwrap();
            computed
                .keys()
                .map(|key| key.clone())
                .collect()
        };

        // Combine them and deduplicate for the final list of keys
        binding_keys.append(&mut computed_keys);
        binding_keys.sort();
        binding_keys.dedup();

        binding_keys
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn nonexistent_value_is_nothing() {
        let viewmodel = DynamicViewModel::new();

        assert!(viewmodel.get_property("Test").get() == PropertyValue::Nothing);
    }

    #[test]
    fn can_set_value() {
        let viewmodel = DynamicViewModel::new();

        viewmodel.set_property("Test", PropertyValue::Int(2));

        assert!(viewmodel.get_property("Test").get() == PropertyValue::Int(2));
    }

    #[test]
    fn can_compute_value() {
        let viewmodel = DynamicViewModel::new();

        viewmodel.set_computed("Test", || PropertyValue::Int(2));

        assert!(viewmodel.get_property("Test").get() == PropertyValue::Int(2));
    }

    #[test]
    fn computed_value_updates() {
        let viewmodel = DynamicViewModel::new();

        viewmodel.set_property("TestSource", PropertyValue::Int(1));

        let test_source = viewmodel.get_property("TestSource");
        viewmodel.set_computed("Test", move || test_source.get());

        assert!(viewmodel.get_property("Test").get() == PropertyValue::Int(1));

        viewmodel.set_property("TestSource", PropertyValue::Int(2));

        assert!(viewmodel.get_property("Test").get() == PropertyValue::Int(2));
    }

    #[test]
    fn property_value_notifies_without_viewmodel() {
        let notified    = Arc::new(Mutex::new(false));

        // For the viewmodel to work, we need property value changes to trigger a notification
        let mut property_value      = bind(PropertyValue::Int(1));

        let computed_source_value   = property_value.clone();
        let computed_property       = computed(move || computed_source_value.get());

        let test_value_notified = notified.clone();
        computed_property.when_changed(notify(move || (*test_value_notified.lock().unwrap()) = true)).keep_alive();

        assert!(computed_property.get() == PropertyValue::Int(1));
        assert!((*notified.lock().unwrap()) == false);

        property_value.set(PropertyValue::Int(2));

        assert!(computed_property.get() == PropertyValue::Int(2));
        assert!((*notified.lock().unwrap()) == true);
    }

    #[test]
    fn standard_value_notifies_after_propagation() {
        let notified    = Arc::new(Mutex::new(false));
        let viewmodel   = DynamicViewModel::new();

        // Creates the 'TestSource' property
        viewmodel.set_property("TestSource", PropertyValue::Int(1));

        // Computes a value equal to the current TestSource property
        let test_source = viewmodel.get_property("TestSource");
        let test_value  = computed(move || test_source.get());

        // Whenever it changes, set a flag
        let test_value_notified = notified.clone();
        test_value.when_changed(notify(move || (*test_value_notified.lock().unwrap()) = true)).keep_alive();

        // Initially unchanged
        assert!(test_value.get() == PropertyValue::Int(1));
        assert!((*notified.lock().unwrap()) == false);

        // Updating the value should cause the notification to fiew
        viewmodel.set_property("TestSource", PropertyValue::Int(2));

        assert!(viewmodel.get_property("TestSource").get() == PropertyValue::Int(2));
        assert!(test_value.get() == PropertyValue::Int(2));
        assert!((*notified.lock().unwrap()) == true);
    }

    #[test]
    fn computed_value_notifies() {
        let notified    = Arc::new(Mutex::new(false));
        let viewmodel   = DynamicViewModel::new();

        viewmodel.set_property("TestSource", PropertyValue::Int(1));

        let test_source = viewmodel.get_property("TestSource");
        viewmodel.set_computed("Test", move || test_source.get());

        let test_value_notified = notified.clone();
        viewmodel.get_property("Test").when_changed(notify(move || (*test_value_notified.lock().unwrap()) = true)).keep_alive();

        assert!(viewmodel.get_property("Test").get() == PropertyValue::Int(1));
        assert!((*notified.lock().unwrap()) == false);

        viewmodel.set_property("TestSource", PropertyValue::Int(2));

        assert!(viewmodel.get_property("Test").get() == PropertyValue::Int(2));
        assert!((*notified.lock().unwrap()) == true);
    }

    #[test]
    fn computed_value_notifies_after_propagation() {
        let notified    = Arc::new(Mutex::new(false));
        let viewmodel   = DynamicViewModel::new();

        viewmodel.set_property("TestSource", PropertyValue::Int(1));

        let test_source = viewmodel.get_property("TestSource");
        viewmodel.set_computed("Test", move || test_source.get());

        let test        = viewmodel.get_property("Test");
        let test_value  = computed(move || test.get());

        let test_value_notified = notified.clone();
        test_value.when_changed(notify(move || (*test_value_notified.lock().unwrap()) = true)).keep_alive();

        assert!(test_value.get() == PropertyValue::Int(1));
        assert!((*notified.lock().unwrap()) == false);

        viewmodel.set_property("TestSource", PropertyValue::Int(2));

        assert!(viewmodel.get_property("Test").get() == PropertyValue::Int(2));
        assert!(test_value.get() == PropertyValue::Int(2));
        assert!((*notified.lock().unwrap()) == true);
    }
}

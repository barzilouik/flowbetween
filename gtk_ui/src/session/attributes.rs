use super::property_action::*;
use super::super::gtk_action::*;

use flo_ui::*;

pub type PropertyWidgetAction = PropertyAction<GtkWidgetAction>;

///
/// Trait implemented by things that can be converted to GTK widget actions
/// 
pub trait ToGtkActions {
    ///
    /// Converts this itme to a set of GtkWIdgetActions required to render it to a GTK widget
    /// 
    fn to_gtk_actions(&self) -> Vec<PropertyWidgetAction>;
}

///
/// Determines the GTK control to use for a button control
/// 
/// We need to use a toggle button for buttons with a selected attribute, or a normal button for everything else
/// 
fn button_type_for_control(control: &Control) -> GtkWidgetType {
    use self::ControlAttribute::*;

    // This works because we don't update attributes except via the viewmodel right now (so if the control hasn't got a selected
    // attribute it will never have one in the future). If at some point, the selected attribute can be added to a control, we may
    // end up with a situation where Button is chosen incorrectly.
    if control.attributes().any(|attribute| match attribute { &StateAttr(State::Selected(_)) => true, _ => false }) {
        GtkWidgetType::ToggleButton
    } else {
        GtkWidgetType::Button
    }
}

///
/// Determines if a control needs an event box or not
/// 
fn needs_event_box(control: &Control) -> bool {
    use self::ControlAttribute::*;

    control.attributes().any(|attribute| {
        match attribute {
            // Controls with a background colour need an event box
            &AppearanceAttr(Appearance::Background(_)) => true,

            // Controls with a z-index need an event box
            &ZIndex(_) => true,

            // Other controls do not need an event box
            _ => false
        }
    })
}

///
/// Creates the actions required to instantiate a control - without its subcomponents
/// 
impl ToGtkActions for Control {
    fn to_gtk_actions(&self) -> Vec<PropertyWidgetAction> {
        use self::ControlType::*;
        use self::GtkWidgetAction::*;

        // Convert the control type into the command to create the appropriate Gtk widget
        let create_new = match self.control_type() {
            Empty               => New(GtkWidgetType::Generic),
            Container           => New(GtkWidgetType::Fixed),
            CroppingContainer   => New(GtkWidgetType::Layout),
            ScrollingContainer  => New(GtkWidgetType::ScrollArea),
            Popup               => New(GtkWidgetType::Popup),
            Button              => New(button_type_for_control(self)),
            Label               => New(GtkWidgetType::Label),
            Canvas              => New(GtkWidgetType::CanvasDrawingArea),
            Slider              => New(GtkWidgetType::Scale),
            Rotor               => New(GtkWidgetType::Generic)
        };
        
        // The widget class allows the style sheet to specifically target Flo widgets
        let widget_class = match self.control_type() {
            Empty               => "flo-empty",
            Container           => "flo-container",
            CroppingContainer   => "flo-cropping-container",
            ScrollingContainer  => "flo-scrolling-container",
            Popup               => "flo-popup",
            Button              => "flo-button",
            Label               => "flo-label",
            Canvas              => "flo-canvas",
            Slider              => "flo-slider",
            Rotor               => "flo-rotor"
        };

        // Build into the 'create control' action
        let mut create_control = vec![ 
            create_new.into(), 
        ];

        // Some controls need an event box (eg, to show a custom background or to allow for z-ordering)
        if needs_event_box(self) {
            create_control.push(GtkWidgetAction::Box.into());
        }

        // Controls have their own class for styling and are displayed by default
        create_control.extend(vec![
            GtkWidgetAction::Content(WidgetContent::AddClass(widget_class.to_string())).into(),
            GtkWidgetAction::Show.into()
        ]);

        // Generate the actions for all of the attributes
        for attribute in self.attributes() {
            let create_attribute = attribute.to_gtk_actions();
            create_control.extend(create_attribute.into_iter());
        }

        create_control
    }
}

impl ToGtkActions for ControlAttribute {
    fn to_gtk_actions(&self) -> Vec<PropertyWidgetAction> {
        use self::ControlAttribute::*;

        match self {
            &BoundingBox(ref bounds)                => vec![ WidgetLayout::BoundingBox(bounds.clone()).into() ].into_actions(),
            &ZIndex(zindex)                         => vec![ WidgetLayout::ZIndex(zindex).into() ].into_actions(),
            &Padding((left, top), (right, bottom))  => vec![ WidgetLayout::Padding((left, top), (right, bottom)).into() ].into_actions(),
            
            &Text(ref text)                         => vec![ PropertyAction::from_property(text.clone(), |text| vec![ WidgetContent::SetText(text.to_string()).into() ]) ],

            &FontAttr(ref font)                     => font.to_gtk_actions(),
            &StateAttr(ref state)                   => state.to_gtk_actions(),
            &PopupAttr(ref popup)                   => popup.to_gtk_actions(),
            &AppearanceAttr(ref appearance)         => appearance.to_gtk_actions(),
            &ScrollAttr(ref scroll)                 => scroll.to_gtk_actions(),

            &Id(ref id)                             => vec![ WidgetContent::AddClass(id.clone()).into() ].into_actions(),
            &Action(ref _trigger, ref _action_name) => vec![],
            &Canvas(ref _canvas)                    => vec![] /* Canvas drawing is done via the UpdateCanvas event from the core UI */,

            // The GTK layout doesn't need to know the controller
            &Controller(ref controller_name)       => vec![ GtkWidgetAction::Content(WidgetContent::AddClass(format!("c-{}", controller_name))) ].into_actions(),

            // Subcomponents are added elsewhere: we don't assign them here
            &SubComponents(ref _components)         => vec![]
        }
    }
}

impl ToGtkActions for State {
    fn to_gtk_actions(&self) -> Vec<PropertyWidgetAction> {
        use self::State::*;

        match self {
            &Selected(ref selected)     => vec![ PropertyAction::from_property(selected.clone(), |value| vec![ WidgetState::SetSelected(value.to_bool().unwrap_or(false)).into() ]) ],
            &Badged(ref badged)         => vec![ PropertyAction::from_property(badged.clone(), |value| vec![ WidgetState::SetBadged(value.to_bool().unwrap_or(false)).into() ]) ],
            &Value(ref value)           => vec![ PropertyAction::from_property(value.clone(), |value| vec![ WidgetState::SetValueFloat(value.to_f32().unwrap_or(0.0)).into() ]) ],
            &Range((ref min, ref max))  => vec![ 
                PropertyAction::from_property(min.clone(), |min| vec![ WidgetState::SetRangeMin(min.to_f32().unwrap_or(0.0)).into() ]),
                PropertyAction::from_property(max.clone(), |max| vec![ WidgetState::SetRangeMax(max.to_f32().unwrap_or(0.0)).into() ]) 
            ]
        }
    }
}

impl ToGtkActions for Popup {
    fn to_gtk_actions(&self) -> Vec<PropertyWidgetAction> {
        use self::Popup::*;

        match self {
            &IsOpen(ref _is_open)       => vec![],
            &Direction(ref _direction)  => vec![],
            &Size(_width, _height)      => vec![],
            &Offset(_distance)          => vec![]
        }
    }
}

impl ToGtkActions for Font {
    fn to_gtk_actions(&self) -> Vec<PropertyWidgetAction> {
        vec![ self.clone().into() ].into_actions()
    }
}

impl ToGtkActions for Appearance {
    fn to_gtk_actions(&self) -> Vec<PropertyWidgetAction> {
        vec![ self.clone().into() ].into_actions()
    }
}

impl ToGtkActions for Scroll {
    fn to_gtk_actions(&self) -> Vec<PropertyWidgetAction> {
        vec![ self.clone().into() ].into_actions()
    }
}

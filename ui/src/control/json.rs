use super::control::*;
use super::attributes::*;
use super::super::json::*;

use serde_json::*;

impl ToJsonValue for ControlAttribute {
    fn to_json(&self) -> Value {
        use ControlAttribute::*;
        use Appearance::*;
        use State::*;

        match self {
            &BoundingBox(ref bounds)                => json!({ "BoundingBox": bounds }),
            &Text(ref property)                     => json!({ "Text": property }),
            &ZIndex(zindex)                         => json!({ "ZIndex": zindex }),
            &Padding((left, top), (right, bottom))  => json!({ "Padding": { "left": left, "top": top, "right": right, "bottom": bottom } }),
            &FontAttr(attr)                         => json!({ "Font": attr }),
            &StateAttr(Selected(ref property))      => json!({ "Selected": property }),
            &StateAttr(Badged(ref property))        => json!({ "Badged": property }),
            &StateAttr(Value(ref property))         => json!({ "Value": property }),
            &StateAttr(Range((ref min, ref max)))   => json!({ "Range": [min, max] }),
            &PopupAttr(ref popup)                   => json!({ "Popup": popup }),
            &ScrollAttr(ref scroll)                 => json!({ "Scroll": scroll }),
            &Id(ref id)                             => json!({ "Id": id }),
            &Controller(ref name)                   => json!({ "Controller": name }),
            &Action(ref trigger, ref action)        => json!({ "Action": (trigger, action) }),
            &HintAttr(ref hint)                     => json!({ "Hint": hint }),

            &SubComponents(ref components)          => {
                let json_components: Vec<_> = components.iter()
                    .map(|component| component.to_json())
                    .collect();

                json!({ "SubComponents": json_components })
            },

            &AppearanceAttr(Image(ref image_resource))  => {
                // For images, we only store the ID: callers need to look it up from the resource manager in the controller that made this control
                json!({ 
                    "Image": {
                        "id":   image_resource.id(),
                        "name": image_resource.name()
                    }
                })
            },

            &AppearanceAttr(Background(ref color))  => json!({ "Background": color.to_rgba_components() }),
            &AppearanceAttr(Foreground(ref color))  => json!({ "Foreground": color.to_rgba_components() }),

            &Canvas(ref canvas_resource)            => {
                json!({
                    "Image": {
                        "id":   canvas_resource.id(),
                        "name": canvas_resource.name()
                    }
                })
            }
        }
    }
}

impl ToJsonValue for Control {
    fn to_json(&self) -> Value {
        let attributes: Vec<_> = self.attributes()
            .map(|attribute| attribute.to_json())
            .collect();

        json!({
            "attributes":   attributes,
            "control_type": self.control_type()
        })
    }
}

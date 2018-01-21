use super::*;

use modifier::*;

///
/// Attributes representing the way a control scrolls its content
/// 
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum Scroll {
    /// The size of the content of this scroll region
    ContentSize(f32, f32),

    /// Whether or not to allow horizontal or vertical scrolling at all
    /// Both are allowed by default (ie, the default value of this is true, true)
    AllowScroll(bool, bool),

    /// Whether or not to auto-hide the horizontal or vertical scroll bars
    /// Both are displayed by default (ie, the default value of this is false, false)
    AutoHide(bool, bool)
}

impl Modifier<Control> for Scroll {
    fn modify(self, control: &mut Control) {
        control.add_attribute(ControlAttribute::ScrollAttr(self))
    }
}

impl<'a> Modifier<Control> for &'a Scroll {
    fn modify(self, control: &mut Control) {
        control.add_attribute(ControlAttribute::ScrollAttr(self.clone()))
    }
}

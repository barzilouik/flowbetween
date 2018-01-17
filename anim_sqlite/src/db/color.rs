use super::*;
use super::db_enum::*;
use super::db_update::*;

use canvas::*;

impl AnimationDbCore {
    ///
    /// Inserts a colour definition, leaving the ID on the database stack
    /// 
    pub fn insert_color(&mut self, color: &Color) -> Result<()> {
        use self::DatabaseUpdate::*;

        match color {
            &Color::Rgba(r, g, b, _) => {
                self.db.update(vec![
                    PushColorType(ColorType::Rgb),
                    PushRgb(r, g, b)
                ])
            },

            &Color::Hsluv(h, s, l, _) => {
                self.db.update(vec![
                    PushColorType(ColorType::Hsluv),
                    PushHsluv(h, s, l)
                ])
            },
        }
    }
}
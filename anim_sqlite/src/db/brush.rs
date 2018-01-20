use super::*;
use super::db_enum::*;
use super::flo_store::*;

impl<TFile: FloFile> AnimationDbCore<TFile> {
    ///
    /// Inserts a brush definition, leaving the ID on the database stack
    /// 
    pub fn insert_brush(db: &mut TFile, brush_definition: &BrushDefinition) -> Result<()> {
        use self::DatabaseUpdate::*;

        match brush_definition {
            &BrushDefinition::Simple => {
                db.update(vec![
                    PushBrushType(BrushDefinitionType::from(brush_definition)),
                ])
            },

            &BrushDefinition::Ink(ref ink_defn) => {
                db.update(vec![
                    PushBrushType(BrushDefinitionType::from(brush_definition)),
                    PushInkBrush(ink_defn.min_width, ink_defn.max_width, ink_defn.scale_up_distance)
                ])
            }
        }
    }

    ///
    /// Inserts some brush properties into the database, leaving the ID on the database stack
    ///
    pub fn insert_brush_properties(db: &mut TFile, brush_properties: &BrushProperties) -> Result<()> {
        use self::DatabaseUpdate::*;

        Self::insert_color(db, &brush_properties.color)?;

        db.update(vec![
            PushBrushProperties(brush_properties.size, brush_properties.opacity)
        ])?;

        Ok(())
    }
}
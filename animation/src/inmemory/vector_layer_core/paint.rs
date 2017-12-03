use super::*;

impl PaintLayer for VectorLayerCore {
    fn start_brush_stroke(&mut self, start_time: Duration, initial_pos: BrushPoint) {
        // Start a new brush stroke, at a time relative to 0
        let element = BrushElement::new(start_time, initial_pos);

        self.active_brush_stroke = Some(element);
    }

    fn continue_brush_stroke(&mut self, point: BrushPoint) {
        // Add points to the active brush stroke
        if let Some(ref mut brush_stroke) = self.active_brush_stroke {
            brush_stroke.add_point(point);
        }
    }

    fn finish_brush_stroke(&mut self) {
        // Copy out the active brush stroke and reset the original
        let mut final_brush_stroke = None;
        mem::swap(&mut final_brush_stroke, &mut self.active_brush_stroke);

        // Add to the appropriate keyframe, if we can find it
        if let Some(mut final_brush_stroke) = final_brush_stroke {
            if let Some(keyframe) = self.find_nearest_keyframe(final_brush_stroke.appearance_time()) {
                // Adjust the time so it's relative to the frame
                let original_appearance = final_brush_stroke.appearance_time();
                let frame_start         = keyframe.start_time();
                final_brush_stroke.set_appearance_time(original_appearance - frame_start);

                // Add to the key frame
                keyframe.add_element(Vector::Brush(final_brush_stroke));
            }
        }
    }

    fn cancel_brush_stroke(&mut self) {
        // Reset the brush stroke
        self.active_brush_stroke = None;
    }

    fn draw_current_brush_stroke(&self, gc: &mut GraphicsContext) {
        // Just pass the buck to the current brush stroke
        if let Some(ref brush_stroke) = self.active_brush_stroke {
            brush_stroke.render(gc);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_generate_brush_stroke() {
        let mut core = VectorLayerCore::new(0);

        // Add the key frame that this brush stroke will be for
        core.add_key_frame(Duration::from_millis(0));

        // Draw a simple brush stroke
        core.start_brush_stroke(Duration::from_millis(0), BrushPoint::from((0.0, 0.0)));
        core.continue_brush_stroke(BrushPoint::from((10.0, 10.0)));
        core.continue_brush_stroke(BrushPoint::from((20.0, 5.0)));
        core.finish_brush_stroke();

        let keyframe = &core.keyframes[0];
        let elements = keyframe.elements();

        assert!(elements.len() == 1);

        if let Vector::Brush(ref brush_stroke) = elements[0] {
            assert!(brush_stroke.appearance_time() == Duration::from_millis(0));
            assert!(brush_stroke.points() == &vec![
                BrushPoint::from((0.0, 0.0)),
                BrushPoint::from((10.0, 10.0)),
                BrushPoint::from((20.0, 5.0))
            ]);
        } else {
            assert!(false);
        }
    }
}
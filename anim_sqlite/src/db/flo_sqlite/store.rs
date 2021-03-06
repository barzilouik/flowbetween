use super::*;

use animation::*;

impl FloSqlite {
    ///
    /// Executes a particular database update
    /// 
    fn execute_update(&mut self, update: DatabaseUpdate) -> Result<()> {
        use self::DatabaseUpdate::*;

        match update {
            Pop                                                             => {
                #[cfg(test)]
                {
                    if self.stack.len() == 0 {
                        panic!("Popping on empty stack");
                    }
                }

                self.stack.pop(); 
            },

            UpdateCanvasSize(width, height)                                 => {
                let mut update_size = Self::prepare(&self.sqlite, FloStatement::UpdateAnimationSize)?;
                update_size.execute(&[&width, &height, &self.animation_id])?;
            },

            PushEditType(edit_log_type)                                     => {
                let edit_log_type   = self.enum_value(DbEnum::EditLog(edit_log_type));
                let edit_log_id     = Self::prepare(&self.sqlite, FloStatement::InsertEditType)?.insert(&[&edit_log_type])?;
                self.stack.push(edit_log_id);
            },

            PopEditLogSetSize(width, height)                                => {
                let edit_log_id     = self.stack.pop().unwrap();
                let mut set_size    = Self::prepare(&self.sqlite, FloStatement::InsertELSetSize)?;
                set_size.insert(&[&edit_log_id, &(width as f64), &(height as f64)])?;
            },

            PushEditLogLayer(layer_id)                                      => {
                let edit_log_id     = self.stack.pop().unwrap();
                let mut set_layer   = Self::prepare(&self.sqlite, FloStatement::InsertELLayer)?;
                set_layer.insert(&[&edit_log_id, &(layer_id as i64)])?;
                self.stack.push(edit_log_id);
            },

            PushEditLogWhen(when)                                           => {
                let edit_log_id     = self.stack.pop().unwrap();
                let mut set_when    = Self::prepare(&self.sqlite, FloStatement::InsertELWhen)?;
                set_when.insert(&[&edit_log_id, &Self::get_micros(&when)])?;
                self.stack.push(edit_log_id);
            },

            PopEditLogBrush(drawing_style)                                  => {
                let brush_id        = self.stack.pop().unwrap();
                let edit_log_id     = self.stack.pop().unwrap();
                let drawing_style   = self.enum_value(DbEnum::DrawingStyle(drawing_style));
                let mut set_brush   = Self::prepare(&self.sqlite, FloStatement::InsertELBrush)?;
                set_brush.insert(&[&edit_log_id, &drawing_style, &brush_id])?;
            },

            PopEditLogBrushProperties                                       => {
                let brush_props_id      = self.stack.pop().unwrap();
                let edit_log_id         = self.stack.pop().unwrap();
                let mut set_brush_props = Self::prepare(&self.sqlite, FloStatement::InsertELBrushProperties)?;
                set_brush_props.insert(&[&edit_log_id, &brush_props_id])?;
            },

            PushEditLogElementId(element_id)                                => {
                let edit_log_id         = self.stack.last().unwrap();
                let mut add_element_id  = Self::prepare(&self.sqlite, FloStatement::InsertELElementId)?;
                
                add_element_id.insert(&[edit_log_id, &element_id])?;
            },

            PushRawPoints(points)                                           => {
                let edit_log_id         = self.stack.last().unwrap();
                let mut add_raw_point   = Self::prepare(&self.sqlite, FloStatement::InsertELRawPoints)?;
                let mut point_bytes     = vec![];

                write_raw_points(&mut point_bytes, &*points).unwrap();
                add_raw_point.insert(&[edit_log_id, &point_bytes])?;
            },

            PushEditLogMotionOrigin(x, y) => {
                let (x, y)          = (x as f64, y as f64);
                let edit_log_id     = self.stack.last().unwrap();
                let mut add_origin  = Self::prepare(&self.sqlite, FloStatement::InsertELMotionOrigin)?;
                
                add_origin.insert(&[edit_log_id, &x, &y])?;
            },

            PushEditLogMotionType(motion_type) => {
                let motion_type     = self.enum_value(DbEnum::MotionType(motion_type));
                let edit_log_id     = self.stack.last().unwrap();
                let mut add_type    = Self::prepare(&self.sqlite, FloStatement::InsertELMotionType)?;

                add_type.insert(&[edit_log_id, &motion_type])?;
            },

            PushEditLogMotionElement(attach_element) => {
                let edit_log_id     = self.stack.last().unwrap();
                let mut add_type    = Self::prepare(&self.sqlite, FloStatement::InsertELMotionElement)?;

                add_type.insert(&[edit_log_id, &attach_element])?;
            },

            PushEditLogMotionPath(num_points) => {
                // Collect the IDs of the points
                let mut point_ids = vec![];
                for _index in 0..num_points {
                    point_ids.push(self.stack.pop().unwrap_or(-1));
                }

                // The edit log ID is found underneath the stack of points
                let edit_log_id = self.stack.last().unwrap();

                // Prepare the insertion statement
                let mut add_point = Self::prepare(&self.sqlite, FloStatement::InsertELMotionTimePoint)?;

                // Insert each of the points in turn
                for index in 0..num_points {
                    let point_index = ((num_points-1)-index) as i64;
                    add_point.insert(&[edit_log_id, &point_index, &point_ids[index]])?;
                }
            },

            PushTimePoint(x, y, millis) => {
                let (x, y, millis)  = (x as f64, y as f64, millis as f64);
                let mut add_point   = Self::prepare(&self.sqlite, FloStatement::InsertTimePoint)?;
                let point_id        = add_point.insert(&[&x, &y, &millis])?;
                self.stack.push(point_id);
            },

            PushBrushType(brush_type)                                       => {
                let brush_type              = self.enum_value(DbEnum::BrushDefinition(brush_type));
                let mut insert_brush_type   = Self::prepare(&self.sqlite, FloStatement::InsertBrushType)?;
                let brush_id                = insert_brush_type.insert(&[&brush_type])?;
                self.stack.push(brush_id);
            },

            PushInkBrush(min_width, max_width, scale_up_distance)           => {
                let brush_id                = self.stack.last().unwrap();
                let mut insert_ink_brush    = Self::prepare(&self.sqlite, FloStatement::InsertInkBrush)?;
                insert_ink_brush.insert(&[brush_id, &(min_width as f64), &(max_width as f64), &(scale_up_distance as f64)])?;
            },

            PushBrushProperties(size, opacity)                              => {
                let color_id                    = self.stack.pop().unwrap();
                let mut insert_brush_properties = Self::prepare(&self.sqlite, FloStatement::InsertBrushProperties)?;
                let brush_props_id              = insert_brush_properties.insert(&[&(size as f64), &(opacity as f64), &color_id])?;
                self.stack.push(brush_props_id);
            },

            PushColorType(color_type)                                       => {
                let color_type              = self.enum_value(DbEnum::Color(color_type));
                let mut insert_color_type   = Self::prepare(&self.sqlite, FloStatement::InsertColorType)?;
                let color_id                = insert_color_type.insert(&[&color_type])?;
                self.stack.push(color_id);
            },

            PushRgb(r, g, b)                                                => {
                let color_id        = self.stack.last().unwrap();
                let mut insert_rgb  = Self::prepare(&self.sqlite, FloStatement::InsertRgb)?;
                insert_rgb.insert(&[color_id, &(r as f64), &(g as f64), &(b as f64)])?;
            },

            PushHsluv(h, s, l)                                              => {
                let color_id            = self.stack.last().unwrap();
                let mut insert_hsluv    = Self::prepare(&self.sqlite, FloStatement::InsertHsluv)?;
                insert_hsluv.insert(&[color_id, &(h as f64), &(s as f64), &(l as f64)])?;
            },

            PopDeleteLayer                                                  => {
                let layer_id            = self.stack.pop().unwrap();
                let mut delete_layer    = Self::prepare(&self.sqlite, FloStatement::DeleteLayer)?;
                delete_layer.execute(&[&layer_id])?;
            },

            PushLayerType(layer_type)                                       => {
                let layer_type              = self.enum_value(DbEnum::Layer(layer_type));
                let mut insert_layer_type   = Self::prepare(&self.sqlite, FloStatement::InsertLayerType)?;
                let layer_id                = insert_layer_type.insert(&[&layer_type])?;
                self.stack.push(layer_id);
            },

            PushAssignLayer(assigned_id)                                    => {
                let layer_id                = self.stack.last().unwrap();
                let mut insert_assign_layer = Self::prepare(&self.sqlite, FloStatement::InsertAssignLayer)?;
                insert_assign_layer.insert(&[&self.animation_id, layer_id, &(assigned_id as i64)])?;
            },

            PushLayerId(layer_id)                                           => {
                self.stack.push(layer_id);
            },

            PushLayerForAssignedId(assigned_id)                             => {
                let mut select_layer_id = Self::prepare(&self.sqlite, FloStatement::SelectLayerId)?;
                let layer_id            = select_layer_id.query_row(&[&self.animation_id, &(assigned_id as i64)], |row| row.get(0))?;
                self.stack.push(layer_id);
            },

            PopAddKeyFrame(when)                                            => {
                let layer_id                = self.stack.pop().unwrap();
                let mut insert_key_frame    = Self::prepare(&self.sqlite, FloStatement::InsertKeyFrame)?;
                insert_key_frame.insert(&[&layer_id, &Self::get_micros(&when)])?;
            },

            PopRemoveKeyFrame(when)                                         => {
                let layer_id                = self.stack.pop().unwrap();
                let mut delete_key_frame    = Self::prepare(&self.sqlite, FloStatement::DeleteKeyFrame)?;
                delete_key_frame.execute(&[&layer_id, &Self::get_micros(&when)])?;
            },

            PushNearestKeyFrame(when)                                       => {
                let layer_id                        = self.stack.pop().unwrap();
                let mut select_nearest_keyframe     = Self::prepare(&self.sqlite, FloStatement::SelectNearestKeyFrame)?;
                let (keyframe_id, start_micros)     = select_nearest_keyframe.query_row(&[&layer_id, &(Self::get_micros(&when))], |row| (row.get(0), row.get(1)))?;
                self.stack.push(start_micros);
                self.stack.push(keyframe_id);
            },

            PushVectorElementType(element_type, when)                       => {
                let keyframe_id                     = self.stack.pop().unwrap();
                let start_micros                    = self.stack.pop().unwrap();
                let element_type                    = self.enum_value(DbEnum::VectorElement(element_type));
                let mut insert_vector_element_type  = Self::prepare(&self.sqlite, FloStatement::InsertVectorElementType)?;
                let when                            = Self::get_micros(&when) - start_micros;
                let element_id                      = insert_vector_element_type.insert(&[&keyframe_id, &element_type, &when])?;
                self.stack.push(start_micros);
                self.stack.push(keyframe_id);
                self.stack.push(element_id);
            },

            PushElementAssignId(assigned_id)                                => {
                let element_id                      = self.stack.last().unwrap();
                let mut insert_element_assigned_id  = Self::prepare(&self.sqlite, FloStatement::InsertElementAssignedId)?;
                insert_element_assigned_id.insert(&[element_id, &assigned_id])?;
            },

            PopVectorBrushElement(drawing_style)                            => {
                let brush_id                            = self.stack.pop().unwrap();
                let element_id                          = self.stack.pop().unwrap();
                let drawing_style                       = self.enum_value(DbEnum::DrawingStyle(drawing_style));
                let mut insert_brush_definition_element = Self::prepare(&self.sqlite, FloStatement::InsertBrushDefinitionElement)?;
                insert_brush_definition_element.insert(&[&element_id, &brush_id, &drawing_style])?;
            },

            PopVectorBrushPropertiesElement                                 => {
                let brush_props_id                  = self.stack.pop().unwrap();
                let element_id                      = self.stack.pop().unwrap();
                let mut insert_brush_props_element  = Self::prepare(&self.sqlite, FloStatement::InsertBrushPropertiesElement)?;
                insert_brush_props_element.insert(&[&element_id, &brush_props_id])?;
            },

            PopBrushPoints(points)                                          => {
                let element_id              = self.stack.pop().unwrap();
                let mut insert_brush_point  = Self::prepare(&self.sqlite, FloStatement::InsertBrushPoint)?;

                let num_points = points.len();
                for (point, index) in points.iter().zip((0..num_points).into_iter()) {
                    insert_brush_point.insert(&[
                        &element_id, &(index as i64),
                        &(point.cp1.0 as f64), &(point.cp1.1 as f64),
                        &(point.cp2.0 as f64), &(point.cp2.1 as f64),
                        &(point.position.0 as f64), &(point.position.1 as f64),
                        &(point.width as f64)
                    ])?;
                }
            },

            CreateMotion(motion_id)                                         => {
                let motion_type         = self.enum_value(DbEnum::MotionType(MotionType::None));
                let mut insert_motion   = Self::prepare(&self.sqlite, FloStatement::InsertMotion)?;

                insert_motion.insert(&[&motion_id, &motion_type])?;
            },

            SetMotionType(motion_id, motion_type)                           => {
                let motion_type         = self.enum_value(DbEnum::MotionType(motion_type));
                let mut update_motion   = Self::prepare(&self.sqlite, FloStatement::UpdateMotionType)?;

                update_motion.insert(&[&motion_type, &motion_id])?;
            },

            SetMotionOrigin(motion_id, x, y)                                => {
                let mut set_origin  = Self::prepare(&self.sqlite, FloStatement::InsertOrReplaceMotionOrigin)?;
                let (x, y)          = (x as f64, y as f64);

                set_origin.insert(&[&motion_id, &x, &y])?;
            },

            SetMotionPath(motion_id, path_type, num_points)                 => {
                let path_type           = self.enum_value(DbEnum::MotionPathType(path_type));
                let mut delete_path     = Self::prepare(&self.sqlite, FloStatement::DeleteMotionPoints)?;
                let mut insert_point    = Self::prepare(&self.sqlite, FloStatement::InsertMotionPathPoint)?;

                // Remove the existing path of this type from the motion
                delete_path.execute(&[&motion_id, &path_type])?;

                // Collect the IDs of the points
                let mut point_ids = vec![];
                for _index in 0..num_points {
                    point_ids.push(self.stack.pop().unwrap_or(-1));
                }

                // Insert these points
                for index in 0..num_points {
                    let point_index = ((num_points-1)-index) as i64;
                    insert_point.insert(&[&motion_id, &path_type, &point_index, &point_ids[index]])?;
                }
            },

            AddMotionAttachedElement(motion_id, element_id)                 => {
                let mut insert_attached_element = Self::prepare(&self.sqlite, FloStatement::InsertMotionAttachedElement)?;
                insert_attached_element.insert(&[&motion_id, &element_id])?;
            },

            DeleteMotion(motion_id)                                         => {
                let mut delete_motion = Self::prepare(&self.sqlite, FloStatement::DeleteMotion)?;
                delete_motion.execute(&[&motion_id])?;
            },

            DeleteMotionAttachedElement(motion_id, element_id)              => {
                let mut delete_attachment = Self::prepare(&self.sqlite, FloStatement::DeleteMotionAttachedElement)?;
                delete_attachment.execute(&[&motion_id, &element_id])?;
            }
        }

        Ok(())
    }

    ///
    /// Performs a set of updates on the database immediately
    /// 
    fn execute_updates_now<I: IntoIterator<Item=DatabaseUpdate>>(&mut self, updates: I) -> Result<()> {
        for update in updates {
            self.execute_update(update)?;
        }
        Ok(())
    }
}

impl FloStore for FloSqlite {
    ///
    /// Performs a set of updates on the database
    /// 
    fn update<I: IntoIterator<Item=DatabaseUpdate>>(&mut self, updates: I) -> Result<()> {
        if let Some(ref mut pending) = self.pending {
            // Queue the updates into the pending queue if we're not performing them immediately
            pending.extend(updates.into_iter());
        } else {
            // Execute these updates immediately
            self.execute_updates_now(updates)?;
        }

        Ok(())
    }

    ///
    /// Starts queuing up database updates for later execution as a batch
    /// 
    fn begin_queuing(&mut self) {
        if self.pending.is_none() {
            self.pending = Some(vec![]);
        }
    }

    ///
    /// Executes the update queue
    /// 
    fn execute_queue(&mut self) -> Result<()> {
        // Fetch the pending updates
        let mut pending = None;
        mem::swap(&mut pending, &mut self.pending);

        // Execute them now
        if let Some(pending) = pending {
            self.execute_updates_now(pending)?;
        }

        Ok(())
    }

    ///
    /// Ensures any pending updates are committed to the database
    /// 
    fn flush_pending(&mut self) -> Result<()> {
        if self.pending.is_some() {
            // Fetch the pending updates
            let mut pending = Some(vec![]);
            mem::swap(&mut pending, &mut self.pending);

            // Execute them now
            if let Some(pending) = pending {
                self.execute_updates_now(pending)?;
            }
        }

        Ok(())
    }
}

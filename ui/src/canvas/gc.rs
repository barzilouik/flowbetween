use super::draw::*;
use super::color::*;
use super::transform2d::*;

///
/// A graphics context provides the basic set of graphics actions that can be performed 
///
pub trait GraphicsContext {
    fn new_path(&mut self);
    fn move_to(&mut self, x: f32, y: f32);
    fn line_to(&mut self, x: f32, y: f32);
    fn bezier_curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32);
    fn fill(&mut self);
    fn stroke(&mut self);
    fn line_width(&mut self, width: f32);
    fn line_join(&mut self, join: LineJoin);
    fn line_cap(&mut self, cap: LineCap);
    fn new_dash_pattern(&mut self);
    fn dash_length(&mut self, length: f32);
    fn dash_offset(&mut self, offset: f32);
    fn fill_color(&mut self, col: Color);
    fn stroke_color(&mut self, col: Color);
    fn blend_mode(&mut self, mode: BlendMode);
    fn identity_transform(&mut self);
    fn canvas_height(&mut self, height: f32);
    fn transform(&mut self, transform: Transform2D);
    fn unclip(&mut self);
    fn clip(&mut self);
    fn store(&mut self);
    fn restore(&mut self);
    fn push_state(&mut self);
    fn pop_state(&mut self);
    fn clear_canvas(&mut self);

    fn draw(&mut self, d: Draw) {
        use self::Draw::*;

        match d {
            NewPath                                     => self.new_path(),
            Move(x, y)                                  => self.move_to(x, y),
            Line(x, y)                                  => self.line_to(x, y),
            BezierCurve((x1, y1), (x2, y2), (x3, y3))   => self.bezier_curve_to(x1, y1, x2, y2, x3, y3),
            Fill                                        => self.fill(),
            Stroke                                      => self.stroke(),
            LineWidth(width)                            => self.line_width(width),
            LineJoin(join)                              => self.line_join(join),
            LineCap(cap)                                => self.line_cap(cap),
            NewDashPattern                              => self.new_dash_pattern(),
            DashLength(dash_length)                     => self.dash_length(dash_length),
            DashOffset(dash_offset)                     => self.dash_offset(dash_offset),
            FillColor(col)                              => self.fill_color(col),
            StrokeColor(col)                            => self.stroke_color(col),
            BlendMode(blendmode)                        => self.blend_mode(blendmode),
            IdentityTransform                           => self.identity_transform(),
            CanvasHeight(height)                        => self.canvas_height(height),
            MultiplyTransform(transform)                => self.transform(transform),
            Unclip                                      => self.unclip(),
            Clip                                        => self.clip(),
            Store                                       => self.store(),
            Restore                                     => self.restore(),
            PushState                                   => self.push_state(),
            PopState                                    => self.pop_state(),
            ClearCanvas                                 => self.clear_canvas()
        }
    }
}

///
/// GraphicsPrimitives adds new primitives that can be built directly from a graphics context
///
pub trait GraphicsPrimitives : GraphicsContext {
    fn rect(&mut self, x1: f32, y1: f32, x2: f32, y2: f32) {
        self.move_to(x1, y1);
        self.line_to(x1, y2);
        self.line_to(x2, y2);
        self.line_to(x2, y1);
        self.line_to(x1, y1);
    }
}
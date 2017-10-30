use super::draw::*;
use super::color::*;

use std::mem;

///
/// Trait implemented by objects that can be encoded into a canvas
///
pub trait CanvasEncoding<Buffer> {
    ///
    /// Encodes this item by appending it to the specified string
    ///
    fn encode_canvas(&self, append_to: &mut Buffer);
}

const ENCODING_CHAR_SET: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'
];

impl CanvasEncoding<String> for char {
    #[inline]
    fn encode_canvas(&self, append_to: &mut String) {
        append_to.push(*self)
    }
}

impl CanvasEncoding<String> for u32 {
    #[inline]
    fn encode_canvas(&self, append_to: &mut String) {
        // Base-64 wastes some bits but requires 2 less characters than hex for a 32-bit number
        let mut remaining = *self;

        for _ in 0..6 {
            let next_part = remaining & 0x3f;
            let next_char = ENCODING_CHAR_SET[next_part as usize];
            append_to.push(next_char);

            remaining >>= 6;
        }
    }
}

impl CanvasEncoding<String> for f32 {
    #[inline]
    fn encode_canvas(&self, append_to: &mut String) {
        let transmuted: u32 = unsafe { mem::transmute(*self) };
        transmuted.encode_canvas(append_to)
    }
}

//
// Some convenience encodings for implementing the main canvas encoding
//

impl<A: CanvasEncoding<String>, B: CanvasEncoding<String>> CanvasEncoding<String> for (A, B) {
    fn encode_canvas(&self, append_to: &mut String) {
        self.0.encode_canvas(append_to);
        self.1.encode_canvas(append_to);
    }
}

impl<A: CanvasEncoding<String>, B: CanvasEncoding<String>, C: CanvasEncoding<String>> CanvasEncoding<String> for (A, B, C) {
    fn encode_canvas(&self, append_to: &mut String) {
        self.0.encode_canvas(append_to);
        self.1.encode_canvas(append_to);
        self.2.encode_canvas(append_to);
    }
}

impl<A: CanvasEncoding<String>, B: CanvasEncoding<String>, C: CanvasEncoding<String>, D: CanvasEncoding<String>> CanvasEncoding<String> for (A, B, C, D) {
    fn encode_canvas(&self, append_to: &mut String) {
        self.0.encode_canvas(append_to);
        self.1.encode_canvas(append_to);
        self.2.encode_canvas(append_to);
        self.3.encode_canvas(append_to);
    }
}

impl<A: CanvasEncoding<String>, B: CanvasEncoding<String>, C: CanvasEncoding<String>, D: CanvasEncoding<String>, E: CanvasEncoding<String>> CanvasEncoding<String> for (A, B, C, D, E) {
    fn encode_canvas(&self, append_to: &mut String) {
        self.0.encode_canvas(append_to);
        self.1.encode_canvas(append_to);
        self.2.encode_canvas(append_to);
        self.3.encode_canvas(append_to);
        self.4.encode_canvas(append_to);
    }
}

//
// Main canvas encoding
//

impl CanvasEncoding<String> for Color {
    fn encode_canvas(&self, append_to: &mut String) {
        match self {
            &Color::Rgba(r,g,b,a) => ('R', r, g, b, a)
        }.encode_canvas(append_to)
    }
}

impl CanvasEncoding<String> for LineJoin {
    fn encode_canvas(&self, append_to: &mut String) {
        use self::LineJoin::*;

        match self {
            &Miter => 'M',
            &Round => 'R',
            &Bevel => 'B'
        }.encode_canvas(append_to)
    }
}

impl CanvasEncoding<String> for LineCap {
    fn encode_canvas(&self, append_to: &mut String) {
        use self::LineCap::*;

        match self {
            &Butt   => 'B',
            &Round  => 'R',
            &Square => 'S'
        }.encode_canvas(append_to)
    }
}

impl CanvasEncoding<String> for Draw {
    fn encode_canvas(&self, append_to: &mut String) {
        use self::Draw::*;

        match self {
            &NewPath                                => ('N', 'p').encode_canvas(append_to),
            &Move(x, y)                             => ('m', x, y).encode_canvas(append_to),
            &Line(x, y)                             => ('l', x, y).encode_canvas(append_to),
            &BezierCurve(p1, p2, p3)                => ('c', p1, p2, p3).encode_canvas(append_to),
            &Rect(p1, p2)                           => ('r', p1, p2).encode_canvas(append_to),
            &Fill                                   => 'F'.encode_canvas(append_to),
            &Stroke                                 => 'S'.encode_canvas(append_to),
            &Blit(col)                              => ('B', col).encode_canvas(append_to),
            &LineWidth(width)                       => ('L', 'w', width).encode_canvas(append_to),
            &LineJoin(join)                         => ('L', 'j', join).encode_canvas(append_to),
            &LineCap(cap)                           => ('L', 'c', cap).encode_canvas(append_to),
            &NewDashPattern                         => ('N', 'd').encode_canvas(append_to),
            &DashLength(length)                     => ('D', 'l', length).encode_canvas(append_to),
            &DashOffset(offset)                     => ('D', 'o', offset).encode_canvas(append_to),
            &StrokeColor(col)                       => ('C', 's', col).encode_canvas(append_to),
            &FillColor(col)                         => ('C', 'f', col).encode_canvas(append_to),
            &BlendMode(mode)                        => unimplemented!(),
            &IdentityTransform                      => ('T', 'i').encode_canvas(append_to),
            &CanvasHeight(height)                   => ('T', 'h', height).encode_canvas(append_to),
            &MultiplyTransform(transform)           => unimplemented!(),
            &Unclip                                 => ('Z', 'n').encode_canvas(append_to),
            &Clip                                   => ('Z', 'c').encode_canvas(append_to),
            &Store                                  => ('Z', 's').encode_canvas(append_to),
            &Restore                                => ('Z', 'r').encode_canvas(append_to),
            &PushState                              => 'P'.encode_canvas(append_to),
            &PopState                               => 'p'.encode_canvas(append_to),
            &ClearCanvas                            => ('N', 'A').encode_canvas(append_to)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_encode_u32() {
        let test_number: u32 = 0xabcd1234;

        let mut encoded = String::new();
        test_number.encode_canvas(&mut encoded);

        assert!(encoded == "0IRzrC".to_string());
    }

    #[test]
    fn can_encode_f32() {
        let test_number: f32 = 3.141;

        let mut encoded = String::new();
        test_number.encode_canvas(&mut encoded);

        assert!(encoded == "lYQSAB".to_string());
    }
}
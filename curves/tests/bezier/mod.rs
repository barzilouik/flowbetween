use curves::*;
use curves::bezier;
use curves::bezier::*;

mod basis;
mod subdivide;
mod derivative;
mod tangent;
mod bounds;

pub fn approx_equal(a: f32, b: f32) -> bool {
    f32::floor(f32::abs(a-b)*10000.0) == 0.0
}

#[test]
fn can_read_curve_control_points() {
    let curve = bezier::Curve::from_points(Coord2(1.0, 1.0), Coord2(2.0, 2.0), Coord2(3.0, 3.0), Coord2(4.0, 4.0));

    assert!(curve.start_point() == Coord2(1.0, 1.0));
    assert!(curve.end_point() == Coord2(2.0, 2.0));
    assert!(curve.control_points() == (Coord2(3.0, 3.0), Coord2(4.0, 4.0)));
}

#[test]
fn can_read_curve_points() {
    let curve = bezier::Curve::from_points(Coord2(1.0, 1.0), Coord2(2.0, 2.0), Coord2(3.0, 3.0), Coord2(4.0, 4.0));

    for x in 0..100 {
        let t = (x as f32)/100.0;

        let point           = curve.point_at_pos(t);
        let another_point   = bezier::de_casteljau4(t, Coord2(1.0, 1.0), Coord2(3.0, 3.0), Coord2(4.0, 4.0), Coord2(2.0, 2.0));

        assert!(point == another_point);
    }
}
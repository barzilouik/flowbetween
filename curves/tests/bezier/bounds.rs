use curves::bezier::BezierCurve;
use curves::bezier;
use curves::Coord2;

#[test]
fn can_get_straight_line_bounds() {
    let straight_line   = bezier::Curve::from_points(Coord2(0.0, 1.0), Coord2(2.0, 3.0), Coord2(0.5, 1.5), Coord2(1.5, 2.5));

    let bounds = straight_line.bounding_box();

    assert!(bounds == (Coord2(0.0, 1.0), Coord2(2.0, 3.0)));
}

#[test]
fn can_get_curved_line_bounds() {
    let straight_line   = bezier::Curve::from_points(Coord2(0.0, 1.0), Coord2(2.0, 3.0), Coord2(-1.1875291, 1.5), Coord2(1.5, 2.5));

    let bounds = straight_line.bounding_box();

    assert!(bounds == (Coord2(-0.3, 1.0), Coord2(2.0, 3.0)));
}
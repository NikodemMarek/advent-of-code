use crate::{Input, Solution};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Coord {
    x: usize,
    y: usize,
}
impl From<Box<str>> for Coord {
    fn from(value: Box<str>) -> Self {
        let mut coords = value.split(',');
        Self {
            x: coords.next().unwrap().parse().unwrap(),
            y: coords.next().unwrap().parse().unwrap(),
        }
    }
}
impl Coord {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

fn get_points(lines: impl Iterator<Item = Box<str>>) -> Vec<Coord> {
    lines.map(Coord::from).collect()
}

fn generate_edges(points: &[Coord]) -> Vec<Edge> {
    let points_len = points.len();
    (0..(points_len - 1))
        .map(|point| (point, point + 1))
        .chain(std::iter::once((points_len - 1, 0)))
        .map(|(a, b)| Edge::new(points[a], points[b]))
        .collect()
}

fn is_point_inside(point: Coord, edges: &[Edge]) -> bool {
    let right = Edge::new(point, Coord::new(10000000, point.y));
    let left = Edge::new(Coord::new(0, point.y), point);
    let up = Edge::new(point, Coord::new(point.x, 10000000));
    let down = Edge::new(Coord::new(point.x, 0), point);

    fn ooz(t: bool) -> usize {
        t.then(|| 1).unwrap_or(0)
    }

    let int = edges.iter().fold([0, 0, 0, 0], |intersections, edge| {
        [
            ooz(right.intersects_strict(edge)) + intersections[0],
            ooz(left.intersects_strict(edge)) + intersections[1],
            ooz(up.intersects_strict(edge)) + intersections[2],
            ooz(down.intersects_strict(edge)) + intersections[3],
        ]
    });

    int.iter().all(|intersections| intersections % 2 == 1)
}

#[derive(Debug)]
struct Rectangle(Coord, Coord, Coord, Coord);
impl Rectangle {
    fn collides(&self, edges: &[Edge]) -> bool {
        let redges = self.edges();

        edges.iter().any(|edge| {
            redges[0].intersects(edge)
                || redges[1].intersects(edge)
                || redges[2].intersects(edge)
                || redges[3].intersects(edge)
        })
    }

    fn is_point_inside(&self, point: &Coord) -> bool {
        self.0.x < point.x && self.2.x > point.x && self.0.y < point.y && self.2.y > point.y
    }
    fn is_edge_inside(&self, edges: &Edge) -> bool {
        match edges {
            Edge::H(a, b) | Edge::V(a, b) => self.is_point_inside(a) || self.is_point_inside(b),
        }
    }
    fn has_edge_inside(&self, edges: &[Edge]) -> bool {
        edges.iter().any(|edge| self.is_edge_inside(edge))
    }

    fn is_center_inside(&self, edges: &[Edge]) -> bool {
        let wid = self.2.x - self.0.x;
        let hei = self.2.y - self.0.y;
        let center = Coord::new(self.0.x + wid / 2, self.0.y + hei / 2);

        is_point_inside(center, edges)
    }

    fn edges(&self) -> [Edge; 4] {
        [
            Edge::new(self.0, self.1),
            Edge::new(self.1, self.2),
            Edge::new(self.2, self.3),
            Edge::new(self.3, self.0),
        ]
    }

    fn area(&self) -> usize {
        let Coord { x: c1x, y: c1y } = self.0;
        let Coord { x: c3x, y: c3y } = self.2;
        let lx = c3x - c1x;
        let ly = c3y - c1y;
        (lx + 1) * (ly + 1)
    }
}

fn sorted_edge(a: Coord, b: Coord) -> (Coord, Coord) {
    (a.x <= b.x && a.y <= b.y)
        .then(|| (a, b))
        .unwrap_or_else(|| (b, a))
}

#[derive(Debug)]
enum Edge {
    V(Coord, Coord),
    H(Coord, Coord),
}
impl Edge {
    fn new(a: Coord, b: Coord) -> Self {
        let (a, b) = sorted_edge(a, b);
        (a.x == b.x)
            .then(|| Self::V(a, b))
            .unwrap_or_else(|| Self::H(a, b))
    }

    fn intersects(&self, other: &Edge) -> bool {
        match (self, other) {
            (Edge::V(_, _), Edge::V(_, _)) | (Edge::H(_, _), Edge::H(_, _)) => false,
            (Edge::V(v1, v2), Edge::H(h1, h2)) | (Edge::H(h1, h2), Edge::V(v1, v2)) => {
                v1.y < h1.y && v2.y > h1.y && h1.x < v1.x && h2.x > v1.x
            }
        }
    }

    fn intersects_strict(&self, other: &Edge) -> bool {
        match (self, other) {
            (
                Edge::V(Coord { y: org1, x: orginl }, Coord { y: org2, .. }),
                Edge::V(Coord { y: oth1, x: othinl }, Coord { y: oth2, .. }),
            )
            | (
                Edge::H(Coord { x: org1, y: orginl }, Coord { x: org2, .. }),
                Edge::H(Coord { x: oth1, y: othinl }, Coord { x: oth2, .. }),
            ) => orginl == othinl && org1 < oth1 && org2 > oth2,
            (Edge::V(v1, v2), Edge::H(h1, h2)) | (Edge::H(h1, h2), Edge::V(v1, v2)) => {
                v1.y <= h1.y && v2.y >= h1.y && h1.x <= v1.x && h2.x >= v1.x
            }
        }
    }
}

fn generate_rectangles(points: &[Coord]) -> impl Iterator<Item = Rectangle> {
    let points_len = points.len();
    (0..(points_len - 1))
        .flat_map(move |i| ((i + 1)..points_len).map(move |j| (i, j)))
        .map(|(i, j)| {
            let p1 = points[i];
            let p2 = points[j];
            let (tl, tr, br, bl) = if p1.x < p2.x {
                if p1.y < p2.y {
                    (p1, Coord::new(p2.x, p1.y), p2, Coord::new(p1.x, p2.y))
                } else {
                    (Coord::new(p1.x, p2.y), p2, Coord::new(p2.x, p1.y), p1)
                }
            } else {
                if p2.y < p1.y {
                    (p2, Coord::new(p1.x, p2.y), p1, Coord::new(p2.x, p1.y))
                } else {
                    (Coord::new(p2.x, p1.y), p1, Coord::new(p1.x, p2.y), p2)
                }
            };
            Rectangle(tl, tr, br, bl)
        })
}

pub(crate) struct D9P2;
impl Solution<usize> for D9P2 {
    fn solution(input: impl Input) -> usize {
        let points = get_points(input.lines());

        let edges = generate_edges(&points);
        let rectangles = generate_rectangles(&points);

        rectangles
            .map(|r| (r.area(), r))
            .fold(0, |max_area, (area, r)| {
                if (max_area < area)
                    && r.is_center_inside(&edges)
                    && !r.collides(&edges)
                    && !r.has_edge_inside(&edges)
                {
                    area
                } else {
                    max_area
                }
            })
    }
}

#[cfg(test)]
pub mod tests {
    use super::{Coord, D9P2, Edge, sorted_edge};
    use crate::{Solution, TestInput};

    #[test]
    fn evaluates_correctly() {
        assert_eq!(
            D9P2::solution(TestInput::new(vec![
                "7,1", "11,1", "11,7", "9,7", "9,5", "2,5", "2,3", "7,3",
            ])),
            24
        )
    }

    #[test]
    fn sorts_edge_correctly() {
        assert_eq!(
            sorted_edge(Coord { x: 1, y: 2 }, Coord { x: 1, y: 4 }),
            (Coord { x: 1, y: 2 }, Coord { x: 1, y: 4 })
        );
        assert_eq!(
            sorted_edge(Coord { x: 1, y: 4 }, Coord { x: 1, y: 2 }),
            (Coord { x: 1, y: 2 }, Coord { x: 1, y: 4 })
        );

        assert_eq!(
            sorted_edge(Coord { x: 2, y: 1 }, Coord { x: 4, y: 1 }),
            (Coord { x: 2, y: 1 }, Coord { x: 4, y: 1 })
        );
        assert_eq!(
            sorted_edge(Coord { x: 4, y: 1 }, Coord { x: 2, y: 1 }),
            (Coord { x: 2, y: 1 }, Coord { x: 4, y: 1 })
        );
    }

    #[test]
    fn checks_intersection_correctly() {
        // Intersecting lines (vertical and horizontal)
        assert!(
            Edge::new(Coord { x: 0, y: 5 }, Coord { x: 10, y: 5 })
                .intersects(&Edge::new(Coord { x: 5, y: 0 }, Coord { x: 5, y: 10 }))
        );

        // No intersection - Parallel lines
        assert!(
            !Edge::new(Coord { x: 0, y: 0 }, Coord { x: 0, y: 10 })
                .intersects(&Edge::new(Coord { x: 5, y: 0 }, Coord { x: 5, y: 10 }))
        );
        assert!(
            !Edge::new(Coord { x: 0, y: 0 }, Coord { x: 10, y: 0 })
                .intersects(&Edge::new(Coord { x: 0, y: 5 }, Coord { x: 10, y: 5 }))
        );

        // No intersection - Collinear lines
        assert!(
            !Edge::new(Coord { x: 0, y: 0 }, Coord { x: 5, y: 0 })
                .intersects(&Edge::new(Coord { x: 5, y: 0 }, Coord { x: 10, y: 0 }))
        ); // touching end-to-end
        assert!(
            !Edge::new(Coord { x: 0, y: 0 }, Coord { x: 10, y: 0 })
                .intersects(&Edge::new(Coord { x: 2, y: 0 }, Coord { x: 8, y: 0 }))
        ); // one inside another

        // No intersection - Touching at endpoint
        assert!(
            !Edge::new(Coord { x: 0, y: 0 }, Coord { x: 5, y: 0 })
                .intersects(&Edge::new(Coord { x: 5, y: 0 }, Coord { x: 5, y: 5 }))
        );
        assert!(
            !Edge::new(Coord { x: 0, y: 0 }, Coord { x: 5, y: 0 })
                .intersects(&Edge::new(Coord { x: 0, y: 0 }, Coord { x: 0, y: 5 }))
        );

        // No intersection - Not overlapping (horizontal)
        assert!(
            !Edge::new(Coord { x: 0, y: 0 }, Coord { x: 10, y: 0 })
                .intersects(&Edge::new(Coord { x: 0, y: 5 }, Coord { x: 10, y: 5 }))
        );

        // No intersection - Not overlapping (vertical)
        assert!(
            !Edge::new(Coord { x: 0, y: 0 }, Coord { x: 0, y: 10 })
                .intersects(&Edge::new(Coord { x: 5, y: 0 }, Coord { x: 5, y: 10 }))
        );

        // No intersection - Crossing outside bounds
        assert!(
            !Edge::new(Coord { x: 0, y: 0 }, Coord { x: 0, y: 10 })
                .intersects(&Edge::new(Coord { x: 1, y: 11 }, Coord { x: 1, y: 12 }))
        );

        // Test case where vertical segment is exactly on the intersection point
        assert!(
            !Edge::new(Coord { x: 0, y: 5 }, Coord { x: 10, y: 5 })
                .intersects(&Edge::new(Coord { x: 5, y: 5 }, Coord { x: 5, y: 10 }))
        ); // Touches at (5,5) but doesn't cross
        assert!(
            !Edge::new(Coord { x: 0, y: 5 }, Coord { x: 10, y: 5 })
                .intersects(&Edge::new(Coord { x: 5, y: 0 }, Coord { x: 5, y: 5 }))
        ); // Touches at (5,5) but doesn't cross
        assert!(
            !Edge::new(Coord { x: 5, y: 0 }, Coord { x: 5, y: 10 })
                .intersects(&Edge::new(Coord { x: 0, y: 5 }, Coord { x: 5, y: 5 }))
        );
        assert!(
            !Edge::new(Coord { x: 5, y: 0 }, Coord { x: 5, y: 10 })
                .intersects(&Edge::new(Coord { x: 5, y: 5 }, Coord { x: 10, y: 5 }))
        );

        // Another intersection
        assert!(
            Edge::new(Coord { x: 10, y: 10 }, Coord { x: 20, y: 10 })
                .intersects(&Edge::new(Coord { x: 15, y: 5 }, Coord { x: 15, y: 15 }))
        );
    }
}

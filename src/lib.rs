use std::collections::{BTreeMap, BTreeSet, HashMap};

use petgraph::graph::NodeIndex;

#[derive(Copy, Clone)]
pub enum BasicCardinalPoint {
    North,
    East,
    South,
    West,
}

#[derive(Copy, Clone)]
pub enum Direction {
    Vertical,
    Horizontal,
    Other,
}

fn direction_of(a: &Point, b: &Point) -> Direction {
    if a.x == b.x {
        Direction::Horizontal
    } else if a.y == b.y {
        Direction::Vertical
    } else {
        Direction::Other
    }
}

#[derive(Copy, Clone)]
pub enum Side {
    Top,
    Right,
    Bottom,
    Left,
}

pub enum BendDirection {
    Cardinal(BasicCardinalPoint),
    Unknown,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Copy, Clone)]
pub struct Size {
    pub width: i32,
    pub height: i32,
}

pub const fn make_size(width: i32, height: i32) -> Size {
    Size { width, height }
}

#[derive(Copy, Clone)]
pub struct Line {
    pub a: Point,
    pub b: Point,
}

#[derive(Copy, Clone)]
pub struct Rect {
    pub origin: Point,
    pub size: Size,
}

#[derive(Copy, Clone)]
pub struct ConnectorPoint {
    pub shape: Rect,
    pub side: Side,
    pub distance: f64,
}

pub struct OrthogonalConnectorByproduct {
    pub h_rulers: Vec<i32>,
    pub v_rulers: Vec<i32>,
    pub spots: Vec<Point>,
    pub grid: Vec<Rect>,
    pub connections: Vec<Line>,
}

pub struct OrthogonalConnectorOpts {
    pub point_a: ConnectorPoint,
    pub point_b: ConnectorPoint,
    pub shape_margin: i32,
    pub global_bounds_margin: i32,
    pub global_bounds: Rect,
}

pub const fn make_point(x: i32, y: i32) -> Point {
    Point { x, y }
}

pub fn distance(a: Point, b: Point) -> f64 {
    (((a.x - b.x).pow(2) + (a.y - b.y).pow(2)) as f64).sqrt()
}

fn min_max(x: &[i32]) -> (i32, i32) {
    (*x.iter().min().unwrap(), *x.iter().max().unwrap())
}

impl Rect {
    const EMPTY: Rect = Rect {
        origin: make_point(0, 0),
        size: make_size(0, 0),
    };

    fn from_ltrb(left: i32, top: i32, right: i32, bottom: i32) -> Rect {
        Self {
            origin: make_point(left, top),
            size: make_size(right - left, bottom - top),
        }
    }

    fn size(&self) -> Size {
        self.size
    }

    fn location(&self) -> Point {
        self.origin
    }

    fn left(&self) -> i32 {
        self.origin.x
    }

    fn right(&self) -> i32 {
        self.origin.x + self.size.width
    }

    fn top(&self) -> i32 {
        self.origin.y
    }

    fn bottom(&self) -> i32 {
        self.origin.y + self.size.height
    }

    fn width(&self) -> i32 {
        self.size.width
    }

    fn height(&self) -> i32 {
        self.size.height
    }

    fn north_east(&self) -> Point {
        make_point(self.right(), self.top())
    }

    fn south_east(&self) -> Point {
        make_point(self.right(), self.bottom())
    }

    fn south_west(&self) -> Point {
        make_point(self.left(), self.bottom())
    }

    fn north_west(&self) -> Point {
        make_point(self.left(), self.top())
    }

    fn east(&self) -> Point {
        make_point(self.left(), self.center().y)
    }

    fn north(&self) -> Point {
        make_point(self.center().x, self.top())
    }

    fn south(&self) -> Point {
        make_point(self.center().x, self.bottom())
    }

    fn west(&self) -> Point {
        make_point(self.left(), self.center().y)
    }

    fn contains(&self, p: &Point) -> bool {
        p.x >= self.left() && p.x <= self.right() && p.y >= self.top() && p.y <= self.bottom()
    }

    fn inflate(&self, horizontal: i32, vertical: i32) -> Self {
        Self::from_ltrb(
            self.left() - horizontal,
            self.top() - vertical,
            self.right() + horizontal,
            self.bottom() + vertical,
        )
    }

    fn intersects(&self, other: &Rect) -> bool {
        let this_x = self.left();
        let this_y = self.top();
        let this_w = self.width();
        let this_h = self.height();
        let rect_x = other.left();
        let rect_y = other.top();
        let rect_w = other.width();
        let rect_h = other.height();
        (rect_x < this_x + this_w)
            && (this_x < (rect_x + rect_w))
            && (rect_y < this_y + this_h)
            && (this_y < rect_y + rect_h)
    }

    fn union(&self, r: &Rect) -> Self {
        let x = [self.left(), self.right(), r.left(), r.right()];
        let y = [self.top(), self.bottom(), r.top(), r.bottom()];
        let (min_x, max_x) = min_max(&x);
        let (min_y, max_y) = min_max(&y);
        Self::from_ltrb(min_x, min_y, max_x, max_y)
    }

    fn center(&self) -> Point {
        make_point(
            self.left() + self.width() / 2,
            self.top() + self.height() / 2,
        )
    }
}

struct PointGraph {
    graph: petgraph::Graph<Point, f64>,
    nodes: HashMap<Point, NodeIndex>,
}

impl PointGraph {
    fn add(&mut self, pt: Point) -> NodeIndex {
        if let Some(ndx) = self.nodes.get(&pt) {
            *ndx
        } else {
            let ndx = self.graph.add_node(pt);
            self.nodes.insert(pt, ndx);
            ndx
        }
    }
    fn connect(&mut self, a: Point, b: Point) {
        let weight = distance(a, b);
        let a = self.get(&a).unwrap();
        let b = self.get(&b).unwrap();
        self.graph.add_edge(a, b, weight);
    }
    fn has(&self, p: &Point) -> bool {
        self.nodes.contains_key(p)
    }
    fn get(&self, p: &Point) -> Option<NodeIndex> {
        self.nodes.get(p).cloned()
    }
    fn direction_of(&self, a: NodeIndex, b: NodeIndex) -> Direction {
        direction_of(&self.graph[a], &self.graph[b])
    }
}

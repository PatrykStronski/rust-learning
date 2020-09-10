#[derive(Clone,Copy,Debug)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn distance_to(&self, p2: &Point) -> f32 {
        ((self.x - p2.x).powi(2)+(self.y - p2.y).powi(2)).sqrt()
    }

    fn midpoint_with(&self, p2: &Point) -> Point {
        Point {
            x: (self.x + p2.x) / 2.,
            y: (self.y + p2.y) / 2.,
        }
    }
}

#[derive(Debug)]
struct Triangle {
    points: [Point; 3]
}

impl Copy for Triangle {}
impl Clone for Triangle {
    fn clone(&self) -> Triangle {
        Triangle {
            points: [ self.points[0], self.points[1], self.points[2]]
        }
    }
}


impl Triangle {
    fn get_area(&self) -> f32 {
        let p1 = &self.points[0];
        let p2 = &self.points[1];
        let p3 = &self.points[2];
        ((p1.x * (p2.y - p3.y) + p2.x * (p3.y - p1.y) + p3.x * (p1.y - p2.y)) / 2.).abs()
    }
}

fn calculate_distance(p1: &Point, p2: &Point) -> f32 {
    ((p1.x - p2.x).powi(2)+(p1.y - p2.y).powi(2)).sqrt()
}

fn main() {
    let point1 = Point {
        x: 1.524,
        y: 9.284,
    };

    let point2 = Point {
        x: 6.27,
        y: 5.264,
    };

    let triangle = Triangle {
        points: [point1, point2, Point { x: 30., y: 10. }]
    };

    let mut triangle2 = triangle;

    let midpoint = point1.midpoint_with(&point2);

    println!("Distance from function {}", calculate_distance(&point1, &point2));
    println!("Distance from method {}", point1.distance_to(&point2));
    println!("Midpoint from method [{}, {}]", midpoint.x, midpoint.y);
    println!("Triangle area is {}", triangle.get_area());
    println!("Triangle2 area is {}", triangle2.get_area());
    println!("{:?}", triangle2);
}


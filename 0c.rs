struct Nil;

struct Pair(i32, f32);

struct Point {
    x: f32,
    y: f32,
}

#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

fn rect_area(r: Rectangle) -> f32 {
    let Rectangle {
        p1: Point { x: x1, y: y1 },
        p2: Point { x: x2, y: y2 },
    } = r;
    (x2 - x1) * (y2 - y1)
}

fn square(p: Point, len: f32) -> Rectangle {
    Rectangle {
        p1: Point { x: p.x, y: p.y} ,
        p2: Point { x: p.x + len, y: p.y + len }
    }
}

fn main() {
    let point: Point = Point { x: 0.3, y: 0.4 };

    println!("point coordinates: ({}, {})", point.x, point.y);

    let Point { x: my_x, y: my_y } = point;

    let _rectangle = Rectangle {
        p1: Point { x: my_x, y: my_y },
        p2: point,
    };

    let _nil = Nil;

    let pair = Pair(1, 0.1);

    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    println!("rectangle has area of {}", rect_area(_rectangle));

    let _rectangle = Rectangle {
        p1: Point { x: 0.0, y: 0.0 },
        p2: Point { x: 4.3, y: 5.3 },
    };

    println!("non-empty rect has area of {}", rect_area(_rectangle));

    let _rectangle = square(Point { x: 0.1, y: 0.2 }, 3f32);

    println!("square has area of {}", rect_area(_rectangle));
}
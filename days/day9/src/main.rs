use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Clone, Copy)]
struct Edge {
    p1: Point,
    p2: Point,
}

fn parse_points(input: &str) -> Vec<Point> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let parts: Vec<&str> = line.split(',').collect();
            let x: i64 = parts[0].trim().parse().expect("invalid x");
            let y: i64 = parts[1].trim().parse().expect("invalid y");
            Point { x, y }
        })
        .collect()
}

fn build_edges(points: &[Point]) -> Vec<Edge> {
    let n = points.len();
    let mut edges = Vec::with_capacity(n);
    for i in 0..n {
        edges.push(Edge {
            p1: points[i],
            p2: points[(i + 1) % n],
        });
    }
    edges
}

fn point_on_segment(p: Point, e: Edge) -> bool {
    if e.p1.x == e.p2.x {
        // vertical
        p.x == e.p1.x && (p.y >= e.p1.y.min(e.p2.y) && p.y <= e.p1.y.max(e.p2.y))
    } else {
        p.y == e.p1.y && (p.x >= e.p1.x.min(e.p2.x) && p.x <= e.p1.x.max(e.p2.x))
    }
}

fn point_in_poly(point: Point, edges: &[Edge]) -> bool {
    // ray casting to the right, with boundary inclusion
    for &e in edges {
        if point_on_segment(point, e) {
            return true;
        }
    }

    let mut count = 0;
    for &edge in edges {
        let (x1, y1) = (edge.p1.x, edge.p1.y);
        let (x2, y2) = (edge.p2.x, edge.p2.y);

        if y1 == y2 {
            continue; // horizontal edge, skip
        }

        // ensure y1 < y2
        let (x_low, y_low, y_high) = if y1 < y2 { (x1, y1, y2) } else { (x2, y2, y1) };

        if point.y >= y_low && point.y < y_high {
            let x_int = x_low as f64 + (point.y - y_low) as f64 * (x2 - x1) as f64 / (y2 - y1) as f64;
            if x_int > point.x as f64 {
                count += 1;
            }
        }
    }

    count % 2 == 1
}

fn solve_part1(input: &str) -> i64 {
    let points = parse_points(input);
    let n = points.len();
    
    let mut max_area = 0;
    
    for i in 0..n {
        for j in (i + 1)..n {
            let dx = (points[i].x - points[j].x).abs() + 1;
            let dy = (points[i].y - points[j].y).abs() + 1;
            let area = dx * dy;
            max_area = max_area.max(area);
        }
    }
    
    max_area
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 || (args[1] != "part1" && args[1] != "part2") {
        eprintln!("Usage: {} <part1|part2>", args[0]);
        eprintln!("Example: {} part1 < input.txt", args[0]);
        std::process::exit(1);
    }
    
    let part = &args[1];

    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("failed to read input");

    let answer = if part == "part1" {
        solve_part1(&input)
    } else {
        solve_part2(&input)
    };
    
    println!("{answer}");
}

fn rect_inside_polygon(a: Point, b: Point, edges: &[Edge]) -> bool {
    let x1 = a.x.min(b.x);
    let x2 = a.x.max(b.x);
    let y1 = a.y.min(b.y);
    let y2 = a.y.max(b.y);

    let corners = [
        Point { x: x1, y: y1 },
        Point { x: x1, y: y2 },
        Point { x: x2, y: y1 },
        Point { x: x2, y: y2 },
    ];

    for &c in &corners {
        if !point_in_poly(c, edges) {
            return false;
        }
    }

    // rectangle edges
    let rect_edges = [
        Edge { p1: corners[0], p2: corners[1] }, // left
        Edge { p1: corners[1], p2: corners[3] }, // top
        Edge { p1: corners[3], p2: corners[2] }, // right
        Edge { p1: corners[2], p2: corners[0] }, // bottom
    ];

    for &pe in edges {
        let p_vertical = pe.p1.x == pe.p2.x;
        let (px1, py1, px2, py2) = (pe.p1.x, pe.p1.y, pe.p2.x, pe.p2.y);
        let (py_min, py_max) = (py1.min(py2), py1.max(py2));
        let (px_min, px_max) = (px1.min(px2), px1.max(px2));

        for &re in &rect_edges {
            let r_vertical = re.p1.x == re.p2.x;
            let (rx1, ry1, rx2, ry2) = (re.p1.x, re.p1.y, re.p2.x, re.p2.y);
            let (ry_min, ry_max) = (ry1.min(ry2), ry1.max(ry2));
            let (rx_min, rx_max) = (rx1.min(rx2), rx1.max(rx2));

            if p_vertical && r_vertical {
                if px1 == rx1 && !(py_max < ry_min || py_min > ry_max) {
                    // overlapping vertical lines on boundary; ok
                    continue;
                }
            } else if !p_vertical && !r_vertical {
                if py1 == ry1 && !(px_max < rx_min || px_min > rx_max) {
                    // overlapping horizontal lines; ok
                    continue;
                }
            } else {
                // one vertical, one horizontal
                let (vx, vy_min, vy_max) = if p_vertical { (px1, py_min, py_max) } else { (rx1, ry_min, ry_max) };
                let (hy, hx_min, hx_max) = if p_vertical { (ry1, rx_min, rx_max) } else { (py1, px_min, px_max) };
                if vx >= hx_min && vx <= hx_max && hy >= vy_min && hy <= vy_max {
                    // intersection point
                    let inter = Point { x: vx, y: hy };
                    // allow if intersection is exactly at a rectangle corner
                    let is_corner = corners.iter().any(|&c| c.x == inter.x && c.y == inter.y);
                    if !is_corner {
                        return false;
                    }
                }
            }
        }
    }

    true
}

fn solve_part2(input: &str) -> i64 {
    let points = parse_points(input);
    let n = points.len();
    if n == 0 {
        return 0;
    }

    let edges = build_edges(&points);
    let mut max_area = 0;

    for i in 0..n {
        for j in (i + 1)..n {
            let p1 = points[i];
            let p2 = points[j];
            if rect_inside_polygon(p1, p2, &edges) {
                let dx = (p1.x - p2.x).abs() + 1;
                let dy = (p1.y - p2.y).abs() + 1;
                let area = dx * dy;
                max_area = max_area.max(area);
            }
        }
    }

    max_area
}

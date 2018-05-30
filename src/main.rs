extern crate svg;

use std::u32;
use std::f32::consts::PI;
use svg::Document;
use svg::Node;
use svg::node;
use svg::node::element::Circle;
use svg::node::element::Line;
use svg::node::element::Marker;
use svg::node::element::Path;
use svg::node::element::Rectangle;
use svg::node::element::TextPath;
use svg::node::element::path::Data;

fn main() {

    let address = "0xb232C56b46AAF1291065Df4AcCe228aB0840a12d";

    let center: f32 = 450.0;
    let radius: f32 = 380.0;

    let outer_circle = Circle::new()
        .set("cx", center)
        .set("cy", center)
        .set("r", radius)
        .set("stroke", "black")
        .set("stroke-width", 5)
        .set("fill", "none");

    let background = Rectangle::new()
        .set("width", "100%")
        .set("height", "100%")
        .set("fill", "white");

    let outer_address_data = Data::new()
        .move_to((450, 450))
        .move_by((-400, 0))
        .elliptical_arc_by((400,400, 0, 0, 1, 800,0))
        .elliptical_arc_by((400,400, 0, 0, 1, -800,0))
        .close();

    let outer_address_path = Path::new()
        .set("id", "outer_address_path")
        .set("d", outer_address_data)
        .set("fill", "none");

    let address_text = node::element::Text::new()
        .add(
            TextPath::new()
                .set("xlink:href", "#outer_address_path")
                .add(node::Text::new(address)))
        .set("textLength", 2450)
        .set("font-size", "60px")
        .set("font-family", "monospace");

    let start_marker = Marker::new()
        .set("id", "start_marker")
        .set("markerWidth", "10")
        .set("markerHeight", "10")
        .set("refX", "5")
        .set("refY", "5")
        .set("markerUnits", "strokeWidth")
        .add(
            Circle::new()
                .set("cx", "5")
                .set("cy", "5")
                .set("r", "5")
                .set("stroke", "black")
                .set("fill", "white"));

    let end_marker = Marker::new()
        .set("id", "end_marker")
        .set("markerWidth", "10")
        .set("markerHeight", "10")
        .set("refX", "0")
        .set("refY", "5")
        .set("orient", "auto")
        .set("markerUnits", "strokeWidth")
        .add(
            Line::new()
                .set("x1", "0")
                .set("y1", "0")
                .set("x2", "0")
                .set("y2", "10")
                .set("stroke", "black"));

    let mut document = Document::new()
        .set("xmlns:xlink", "http://www.w3.org/1999/xlink")
        .set("viewBox", (0, 0, 900, 900))
        .add(background)
        .add(outer_circle)
        .add(outer_address_path)
        .add(start_marker)
        .add(end_marker)
        .add(address_text);

/*    for number in 0..65536 {
        let point = get_point(number);
        let point_element = Circle::new()
            .set("cx", point.0)
            .set("cy", point.1)
            .set("r", 3)
            .set("fill", "red")
            .set("fill-opacity", 1);
        document.append(point_element);
}*/

    let points = get_sigil_points(&address[2..]);
    let lines = get_sigil_lines(points);
    for line in lines {
        document.append(line);
    }

    svg::save("sigil.svg", &document).unwrap();
}

fn get_sigil_points(mut address: &str) -> Vec<(f32, f32)> {
    let mut points = Vec::new();
    while address != "" {
        let sigil_digit = get_next_sigil_digit(address);
        let point = get_point(sigil_digit);
        points.push(point);
        println!("{:x} {} {}", sigil_digit, point.0, point.1);
        address = &address[4..];
    }
    return points;
}

fn get_point(number: u32) -> (f32, f32) {
    let center: f32 = 450.0;
    if number == 0 {
        return (center, center);
    } else {
        let (radius, angle) = get_radius_and_angle(number, 2, 1);
        return (
            center + radius * angle.cos() / 200.0,
            center + radius * angle.sin() / 200.0
        )
    }
}

fn get_radius_and_angle(number: u32, circle_size: u32, prev_circle_size: u32) -> (f32, f32) {
    if number < circle_size {
        let radius = circle_size as f32;
        let angle = 2.0 * PI * ((number - prev_circle_size + 1) as f32) / (circle_size as f32 - prev_circle_size as f32);
        return (radius, angle);
    } else {
        // XXX Recursive function. --elopio - 20180530
        return get_radius_and_angle(number, circle_size * 2, circle_size);
    }
}

fn get_sigil_lines(mut points: Vec<(f32, f32)>) -> Vec<Line> {
    let mut lines = Vec::new();
    let mut point1 = points.remove(0);
    let initial_len = points.len();
    while points.len() > 0 {
        let point2 = points.remove(0);
        // TODO Add the markers at the end of loop instead of checking for
        // each point. --elopio - 20180530
        let line = Line::new()
            .set("x1", point1.0)
            .set("y1", point1.1)
            .set("x2", point2.0)
            .set("y2", point2.1)
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", 5)
            .set("stroke-linecap", "round")
            .set("marker-start",
                 if points.len() == initial_len - 1 {
                     "url(#start_marker)"
                 }
                 else {
                     ""
                 })
            .set("marker-end",
                 if points.len() == 0 {
                     "url(#end_marker)"
                 } else {
                     ""
                 });
        lines.push(line);
        point1 = point2;
    }
    return lines;
}

fn get_next_sigil_digit(address: &str) -> u32 {
    return u32::from_str_radix(&address[..4], 16)
        .expect("Wrong address");
}

mod svg_handle;

fn main() {
    let a = svg_handle::SVGHandle::new()
        .add_inside(String::from("value"))
        .add_inside(String::from("value2"));
}
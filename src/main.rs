winrt::include_bindings!();

fn main() {
    let _device =
        microsoft::graphics::canvas::CanvasDevice::new().expect("Failed to create CanvasDevice");

    println!("ok!");
}

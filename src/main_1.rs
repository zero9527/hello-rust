mod box1;
mod for_iteration;
mod hash_map;
mod slice;
mod struct1;
mod study;
mod tools;

fn main() -> std::io::Result<()> {
    box1::handle_test();

    // study::handle_test();
    // tools::handle_test();
    // for_iteration::handle_test();

    // struct1::handle_test();
    // hash_map::handle_test();
    // slice::handle_test();
}

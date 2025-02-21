pub mod array;
pub mod closure_capture;
pub mod command;
pub mod debug;
pub mod display;
pub mod enum1;
pub mod file;
pub mod format;
pub mod function_methods;
pub mod if_let;
// pub mod linked_list_enum;
pub mod list;
pub mod match1;
pub mod match2;
pub mod tuples;
pub mod while_let;

pub fn handle_test() {
    array::handle_test();

    closure_capture::handle_test();
    command::handle_test();
    enum1::handle_test();

    debug::handle_test();
    display::handle_test();
    format::handle_test();

    file::handle_test();

    function_methods::handle_test();
    if_let::handle_test();
    while_let::handle_test();

    // TODO
    // linked_list_enum::handle_test();

    list::handle_test();
    tuples::handle_test();

    match2::handle_test();
}

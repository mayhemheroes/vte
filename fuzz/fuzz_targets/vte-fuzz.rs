#![no_main]
use libfuzzer_sys::fuzz_target;
use vte::*;

// Create a dummy performer
#[derive(Default)]
struct Dispatcher {}

impl Perform for Dispatcher {
    fn osc_dispatch(&mut self, _params: &[&[u8]], _bell_terminated: bool) {}

    fn csi_dispatch(&mut self, _params: &Params, _intermediates: &[u8], _ignore: bool, _c: char) {}

    fn esc_dispatch(&mut self, _intermediates: &[u8], _ignore: bool, _byte: u8) {}

    fn hook(&mut self, _params: &Params, _intermediates: &[u8], _ignore: bool, _c: char) {}

    fn put(&mut self, _byte: u8) {}

    fn unhook(&mut self) {}
}

fuzz_target!(|data: &[u8]| {
    let mut dispatcher = Dispatcher::default();
    let mut parser = Parser::new();

    for byte in data {
        parser.advance(&mut dispatcher, *byte);
    }
});

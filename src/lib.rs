use binaryninja::{binaryview::*, command::register_for_address, command::AddressCommand};
use copypasta::{ClipboardContext, ClipboardProvider};
use log::info;

struct RelativeOffset;

impl AddressCommand for RelativeOffset {
    fn action(&self, view: &BinaryView, addr: u64) {
        let mut clip_ctx = ClipboardContext::new().expect("failed to grab clipboard context");

        // Get dos_header from symbol name to use to subtract
        let symbol = view.symbols_by_name("__dos_header");
        let addr_relative = addr - symbol.get(0).address();

        // Format string for printing to debug console and setting to clipboard
        let addr_to_string = format!("0x{:x}", addr_relative);
        info!("Relative address {}", addr_to_string.as_str());
        clip_ctx.set_contents(addr_to_string).unwrap();
    }

    fn valid(&self, _view: &BinaryView, _addr: u64) -> bool {
        true
    }
}

#[no_mangle]
pub extern "C" fn UIPluginInit() -> bool {
    binaryninja::logger::init(log::LevelFilter::Trace).expect("logger failed to initialize!");

    register_for_address(
        "Get relative Address",
        "Returns the relative address from header",
        RelativeOffset {},
    );
    true
}

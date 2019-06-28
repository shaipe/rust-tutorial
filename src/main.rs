// #[warn(unused_imports)]
// #[macro_use] 

extern crate nickel;

use std::io::Write;
use nickel::status::StatusCode::NotFound;
use nickel::{Nickel, NickelError, Action, Continue, Halt, Request};

fn main() {
    let mut server = Nickel::new();

    //this is how to overwrite the default error handler to handle 404 cases with a custom view
    fn custom_404<'a>(err: &mut NickelError, _req: &mut Request) -> Action {
        if let Some(ref mut res) = err.stream {
            if res.status() == NotFound {
                let _ = res.write_all(b"<h1>Call the police!</h1>");
                return Halt(())
            }
        }

        Continue(())
    }


    // issue #20178
    let custom_handler: fn(&mut NickelError, &mut Request) -> Action = custom_404;

    server.handle_error(custom_handler);

    server.listen("127.0.0.1:6767").unwrap();

}
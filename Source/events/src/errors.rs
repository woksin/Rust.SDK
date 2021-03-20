use snafu::{ResultExt, Snafu};

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("An event can't be None"))]
    EventCanNotBeNone
}

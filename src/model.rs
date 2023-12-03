#[derive(PartialEq, Debug)]
pub(crate) enum RequestMethod {
    Get,
    Post,
    Delete,
    Put,
    Unknown,
}

#[derive(PartialEq, Debug)]
pub(crate) enum RequestHandler {
    Add,
    Remove,
    Get,
    Unknown,
}
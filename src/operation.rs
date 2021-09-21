use std::path;

#[derive(Debug)]
pub enum Operation<'a> {
    Get(&'a path::Path),
    Set(&'a path::Path, &'a str),
}

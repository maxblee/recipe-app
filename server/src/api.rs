#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[cfg(test)]
mod tests {}

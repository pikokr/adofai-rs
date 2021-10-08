pub mod level;

#[cfg(test)]
mod tests {
    use crate::level::level;

    #[test]
    fn load() {
        level::Level::load(&"")
    }
}

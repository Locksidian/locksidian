fn hello(name: &'static str) -> String {
    return format!("Hello, {}!", name);
}

fn main() {
    println!("{}", hello("World"));
}

#[cfg(test)]
mod test {
    use hello;

    #[test]
    fn test_hello() {
        let world: String = hello("World");
        assert_eq!(world, "Hello, World!");
    }
}

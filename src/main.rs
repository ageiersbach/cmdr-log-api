fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(2, 1+1);
    }

}
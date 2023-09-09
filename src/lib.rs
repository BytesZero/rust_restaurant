mod front_of_house;
pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();
    // 相对路径
    front_of_house::hosting::add_to_waitlist();
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::front_of_house::hosting;
    use std::collections::HashMap;
    use std::fmt::Result;
    use std::io::Result as IoResult;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn add_waitlist() {
        let result = hosting::add_to_waitlist();
        assert_eq!(result, true);
    }

    #[test]
    fn add_hashmap() {
        let mut map = HashMap::new();
        map.insert(0, 2);
        assert_eq!(map.get(&0), Some(&2));
    }
}

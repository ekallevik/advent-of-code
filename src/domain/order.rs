
use itertools::Itertools;

pub fn pop_max<T: Ord>(list: &mut Vec<T>) -> T {
    let index = list
        .iter()
        .position_max_by(|x, y| x.cmp(y))
        .unwrap();

    list.remove(index)
}

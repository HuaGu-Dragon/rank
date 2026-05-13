use crate::table::{Data, RESOURCE_COUNT};

pub fn check_safety(input: &mut [Data], cur: &[usize; RESOURCE_COUNT]) -> bool {
    let mut available = *cur;
    let mut complete = 0;

    input.sort_unstable_by_key(|p| p.id);

    while let Some((idx, p)) = input[complete..]
        .iter_mut()
        .enumerate()
        .find(|(_, p)| p.need.iter().zip(available.iter()).all(|(n, a)| n <= a))
    {
        (0..RESOURCE_COUNT).for_each(|i| available[i] += p.allocation[i]);
        input.swap(complete + idx, complete);
        complete += 1;
    }

    complete == input.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::table::{Data, RESOURCE_COUNT};

    fn make_data(
        id: usize,
        allocation: [usize; RESOURCE_COUNT],
        need: [usize; RESOURCE_COUNT],
    ) -> Data {
        Data {
            id,
            name: format!("P{}", id),
            allocation,
            max: [0; RESOURCE_COUNT],
            need,
            finish: false,
        }
    }

    #[test]
    fn test_check_safety_safe() {
        let mut procs = [
            make_data(0, [0, 1, 0, 0, 0, 0], [7, 4, 3, 0, 0, 0]),
            make_data(1, [2, 0, 0, 0, 0, 0], [1, 2, 2, 0, 0, 0]),
            make_data(2, [3, 0, 2, 0, 0, 0], [6, 0, 0, 0, 0, 0]),
            make_data(3, [2, 1, 1, 0, 0, 0], [0, 1, 1, 0, 0, 0]),
            make_data(4, [0, 0, 2, 0, 0, 0], [4, 3, 1, 0, 0, 0]),
        ];

        let cur = [3, 3, 2, 0, 0, 0];

        assert!(check_safety(&mut procs, &cur));
    }

    #[test]
    fn test_check_safety_unsafe() {
        let mut procs = [
            make_data(0, [0, 1, 0, 0, 0, 0], [7, 4, 3, 0, 0, 0]),
            make_data(1, [2, 0, 0, 0, 0, 0], [1, 2, 2, 0, 0, 0]),
            make_data(2, [3, 0, 2, 0, 0, 0], [6, 0, 0, 0, 0, 0]),
            make_data(3, [2, 1, 1, 0, 0, 0], [0, 1, 1, 0, 0, 0]),
            make_data(4, [0, 0, 2, 0, 0, 0], [4, 3, 1, 0, 0, 0]),
        ];

        let cur = [0, 1, 1, 0, 0, 0];

        assert!(!check_safety(&mut procs, &cur));
    }
}

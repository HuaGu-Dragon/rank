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

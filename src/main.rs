use std::io::{Read, Write};

fn main() {
    let mut line = String::new();
    std::io::stdin().lock().read_to_string(&mut line).unwrap();
    let mut nums = line
        .split_ascii_whitespace()
        .skip(1)
        .map(|num| num.parse().unwrap())
        .collect();
    introspective(&mut nums);

    let out = std::io::stdout();
    for num in nums {
        out.lock().write_fmt(format_args!("{} ", num)).unwrap();
    }
}

#[inline]
fn introspective(seq: &mut Vec<i32>) {
    let depthlimit = 2 * ((seq.len() as f32).log2().floor() as u32);
    introsort(seq, depthlimit);
}

fn introsort(seq: &mut [i32], maxdepth: u32) {
    let n = seq.len();
    if n < 2 {
        return;
    } else if n <= 16 {
        insertionsort(seq);
    } else if maxdepth == 0 {
        heapsort(seq);
    } else {
        let p = partition(seq);
        introsort(&mut seq[0..p], maxdepth - 1);
        introsort(&mut seq[p + 1..n], maxdepth - 1);
    }
}

#[inline]
fn partition(v: &mut [i32]) -> usize {
    let len = v.len();
    let last_index = len - 1;
    let pivot_index = median3(v, 0, len / 2, last_index);

    v.swap(pivot_index, last_index);

    let mut store_index = 0;
    for i in 0..last_index {
        if &v[i] < &v[last_index] {
            v.swap(i, store_index);
            store_index += 1;
        }
    }

    v.swap(store_index, len - 1);
    store_index
}

#[inline]
fn heapsort(seq: &mut [i32]) {
    let end = seq.len();

    // construct heap
    for start in (0..end / 2).rev() {
        sift_down(seq, start, end - 1);
    }

    // sort
    for end in (1..seq.len()).rev() {
        seq.swap(end, 0);
        sift_down(seq, 0, end - 1);
    }
}

#[inline]
fn sift_down(seq: &mut [i32], start: usize, end: usize) {
    let mut root = start;
    loop {
        let mut child = root * 2 + 1;
        if child > end {
            break;
        }
        if child < end && seq[child] < seq[child + 1] {
            child += 1;
        }
        if seq[root] < seq[child] {
            seq.swap(root, child);
            root = child;
        } else {
            break;
        }
    }
}

#[inline]
fn insertionsort(seq: &mut [i32]) {
    let items = seq.len();
    if items < 2 {
        return;
    }
    for i in 1..items {
        let mut j = i;
        // move to left until in right place
        while j > 0 && seq[j - 1] > seq[j] {
            seq.swap(j, j - 1);
            j -= 1;
        }
    }
}

fn median3(seq: &[i32], a: usize, b: usize, c: usize) -> usize {
    let (aa, bb, cc) = (seq[a], seq[b], seq[c]);
    if (aa > bb) ^ (aa > cc) {
        a
    } else if (bb < aa) ^ (bb < cc) {
        b
    } else {
        c
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_introspective() {
        let mut elements = vec![];
        introspective(&mut elements);
        assert_eq!(elements, vec![]);

        let mut elements = vec![1];
        introspective(&mut elements);
        assert_eq!(elements, vec![1]);

        let mut elements = vec![1, 5, 6, 2, 3, 4];
        introspective(&mut elements);
        assert_eq!(elements, vec![1, 2, 3, 4, 5, 6]);

        let mut elements = vec![
            7, 4, 7, 9, 4, 2, 6, 7, 8, 3, 1, 3, 5, 6, 7, 8, 2, 1, 9, 234, 534, 534, 423, 123, 4,
            54, 34, 6,
        ];
        introspective(&mut elements);
        assert_eq!(
            elements,
            vec![
                1, 1, 2, 2, 3, 3, 4, 4, 4, 5, 6, 6, 6, 7, 7, 7, 7, 8, 8, 9, 9, 34, 54, 123, 234,
                423, 534, 534
            ]
        );

        let mut nums: Vec<i32> = (0..10000).map(|_| rand::random()).collect();
        let mut nums_copy = nums.clone();
        introspective(&mut nums);
        nums_copy.sort_unstable();
        assert_eq!(nums, nums_copy);
    }

    #[test]
    fn test_heap() {
        let mut elements = Vec::new();
        heapsort(&mut elements);
        assert_eq!(elements, vec![]);
        elements.push(1);
        heapsort(&mut elements);
        assert_eq!(elements, vec![1]);
        elements = vec![1, 5, 6, 2, 3, 4];
        heapsort(&mut elements);
        assert_eq!(elements, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_insertion() {
        let mut elements = Vec::new();
        insertionsort(&mut elements);
        assert_eq!(elements, vec![]);
        elements.push(1);
        insertionsort(&mut elements);
        assert_eq!(elements, vec![1]);
        elements = vec![1, 5, 6, 2, 3, 4];
        insertionsort(&mut elements);
        assert_eq!(elements, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn benchmark_sort() {
        let start = std::time::Instant::now();
        introspective(&mut vec![1, 3, 2]);
        println!("{:#?}", start.elapsed());
    }
}

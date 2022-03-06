use std::io::{BufRead, Write};

fn main() {
    let mut line = String::new();
    std::io::stdin().lock().read_line(&mut line).unwrap();
    let mut nums: Vec<i32> = line
        .split_ascii_whitespace()
        .skip(1)
        .map(|num| num.parse().unwrap())
        .collect();
    introspective(&mut nums);

    let out = std::io::stdout();
    let mut out = out.lock();
    for num in nums {
        let bytes = num_to_bytes(num);
        out.write_all(&bytes).unwrap();
        out.write_all(&[' ' as u8]).unwrap();
    }
}

#[inline(always)]
fn num_to_bytes(num: i32) -> [u8; 11] {
    // sign of num (two's complement, first bit)
    let mask = num >> 31;
    // val as abs of num
    let mut val = (num ^ mask) - mask;
    // register for digits (11 digits is enough for all i32)
    let mut msg = [0; 11];

    // add minus sign if negative
    if mask != 0 {
        msg[0] = '-' as u8; // char code for minus sign '-'
    }
    for i in 0..10 {
        // multiply by zero if no more digits (more efficient than branching)
        msg[11 - i - 1] = (val != 0) as u8 * (48 + val % 10) as u8;
        val /= 10;
    }
    msg
}

#[inline(always)]
fn introspective(seq: &mut [i32]) {
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

#[inline(always)]
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

#[inline(always)]
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

#[inline(always)]
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

#[inline(always)]
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
    use std::time::Instant;

    const SIZE: usize = 2048;

    fn data_random<F>(mut f: F)
    where
        F: FnMut(&mut [i32]),
    {
        let mut nums: Vec<i32> = (0..SIZE).map(|_| rand::random()).collect();
        let mut nums_copy = nums.clone();
        let start = Instant::now();
        f(&mut nums);
        println!("{:#?}", start.elapsed());
        nums_copy.sort();
        assert_eq!(nums, nums_copy);
    }

    #[test]
    fn test_random() {
        println!("Introspective:");
        data_random(introspective);
        println!("Insertion:");
        data_random(insertionsort);
        println!("Heapsort:");
        data_random(heapsort);
    }

    fn data_increasing<F>(mut f: F)
    where
        F: FnMut(&mut [i32]),
    {
        let mut nums: Vec<i32> = (-(SIZE as i32) / 2..(SIZE as i32) / 2).collect();
        let mut nums_copy = nums.clone();
        let start = Instant::now();
        f(&mut nums);
        println!("{:#?}", start.elapsed());
        nums_copy.sort();
        assert_eq!(nums, nums_copy);
    }

    #[test]
    fn test_increasing() {
        println!("Increasing data");

        println!("Introspective:");
        data_increasing(introspective);
        println!("Insertion:");
        data_increasing(insertionsort);
        println!("Heapsort:");
        data_increasing(heapsort);
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
        let start = Instant::now();
        introspective(&mut vec![1, 3, 2]);
        println!("{:#?}", start.elapsed());
    }
}

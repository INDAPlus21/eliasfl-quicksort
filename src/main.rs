use std::io::Read;

fn main() {
    let mut line = String::new();
    std::io::stdin().lock().read_to_string(&mut line).unwrap();
    let mut numbers = line
        .split_ascii_whitespace()
        .map(|num| num.parse::<i32>().unwrap());
    // discard length of following numbers
    let length = numbers.next().unwrap();
    // TODO: multithreaded https://doc.rust-lang.org/std/thread/
    if length > 50000 {
        // TODO: split sorting into 4 threads
    } else if length > 256 {
        // TODO: split sorting into 2 threads
    }
    let mut nums: Vec<_> = numbers.collect();
    nums.sort_unstable();
}

fn introspective(seq: &mut Vec<i32>) {
    let depthlimit = 2 * ((seq.len() as f32).log2().floor() as u32);
    introsort(seq, depthlimit);
}

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
        7, 4, 7, 9, 4, 2, 6, 7, 8, 3, 1, 3, 5, 6, 7, 8, 2, 1, 9, 234, 534, 534, 423, 123, 4, 54,
        34, 6,
    ];
    introspective(&mut elements);
    assert_eq!(
        elements,
        vec![
            1, 1, 2, 2, 3, 3, 4, 4, 4, 5, 6, 6, 6, 7, 7, 7, 7, 8, 8, 9, 9, 34, 54, 123, 234, 423,
            534, 534
        ]
    );

    let mut nums: Vec<i32> = (0..10000).map(|_| rand::random()).collect();
    let mut nums_copy = nums.clone();
    introspective(&mut nums);
    nums_copy.sort_unstable();
    assert_eq!(nums, nums_copy);
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
        let mid = (n - 1) / 2;
        let pivot = median3(seq, 0, mid, n - 1);
        let (start, _pivot, end) = seq.select_nth_unstable(pivot);

        introsort(start, maxdepth - 1);
        introsort(end, maxdepth - 1);
    }
}

fn median3(seq: &[i32], a: usize, b: usize, c: usize) -> usize {
    let (aa, bb, cc) = (seq[a], seq[b], seq[c]);
    if aa < bb {
        if bb < cc {
            b
        } else if aa < cc {
            c
        } else {
            a
        }
    } else {
        if aa < cc {
            a
        } else if bb < cc {
            c
        } else {
            b
        }
    }
}

fn partition(seq: &mut [i32], low: usize, high: usize) -> usize {
    let pivot = seq[low];
    let mut i: i32 = low as i32 - 1;
    let mut j: i32 = high as i32 + 1;
    loop {
        i = i + 1; // do, while
        while seq[i as usize] < pivot {
            i = i + 1;
        }
        j = j - 1; // do, while
        while seq[j as usize] > pivot {
            j = j - 1;
        }
        if i >= j {
            return j as usize;
        }
        seq.swap(i as usize, j as usize);
    }
}

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

fn quicksort(seq: &mut Vec<i32>) {
    if seq.len() < 2 {
        return;
    }
    // Insert
    return;
}

#[test]
fn test_quick() {
    let mut elements = Vec::new();
    quicksort(&mut elements);
    assert_eq!(elements, vec![]);
    elements.push(1);
    quicksort(&mut elements);
    assert_eq!(elements, vec![1]);
    elements = vec![1, 5, 6, 2, 3, 4];
    quicksort(&mut elements);
    assert_eq!(elements, vec![1, 2, 3, 4, 5, 6]);
}

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
    quicksort(&mut vec![1, 3, 2]);
    println!("{:#?}", start.elapsed());
}

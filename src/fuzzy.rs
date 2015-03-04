use std::ops::{Index, IndexMut};
use std::usize;

struct MemoMatrix<T> {
    rows: usize,
    cols: usize,
    v: Vec<Option<T>>,
}

impl<T> MemoMatrix<T> {
    fn new(rows: usize, cols: usize) -> Self {
        let mut v = Vec::with_capacity(rows*cols);
        for _ in 0..(rows*cols) {
            v.push(None);
        }
        MemoMatrix { rows: rows, cols: cols, v: v }
    }
}

impl<T> Index<(usize, usize)> for MemoMatrix<T> {
    type Output = Option<T>;
    fn index<'a>(&'a self, index: &(usize, usize)) -> &'a Option<T> {
        let (r, c) = *index;
        assert!(r < self.rows, "row index '{}' is out of bounds", r);
        assert!(c < self.cols, "column index '{}' is out of bounds", c);
        &self.v[r * self.cols + c]
    }
}

impl<T> IndexMut<(usize, usize)> for MemoMatrix<T> {
    fn index_mut<'a>(&'a mut self, index: &(usize, usize)) -> &'a mut Option<T> {
        let (r, c) = *index;
        assert!(r < self.rows, "row index '{}' is out of bounds", r);
        assert!(c < self.cols, "column index '{}' is out of bounds", c);
        &mut self.v[r * self.cols + c]
    }
}

pub fn fuzzy_contains<'a, 'b>(p: &'a str, t: &'b str) -> bool {
    fuzzy_contains_err(p, t, 1)
}

fn fuzzy_contains_err<'a, 'b>(p: &'a str, t: &'b str, k: usize) -> bool { 
    lev_substring(p, t) <= k
}

// returns the minimum edit distance between `p` and all substrings of `t`
fn lev_substring<'a, 'b>(p: &'a str, t: &'b str) -> usize {
    let p_chars: Vec<char> = p.chars().collect();
    let t_chars: Vec<char> = t.chars().collect();

    // M(i, j) is the minimal cost of an edit sequence that turns p[..i] into t[..j]
    let n = t.chars().count();
    let mut rect = MemoMatrix::new(p.chars().count() + 1, n + 1);

    let mut min = usize::MAX;
    for k in 0..(n+1) {
        let dist = ed(&mut rect, &p_chars[], &t_chars[..k]);
        if dist < min {
            min = dist;
        }
    }

    min
}

fn ed<'a, 'b>(rect: &mut MemoMatrix<usize>, p: &'a [char], t: &'b [char]) -> usize {
    let (i, j) = (p.len(), t.len());

    // check if this has already been computed and use it if so
    match rect[(i, j)] {
        Some(dist) => return dist,
        None => {},
    }

    let dist = if i == 0 {
        0
    } else if j == 0 {
        i
    } else {
        let (a, b) = (i-1, j-1);
        if p[a] == t[b] {
            ed(rect, &p[..a], &t[..b])
        } else {
            let v = vec![
                ed(rect, &p[..a], &t[..b]),
                ed(rect, &p[..a], t),
                ed(rect, p, &t[..b])
            ];
            v.into_iter().min().unwrap() + 1
        }
    };

    rect[(i, j)] = Some(dist);
    dist
}

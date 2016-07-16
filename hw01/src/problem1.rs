pub fn sum(slice: &[i32]) -> i32 {
    // TODO
    let mut out : i32 = 0;
    for x in slice {
        out = out + x
    }
    out
}

/// Deduplicates items in the input vector `vs`. Produces a vector containing
/// the first instance of each distinct element of `vs`, preserving the
/// original order.
pub fn dedup(vs: &Vec<i32>) -> Vec<i32> {
    // TODO
    let mut out : Vec<i32> = Vec::new();
    for x in vs {
        if !out.contains(x) {
            out.push(*x)
        }
    }
    out
}

/// Filters a vector `vs` using a predicate `pred` (a function from `i32` to
/// `bool`). Returns a new vector containing only elements that satisfy `pred`.
pub fn filter(vs: &Vec<i32>, pred: &Fn(i32) -> bool) -> Vec<i32> {
    // TODO
    let mut filtered : Vec<i32> = Vec::new();
    
    for x in vs {
        if pred(*x) {
            filtered.push(*x)
        }
    }
    filtered
}


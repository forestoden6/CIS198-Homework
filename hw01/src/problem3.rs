/// Find all prime numbers less than 'n'.
/// For example, 'sieve(7)' should return '[2, 3, 5]'
pub fn sieve(n: u32) -> Vec<u32> {
    // TODO
    
    let mut nums : Vec<u32> = Vec::new();

    let mut primes : Vec<u32> = Vec::new();

    for i in 2..n {
        if !nums.contains(&i) {
            primes.push(i);
            for j in i*i..n {
                if j % i == 0 { 
                    nums.push(j);
                }
            }
        }
    }
    primes
}

struct Prime {
    prime: usize,
    next: usize
}

pub struct Sieve {
    index: usize,
    end: usize,
    primes: Vec<Prime>
}

impl Iterator for Sieve {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        Some(self.next_prime())
    }
}

pub fn new() -> Sieve {
    Sieve {
        index: 0,
        end: 0,
        primes: Vec::new()
    }
}

impl Sieve {
    fn continue_sieve(&mut self) {
        let start = self.end | 1;
        let end = start * 2;
        let mut a: Vec<bool> = vec!(false; (end-start+1)/2);

        for p in &mut self.primes {
            while p.next < end {
                a[(p.next-start)/2] = true;
                (*p).next = p.next + p.prime * 2; 
            }
        }

        let mut prime = start;
        while prime < end {
            if !a[(prime-start)/2] {
                let mut next = prime * prime;
                while next < end {
                    a[(next-start)/2] = true;
                    next = next + prime * 2;
                }
                self.primes.push(Prime{prime: prime, next: next});
            }
            prime = prime + 2;
        }
        self.end = end;
    }

    fn next_prime(&mut self) -> usize {
        if self.end < 3 {
            self.end = 3;
            return 2;
        }
        while self.index >= self.primes.len() {
            self.continue_sieve();
        }
        self.index = self.index + 1;
        self.primes[self.index-1].prime
    }
}
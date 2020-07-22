//! Efficiently enumerate integer partitions.
//!
//! This is an implementation of a method described by
//! [Jerome Kelleher](http://jeromekelleher.net/generating-integer-partitions.html),
//! which takes a constant amount of time for each partition.

extern crate streaming_iterator;
pub use streaming_iterator::StreamingIterator;

/// Iterates over the partitions of a given positive integer.
#[derive(Debug)]
pub struct Partitions {
    a: Vec<usize>,
    k: usize,
    y: usize,
    next: State,
}

#[derive(Debug, PartialEq, Eq)]
enum State {
    A,
    B { x: usize, l: usize },
}

impl Partitions {
    /// Makes a new iterator.
    #[inline]
    pub fn new(n: usize) -> Partitions {
		if n == 0 {
			return Partitions {
				a: vec!(1),
				k: 0,
				y: 0,
				next: State::A,
			}
		}
        Partitions {
            a: vec![0; n + 1],
            k: 1,
            y: n - 1,
            next: State::A,
        }
    }

    /// Makes a new iterator, trying to avoid allocations.
    ///
    /// Any vector can be passed to this function, since its contents
    /// will be cleared and it will be filled with zeroes, but note
    /// that the vector will still reallocate if its capacity is less
    /// than `n + 1`.
    #[inline]
    pub fn recycle(n: usize, mut vec: Vec<usize>) -> Partitions {
        vec.clear();
		
		if n == 0 {
			vec.push(1);
			return Partitions {
				a: vec,
				k: 0,
				y: 0,
				next: State::A,
			}
		}
		
        vec.reserve(n + 1);
        for _ in 0..(n + 1) {
            vec.push(0);
        }

        Partitions {
            a: vec,
            k: 1,
            y: n - 1,
            next: State::A,
        }
    }

    /// Destroys the iterator and returns a vector for further use.
    ///
    /// You only need to call this function if you want to reuse the
    /// vector for something else. Its contents will be in an undefined
    /// state, and so cannot be relied upon.
    #[inline]
    pub fn end(self) -> Vec<usize> {
        self.a
    }
}

impl StreamingIterator for Partitions {
    type Item = [usize];

    fn get(&self) -> Option<&Self::Item> {
        if self.next == State::A && self.k == 0 && (self.a[0] == 0 || self.a.len() == 1) {
			if self.a[0] == 0 {
				None
			} else {
				Some(&[])
			}
		} else {
			Some(&self.a[..self.k + match self.next {
				State::A => 1,
				State::B { .. } => 2,
			}])
		}
    }

    #[inline]
    fn advance(&mut self) {
        let Partitions {
            ref mut a,
            ref mut k,
            ref mut y,
            ref mut next
        } = *self;

        match *next {
            State::A => {
                if *k == 0 {
                    if a.len() == 1 && a[0] == 1 {
                        a[0] = 2;
                    } else {
						a[0] = 0;
                    }
                } else {
                    *k -= 1;
                    let x = a[*k] + 1;

                    while 2 * x <= *y {
                        a[*k] = x;
                        *y -= x;
                        *k += 1;
                    }

                    let l = *k + 1;

                    if x <= *y {
                        a[*k] = x;
                        a[l] = *y;
                        *next = State::B { x, l };
                    } else {
                        a[*k] = x + *y;
                        *y = x + *y - 1;
                    }
                }
            },
            State::B { mut x, l } => {
                x += 1;
                *y -= 1;

                if x <= *y {
                    a[*k] = x;
                    a[l] = *y;
                    *next = State::B { x, l };
                } else {
                    a[*k] = x + *y;
                    *y = x + *y - 1;
                    *next = State::A;
                }
            },
        }
    }
}

#[test]
fn oeis() {
    //! Tests the first few entries of A000041.

    let tests: &[usize] = &[
        1, 1, 2, 3, 5, 7, 11, 15, 22,
        30, 42, 56, 77, 101, 135, 176, 231,
        297, 385, 490, 627, 792, 1002, 1255, 1575,
        1958, 2436, 3010, 3718, 4565, 5604, 6842, 8349,
        10143, 12310, 14883, 17977, 21637, 26015, 31185, 37338,
        44583, 53174, 63261, 75175, 89134, 105558, 124754, 147273,
        173525,
    ];

    for (i, &n) in tests.iter().enumerate().skip(1) {
        let mut p = Partitions::new(i);
        let mut c = 0;

        while let Some(x) = p.next() {
            let sum: usize = x.iter().cloned().sum();
            assert_eq!(sum, i);
            c += 1;
        }

        assert_eq!(c, n);
    }
}

#[test]
fn n0() {
	//! Tests the special case n == 0.

	let mut p = Partitions::new(0);
	assert_eq!(p.next().unwrap().len(), 0);
	assert_eq!(p.next(), None);
}

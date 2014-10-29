#![feature(macro_rules)]

macro_rules! count_exprs {
    () => (0);
    ($head:expr $(, $tail:expr)*) => (1 + count_exprs!($($tail),*));
}

macro_rules! recurrence {
    ( a[n]: $sty:ty = $($inits:expr),+ ... $recur:expr ) => {
        {
            const MEMORY: uint = count_exprs!($($inits),+);

            struct Recurrence {
                mem: [$sty, ..MEMORY],
                pos: uint,
            }

            struct IndexOffset<'a> {
                slice: &'a [$sty, ..MEMORY],
                offset: uint,
            }

            impl<'a> Index<uint, $sty> for IndexOffset<'a> {
                #[inline(always)]
                fn index<'b>(&'b self, index: &uint) -> &'b $sty {
                    let real_index = *index - self.offset + 2;
                    &self.slice[real_index]
                }
            }

            impl Iterator<u64> for Recurrence {
                fn next(&mut self) -> Option<u64> {
                    if self.pos < MEMORY {
                        let next_val = self.mem[self.pos];
                        self.pos += 1;
                        Some(next_val)
                    } else {
                        let next_val = {
                            let n = self.pos;
                            let a = IndexOffset { slice: &self.mem, offset: n };
                            $recur
                        };

                        {
                            use std::mem::swap;

                            let mut swap_tmp = next_val;
                            for i in range(0, MEMORY).rev() {
                                swap(&mut swap_tmp, &mut self.mem[i]);
                            }
                        }

                        self.pos += 1;
                        Some(next_val)
                    }
                }
            }

            Recurrence { mem: [$($inits),+], pos: 0 }
        }
    };
}

fn main() {
    let fib = recurrence![a[n]: u64 = 1, 1 ... a[n-1] + a[n-2]];

    for e in fib.take(10) { println!("{}", e) }
}

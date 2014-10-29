/*!
This crate defines a macro for creating iterators which implement recurrence
relations.
*/
#![feature(macro_rules)]

#[doc(hidden)]
#[macro_export]
macro_rules! _recurrence_count_exprs {
    () => (0);
    ($head:expr $(, $tail:expr)*) => (1 + _recurrence_count_exprs!($($tail),*));
}

/**
Expands to an expression implementing the `Iterator` trait, which yields successive
elements of the given recurrence relationship.

For example, you can define a Fibonacci sequence iterator like so:

```
# #![feature(phase)]
# #[phase(plugin)] extern crate recurrence;
# fn main() {
#     let _ =
recurrence![ fib[n]: f64 = 0.0, 1.0 ... fib[n-1] + fib[n-2] ]
#     ;
# }
```
*/
#[macro_export]
macro_rules! recurrence {
    ( $seq:ident [ $ind:ident ]: $sty:ty = $($inits:expr),+ ... $recur:expr ) => {
        {
            const MEMORY: uint = _recurrence_count_exprs!($($inits),+);

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
                    let real_index = *index - self.offset + MEMORY;
                    &self.slice[real_index]
                }
            }

            impl Iterator<$sty> for Recurrence {
                #[inline]
                fn next(&mut self) -> Option<$sty> {
                    if self.pos < MEMORY {
                        let next_val = self.mem[self.pos];
                        self.pos += 1;
                        Some(next_val)
                    } else {
                        let next_val = {
                            let $ind = self.pos;
                            let $seq = IndexOffset { slice: &self.mem, offset: $ind };
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

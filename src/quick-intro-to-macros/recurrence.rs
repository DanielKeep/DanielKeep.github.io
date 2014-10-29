#![allow(dead_code)]
#![feature(macro_rules)]

macro_rules! count_exprs {
	() => (0);
	($head:expr $(, $tail:expr)*) => (1 + count_exprs!($($tail),*));
}

macro_rules! recurrence {
	(
		$seq:ident [ $ind:ident ] : $sty:ty = $($inits:expr),+ ... $recur:expr
	) => {
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
					let real_index = *index - self.offset + MEMORY;
					&self.slice[real_index]
				}
			}

			impl Iterator<$sty> for Recurrence {
				#[inline]
				fn next(&mut self) -> Option<$sty> {
					use std::mem::swap;

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
							let mut swap_tmp = next_val.clone();
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

macro_rules! closed_form {
	(
		$seq:ident [ $ind:ident ] : $sty:ty = $($inits:expr),+ ... $recur:expr
	) => {
		{
			const INITS: uint = count_exprs!($($inits),+);
			struct ClosedForm {
			    inits: [$sty, ..INITS],
			    pos: uint,
			}

			impl Iterator<$sty> for ClosedForm {
				#[inline(always)]
				fn next(&mut self) -> Option<$sty> {
					let n = self.pos;
					match self.idx(n) {
						Some(v) => {
							self.pos += 1;
							Some(v)
						},
						None => None
					}
				}
			}

			impl RandomAccessIterator<$sty> for ClosedForm {
				#[inline(always)]
				fn indexable(&self) -> uint {
					::std::uint::MAX
				}

				#[inline]
				fn idx(&mut self, index: uint) -> Option<$sty> {
					if self.pos < INITS {
						Some(self.inits[index])
					} else {
						Some({
							let $ind = index;
							let $seq = &self.inits;
							{
								$recur
							}
						})
					}
				}
			}

			ClosedForm { inits: [$($inits),+], pos: 0 }
		}
	};
}

fn main() {
	println!("First 10 Fibonnaci numbers:");
	let fib = recurrence!{ a[n]: u64 = 1, 1 ... a[n-1] + a[n-2] };
	for e in fib.take(10) { println!("{}", e) }

	println!("\n0! thru 9!:");
	for e in recurrence!{ a[n]: u64 = 1 ... a[n-1] * n as u64 }.take(10) { println!("{}", e) }

	println!("\nFirst 10 odd integers:");
	for e in closed_form!(a[n]: u64 = 1, 2 ... a[0] + n as u64 * a[1]).take(10) { println!("{}", e) }
}

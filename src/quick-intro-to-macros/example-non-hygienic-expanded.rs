#![feature(macro_rules)]
#![feature(phase)]
#![no_std]
#![feature(globs)]
#[phase(plugin, link)]
extern crate "std" as std;
extern crate "native" as rt;
#[prelude_import]
use std::prelude::*;
fn main() {
    let fib =
        {
            const MEMORY: uint = 1 + 1 + 0;
            struct Recurrence {
                mem: [u64, ..MEMORY],
                pos: uint,
            }
            struct IndexOffset<'a> {
                slice: &'a [u64, ..MEMORY],
                offset: uint,
            }
            impl <'a> Index<uint, u64> for IndexOffset<'a> {
                #[inline(always)]
                fn index<'b>(&'b self, index: &uint) -> &'b u64 {
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
                        let next_val =
                            {
                                let n = self.pos;
                                let a =
                                    IndexOffset{slice: &self.mem, offset: n,};
                                a[n - 1] + a[n - 2]
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
            Recurrence{mem: [1, 1], pos: 0,}
        };
    for e in fib.take(10) {
        match (&e,) {
            (__arg0,) => {
                #[inline]
                #[allow(dead_code)]
                static __STATIC_FMTSTR: [&'static str, ..1u] = [""];
                let __args_vec =
                    &[::std::fmt::argument(::std::fmt::secret_show, __arg0)];
                let __args =
                    unsafe {
                        ::std::fmt::Arguments::new(__STATIC_FMTSTR,
                                                   __args_vec)
                    };
                ::std::io::stdio::println_args(&__args)
            }
        }
    }
}

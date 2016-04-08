
//
// This algoritm is based on the c implementation of xxHash
// The origonal algoritme has the following license

/*
   xxHash - Extremely Fast Hash algorithm Header File
   Copyright (C) 2012-2016, Yann Collet.
   BSD 2-Clause License (http://www.opensource.org/licenses/bsd-license.php)
   Redistribution and use in source and binary forms, with or without
   modification, are permitted provided that the following conditions are
   met:
       * Redistributions of source code must retain the above copyright
   notice, this list of conditions and the following disclaimer.
       * Redistributions in binary form must reproduce the above
   copyright notice, this list of conditions and the following disclaimer
   in the documentation and/or other materials provided with the
   distribution.
   THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
   LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
   A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
   OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
   SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
   LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
   DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
   THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
   (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
   OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
   You can contact the author at :
   - xxHash source repository : https://github.com/Cyan4973/xxHash
*/

use std::hash::Hasher;

use std::mem;
use std::ptr;


const PRIME_1: u64 = 11400714785074694791;
const PRIME_2: u64 = 14029467366897019727;
const PRIME_3: u64 =  1609587929392839161;
const PRIME_4: u64 =  9650029242287828579;
const PRIME_5: u64 =  2870177450012600261;

#[derive(Clone,Copy)]
pub struct XXHasher{
    total_len: u64,
    seed: u64,
    v1: u64,
    v2: u64,
    v3: u64,
    v4: u64,
    mem: [u64; 4],
    mem_size: u32,
}

//
//Macro taken form the std sip hash
//
macro_rules! rotl {
    ($x:expr, $b:expr) =>
    (($x << $b) | ($x >> (64_i64.wrapping_sub($b))))
}

impl XXHasher{

    pub fn new() -> Self{
        XXHasher{
            v1: PRIME_1.wrapping_add(PRIME_2),
            v2: PRIME_2,
            v3: 0,
            v4: 0_u64.wrapping_sub(PRIME_1),
            mem: [0,0,0,0],
            mem_size: 0,
            total_len: 0,
            seed: 0,
        }
    }

    pub fn reset(&mut self){
        *self = Self::new();
    }

    #[inline(always)]
    unsafe fn eat(p: *const u64,mut to:u64) -> u64{
        to = to.wrapping_add((*p).wrapping_mul(PRIME_2));
        to = rotl!(to,31);
        to = to.wrapping_mul(PRIME_1);
        to
    }
}

impl Hasher for XXHasher{
    fn write(&mut self,input: &[u8]){
        unsafe{
            let mut p: *const u8 = input.as_ptr();
            let b_end: *const u8 = p.offset(input.len() as isize);

            self.total_len += input.len() as u64;

            if self.mem_size + (input.len() as u32) < 32{

                let src: *mut u8 = mem::transmute(self.mem.as_mut_ptr());
                ptr::copy_nonoverlapping(input.as_ptr()
                                         ,src.offset(self.mem_size as isize)
                                         ,input.len());

                self.mem_size += input.len() as u32;
                return;
            }

            if self.mem_size != 0{

                let src: *mut u8 = mem::transmute(self.mem.as_ptr());
                ptr::copy_nonoverlapping(input.as_ptr()
                                         ,src.offset(self.mem_size as isize)
                                         ,32-self.mem_size as usize);

                {
                    let mut p: *const u64 = mem::transmute(self.mem.as_ptr());
                    self.v1 = Self::eat(p,self.v1);
                    p = p.offset(1);
                    self.v2 = Self::eat(p,self.v2);
                    p = p.offset(1);
                    self.v3 = Self::eat(p,self.v3);
                    p = p.offset(1);
                    self.v4 = Self::eat(p,self.v4);
                }
                p = p.offset(32-self.mem_size as isize);
                self.mem_size = 0;
            }

            if p.offset(32) <= b_end{
                println!("Called");
                let limit: *const u8 = b_end.offset(-32);
                loop{
                    self.v1 = Self::eat(mem::transmute(p),self.v1);
                    p = p.offset(8);
                    self.v2 = Self::eat(mem::transmute(p),self.v2);
                    p = p.offset(8);
                    self.v3 = Self::eat(mem::transmute(p),self.v3);
                    p = p.offset(8);
                    self.v4 = Self::eat(mem::transmute(p),self.v4);
                    p = p.offset(8);
                    if p <= limit{
                        break;
                    }
                }
            }

            if p < b_end{
                let src: *mut u8 = mem::transmute(self.mem.as_ptr());
                ptr::copy_nonoverlapping(p,src,(b_end as usize)-(p as usize));
                self.mem_size = ((b_end as usize)-(p as usize)) as u32;
            }
        }
    }

    fn finish(&self) -> u64{
        unsafe{
            let mut p: *const u8 = mem::transmute(self.mem.as_ptr());
            let b_end: *const u8 = p.offset(self.mem_size as isize);

            let mut h: u64;

            if self.total_len >= 32{
                let mut v1 = self.v1;
                let mut v2 = self.v2;
                let mut v3 = self.v3;
                let mut v4 = self.v4;

                h = rotl!(v1,1).wrapping_add(rotl!(v2,7))
                                .wrapping_add(rotl!(v3, 12))
                                  .wrapping_add(rotl!(v4, 18));

                v1 = v1.wrapping_mul(PRIME_2);
                v1 = rotl!(v1, 31);
                v1 = v1.wrapping_mul(PRIME_1);
                h ^= v1;
                h = h.wrapping_mul(PRIME_1).wrapping_add(PRIME_4);

                v2 = v2.wrapping_mul(PRIME_2);
                v2 = rotl!(v2, 31);
                v2 = v2.wrapping_mul(PRIME_1);
                h ^= v2;
                h = h.wrapping_mul(PRIME_1).wrapping_add(PRIME_4);

                v3 = v3.wrapping_mul(PRIME_2);
                v3 = rotl!(v3, 31);
                v3 = v3.wrapping_mul(PRIME_1);
                h ^= v3;
                h = h.wrapping_mul(PRIME_1).wrapping_add(PRIME_4);

                v4 = v4.wrapping_mul(PRIME_2);
                v4 = rotl!(v4, 31);
                v4 = v4.wrapping_mul(PRIME_1);
                h ^= v4;
                h = h.wrapping_mul(PRIME_1).wrapping_add(PRIME_4);

            }else{
                h = self.seed.wrapping_add(PRIME_5);
            }

            h += self.total_len;

            while p.offset(8) <= b_end{
                let p_u64: *const u64 = mem::transmute(p);
                let mut k1: u64 = *p_u64;

                k1 = k1.wrapping_mul(PRIME_2);
                k1 = rotl!(k1,31);
                k1 = k1.wrapping_mul(PRIME_1);
                h ^= k1;
                h = rotl!(h,27).wrapping_mul(PRIME_1)
                    .wrapping_add(PRIME_4);
                p = p.offset(8);
            }

            if p.offset(4) <= b_end{
                println!("{}",*p);
                let p_32: *const u32 = mem::transmute(p);
                h ^= ((*p_32) as u64).wrapping_mul(PRIME_1);
                h = rotl!(h, 23).wrapping_mul(PRIME_2)
                    .wrapping_add(PRIME_3);
                p = p.offset(4);
            }

            while p < b_end{
                h ^= ((*p) as u64).wrapping_mul(PRIME_5);
                h = rotl!(h, 11).wrapping_mul(PRIME_1);
                p = p.offset(1);
            }

            h ^= h >> 33;
            h = h.wrapping_mul(PRIME_2);
            h ^= h >> 29;
            h = h.wrapping_mul(PRIME_3);
            h ^= h >>32;

            h 
        }
    }
}

#[cfg(test)]
mod test{

    use super::*;

    use std::hash::Hasher;

//    #[test]
    fn test_value(){
        let mut hasher = XXHasher::new();
        let mut vec = Vec::new();
        hasher.write(&vec);
        let result = hasher.finish();
        println!("Result: {}",result);
        assert!(result == 0xEF46DB3751D8E999);

        let mut gen: u32 = 2654435761;
        for _ in 0..101{
            vec.push((gen >> 24) as u8);
            println!("{}",gen);
            gen = gen.wrapping_mul(gen);
        }

        hasher.reset();
        hasher.write(&vec[0..1]);
        let result = hasher.finish();
        println!("input: {:?}",&vec[0..1]);
        println!("result: {}",result);
        assert!(result == 0x4FCE394CC88952D8);

        hasher.reset();
        hasher.write(&vec[0..14]);
        let result = hasher.finish();
        println!("Result: {}",result);
        assert!(result == 0xCFFA8DB881BC3A3D);

        hasher.reset();
        hasher.write(&vec[0..101]);
        let result = hasher.finish();
        println!("Result: {}",result);
        assert!(result == 0x5B9611585EFCC9CB);
    }
}

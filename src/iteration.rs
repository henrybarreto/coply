use crate::{CHUNKS_BY_BUFFER, CHUNK_SIZE};

#[derive(Debug, Clone)]
pub struct Iteration {
    pub steps: u32,
    pub actual_step: u32,
    pub bytes: u8,
    pub last_bytes: u8,
}
impl Iteration {
    pub fn new(file_size: u64, chunk_size: u8) -> Self {
        if (file_size % CHUNK_SIZE as u64) == 0 {
            Iteration {
                steps: (file_size / chunk_size as u64) as u32,
                actual_step: 1,
                bytes: chunk_size,
                last_bytes: chunk_size,
            }
        } else {
            Iteration {
                steps: ((file_size / chunk_size as u64) + 1) as u32,
                actual_step: 1,
                bytes: chunk_size,
                last_bytes: (file_size % chunk_size as u64) as u8,
            }
        }
    }
    pub fn iter<N>(&mut self, mut iter: N)
    where
        N: FnMut(u8) -> (),
    {
        for _i in 0..CHUNKS_BY_BUFFER {
            if self.actual_step <= self.steps {
                if self.is_last(self.actual_step) {
                    self.actual_step = self.actual_step + 1;
                    iter(self.last_bytes);
                } else {
                    self.actual_step = self.actual_step + 1;
                    iter(self.bytes);
                }
            } else {
                break;
            }
        }
    }
    pub fn is_last(&self, iteration: u32) -> bool {
        if self.steps == iteration {
            true
        } else {
            false
        }
    }
}

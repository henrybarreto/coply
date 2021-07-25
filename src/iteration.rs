/// Structure what helps split and iterate through the bytes of the file.
#[derive(Debug, Clone)]
pub struct Iteration {
    pub steps: u32,
    pub actual_step: u32,
    pub bytes: u8,
    pub last_bytes: u8,
}
impl Iteration {
    fn count_steps(file_size: u64, chunk_size: u8) -> u32 {
        let module = file_size % chunk_size as u64;
        if module == 0 {
            (file_size / chunk_size as u64) as u32
        } else {
            (file_size / chunk_size as u64) as u32 + 1
        }
    }
    fn last_bytes(file_size: u64, chunk_size: u8) -> u8 {
        let module = file_size % chunk_size as u64;
        if module == 0 {
            chunk_size
        } else {
            (file_size % chunk_size as u64) as u8
        }
    }

    pub fn new(file_size: u64, chunk_size: u8) -> Self {
        Iteration {
            steps: Self::count_steps(file_size, chunk_size),
            actual_step: 1,
            bytes: chunk_size,
            last_bytes: Self::last_bytes(file_size, chunk_size),
        }
        // if (file_size % CHUNK_SIZE as u64) == 0 {
        //     Iteration {
        //         steps: (file_size / chunk_size as u64) as u32,
        //         actual_step: 1,
        //         bytes: chunk_size,
        //         last_bytes: chunk_size,
        //     }
        // } else {
        //     Iteration {
        //         steps: ((file_size / chunk_size as u64) + 1) as u32,
        //         actual_step: 1,
        //         bytes: chunk_size,
        //         last_bytes: (file_size % chunk_size as u64) as u8,
        //     }
        // }
    }
    pub fn iter<N>(&mut self, mut iter: N)
    where
        N: FnMut(u8) -> (),
    {
        for _i in 1..5 {
            if self.actual_step <= self.steps {
                if self.is_last(self.actual_step) {
                    // self.actual_step = self.actual_step + 1;
                    iter(self.last_bytes);
                    self.actual_step += 1;
                    break;
                } else {
                    iter(self.bytes);
                    self.actual_step += 1;
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

#[cfg(test)]
mod interation_test {
    use super::Iteration;
    #[test]
    fn test_count_steps_less_chunk() {
        let r = Iteration::count_steps(100, 128);
        assert_eq!(r, 1);
    }
    #[test]
    fn test_count_steps_exact() {
        let r = Iteration::count_steps(512, 128);
        assert_eq!(r, 4);
    }
    #[test]
    fn test_count_steps() {
        let r = Iteration::count_steps(230, 128);
        assert_eq!(r, 2);
    }
    #[test]
    fn test_last_bytes() {
        let r = Iteration::last_bytes(130, 128);
        assert_eq!(r, 2);
    }
    #[test]
    fn test_last_bytes_exact() {
        let r = Iteration::last_bytes(512, 128);
        assert_eq!(r, 128);
    }
}

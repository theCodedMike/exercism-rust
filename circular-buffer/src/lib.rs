use std::fmt::Debug;

#[derive(Debug)]
pub struct CircularBuffer<T> {
    cap: usize,           // 容量，一旦初始化后就不再改变
    idx: usize,           // 开始元素的索引
    len: usize,           // 实际存储的元素的个数
    data: Vec<Option<T>>, // 容器
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T: Debug> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        assert_eq!(true, capacity > 0);

        let mut data = Vec::with_capacity(capacity);
        (0..capacity).for_each(|_| data.push(None));

        CircularBuffer {
            cap: capacity,
            idx: 0,
            len: 0,
            data,
        }
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        if self.len == self.cap {
            // full
            return Err(Error::FullBuffer);
        }
        let new_idx = (self.idx + self.len) % self.cap;
        self.data[new_idx] = Some(element);
        self.len += 1;
        Ok(())
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.len == 0 {
            // empty
            return Err(Error::EmptyBuffer);
        }

        self.data[self.idx]
            .take()
            .map(|v| {
                self.idx = (self.idx + 1) % self.cap;
                self.len -= 1;
                v
            })
            .ok_or(Error::EmptyBuffer)
    }

    pub fn clear(&mut self) {
        self.len = 0;
        self.idx = 0;
        self.data.iter_mut().for_each(|v| {
            if v.is_some() {
                *v = None;
            }
        });
    }

    pub fn overwrite(&mut self, element: T) {
        if self.len != self.cap {
            // not full
            if self.write(element).is_err() {
                panic!("Failed to write: overwrite(), self: {:?}", self);
            };
        } else {
            // full
            self.data[self.idx] = Some(element);
            self.idx = (self.idx + 1) % self.cap;
        }
    }
}

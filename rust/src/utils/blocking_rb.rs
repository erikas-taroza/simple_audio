// This file is a part of simple_audio
// Copyright (c) 2022-2023 Erikas Taroza <erikastaroza@gmail.com>
//
// This program is free software: you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation, either version 3 of
// the License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of 
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
// See the GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License along with this program.
// If not, see <https://www.gnu.org/licenses/>.

use std::{sync::{Mutex, atomic::AtomicUsize, Arc, Condvar}, ops::Range};

#[derive(Clone)]
pub struct BlockingRb<T>
{
    size: usize,
    buf: Arc<Mutex<Vec<T>>>,
    /// The spaces in the buffer that have data.
    /// This is represented with 2 ranges because of
    /// the ring buffer's wrapping nature.
    taken_spaces: Arc<Mutex<(Range<usize>, Range<usize>)>>,
    read_pos: Arc<AtomicUsize>,
    write_pos: Arc<AtomicUsize>,
    producer_events: Arc<(Mutex<Event>, Condvar)>,
}

impl<T: Copy + Clone + Default> BlockingRb<T>
{
    pub fn new(size: usize) -> Self
    {
        BlockingRb
        {
            size,
            buf: Arc::new(Mutex::new(vec![T::default(); size])),
            taken_spaces: Arc::new(Mutex::new((0..0, 0..0))),
            read_pos: Arc::new(AtomicUsize::new(0)),
            write_pos: Arc::new(AtomicUsize::new(0)),
            producer_events: Arc::new((Mutex::new(Event::None), Condvar::new()))
        }
    }

    /// Returns the number of free spaces in the ring buffer.
    fn num_free(&self) -> usize
    {
        let (range1, range2) = &*self.taken_spaces.lock().unwrap();
        self.size - (range1.len() + range2.len())
    }

    // ---------------------------------
    //             PRODUCER
    // ---------------------------------

    /// Blocks the thread until there is space in the
    /// buffer to write to. This operation can be cancelled
    /// by calling `cancel`.
    /// 
    /// Returns the number of items written.
    /// Returns `None` if the given slice is empty
    /// or the operation was cancelled.
    pub fn write(&self, slice: &[T]) -> Option<usize>
    {
        if slice.is_empty() { return None; }

        let num_free = self.num_free();

        // Don't block if the buffer has space for the slice.
        if num_free < slice.len()
        {
            // Wait for the event to tell us that there free space
            // available or that the operation should be cancelled.
            let (mutex, cvar) = &*self.producer_events;
            let mut event = mutex.lock().unwrap();
            event = cvar.wait(event).unwrap();

            match *event
            {
                Event::CancelWrite => return None,
                Event::FreeSpace => (),
                _ => panic!("This event is not supported by `write()`.")
            }
        }

        let mut buf = self.buf.lock().unwrap();

        // Write as much of the given slice as possible.
        // If the slice is larger than the buffer, then write until
        // the buffer size.
        let count = slice.len().min(num_free);

        let write_pos = self.write_pos.load(std::sync::atomic::Ordering::SeqCst);

        // The data can fit in line in the buffer.
        if write_pos + count < self.size
        {
            buf[write_pos..write_pos + count]
                .copy_from_slice(&slice[..count]);

            let mut lock = self.taken_spaces.lock().unwrap();
            lock.0 = lock.0.start..lock.0.end + count;
        }
        // The data is towards the end of the buffer and
        // needs to be wrapped.
        else
        {
            // How much data can be written before wrapping.
            let num_end = self.size - write_pos;
            buf[write_pos..].copy_from_slice(&slice[..num_end]);
            buf[..count - num_end].copy_from_slice(&slice[num_end..count]);

            let mut lock = self.taken_spaces.lock().unwrap();
            lock.0 = lock.0.start..self.size;
            lock.1 = 0..count - num_end;
        }

        self.write_pos.store((write_pos + count) % self.size, std::sync::atomic::Ordering::SeqCst);

        Some(count)
    }

    /// Cancels the current write operation.
    pub fn cancel_write(&self)
    {
        let (mutex, cvar) = &*self.producer_events;
        *mutex.lock().unwrap() = Event::CancelWrite;
        cvar.notify_all();
    }

    // ---------------------------------
    //             CONSUMER
    // ---------------------------------

    /// Reads from the ring buffer and fills the given slice
    /// with as much data as possible.
    /// 
    /// Returns the number of items written.
    /// Returns `None` if the given slice is empty
    /// or the buffer is empty.
    pub fn read(&self, slice: &mut [T]) -> Option<usize>
    {
        if slice.is_empty() || self.num_free() == self.size { return None; }

        let buf = self.buf.lock().unwrap();

        // Fill as much of the slice as possible.
        // If the slice is larger than the buffer, then read until
        // the buffer size.
        let count = slice.len().min(self.size);

        let read_pos = self.read_pos.load(std::sync::atomic::Ordering::SeqCst);

        // The data can be read in line from the buffer.
        if read_pos + count < self.size
        {
            slice[..count].copy_from_slice(
                &buf[read_pos..read_pos + count]);

            let mut lock = self.taken_spaces.lock().unwrap();
            lock.0 = lock.0.start + count..lock.0.end;
        }
        // The read position is towards the end of the buffer and
        // needs to be wrapped.
        else
        {
            // How much data can be written before wrapping.
            let num_end = self.size - read_pos;
            slice[..num_end].copy_from_slice(&buf[read_pos..]);
            slice[num_end..count].copy_from_slice(&buf[..count - num_end]);

            let mut lock = self.taken_spaces.lock().unwrap();
            lock.0 = count - num_end..lock.0.start;
            lock.1 = 0..0;
        }

        self.read_pos.store((read_pos + count) % self.size, std::sync::atomic::Ordering::SeqCst);

        let (mutex, cvar) = &*self.producer_events;
        *mutex.lock().unwrap() = Event::FreeSpace;
        cvar.notify_all();

        Some(count)
    }

    /// Sets the read position to the write position.
    /// This lets the consumer skip reading all the data
    /// in between in case it is useless.
    pub fn skip_all(&self)
    {
        let write_pos = self.write_pos.load(std::sync::atomic::Ordering::SeqCst);
        self.read_pos.store(write_pos, std::sync::atomic::Ordering::SeqCst);
    }
}

/// Ring buffer events.
#[derive(Clone, Copy)]
enum Event
{
    None,
    /// There is free space in the buffer (sent after the buffer was read).
    FreeSpace,
    /// The write operation has been cancelled.
    CancelWrite
}

mod tests
{
    /// Expected output:
    /// [1, 2, 3, 4, 5, 6, 7, 0, 0, 0]
    /// (0..7, 0..0)
    #[test]
    fn test_write()
    {
        let rb = crate::utils::blocking_rb::BlockingRb::<u32>::new(10);
        let writer = rb.clone();

        let data = vec![1, 2, 3, 4, 5];
        let _ = writer.write(&data);
        println!("{:?} {:?}", *rb.buf.lock().unwrap(), &*rb.taken_spaces.lock().unwrap());

        let data = vec![6, 7];
        let _ = writer.write(&data);
        println!("{:?} {:?}", *rb.buf.lock().unwrap(), &*rb.taken_spaces.lock().unwrap());
    }

    /// Expected output:
    /// [11, 12, 3, 4, 5, 6, 7, 8, 9, 10]
    /// (2..10, 0..2)
    /// 
    /// *Thread Blocked*
    #[test]
    fn test_write_wrap()
    {
        let rb = crate::utils::blocking_rb::BlockingRb::<u32>::new(10);
        let writer = rb.clone();
        let reader = rb.clone();

        let data = vec![1, 2, 3, 4, 5];
        let _ = writer.write(&data);
        println!("{:?} {:?}", *rb.buf.lock().unwrap(), &*rb.taken_spaces.lock().unwrap());

        let mut read_buf = vec![0; 2];
        let _ = reader.read(&mut read_buf);
        println!("{:?} {:?}", *rb.buf.lock().unwrap(), &*rb.taken_spaces.lock().unwrap());

        let data = vec![6, 7, 8, 9, 10, 11, 12];
        let _ = writer.write(&data);
        println!("{:?} {:?}", *rb.buf.lock().unwrap(), &*rb.taken_spaces.lock().unwrap());

        // This should block to prevent overwriting.
        let data = vec![13, 14, 15];
        let _ = writer.write(&data);
        println!("{:?} {:?}", *rb.buf.lock().unwrap(), &*rb.taken_spaces.lock().unwrap());
    }

    /// Expected output:
    /// [1, 2, 3, 4, 5, 0, 0, 0, 0, 0]
    /// (4..5, 0..0)
    #[test]
    fn test_read()
    {
        let rb = crate::utils::blocking_rb::BlockingRb::<u32>::new(10);
        let writer = rb.clone();
        let reader = rb.clone();

        let data = vec![1, 2, 3, 4, 5];
        let _ = writer.write(&data);
        println!("{:?} {:?}", *rb.buf.lock().unwrap(), &*rb.taken_spaces.lock().unwrap());

        let mut read_buf = vec![0; 2];
        let _ = reader.read(&mut read_buf);
        println!("{:?} {:?}", *rb.buf.lock().unwrap(), &*rb.taken_spaces.lock().unwrap());

        let mut read_buf = vec![0; 2];
        let _ = reader.read(&mut read_buf);
        println!("{:?} {:?}", *rb.buf.lock().unwrap(), &*rb.taken_spaces.lock().unwrap());
    }

    /// Expected output:
    /// [11, 12, 13, 14, 15, 6, 7, 8, 9, 10]
    /// (2..5, 0..0)
    #[test]
    fn test_read_wrap()
    {
        let rb = crate::utils::blocking_rb::BlockingRb::<u32>::new(10);
        let writer = rb.clone();
        let reader = rb.clone();

        let data = vec![1, 2, 3, 4, 5];
        let _ = writer.write(&data);
        println!("{:?} {:?}", *rb.buf.lock().unwrap(), &*rb.taken_spaces.lock().unwrap());

        let mut read_buf = vec![0; 5];
        let _ = reader.read(&mut read_buf);
        println!("{:?} {:?}", *rb.buf.lock().unwrap(), &*rb.taken_spaces.lock().unwrap());

        let data = vec![6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let _ = writer.write(&data);
        println!("{:?} {:?}", *rb.buf.lock().unwrap(), &*rb.taken_spaces.lock().unwrap());

        println!("{}", reader.read_pos.load(std::sync::atomic::Ordering::SeqCst));
        let mut read_buf = vec![0; 7];
        let _ = reader.read(&mut read_buf);
        println!("{:?} {:?}", *rb.buf.lock().unwrap(), &*rb.taken_spaces.lock().unwrap());
    }
}
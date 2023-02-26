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

use std::sync::{Mutex, atomic::AtomicUsize, Arc, Condvar};

pub struct BlockingRb<T>
{
    buf: Arc<Mutex<Vec<T>>>,
    read_pos: Arc<AtomicUsize>,
    write_pos: Arc<AtomicUsize>,
    producer: BlockingRbProducer<T>,
    consumer: BlockingRbConsumer<T>,
}

impl<T: Copy + Clone + Default> BlockingRb<T>
{
    pub fn new(size: usize) -> Self
    {
        let buf = Arc::new(Mutex::new(vec![T::default(); size]));
        let read_pos = Arc::new(AtomicUsize::new(0));
        let write_pos = Arc::new(AtomicUsize::new(0));
        
        let producer_events = Arc::new((Mutex::new(Event::None), Condvar::new()));

        let producer = BlockingRbProducer::new(
            size.clone(),
            buf.clone(),
            read_pos.clone(),
            write_pos.clone(),
            producer_events.clone()
        );

        let consumer = BlockingRbConsumer::new(
            size,
            buf.clone(),
            read_pos.clone(),
            write_pos.clone(),
            producer_events.clone()
        );

        BlockingRb
        {
            buf,
            read_pos,
            write_pos,
            producer,
            consumer
        }
    }

    pub fn producer(&self) -> BlockingRbProducer<T> {
        self.producer.clone()
    }

    pub fn consumer(&self) -> BlockingRbConsumer<T> {
        self.consumer.clone()
    }
}

/// The producer part of the ring buffer that provides
/// functions related to writing data.
#[derive(Clone)]
pub struct BlockingRbProducer<T>
{
    size: usize,
    buf: Arc<Mutex<Vec<T>>>,
    read_pos: Arc<AtomicUsize>,
    write_pos: Arc<AtomicUsize>,
    producer_events: Arc<(Mutex<Event>, Condvar)>
}

impl<T: Copy + Clone> BlockingRbProducer<T>
{
    fn new(
        size: usize,
        buf: Arc<Mutex<Vec<T>>>,
        read_pos: Arc<AtomicUsize>,
        write_pos: Arc<AtomicUsize>,
        producer_events: Arc<(Mutex<Event>, Condvar)>
    ) -> Self
    {
        BlockingRbProducer
        {
            size,
            buf,
            read_pos,
            write_pos,
            producer_events,
        }
    }

    /// Blocks the thread until there is space in the
    /// buffer to write to. This operation can be cancelled
    /// by calling `cancel`.
    /// 
    /// Returns the number of items written.
    /// Returns `None` if the given slice is empty,
    /// or the operation was cancelled.
    pub fn write(&self, slice: &[T]) -> Option<usize>
    {
        if slice.is_empty() { return None; }

        let num_free = helpers::num_free(
            self.read_pos.clone(),
            self.write_pos.clone(),
            self.size
        );

        println!("{} {}", num_free, self.size);

        // Don't block if the buffer is empty.
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
        }
        // The data is towards the end of the buffer and
        // needs to be wrapped.
        else
        {
            // How much data can be written before wrapping.
            let num_end = self.size - write_pos;
            buf[write_pos..].copy_from_slice(&slice[..num_end]);
            buf[..count - num_end].copy_from_slice(&slice[num_end..count]);
        }

        self.write_pos.store((write_pos + count) % self.size, std::sync::atomic::Ordering::SeqCst);

        Some(count)
    }

    /// Cancels the current write operation.
    pub fn cancel(&self)
    {
        let (mutex, cvar) = &*self.producer_events;
        *mutex.lock().unwrap() = Event::CancelWrite;
        cvar.notify_all();
    }
}

/// The consumer part of the ring buffer that provides
/// functions related to reading data.
#[derive(Clone)]
pub struct BlockingRbConsumer<T>
{
    size: usize,
    buf: Arc<Mutex<Vec<T>>>,
    read_pos: Arc<AtomicUsize>,
    write_pos: Arc<AtomicUsize>,
    producer_events: Arc<(Mutex<Event>, Condvar)>
}

impl<T: Copy + Clone> BlockingRbConsumer<T>
{
    fn new(
        size: usize,
        buf: Arc<Mutex<Vec<T>>>,
        read_pos: Arc<AtomicUsize>,
        write_pos: Arc<AtomicUsize>,
        producer_events: Arc<(Mutex<Event>, Condvar)>
    ) -> Self
    {
        BlockingRbConsumer { size, buf, read_pos, write_pos, producer_events }
    }

    /// Reads from the ring buffer and fills the given slice
    /// with as much data as possible.
    /// 
    /// Returns the number of items written or `None` if the given
    /// slice is empty.
    pub fn read(&self, slice: &mut [T]) -> Option<usize>
    {
        if slice.is_empty() { return None; }

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
        }
        // The read position is towards the end of the buffer and
        // needs to be wrapped.
        else
        {
            // How much data can be written before wrapping.
            let num_end = self.size - read_pos;
            slice[..num_end].copy_from_slice(&buf[read_pos..]);
            slice[num_end..count].copy_from_slice(&buf[..count - num_end]);
        }

        self.read_pos.store((read_pos + count) % self.size, std::sync::atomic::Ordering::SeqCst);

        let (mutex, cvar) = &*self.producer_events;
        *mutex.lock().unwrap() = Event::FreeSpace;
        cvar.notify_all();

        Some(count)
    }

    /// Sets the consumer's read position to the producer's write position.
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

mod helpers
{
    use std::sync::{Arc, atomic::AtomicUsize};

    /// Returns the number of free spaces in the ring buffer.
    pub fn num_free(read_pos: Arc<AtomicUsize>, write_pos: Arc<AtomicUsize>, size: usize) -> usize
    {
        let read_pos = read_pos.load(std::sync::atomic::Ordering::SeqCst);
        let write_pos = write_pos.load(std::sync::atomic::Ordering::SeqCst);

        if write_pos < read_pos {
            read_pos - write_pos
        }
        else {
            size - write_pos + read_pos
        }
    }
}

mod tests
{
    use super::BlockingRb;

    #[test]
    fn test_write()
    {
        let rb = BlockingRb::<u32>::new(10);
        let writer = rb.producer();

        let data = vec![1, 2, 3, 4, 5];
        
        let written = writer.write(&data);
        println!("{:?} {written:?} {}", *rb.buf.lock().unwrap(), rb.write_pos.load(std::sync::atomic::Ordering::SeqCst));
    }

    #[test]
    fn test_write_wrap()
    {
        let rb = BlockingRb::<u32>::new(10);
        let writer = rb.producer();
        let reader = rb.consumer();

        let data = vec![1, 2, 3, 4, 5];
        let written = writer.write(&data);
        println!("{:?} {written:?} {}", *rb.buf.lock().unwrap(), rb.write_pos.load(std::sync::atomic::Ordering::SeqCst));

        let mut read_buf = vec![0; 2];
        let read = reader.read(&mut read_buf);
        println!("{:?} {read:?} {}", read_buf, rb.read_pos.load(std::sync::atomic::Ordering::SeqCst));

        let data = vec![1, 2, 3, 4, 5, 6, 7];
        let written = writer.write(&data);
        println!("{:?} {written:?} {}", *rb.buf.lock().unwrap(), rb.write_pos.load(std::sync::atomic::Ordering::SeqCst));

        // This is overwriting (BAD).
        let data = vec![10, 11, 12];
        let written = writer.write(&data);
        println!("{:?} {written:?} {}", *rb.buf.lock().unwrap(), rb.write_pos.load(std::sync::atomic::Ordering::SeqCst));
    }

    #[test]
    fn test_read()
    {

    }

    #[test]
    fn test_num_slots_free()
    {

    }
}
use futures::Stream;
use pin_project::pin_project;
use std::{
    pin::Pin,
    task::{Context, Poll},
};

use crate::par_stream::{DistributedStream, ParallelStream};

pub struct ResultExpand<T, E>(pub Result<T, E>);
impl<T, E> IntoIterator for ResultExpand<T, E>
where
    T: IntoIterator,
{
    type Item = Result<T::Item, ()>;
    type IntoIter = ResultExpandIter<T::IntoIter>;
    fn into_iter(self) -> Self::IntoIter {
        loop {}
    }
}
#[pin_project(project=ResultExpandIterProj)]
pub enum ResultExpandIter<T> {
    Ok(#[pin] T),
}
impl<T> ResultExpandIter<T> {
    pub fn new(_: Result<T, ()>) -> ResultExpandIter<T> {
        loop {}
    }
}
impl<T> Iterator for ResultExpandIter<T>
where
    T: Iterator,
{
    type Item = Result<T::Item, ()>;
    fn next(&mut self) -> Option<Self::Item> {
        loop {}
    }
}
impl<T> Stream for ResultExpandIter<T>
where
    T: Stream,
{
    type Item = Result<T::Item, ()>;
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        match self.project() {
            ResultExpandIterProj::Ok(t) => t.poll_next(cx),
        };
        Poll::Ready(Some(Err(())))
    }
}

#[repr(transparent)]
pub struct DistParStream<S>(S);
impl<S> ParallelStream for DistParStream<S>
where
    S: DistributedStream,
{
    type Item = S::Item;
    type Task = S::Task;
}

impl<S> DistParStream<S> {
    pub fn new(s: S) -> Self {
        loop {}
    }
}

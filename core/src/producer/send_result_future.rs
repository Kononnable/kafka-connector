use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use super::error::SendError;

pub struct SendResultFuture {}
impl Future for SendResultFuture {
    type Output = Result<(), SendError>;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        todo!()
    }
}

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

// /// A SimpleFuture that runs two other futures to completion concurrently.
// ///
// /// Concurrency is achieved via the fact that calls to `poll` each future
// /// may be interleaved, allowing each future to advance itself at its own pace.
// pub struct Join<FutureA, FutureB> {
//     // Each field may contain a future that should be run to completion.
//     // If the future has already completed, the field is set to `None`.
//     // This prevents us from polling a future after it has completed, which
//     // would violate the contract of the `Future` trait.
//     a: Option<FutureA>,
//     b: Option<FutureB>,
// }

// impl<FutureA, FutureB> SimpleFuture for Join<FutureA, FutureB>
// where
//     FutureA: SimpleFuture<Output = ()>,
//     FutureB: SimpleFuture<Output = ()>,
// {
//     type Output = ();
//     fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
//         // Attempt to complete future `a`.
//         if let Some(a) = &mut self.a {
//             if let Poll::Ready(()) = a.poll(wake) {
//                 self.a.take();
//             }
//         }

//         // Attempt to complete future `b`.
//         if let Some(b) = &mut self.b {
//             if let Poll::Ready(()) = b.poll(wake) {
//                 self.b.take();
//             }
//         }

//         if self.a.is_none() && self.b.is_none() {
//             // Both futures have completed-- we can return successfully
//             Poll::Ready(())
//         } else {
//             // One or both futures returned `Poll::Pending` and still have
//             // work to do. They will call `wake()` when progress can be made.
//             Poll::Pending
//         }
//     }
// }

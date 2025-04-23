use std::{
    result::Result,
    sync::mpsc::{Receiver, RecvError, SendError, SyncSender, TryRecvError},
};

pub struct Comms<T> {
    rx: Receiver<T>,
    tx: SyncSender<T>,
}

impl<T> Comms<T> {
    pub fn new(rx: Receiver<T>, tx: SyncSender<T>) -> Comms<T> {
        Comms { rx, tx }
    }

    pub fn recv(&self) -> Result<T, RecvError> {
        self.rx.recv()
    }

    pub fn try_recv(&self) -> Result<T, TryRecvError> {
        self.rx.try_recv()
    }

    pub fn send(&self, t: T) -> Result<(), SendError<T>> {
        self.tx.send(t)
    }
}

pub mod board;
use self::board::{Board};
use std::sync::{RwLock, LockResult, RwLockWriteGuard};

pub struct Game<T> {
    board: Board<T>
}


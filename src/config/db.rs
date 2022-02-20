pub mod postgres;

pub trait DbPool {
    fn retrieve() -> Self;
}

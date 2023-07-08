pub trait Stable<S, R> {
    fn save(&mut self) -> S;
    fn restore(&mut self, restore: R);
}

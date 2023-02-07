pub trait Position {
    fn position(&self) -> (f64, f64);
    fn distance<T: Position>(&self, other: &T) -> f64 {
        let first = self.position();
        let second = other.position();
        let delta_x = first.0 - second.0;
        let delta_y = first.1 - second.1;
        (delta_x.powf(2.0) + delta_y.powf(2.0)).sqrt()
    }
}
pub trait Time {
    fn time(&self) -> i64;
}

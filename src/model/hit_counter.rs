pub trait HitCounter {
    fn hit_count(&self) -> i32;
    fn increase_hit_count(&mut self);
    fn reset_hit_count(&mut self);
}

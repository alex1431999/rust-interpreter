pub trait Cursor<T: std::fmt::Debug + PartialEq> {
    fn list(&self) -> &[T];
    fn position(&self) -> usize;
    fn position_mut(&mut self) -> &mut usize;

    fn consume(&mut self, item: &T) {
        let list = self.list();
        let item_resolved = list.get(self.position());

        if item_resolved != Some(item) {
            panic!("Expected {:?} but got {:?} instead", item, item_resolved);
        }

        self.advance(1);
    }

    fn advance(&mut self, steps: usize) {
        *self.position_mut() += steps;
    }
}

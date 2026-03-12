pub trait Cursor<T: std::fmt::Debug + PartialEq + Clone> {
    fn items(&self) -> &[T];
    fn position(&self) -> usize;
    fn position_mut(&mut self) -> &mut usize;

    fn consume(&mut self, item: &T) {
        let items = self.items();
        let item_resolved = items.get(self.position());

        if item_resolved != Some(item) {
            panic!("Expected {:?} but got {:?} instead", item, item_resolved);
        }

        self.advance(1);
    }

    fn advance(&mut self, steps: usize) {
        *self.position_mut() += steps;
    }

    fn has_more(&self) -> bool {
        self.position() < self.items().len()
    }

    fn get_current(&self) -> T {
        self.items()[self.position()].clone()
    }

    fn get_next(&self) -> T {
        self.items()[self.position() + 1].clone()
    }

    fn items_left(&self) -> usize {
        self.items().len() - self.position()
    }
}

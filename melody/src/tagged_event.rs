use std::cmp::Ordering;

#[derive(Debug)]
pub struct TaggedEvent<Event> {
    pub tag: u32,
    pub event: Event,
}

impl<Event> Ord for TaggedEvent<Event> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.tag.cmp(&(other.tag))
    }
}

impl<Event> PartialEq for TaggedEvent<Event> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl<Event> Eq for TaggedEvent<Event> {}

impl<Event> PartialOrd for TaggedEvent<Event> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

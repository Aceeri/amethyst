extern crate specs;

use self::specs::{Component, FlaggedStorage, VecStorage, Entity};

/// Component for defining a parent entity.
pub struct Child {
    /// The parent entity
    parent: Entity,
}

impl Child {
    /// Creates a new child
    pub fn new(entity: Entity) -> Child {
        Child {
            parent: entity,
        }
    }

    /// Returns our parent entity.
    #[inline]
    pub fn parent(&self) -> Entity {
        self.parent
    }

    /// Sets the given entity as our parent.
    #[inline]
    pub fn set_parent(&mut self, entity: Entity) {
        self.parent = entity;
    }
}

impl Component for Child {
    type Storage = FlaggedStorage<Self, VecStorage<Self>>;
}

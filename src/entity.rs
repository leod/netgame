use std::collections::BTreeMap;

use specs::prelude::*;

/// A unique id assigned by the server to an entity.
#[derive(Component, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub struct Id(u32);

/// A unique id assigned to an entity class.
#[derive(Component, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub struct ClassId(u32);

type Ctor = fn(EntityBuilder) -> EntityBuilder;

struct Class {
    name: &'static str,
    ctors: Vec<Ctor>,
}

/// Registry for entity classes.
/// 
/// The purpose of the registry is to allow creating entities in the same way
/// on the server and clients.
#[derive(Default)]
pub struct Reg {
    next_class_id: ClassId,
    classes: BTreeMap<ClassId, Class>,
    class_ids: BTreeMap<&'static str, ClassId>,
}

impl Reg {
    /// Register a new entity class.
    ///
    /// Calls to this function should always be performed identically on
    /// server and client, before the game starts (although the calls do not
    /// have to be in the same order)
    pub fn add(&mut self, name: &'static str, ctor: Ctor) -> ClassId {
        let class_id = self.next_class_id;
        self.next_class_id.0 += 1;

        let class = Class {
            name,
            ctors: vec![ctor],
        };

        self.classes.insert(class_id, class);
        self.class_ids.insert(name, class_id);

        class_id
    }

    /// Add a constructor for an existing class.
    ///
    /// This may be used to add components to an entity class which are
    /// necessary only on client or server side. For example, clients can add
    /// components containing information for rendering.
    pub fn add_ctor(&mut self, name: &'static str, ctor: Ctor) {
        let class_id = self.class_ids.get(name).expect(&format!(
            "Can't add ctor to unregistered entity class {}",
            name
        ));

        self.classes.get_mut(&class_id).unwrap().ctors.push(ctor);
    }
}

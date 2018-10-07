use std::collections::BTreeMap;

use specs::prelude::*;

/// A unique id assigned by the server to a client.
pub struct ClientId(u32);

/// A unique id assigned by the server to a player.
pub struct PlayerId(u32);

/// A unique id assigned by the server to an entity.
#[derive(Component, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub struct EntityId(u32);

/// A unique id assigned to an entity class.
#[derive(Component, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub struct EntityClassId(u32);

/// A unique id assigned to a component type.
#[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub struct ComponentTypeId(u32);

/// A function for adding components to an entity during its construction.
type EntityCtor = fn(EntityBuilder) -> EntityBuilder;

struct EntityType {
    name: &'static str,
    ctors: Vec<EntityCtor>,
    components: Vec<Box<ComponentType>>,
}

/// Meta information about component types that can be synchronized over the network.
pub struct ComponentType {
    name: &'static str,
    size: usize,
}

/// Registry for entity types.
/// 
/// The purpose of the registry is to allow creating entities in the same way
/// on the server and clients.
#[derive(Default)]
pub struct Registry {
    next_entity_type_id: EntityClassId,
    entity_type_ids: BTreeMap<&'static str, EntityClassId>,
    entity_types: BTreeMap<EntityClassId, EntityType>,

    next_component_type_id: ComponentTypeId,
    component_type_ids: BTreeMap<&'static str, ComponentTypeId>,
    component_types: BTreeMap<ComponentTypeId, ComponentType>,
}

impl Registry {  
    /// Register a new entity class.
    ///
    /// Calls to this function should always be performed identically on
    /// server and client, before the game starts (although the calls do not
    /// have to be in the same order)
    pub fn add_entity_type(&mut self, name: &'static str, ctor: EntityCtor) -> EntityClassId {
        let class_id = self.next_entity_type_id;
        self.next_entity_type_id.0 += 1;

        let class = EntityType {
            name,
            ctors: vec![ctor],
            components: vec![],
        };

        self.entity_types.insert(class_id, class);
        self.entity_type_ids.insert(name, class_id);

        class_id
    }

    /// Add a constructor for an existing entity class.
    ///
    /// This may be used to add components to an entity class which are
    /// necessary only on client or server side. For example, clients can add
    /// components containing information for rendering.
    pub fn add_entity_ctor(&mut self, name: &'static str, ctor: EntityCtor) {
        let class_id = self.entity_type_ids.get(name).expect(&format!(
            "Can't add ctor to unregistered entity class {}",
            name
        ));

        self.entity_types.get_mut(&class_id).unwrap().ctors.push(ctor);
    }
}
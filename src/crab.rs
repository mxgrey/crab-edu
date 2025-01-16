/*
 * Copyright (C) 2024 Michael X. Grey
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
*/

use bevy::{
    prelude::{
        Component, Entity, Command, World, StandardMaterial, Mesh, Assets, error,
        BuildChildren, Mesh3d, MeshMaterial3d, Transform, Visibility,
    },
    render::mesh::primitives::{Meshable, ConeMeshBuilder, MeshBuilder},
    math::{
        primitives::{Cone, Rectangle},
    }
};

use crate::{Pen, Stroke};

mod shapes;
use shapes::*;

#[derive(Debug, Clone)]
pub struct Crab {
    pub name: String,
    pub show_arrow: bool,
}

impl Default for Crab {
    fn default() -> Self {
        Crab {
            name: String::new(),
            show_arrow: true,
        }
    }
}

#[derive(Debug, Component)]
pub struct CrabName(pub String);

pub(crate) struct AddCrab {
    pub(crate) pen: Entity,
    pub(crate) crab: Crab,
}

impl Command for AddCrab {
    fn apply(self, world: &mut World) {
        let Some(pen) = world.get::<Pen>(self.pen).cloned() else {
            error!("Pen unavailable for crab [{}]", self.crab.name);
            return;
        };

        world.entity_mut(self.pen).insert((
            Transform::IDENTITY,
            Visibility::Inherited,
        ));

        if self.crab.show_arrow {
            let mesh = match pen.stroke {
                Stroke::Volume(diameter) => {
                    make_cylinder_arrow_mesh(diameter/2.0)
                }
            };

            let mesh_handle = world.resource_mut::<Assets<Mesh>>().add(mesh);
            let material_handle = world.resource_mut::<Assets<StandardMaterial>>().add(
                StandardMaterial::from_color(pen.color)
            );

            let crab = world.spawn((
                Mesh3d(mesh_handle),
                MeshMaterial3d(material_handle),
            )).id();

            world.entity_mut(self.pen).add_child(crab);
        }

        world.entity_mut(self.pen).insert(CrabName(self.crab.name));
    }
}

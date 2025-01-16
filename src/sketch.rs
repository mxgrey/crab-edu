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

use bevy::prelude::{
    App, AmbientLight, DefaultPlugins, Commands, Resource, Entity, Camera3d, Transform,
    Vec3,
};
pub use bevy::prelude::{AppExit, Color};

use crate::{AddCrab, Crab, CrabName, PenCommands, Pen, PenHandle, Schedule, Timeline};

pub struct Sketch {
    pub app: App,
}

impl Sketch {
    pub fn new() -> Self {
        let mut app = App::new();
        app
            .insert_resource(AmbientLight {
                color: Color::WHITE,
                brightness: 2000.,
                ..Default::default()
            })
            .init_resource::<Schedule>()
            .init_resource::<Timeline>()
            .add_plugins(DefaultPlugins);

        let main_camera = app.world_mut().spawn((
            Camera3d::default(),
            Transform::from_xyz(0., 0., 1.).looking_at(Vec3::ZERO, Vec3::Y),
        )).id();
        app.world_mut().insert_resource(MainCamera { entity: main_camera });

        Sketch { app }
    }

    pub fn spawn_pen(&mut self, pen: impl Into<Settings>) -> PenCommands {
        let settings: Settings = pen.into();
        let mut commands = self.app.world_mut().commands();
        let pen = settings.spawn_pen(&mut commands);
        PenCommands { pen, commands }
    }

    pub fn run(&mut self) -> AppExit {
        self.app.world_mut().flush();
        self.app.run()
    }
}

#[derive(Debug, Default, Clone)]
pub struct Settings {
    pub pen: Pen,
    pub crab: Crab,
}

impl Settings {
    pub fn spawn_pen(self, commands: &mut Commands) -> PenHandle {
        let pen = commands.spawn(self.pen).id();
        commands.queue(AddCrab { pen, crab: self.crab });

        PenHandle(pen)
    }
}

impl<T: Into<Pen>> From<T> for Settings {
    fn from(value: T) -> Self {
        Settings {
            pen: value.into(),
            crab: Default::default(),
        }
    }
}

#[derive(Resource)]
struct MainCamera {
    entity: Entity,
}

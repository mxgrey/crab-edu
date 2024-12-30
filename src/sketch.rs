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

use bevy::prelude::{App, AppExit, AmbientLight, Color, DefaultPlugins, Commands};

use crate::{Crab, CrabName, PenCommands, Pen, PenHandle, Schedule, Timeline};

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

        Sketch { app }
    }

    pub fn spawn_pen(&mut self, pen: impl Into<Settings>) -> PenHandle {
        let settings: Settings = pen.into();
        self.command(|commands| settings.spawn_pen(commands))
    }

    pub fn run(&mut self) -> AppExit {
        self.app.run()
    }

    fn command<U>(&mut self, f: impl FnOnce(&mut Commands) -> U) -> U{
        let u = f(&mut self.app.world_mut().commands());
        self.app.world_mut().flush();
        u
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

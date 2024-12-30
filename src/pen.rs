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

use bevy::prelude::{Color, Commands, Component, Entity, Vec2, Vec3, Command, World};

use crate::Schedule;

#[derive(Debug, Default, Component, Clone, Copy)]
pub struct Pen {
    pub color: Color,
    pub stroke: Stroke,
}

impl From<Color> for Pen {
    fn from(color: Color) -> Self {
        Self { color, stroke: Default::default() }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Stroke {
    Volume(f32),
    // Ribbon(f32),
    // Pixels(u32),
}

impl Default for Stroke {
    fn default() -> Self {
        Stroke::Volume(0.01)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PenHandle(pub(crate) Entity);

impl PenHandle {
    pub fn command<'w, 's, 'a>(
        &'a self,
        commands: &'a mut Commands<'w, 's>,
    ) -> PenCommands<'w, 's, 'a> {
        PenCommands { pen: *self, commands }
    }
}

pub struct PenCommands<'w, 's, 'a> {
    pen: PenHandle,
    commands: &'a mut Commands<'w, 's>,
}

impl<'w, 's, 'a> PenCommands<'w, 's, 'a> {

    pub fn draw(&mut self, movement: Movement) {
        self.commands.queue(PenAction {
            pen: self.pen.0,
            movement,
            draw: true,
        });
    }

    pub fn move_pen(&mut self, movement: Movement) {
        self.commands.queue(PenAction {
            pen: self.pen.0,
            movement,
            draw: false,
        });
    }

    pub fn draw_to(&mut self, point: impl IntoPoint) {
        self.draw(Movement::Point(point.into_point()));
    }

    pub fn draw_forward(&mut self, distance: f32) {
        self.draw(Movement::Relative { distance, direction: Direction::Forward });
    }

    pub fn draw_backward(&mut self, distance: f32) {
        self.draw(Movement::Relative { distance, direction: Direction::Backward });
    }

    pub fn draw_left(&mut self, distance: f32) {
        self.draw(Movement::Relative { distance, direction: Direction::Left });
    }

    pub fn draw_right(&mut self, distance: f32) {
        self.draw(Movement::Relative { distance, direction: Direction::Right });
    }

    pub fn draw_up(&mut self, distance: f32) {
        self.draw(Movement::Relative { distance, direction: Direction::Up });
    }

    pub fn draw_down(&mut self, distance: f32) {
        self.draw(Movement::Relative { distance, direction: Direction::Down });
    }

    pub fn move_to(&mut self, point: impl IntoPoint) {
        self.move_pen(Movement::Point(point.into_point()));
    }

    pub fn handle(self) -> PenHandle {
        self.pen
    }

    pub fn unpack(self) -> (PenHandle, &'a mut Commands<'w, 's>) {
        (self.pen, self.commands)
    }
}

pub trait IntoPoint {
    fn into_point(self) -> Vec3;
}

impl IntoPoint for Vec3 {
    fn into_point(self) -> Vec3 {
        self
    }
}

impl IntoPoint for Vec2 {
    fn into_point(self) -> Vec3 {
        Vec3::new(self.x, self.y, 0.)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Movement {
    Point(Vec3),
    Relative{ direction: Direction, distance: f32 },
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Forward,
    Backward,
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct PenAction {
    pen: Entity,
    movement: Movement,
    draw: bool,
}

impl Command for PenAction {
    fn apply(self, world: &mut World) {
        world.get_resource_or_init::<Schedule>().actions.push(self);
    }
}

//! Trixel Rust Library, serving at the core of Trixel,
//! a FOSS trigonometric painting library, allowing you build applications
//! to paint with triangles, pixels and hexagons.

/*
Trixel, a FOSS trigonometric painting app.
Copyright (C) 2021  Glen Henri J. De Cauwsemaeker and Elizabeth C. Gonzales Belsuzarri.

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.
*/

pub struct Color {}

pub struct Index {}

pub struct Unit {
    pub Index: Index,
    pub Color: Color,
}

pub enum Shape {}

pub struct Layer {
    pub Hidden: bool,
    pub Shape: Shape,
    pub Scale: u16,
    pub Units: Vec<Unit>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

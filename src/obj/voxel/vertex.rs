/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: obj/voxel/vertex.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      A vertex specific to voxels.
*/

use math::{ Vec3i8, Vec3u8 };
use super::{ Behavior, Default };

#[packed]
pub struct Vertex
{
  position: Vec3i8,
  color: Vec3u8,
  behavior: Behavior,
}

impl Vertex
{
  #[inline(always)]
  pub fn new(pos: Vec3i8, col: Vec3u8) -> Vertex
  { Vertex { position: pos, behavior: Default, color: col } }
  #[inline(always)]
  pub fn new_with_position(pos: Vec3i8) -> Vertex
  { Vertex { position: pos, behavior: Default, color: Vec3u8::new(pos.x as u8, pos.y as u8, pos.z as u8) } }
  #[inline(always)]
  pub fn zero() -> Vertex
  { Vertex { position: Vec3i8::zero(), behavior: Default, color: Vec3u8::new(1, 1, 1) } }
}

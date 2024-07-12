use std::marker::PhantomData;

use glam::Vec3;

use crate::{
    driver::RigDriver, handedness::Handedness, rig::RigUpdateParams, transform::Transform,
};

/// Offsets the camera along a vector, in the coordinate space of the parent.
#[derive(Debug)]
pub struct Arm {
    pub offset: Vec3,
}

impl Arm {
    pub fn new<V>(offset: V) -> Self
    where
        V: Into<Vec3>,
    {
        let offset = offset.into();

        Self { offset }
    }
}

impl<H: Handedness> RigDriver<H> for Arm {
    fn update(&mut self, params: RigUpdateParams<H>) -> Transform<H> {
        let parent_position = params.parent.position;
        let parent_rotation = params.parent.rotation;
        let offset = self.offset;

        let position = parent_position + parent_rotation * offset;

        Transform {
            rotation: params.parent.rotation,
            position,
            phantom: PhantomData,
        }
    }
}

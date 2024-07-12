use core::fmt::Debug;
use glam::{Quat, Vec3};
use std::marker::PhantomData;

use crate::handedness::Handedness;

/// A thin wrapper over a `Point3<f32>` and a `Quaternion<f32>`
#[derive(Clone, Copy, Debug)]
pub struct Transform<H: Handedness> {
    pub position: Vec3,
    pub rotation: Quat,
    pub phantom: PhantomData<H>,
}

impl<H: Handedness> Transform<H> {
    pub fn from_position_rotation<P, Q>(position: P, rotation: Q) -> Self
    where
        P: Into<Vec3>,
        Q: Into<Quat>,
    {
        let position = position.into();
        let rotation = rotation.into();

        Self {
            position,
            rotation,
            phantom: PhantomData,
        }
    }

    pub fn into_position_rotation<P, Q>(self) -> (P, Q)
    where
        P: From<Vec3>,
        Q: From<Quat>,
    {
        (From::from(self.position), From::from(self.rotation))
    }

    /// +X
    pub fn right<V>(&self) -> V
    where
        V: From<Vec3>,
    {
        let rotation: Quat = self.rotation;
        From::from(rotation * Vec3::X)
    }

    /// +Y
    pub fn up<V>(&self) -> V
    where
        V: From<Vec3>,
    {
        let rotation: Quat = self.rotation;
        From::from(rotation * Vec3::Y)
    }

    /// +/-Z
    pub fn forward<V>(&self) -> V
    where
        V: From<Vec3>,
    {
        let rotation: Quat = self.rotation;
        From::from(rotation * H::FORWARD)
    }

    pub const IDENTITY: Transform<H> = Transform {
        position: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        rotation: Quat::from_xyzw(0.0, 0.0, 0.0, 1.0),
        phantom: PhantomData,
    };
}

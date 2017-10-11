//! Global transform component.

use std::borrow::Borrow;
use std::ops::{Deref, DerefMut};

use ecs::{Component, DenseVecStorage, FlaggedStorage};

use mint::{ColumnMatrix4, Vector4};

/// Performs a global transformation on the entity (transform from origin).
///
/// Used for rendering position and orientation.
///
/// If this component is used, and `TransformSystem` is not used, then make sure to clear the flags
/// on the `FlaggedStorage` at the appropriate times (before updating any `Transform` in the frame).
/// See documentation on `FlaggedStorage` for more information.
#[derive(Debug, Clone)]
pub struct Transform(pub ColumnMatrix4<f32>);

impl Component for Transform {
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}

impl Default for Transform {
    fn default() -> Self {
        Transform(
            ColumnMatrix4 {
                x: Vector4 { x: 1.0, y: 0.0, z: 0.0, w: 0.0 },
                y: Vector4 { x: 0.0, y: 1.0, z: 0.0, w: 0.0 },
                z: Vector4 { x: 0.0, y: 0.0, z: 1.0, w: 0.0 },
                w: Vector4 { x: 0.0, y: 0.0, z: 0.0, w: 1.0 },
            }
        )
    }
}

impl Deref for Transform {
    type Target = ColumnMatrix4<f32>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Transform {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<[[f32; 4]; 4]> for Transform {
    fn from(matrix: [[f32; 4]; 4]) -> Self {
        Transform(matrix.into())
    }
}

impl Into<[[f32; 4]; 4]> for Transform {
    fn into(self) -> [[f32; 4]; 4] {
        self.0.into()
    }
}

impl AsRef<[[f32; 4]; 4]> for Transform {
    fn as_ref(&self) -> &[[f32; 4]; 4] {
        &self.0.into()
    }
}


impl Borrow<[[f32; 4]; 4]> for Transform {
    fn borrow(&self) -> &[[f32; 4]; 4] {
        &self.0.into()
    }
}

impl From<::mint::ColumnMatrix4<f32>> for Transform {
    fn from(matrix: ::mint::ColumnMatrix4<f32>) -> Self {
        Transform(matrix)
    }
}

impl Into<::mint::ColumnMatrix4<f32>> for Transform {
    fn into(self) -> ::mint::ColumnMatrix4<f32> {
        self.0
    }
}

impl AsRef<::mint::ColumnMatrix4<f32>> for Transform {
    fn as_ref(&self) -> &::mint::ColumnMatrix4<f32> {
        &self.0
    }
}


impl Borrow<::mint::ColumnMatrix4<f32>> for Transform {
    fn borrow(&self) -> &::mint::ColumnMatrix4<f32> {
        &self.0
    }
}

//! Local transform component.

use ecs::{Component, DenseVecStorage, FlaggedStorage};

/// Local position, rotation, and scale (from parent if it exists).
///
/// Used for rendering position and orientation.
#[derive(Clone, Debug)]
pub struct LocalTransform {
    /// Translation/position vector [x, y, z]
    pub translation: ::mint::Vector3<f32>,
    /// Quaternion [w (scalar), x, y, z]
    pub rotation: ::mint::Quaternion<f32>,
    /// Scale vector [x, y, z]
    pub scale: ::mint::Vector3<f32>,
}

impl LocalTransform {
    /// Returns the local object matrix for the transform.
    ///
    /// Combined with the parent's global `Transform` component it gives
    /// the global (or world) matrix for the current entity.
    #[inline]
    pub fn matrix(&self) -> ::mint::ColumnMatrix4<f32> {
        let quat: ::cgmath::Matrix3<f32> = ::cgmath::Quaternion::from(self.rotation.into()).into();
        let scale: ::cgmath::Matrix3<f32> = ::cgmath::Matrix3::<f32> {
            x: [self.scale.x, 0.0, 0.0].into(),
            y: [0.0, self.scale.y, 0.0].into(),
            z: [0.0, 0.0, self.scale.z].into(),
        };
        let mut matrix: ::cgmath::Matrix4<f32> = (&quat * scale).into();
        matrix.w = ::cgmath::Vector3::from(self.translation.into()).extend(1.0f32);
        matrix
    }
}

impl Default for LocalTransform {
    fn default() -> Self {
        LocalTransform {
            translation: ::mint::Vector3 { x: 0.0, y: 0.0, z: 0.0, },
            rotation: ::mint::Quaternion {
                s: 1.0,
                v: ::mint::Vector3 { x: 0.0, y: 0.0, z: 0.0, },
            },
            scale: ::mint::Vector3 { x: 1.0, y: 1.0, z: 1.0, },
        }
    }
}

impl Component for LocalTransform {
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}


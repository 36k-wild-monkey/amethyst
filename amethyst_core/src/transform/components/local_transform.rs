//! Local transform component.

use cgmath::{
    Deg,
    EuclideanSpace,
    InnerSpace,
    Matrix3,
    Matrix4,
    Point3,
    Quaternion,
    Rotation,
    Rotation3,
    Vector3
};
use orientation::Orientation;
use specs::{Component, DenseVecStorage, FlaggedStorage};

/// Local position, rotation, and scale (from parent if it exists).
///
/// Used for rendering position and orientation.
#[derive(Clone, Debug, PartialEq)]
pub struct LocalTransform {
    /// Quaternion [w (scalar), x, y, z]
    pub rotation: Quaternion<f32>,
    /// Scale vector [x, y, z]
    pub scale: Vector3<f32>,
    /// Translation/position vector [x, y, z]
    pub translation: Point3<f32>,
}

impl LocalTransform {
    /// Rotate to look at a point in space (without rolling)
    pub fn look_at(&mut self, orientation: &Orientation, position: Point3<f32>) -> &mut Self {

        self.rotation = Quaternion::look_at(position - self.translation, orientation.up.into()).into();
        self
    }

    /// Returns the local object matrix for the transform.
    ///
    /// Combined with the parent's global `Transform` component it gives
    /// the global (or world) matrix for the current entity.
    #[inline]
    pub fn matrix(&self) -> Matrix4<f32> {
        let quat: Matrix3<f32> = Quaternion::from(self.rotation).into();
        let scale: Matrix3<f32> = Matrix3::<f32> {
            x: [self.scale[0], 0.0, 0.0].into(),
            y: [0.0, self.scale[1], 0.0].into(),
            z: [0.0, 0.0, self.scale[2]].into(),
        };
        let mut matrix: Matrix4<f32> = (&quat * scale).into();
        matrix.w = Point3::to_vec(self.translation).extend(1.0f32);
        matrix
    }

    /// Move relatively to its current position and orientation.
    pub fn move_forward(&mut self, orientation: &Orientation, amount: f32) -> &mut Self {
        self.move_local(orientation.forward.into(), amount)
    }

    /// Move relatively to its current position, but independently from its orientation.
    /// Ideally, first normalize the direction and then multiply it
    /// by whatever amount you want to move before passing the vector to this method
    #[inline]
    pub fn move_global(&mut self, direction: Vector3<f32>) -> &mut Self {
        self.translation = Point3::from_vec(Point3::to_vec(self.translation) + direction);
        self
    }

    /// Move relatively to its current position and orientation.
    #[inline]
    pub fn move_local(&mut self, axis: Vector3<f32>, amount: f32) -> &mut Self {
        let delta = Quaternion::from(self.rotation).conjugate() * axis.normalize() * amount;

        self.translation = self.translation + delta;
        self
    }

    /// Move relatively to its current position and orientation.
    pub fn move_right(&mut self, orientation: &Orientation, amount: f32) -> &mut Self {
        self.move_local(orientation.right.into(), amount)
    }

    /// Move relatively to its current position and orientation.
    pub fn move_up(&mut self, orientation: &Orientation, amount: f32) -> &mut Self {
        self.move_local(orientation.up.into(), amount)
    }

    /// Pitch relatively to the world.
    pub fn pitch_global(&mut self, orientation: &Orientation, angle: Deg<f32>) -> &mut Self {
        self.rotate_global(orientation.right.into(), angle)
    }

    /// Pitch relatively to its own rotation.
    pub fn pitch_local(&mut self, orientation: &Orientation, angle: Deg<f32>) -> &mut Self {
        self.rotate_local(orientation.right.into(), angle)
    }

    /// Roll relatively to the world.
    pub fn roll_global(&mut self, orientation: &Orientation, angle: Deg<f32>) -> &mut Self {
        self.rotate_global(orientation.forward.into(), angle)
    }

    /// Roll relatively to its own rotation.
    pub fn roll_local(&mut self, orientation: &Orientation, angle: Deg<f32>) -> &mut Self {
        self.rotate_local(orientation.forward.into(), angle)
    }

    /// Add a rotation to the current rotation
    #[inline]
    pub fn rotate(&mut self, quat: Quaternion<f32>) -> &mut Self {
        self.rotation = (quat * Quaternion::from(self.rotation)).into();
        self
    }

    /// Rotate relatively to the world
    #[inline]
    pub fn rotate_global(&mut self, axis: Vector3<f32>, angle: Deg<f32>) -> &mut Self {
        let axis_normalized = Vector3::from(axis).normalize();
        let q = Quaternion::from_axis_angle(axis_normalized, angle);

        self.rotate(q)
    }

    /// Rotate relatively to the current orientation
    #[inline]
    pub fn rotate_local(&mut self, axis: Vector3<f32>, angle: Deg<f32>) -> &mut Self {
        let rel_axis_normalized = Quaternion::from(self.rotation)
            .rotate_vector(Vector3::from(axis))
            .normalize();
        let q = Quaternion::from_axis_angle(rel_axis_normalized, angle);

        self.rotate(q)
    }

    /// Set the position.
    pub fn set_position(&mut self, position: Point3<f32>) -> &mut Self {
        self.translation = position;
        self
    }

    /// Set the rotation using Euler x, y, z.
    pub fn set_rotation<D: Into<Deg<f32>>>(&mut self, x: D, y: D, z: D) -> &mut Self {
        let rotation =
            Quaternion::from_angle_x(x.into()) *
            Quaternion::from_angle_y(y.into()) *
            Quaternion::from_angle_z(z.into());

        self.rotation = rotation.into();
        self
    }

    /// Calculate the view matrix from the given data.
    pub fn to_view_matrix(&self, orientation: &Orientation) -> Matrix4<f32> {
        let center = self.translation + orientation.forward;
        Matrix4::look_at(self.translation, center, orientation.up)
    }

    /// Yaw relatively to the world.
    pub fn yaw_global(&mut self, orientation: &Orientation, angle: Deg<f32>) -> &mut Self {
        self.rotate_global(orientation.up.into(), angle)
    }

    /// Yaw relatively to its own rotation.
    pub fn yaw_local(&mut self, orientation: &Orientation, angle: Deg<f32>) -> &mut Self {
        self.rotate_local(orientation.up.into(), angle)
    }
}

impl Default for LocalTransform {
    fn default() -> Self {
        LocalTransform {
            translation: [0.0, 0.0, 0.0].into(),
            rotation: [1.0, 0.0, 0.0, 0.0].into(),
            scale: [1.0, 1.0, 1.0].into(),
        }
    }
}

impl LocalTransform {
    /// Create a new `LocalTransform`.
    ///
    /// If you call `matrix` on this, then you would get an identity matrix.
    pub fn new() -> Self {
        Default::default()
    }
}

impl Component for LocalTransform {
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}

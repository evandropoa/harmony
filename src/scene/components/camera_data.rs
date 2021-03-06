use nalgebra_glm::{Mat4, Vec3};
use crate::core::Frustum;

#[derive(Clone)]
enum ProjectionData {
    Perspective {
        fov: f32,
        z_near: f32,
        z_far: f32,
    },
    Orthographic {
        world_height: f32,
        z_near: f32,
        z_far: f32,
    },
}

impl ProjectionData {
    /// get_projection calculates a new projection for the specified viewport width & height. TODO: Div by 0 possible for orthographic.
    fn get_projection(&self, width: f32, height: f32) -> Mat4 {
        match self {
            ProjectionData::Perspective { fov, z_near, z_far } => {
                nalgebra_glm::perspective_fov_lh_no(
                    fov.to_radians(),
                    width,
                    height,
                    *z_near,
                    *z_far,
                )
            }
            ProjectionData::Orthographic {
                world_height,
                z_near,
                z_far,
            } => nalgebra_glm::ortho_lh_no(
                -0.5 * world_height * width / height,
                0.5 * world_height * width / height,
                -0.5 * world_height,
                0.5 * world_height,
                *z_near,
                *z_far,
            ),
        }
    }

    fn get_projection_range(&self, width: f32, height: f32, z_near: f32, z_far: f32) -> Mat4 {
        match self {
            ProjectionData::Perspective { fov, .. } => {
                nalgebra_glm::perspective_fov_lh_no(
                    fov.to_radians(),
                    width,
                    height,
                    z_near,
                    z_far,
                )
            }
            ProjectionData::Orthographic {
                world_height,
                ..
            } => nalgebra_glm::ortho_lh_no(
                -0.5 * world_height * width / height,
                0.5 * world_height * width / height,
                -0.5 * world_height,
                0.5 * world_height,
                z_near,
                z_far,
            ),
        }
    }
}

/// CameraData holds all necessary data to calculate the cameras matrices
/// and offers basic constructors.
#[derive(Clone)]
pub struct CameraData {
    pub active: bool,
    pub position: Vec3,
    pub projection: Mat4,
    pub view: Mat4,
    pub yaw: f32,
    pub pitch: f32,
    pub width: f32,
    pub height: f32,
    /// If true this will cull objects from the scene.
    pub cull: bool,
    projection_data: ProjectionData,
    pub frustum: Frustum,
}

impl Default for CameraData {
    fn default() -> Self {
        Self {
            active: false,
            cull: false,
            frustum: Frustum::new(),
            height: 0.0,
            pitch: 0.0,
            position: Vec3::zeros(),
            projection: Mat4::identity(),
            projection_data: ProjectionData::Perspective {
                fov: 70.0,
                z_near: 0.1,
                z_far: 100.0,
            },
            view: Mat4::identity(),
            width: 0.0,
            yaw: 0.0,
        }
    }
}

impl CameraData {
    /// new_perspective constructs a new Perspective Camera
    ///
    /// # Arguments
    ///
    /// * 'fov'             - the field of view in degrees
    /// * 'width'           - the width of the viewport
    /// * 'height'          - the height of the viewport
    /// * 'z_near'          - the distance to the near clipping plane
    /// * 'z_far'           - the distance to the far clipping plane
    pub fn new_perspective(fov: f32, width: f32, height: f32, z_near: f32, z_far: f32) -> Self {
        let projection_data = ProjectionData::Perspective { fov, z_near, z_far };
        Self {
            active: true,
            cull: false,
            frustum: Frustum::new(),
            height,
            pitch: 0.0,
            position: Vec3::zeros(),
            projection: projection_data.get_projection(width, height),
            projection_data,
            view: Mat4::identity(),
            width,
            yaw: 0.0,
        }
    }

    /// new_orthographic constructs a new Orthographic Camera
    /// uses the aspect ratio of the viewport
    ///
    /// # Arguments
    ///
    /// * 'world_height'    - the height of the "camera-box" in world units
    /// * 'width'           - the width of the viewport
    /// * 'height'          - the height of the viewport
    /// * 'z_near'          - the distance to the near clipping plane
    /// * 'z_far'           - the distance to the far clipping plane
    pub fn new_orthographic(
        world_height: f32,
        width: f32,
        height: f32,
        z_near: f32,
        z_far: f32,
    ) -> Self {
        let projection_data = ProjectionData::Orthographic {
            world_height,
            z_near,
            z_far,
        };
        Self {
            active: true,
            cull: false,
            frustum: Frustum::new(),
            height,
            pitch: 0.0,
            position: Vec3::zeros(),
            projection: projection_data.get_projection(width, height),
            projection_data,
            view: Mat4::identity(),
            width,
            yaw: 0.0,
        }
    }

    /// resize recalculates the projection matrix. Needs to be called on window resize
    pub fn resize(&mut self, width: f32, height: f32) {
        self.projection = self.projection_data.get_projection(width, height);
        self.frustum = Frustum::from_matrix(self.projection * self.view);
    }

    pub fn resize_range(&mut self, width: f32, height: f32, near: f32, far: f32) {
        self.projection = self.projection_data.get_projection_range(width, height, near, far);
        self.frustum = Frustum::from_matrix(self.projection * self.view);
    }

    /// updates the view matrix. Needs to be called when the camera moved
    pub fn update_view(&mut self, eye: Vec3, at: Vec3, up: Vec3) {
        self.view = nalgebra_glm::look_at_lh(&eye, &at, &up);
        self.frustum = Frustum::from_matrix(self.projection * self.view);
    }

    pub fn get_inverse_proj(&self) -> Mat4 {
        self.projection.try_inverse().unwrap()
    }

    /// returns the view-projection matrix
    pub fn get_matrix(&self) -> Mat4 {
        self.projection * self.view
    }

    pub(crate) fn set_reflect_cubic_camera(&mut self, position: Vec3, face_id: u32) {
        let mut eye = Vec3::zeros();
        let mut up = Vec3::new(0.0, 1.0, 0.0);
        match face_id {
            0 => {
                eye = Vec3::new(1.0, 0.0, 0.0);
            } // X+
            1 => {
                eye = Vec3::new(-1.0, 0.0, 0.0);
            } // X-
            2 => {
                eye = Vec3::new(0.0, 1.0, 0.0);
                up = Vec3::new(0.0, 0.0, -1.0);
            } // Y+
            3 => {
                eye = Vec3::new(0.0, -1.0, 0.0);
                up = Vec3::new(0.0, 0.0, 1.0);
            } // Y-
            4 => {
                eye = Vec3::new(0.0, 0.0, 1.0);
            } // Z+
            5 => {
                eye = Vec3::new(0.0, 0.0, -1.0);
            } // Z-
            _ => (),
        }
        self.update_view(eye + position, position, up);
    }

    pub(crate) fn set_cubic_camera(&mut self, position: Vec3, face_id: u32) {
        let mut eye = Vec3::zeros();
        let mut up = Vec3::new(0.0, 1.0, 0.0);
        match face_id {
            0 => {
                eye = Vec3::new(1.0, 0.0, 0.0);
            } // X+
            1 => {
                eye = Vec3::new(-1.0, 0.0, 0.0);
            } // X-
            2 => {
                eye = Vec3::new(0.0, 1.0, 0.0);
                up = Vec3::new(0.0, 0.0, -1.0);
            } // Y+
            3 => {
                eye = Vec3::new(0.0, -1.0, 0.0);
                up = Vec3::new(0.0, 0.0, 1.0);
            } // Y-
            4 => {
                eye = Vec3::new(0.0, 0.0, 1.0);
            } // Z+
            5 => {
                eye = Vec3::new(0.0, 0.0, -1.0);
            } // Z-
            _ => (),
        }
        self.update_view(position, position + eye, up);
    }
}

#[cfg(test)]
mod tests {
    use super::CameraData;
    ///just tests for projection matrix calculation
    #[test]
    fn test_perspective_projection() {
        let fov = 70.0;
        let (width, height) = (800f32, 600f32);
        let (z_near, z_far) = (0.01f32, 10f32);
        let camera_data = CameraData::new_perspective(fov, width, height, z_near, z_far);
        assert_eq!(
            camera_data.projection,
            nalgebra_glm::perspective_fov_lh_no(fov.to_radians(), width, height, z_near, z_far)
        );
    }
    ///just tests for projection matrix calculation
    #[test]
    fn test_orthographic_projection() {
        let (width, height) = (800f32, 600f32);
        let world_height = 5f32;
        let world_width = width * world_height / height;
        let (z_near, z_far) = (0.01f32, 10f32);
        let camera_data = CameraData::new_orthographic(world_height, width, height, z_near, z_far);
        assert_eq!(
            camera_data.projection,
            nalgebra_glm::ortho_lh_no(
                -world_width / 2f32,
                world_width / 2f32,
                -world_height / 2f32,
                world_height / 2f32,
                z_near,
                z_far
            )
        );
    }
}

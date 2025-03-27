use nalgebra_glm as glm;

pub struct Camera {
    pub window_size: (u32, u32),
    pub position: glm::Vec3,
    pub horizontal_angle: f32,
    pub vertical_angle: f32,
    pub fov: f32,
    pub up: glm::Vec3,
    pub direction: glm::Vec3,
}

impl Camera {
    //const CAMERA_SPEED: f32 = 3.0;
    //const MOUSE_SPEED: f32 = 0.00005;

    pub fn new(window_size: (u32, u32)) -> Self {
        Self {
            window_size,
            position: glm::vec3(0.0, 0.0, 0.0),
            horizontal_angle: std::f32::consts::PI,
            vertical_angle: 0.0,
            fov: 90.0,
            up: glm::vec3(0.0, 1.0, 0.0),
            direction: glm::vec3(0.0, 0.0, 0.0),
        }
    }

    /*
    pub fn update_with_mouse_pos(&mut self, mouse_pos: (i32, i32), delta: f32) {
        let (mouse_pos_x, mouse_pos_y) = (mouse_pos.0 as f32, mouse_pos.1 as f32);

        self.horizontal_angle +=
            Self::MOUSE_SPEED * delta * (self.window_size.0 as f32 / 2.0 - mouse_pos_x);
        self.vertical_angle +=
            Self::MOUSE_SPEED * delta * (self.window_size.1 as f32 / 2.0 - mouse_pos_y);

        self.direction = glm::vec3(
            self.vertical_angle.cos() * self.horizontal_angle.sin(),
            self.vertical_angle.sin(),
            self.vertical_angle.sin() * self.horizontal_angle.cos(),
        );

        let right = glm::vec3(
            (self.horizontal_angle - 3.14 / 2.0).sin(),
            0.0,
            (self.horizontal_angle - 3.14 / 2.0).cos(),
        );

        self.up = glm::cross(&right, &self.direction);
    }
    */

    const NEAR: f32 = 0.1;
    const FAR: f32 = 100.0;

    pub fn calculate_projection(&self) -> glm::Mat4 {
        glm::perspective(
            self.window_size.0 as f32 / self.window_size.1 as f32,
            self.fov.to_radians(),
            Self::NEAR,
            Self::FAR,
        )
    }

    pub fn calculate_view(&self) -> glm::Mat4 {
        glm::look_at(&-self.position, &(self.position + self.direction), &self.up)
    }
}

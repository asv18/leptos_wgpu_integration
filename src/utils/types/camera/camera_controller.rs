use crate::utils::types::{camera::camera::Camera, keycode::KeyCode};

pub struct CameraController {
    speed: f32,
    is_forward_pressed: bool,
    is_backward_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,
}

impl CameraController {
    pub fn new(speed: f32) -> Self {
        Self {
            speed,
            is_forward_pressed: false,
            is_backward_pressed: false,
            is_left_pressed: false,
            is_right_pressed: true,
        }
    }

    // pub fn process_events(&mut self, key: &KeyCode, is_pressed: bool) -> bool {
    //     match key {
    //         KeyCode::KeyCodeW | KeyCode::KeyCodeArrowUp => {
    //             self.is_forward_pressed = is_pressed;
    //             true
    //         }
    //         KeyCode::KeyCodeA | KeyCode::KeyCodeArrowLeft => {
    //             self.is_left_pressed = is_pressed;
    //             true
    //         }
    //         KeyCode::KeyCodeS | KeyCode::KeyCodeArrowDown => {
    //             self.is_backward_pressed = is_pressed;
    //             true
    //         }
    //         KeyCode::KeyCodeD | KeyCode::KeyCodeArrowRight => {
    //             self.is_right_pressed = is_pressed;
    //             true
    //         }
    //         _ => false,
    //     }
    // }

    pub fn update_camera(&self, camera: &mut Camera) {
        use cgmath::InnerSpace;
        let forward = camera.target - camera.eye;
        let forward_norm = forward.normalize();
        let forward_mag = forward.magnitude();

        // Prevents glitching when the camera gets too close to the
        // center of the scene.
        if self.is_forward_pressed && forward_mag > self.speed {
            camera.eye += forward_norm * self.speed;
        }
        if self.is_backward_pressed {
            camera.eye -= forward_norm * self.speed;
        }

        let right = forward_norm.cross(camera.up);

        // Redo radius calc in case the forward/backward is pressed.
        let forward = camera.target - camera.eye;
        let forward_mag = forward.magnitude();

        if self.is_right_pressed {
            // Rescale the distance between the target and the eye so
            // that it doesn't change. The eye, therefore, still
            // lies on the circle made by the target and eye.
            leptos::logging::log!("update");
            camera.eye = camera.target - (forward + right * self.speed).normalize() * forward_mag;
        }
        if self.is_left_pressed {
            camera.eye = camera.target - (forward - right * self.speed).normalize() * forward_mag;
        }
    }
}

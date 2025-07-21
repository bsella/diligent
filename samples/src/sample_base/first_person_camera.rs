use core::f32;

use diligent::graphics_types::SurfaceTransform;
use diligent_tools::native_app::events::{Event, Key, MouseButton};
use glam::Vec4Swizzles;

pub struct FirstPersonCamera {
    last_mouse_pos: [i16; 2],
    left_mouse_pressed: bool,

    reference_right_axis: glam::Vec4,
    reference_up_axis: glam::Vec4,
    reference_ahead_axis: glam::Vec4,

    view_matrix: glam::Mat4,
    world_matrix: glam::Mat4,
    proj_matrix: glam::Mat4,

    rotation_speed: f32,
    move_direction: glam::Vec3,
    current_speed: f32,

    yaw_angle: f32,   // Yaw angle of camera
    pitch_angle: f32, // Pitch angle of camera
    speed_up_scale: f32,
    super_speed_up_scale: f32,
    handness: f32, // -1 - left handed
                   // +1 - right handed
}

impl FirstPersonCamera {
    pub fn new(
        reference_right_axis: &glam::Vec3,
        reference_up_axis: &glam::Vec3,
        is_right_handed: bool,
        near_clip_plane: f32,
        far_clip_plane: f32,
        aspect_ratio: f32,
        fov_y: f32,
        srf_pre_transform: SurfaceTransform,
    ) -> Self {
        let reference_right_axis = reference_right_axis.normalize();
        let mut reference_up_axis =
            reference_up_axis - reference_up_axis.dot(reference_right_axis) * reference_right_axis;
        let mut up_len = reference_up_axis.length();
        if up_len < f32::EPSILON {
            up_len = f32::EPSILON;
            //LOG_WARNING_MESSAGE("Right and Up axes are collinear");
        }

        reference_up_axis /= up_len;

        let handness = if is_right_handed { 1.0 } else { -1.0 };

        let reference_ahead_axis = handness * reference_right_axis.cross(reference_up_axis);

        let mut camera = Self {
            last_mouse_pos: [0, 0],
            left_mouse_pressed: false,
            reference_right_axis: glam::Vec4::new(
                reference_right_axis.x,
                reference_right_axis.y,
                reference_right_axis.z,
                0.0,
            ),
            reference_up_axis: glam::Vec4::new(
                reference_up_axis.x,
                reference_up_axis.y,
                reference_up_axis.z,
                0.0,
            ),
            reference_ahead_axis: glam::Vec4::new(
                reference_ahead_axis.x,
                reference_ahead_axis.y,
                reference_ahead_axis.z,
                0.0,
            ),
            view_matrix: glam::Mat4::IDENTITY,
            world_matrix: glam::Mat4::IDENTITY,
            proj_matrix: glam::Mat4::IDENTITY,
            rotation_speed: 0.01,
            move_direction: glam::Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            current_speed: 1.0,
            yaw_angle: 0.0,
            pitch_angle: 0.0,
            speed_up_scale: 5.0,
            super_speed_up_scale: 10.0,
            handness: if is_right_handed { 1.0 } else { -1.0 },
        };

        camera.set_projection_attribs(
            near_clip_plane,
            far_clip_plane,
            aspect_ratio,
            fov_y,
            srf_pre_transform,
        );

        camera
    }

    pub fn apply_event(&mut self, event: &Event) {
        match event {
            Event::KeyPress(key) => match key {
                Key::W => self.move_direction.z += 1.0,
                Key::A => self.move_direction.x -= 1.0,
                Key::S => self.move_direction.z -= 1.0,
                Key::D => self.move_direction.x += 1.0,
                Key::E => self.move_direction.y += 1.0,
                Key::Q => self.move_direction.y -= 1.0,
                Key::LeftShift | Key::RightShift => self.current_speed *= self.speed_up_scale,
                Key::LeftCtrl | Key::RightCtrl => self.current_speed *= self.super_speed_up_scale,
                _ => {}
            },
            Event::MouseDown { button } => {
                if let MouseButton::Left = button {
                    self.left_mouse_pressed = true
                }
            }
            Event::MouseUp { button } => {
                if let MouseButton::Left = button {
                    self.left_mouse_pressed = false
                }
            }

            Event::MouseMove { x, y } => {
                if self.left_mouse_pressed {
                    let mouse_delta = (*x - self.last_mouse_pos[0], *y - self.last_mouse_pos[1]);
                    let yaw_delta = mouse_delta.0 as f32 * self.rotation_speed;
                    let pitch_delta = mouse_delta.1 as f32 * self.rotation_speed;

                    self.yaw_angle += yaw_delta * -self.handness;
                    self.pitch_angle += pitch_delta * -self.handness;

                    self.pitch_angle = f32::clamp(
                        self.pitch_angle,
                        -f32::consts::PI / 2.0,
                        f32::consts::PI / 2.0,
                    );
                }
                self.last_mouse_pos = [*x, *y];
            }

            Event::KeyRelease(key) => match key {
                Key::W => self.move_direction.z -= 1.0,
                Key::A => self.move_direction.x += 1.0,
                Key::S => self.move_direction.z += 1.0,
                Key::D => self.move_direction.x -= 1.0,
                Key::E => self.move_direction.y -= 1.0,
                Key::Q => self.move_direction.y += 1.0,
                Key::LeftShift | Key::RightShift => self.current_speed /= self.speed_up_scale,
                Key::LeftCtrl | Key::RightCtrl => self.current_speed /= self.super_speed_up_scale,
                _ => {}
            },
            _ => {}
        }
    }

    pub fn update(&mut self, elapsed_time: f64) {
        let move_vector = self.move_direction * self.current_speed;
        let move_delta = move_vector * elapsed_time as f32;

        let reference_rotation = glam::Mat4::from_cols(
            self.reference_right_axis,
            self.reference_up_axis,
            self.reference_ahead_axis,
            glam::Vec4::new(0.0, 0.0, 0.0, 1.0),
        );

        let camera_rotation = reference_rotation
            * glam::Mat4::from_axis_angle(
                glam::vec3(
                    self.reference_right_axis.x,
                    self.reference_right_axis.y,
                    self.reference_right_axis.z,
                ),
                self.pitch_angle,
            )
            * glam::Mat4::from_axis_angle(
                glam::vec3(
                    self.reference_up_axis.x,
                    self.reference_up_axis.y,
                    self.reference_up_axis.z,
                ),
                self.yaw_angle,
            );

        let world_rotation = camera_rotation.transpose();

        let pos_delta_world =
            world_rotation * glam::vec4(move_delta.x, move_delta.y, move_delta.z, 1.0);

        let mut position = self.world_matrix.w_axis.xyz();

        position += glam::vec3(pos_delta_world.x, pos_delta_world.y, pos_delta_world.z);

        self.view_matrix = camera_rotation * glam::Mat4::from_translation(-position);
        self.world_matrix = glam::Mat4::from_translation(position) * world_rotation;
    }

    pub fn world_matrix(&self) -> &glam::Mat4 {
        &self.world_matrix
    }
    pub fn view_matrix(&self) -> &glam::Mat4 {
        &self.view_matrix
    }
    pub fn projection_matrix(&self) -> &glam::Mat4 {
        &self.proj_matrix
    }

    pub fn set_pos(&mut self, pos: &glam::Vec3) {
        *self.world_matrix.col_mut(3) = glam::vec4(pos.x, pos.y, pos.z, 1.0);
    }
    pub fn set_rotation(&mut self, yaw: f32, pitch: f32) {
        self.yaw_angle = yaw;
        self.pitch_angle = pitch;
    }
    pub fn set_rotation_speed(&mut self, speed: f32) {
        self.rotation_speed = speed
    }

    //pub fn set_move_speed(&mut self, speed: f32) {}

    pub fn set_speed_up_scales(&mut self, speed: f32, super_speed: f32) {
        self.speed_up_scale = speed;
        self.super_speed_up_scale = super_speed;
    }

    pub fn set_projection_attribs(
        &mut self,
        near_clip_plane: f32,
        far_clip_plane: f32,
        aspect_ratio: f32,
        fov_y: f32,
        srf_pre_transform: SurfaceTransform,
    ) {
        let fov = match srf_pre_transform {
            SurfaceTransform::Rotate90
            | SurfaceTransform::Rotate270
            | SurfaceTransform::HorizontalMirrorRotate90
            | SurfaceTransform::HorizontalMirrorRotate270 => {
                // When the screen is rotated, vertical FOV becomes horizontal FOV
                fov_y * aspect_ratio
            }

            _ => fov_y,
        };

        self.proj_matrix =
            glam::Mat4::perspective_lh(fov, aspect_ratio, near_clip_plane, far_clip_plane);
    }
}

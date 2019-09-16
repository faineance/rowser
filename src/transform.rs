use crate::view_state::ViewState;
use cgmath::{Matrix4, Rad, Transform, Vector3};

pub fn generate_transform(width: f32, height: f32, view_state: &ViewState) -> Matrix4<f32> {
    let offset = Matrix4::from_translation(Vector3::new(-width / 2.0, -height / 2.0, 0.0));
    let rotation =
        offset.inverse_transform().unwrap() * Matrix4::from_angle_z(Rad(view_state.angle)) * offset;

    // Default projection
    let projection: Matrix4<f32> = gfx_glyph::default_transform((width, height)).into();

    // Here an example transform is used as a cheap zoom out (controlled with ctrl-scroll)
    let zoom = Matrix4::from_scale(view_state.zoom);

    // Combined transform
    zoom * projection * rotation
}

use crate::{
    archetypes::{Orientation, Position},
    ecs::{ECS, EntityID},
    plugins::wgpu_plugin::{
        self, Camera, Mesh, MeshAtlas, RenderingPipelineAndBind, RenderingPipelinesAndBinds,
        state::State,
    },
};
use cgmath::{
    self, Angle, Basis3, Deg, Euler, InnerSpace, Matrix3, Matrix4, Point3, Rad, Vector2, Vector3,
    Vector4, frustum, perspective,
};
use wgpu::Buffer;
// cehck for what entities have and shi
pub fn render_system(ecs: &mut ECS) {
    let size = ecs.get_recource_ref::<State>().unwrap().window.inner_size();
    if size.width == 0 || size.height == 0 {
        return;
    }
    let camera = get_camera_info(ecs);
    let camera = match camera {
        Some(camera) => camera,
        None => {
            draw_no_camera(ecs);
            return;
        }
    };
    // todo fix matricies;
    let (view_matrix, perspective_matrix) = calc_camera_maricies(&camera);
    let mut state = ecs.pop_recource::<State>().unwrap();

    for entity_id in ecs.iter_over_alive_entity_ids() {
        if ecs.has_component_immut::<Mesh>(entity_id) {
            let entity_render_info = get_entity_render_info(&ecs, entity_id).unwrap();
            let translation = Matrix4::from_translation(Vector3 {
                x: entity_render_info.posx,
                y: entity_render_info.posy,
                z: entity_render_info.posz,
            });
            let rotx = Matrix4::from_angle_x(Deg(entity_render_info.angx));
            let roty = Matrix4::from_angle_y(Deg(360.0 - (entity_render_info.angy % 360.0)));
            let rotz = Matrix4::from_angle_z(Deg(entity_render_info.angz));
            let rotation = roty * rotx * rotz;
            // ! maybe add something later
            let scale = Matrix4::from_scale(0.5f32);

            let model_matrix = translation * rotation * scale;
            let mvp = model_matrix * view_matrix * perspective_matrix;

            let ses = ecs
                .get_recource_ref::<RenderingPipelinesAndBinds>()
                .unwrap();
            let entry = &ses.rendering_infos["3d"];
            let render_pipeline = &entry.render_pipeline;
            let bind_group_layout = &entry.bind_group_layout;

            state.render(
                Some(entity_render_info.mesh),
                Some(entity_render_info.texture_atlas),
                Some(entity_render_info.texture_info),
                model_matrix.into(),
                view_matrix.into(),
                perspective_matrix.into(),
                render_pipeline,
                bind_group_layout,
            );
        }
    }
    ecs.insert_recource(state);
}
fn get_entity_render_info<'a>(ecs: &'a ECS, entity_id: EntityID) -> Option<EntityRernderInfo<'a>> {
    if !ecs.has_component_immut::<Mesh>(entity_id) {
        return None;
    }
    let mesh = ecs.get_component_ref::<Mesh>(entity_id).unwrap();
    let position = if ecs.has_component_immut::<Position>(entity_id) {
        let pos = ecs.get_component_ref::<Position>(entity_id).unwrap();
        (pos.x, pos.y, pos.z)
    } else {
        eprint!("entity_id with a `Mesh` has no `Position`, wich suspicius");
        (0.0, 0.0, 0.0)
    };
    let oreintation = if ecs.has_component_immut::<Orientation>(entity_id) {
        let oreintation = ecs.get_component_ref::<Orientation>(entity_id).unwrap();
        (oreintation.x, oreintation.y, oreintation.z)
    } else {
        (0.0, 0.0, 0.0)
    };
    let texture = if ecs.has_component_immut::<wgpu_plugin::Texture>(entity_id) {
        let texture_id = ecs
            .get_component_ref::<wgpu_plugin::Texture>(entity_id)
            .unwrap()
            .texture_id;
        let texture_atlas = ecs.get_recource_ref::<wgpu_plugin::TextureAtlas>().unwrap();
        texture_atlas.get_texture(texture_id)
    } else {
        eprint!(
            "entity_id with a `Mesh` has no `Texture`, wich suspicius. Resorting to default texture"
        );
        let texture_atlas = ecs.get_recource_ref::<wgpu_plugin::TextureAtlas>().unwrap();
        texture_atlas.get_texture(0)
    };
    let mesh = ecs
        .get_recource_ref::<MeshAtlas>()
        .unwrap()
        .get_mesh(mesh.mesh_id);
    Some(EntityRernderInfo {
        mesh,
        texture_atlas: texture.0,
        texture_info: texture.1,
        posx: position.0,
        posy: position.1,
        posz: position.2,
        angx: oreintation.0,
        angy: oreintation.1,
        angz: oreintation.2,
    })
}
fn calc_camera_maricies(camera: &CamInfo) -> (Matrix4<f32>, Matrix4<f32>) {
    let rot = Matrix3::from(Euler {
        x: Deg(-camera.angx),
        y: Deg(camera.angy),
        z: Deg(camera.angz),
    });
    let rotx = Matrix3::from_angle_x(Deg(-camera.angx));
    let roty = Matrix3::from_angle_y(Deg(camera.angy));
    let rotz = Matrix3::from_angle_z(Deg(camera.angz));

    let eye = Point3 {
        x: camera.posx,
        y: camera.posy,
        z: camera.posz,
    };
    let center = eye
        + (roty
            * rotx
            * rotz
            * Vector3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            }); // point in fornt of the camera (rotates)

    let up = Vector3 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };
    let view_matrix = cgmath::Matrix4::look_at_rh(eye, center, up);
    let perspective = perspective(
        Rad(camera.fovy.to_radians()),
        camera.aspect,
        0.1,
        camera.range,
    );
    let opengl_to_wgpu = Matrix4::new(
        1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.5, 0.5, 0.0, 0.0, 0.0, 1.0,
    );

    (view_matrix, opengl_to_wgpu * perspective)
}
fn get_camera_info(ecs: &mut ECS) -> Option<CamInfo> {
    let mut caminfo = None;
    let dimensions = ecs.get_recource_ref::<State>().unwrap().size.clone();
    let aspect_ratio = if dimensions.width == 0 || dimensions.height == 0 {
        1.0
    } else {
        dimensions.width as f32 / dimensions.height as f32
    };

    for entity_id in ecs.iter_over_alive_entity_ids() {
        if ecs.has_component_immut::<Camera>(entity_id) {
            let fovy;
            let pos;
            let orientation;
            let (fovx, range) = (
                ecs.get_component_ref::<Camera>(entity_id).unwrap().fov,
                ecs.get_component_ref::<Camera>(entity_id).unwrap().range,
            );
            pos = if ecs.has_component_immut::<Position>(entity_id) {
                let pos = ecs.get_component_ref::<Position>(entity_id).unwrap();
                (pos.x, pos.y, pos.z)
            } else {
                (0.0, 0.0, 0.0)
            };
            orientation = if ecs.has_component_immut::<Orientation>(entity_id) {
                let ang = ecs.get_component_ref::<Orientation>(entity_id).unwrap();
                (ang.x, ang.y, ang.z)
            } else {
                (0.0, 0.0, 0.0)
            };
            fovy = ((fovx.to_radians() / 2.0).tan() / aspect_ratio)
                .atan()
                .to_degrees()
                * 2.0;
            caminfo = Some(CamInfo {
                posx: pos.0,
                posy: pos.1,
                posz: pos.2,
                angx: orientation.0,
                angy: orientation.1,
                angz: orientation.2,
                fovx,
                fovy,
                aspect: aspect_ratio,
                range,
            })
        }
    }
    return caminfo;
}
fn draw_no_camera(ecs: &mut ECS) {}
#[derive(Debug, Clone, Copy)]
struct CamInfo {
    posx: f32,
    posy: f32,
    posz: f32,
    angx: f32,
    angy: f32,
    angz: f32,
    fovx: f32,
    fovy: f32,
    aspect: f32,
    range: f32,
}
#[derive(Debug, Clone, Copy)]
struct EntityRernderInfo<'a> {
    mesh: (&'a Buffer, u32),
    texture_atlas: &'a wgpu::TextureView,
    texture_info: &'a wgpu_plugin::TextureInfo,
    posx: f32,
    posy: f32,
    posz: f32,
    angx: f32,
    angy: f32,
    angz: f32,
}

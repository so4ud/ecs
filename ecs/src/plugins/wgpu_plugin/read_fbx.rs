use asset_importer::{Importer, postprocess::PostProcessSteps};
use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;

use crate::plugins::wgpu_plugin::vertex::Vertex;

pub fn read_fbx(device: &wgpu::Device, fbx_path: String) -> (wgpu::Buffer, u32) {
    let scene = Importer::new()
        .import_file_with(fbx_path, |b| {
            b.with_post_process(PostProcessSteps::TRIANGULATE | PostProcessSteps::FLIP_UVS)
        })
        .unwrap();

    let mesh = scene.meshes().next().ok_or("No meshes in FBX").unwrap();

    // 3. Конвертируем вершины
    let mut vertices = Vec::new();
    let positions = mesh.vertices();
    let normals = mesh.normals().unwrap();
    let uv_coords = mesh.texture_coords(0).unwrap();

    for i in 0..positions.len() {
        let position = [positions[i].x, positions[i].y, positions[i].z];
        dbg!(&position);
        let normal = if i < normals.len() {
            [normals[i].x, normals[i].y, normals[i].z]
        } else {
            [0.0, 1.0, 0.0] // fallback
        };

        let uv = if i < uv_coords.len() {
            [uv_coords[i].x, uv_coords[i].y]
        } else {
            [0.0, 0.0]
        };

        vertices.push(Vertex {
            position,
            normal,
            uv,
        });
    }

    // 4. Создаем GPU буфер
    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("FBX Vertex Buffer"),
        contents: bytemuck::cast_slice(&vertices),
        usage: wgpu::BufferUsages::VERTEX,
    });

    (vertex_buffer, vertices.len() as u32)

    // 4. Создание wgpu::Buffer
    // let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
    //     label: Some("FBX Vertex Buffer"),
    //     contents: bytemuck::cast_slice(&vertices),
    //     usage: wgpu::BufferUsages::VERTEX,
    // });

    // (vertex_buffer, vertices.len() as u32)
}

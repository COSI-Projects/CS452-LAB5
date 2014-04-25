#![crate_id = "lab4"]
#![crate_type = "bin"]
#![feature(phase)]

#[phase(syntax, link)]
extern crate log;

extern crate native;
extern crate glfw;
extern crate gl;
extern crate hgl;
extern crate cgmath;
extern crate collections;
extern crate png;

use glfw::Context;
use hgl::{Vbo, Ebo, Vao, Shader, Program};
use cgmath::matrix::{Matrix4, ToMatrix4, Matrix};
use cgmath::transform::{Transform3D, AffineMatrix3, Transform};
use cgmath::array::Array;
use cgmath::point::Point3;
use cgmath::vector::Vector3;
use cgmath::rotation::Rotation3;

mod loader;

#[start]
fn start(argc: int, argv: **u8) -> int {
    native::start(argc, argv, main)
}

#[inline(always)]
fn set_mat(uniform: gl::types::GLint, mat: &Matrix4<gl::types::GLfloat>) {
    unsafe {
        gl::UniformMatrix4fv(uniform, 1, gl::FALSE, mat.as_slice().as_ptr() as *f32);
    }
}


fn main() {
    let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.window_hint(glfw::ContextVersion(3, 3));
    glfw.window_hint(glfw::OpenglProfile(glfw::OpenGlCoreProfile));
    glfw.window_hint(glfw::Samples(32));

    let (window, events) = glfw.create_window(640, 480, "Lab 4", glfw::Windowed).expect("Failed to create window.");

    window.make_current();

    glfw.set_swap_interval(1);
    window.set_size_polling(true);
    window.set_key_polling(true);

    gl::load_with(|s| glfw.get_proc_address(s));

    gl::Viewport(0, 0, 640, 480);
    gl::Enable(gl::DEPTH_TEST);
    gl::Enable(gl::CULL_FACE);

    let (verts, norms, uvs, inds) = loader::load_obj("tmonkey.obj");

    let (vshdr, fshdr) = (Shader::from_file("vertex.glsl", hgl::VertexShader).unwrap().unwrap(), Shader::from_file("fragment.glsl", hgl::FragmentShader).unwrap().unwrap());

    let prog = Program::link([vshdr, fshdr]).unwrap();

    prog.bind();

    let vao = Vao::new();

    vao.bind();

    let vbuf = Vbo::from_data(verts.as_slice(), hgl::StaticDraw);
    let nbuf = Vbo::from_data(norms.as_slice(), hgl::StaticDraw);
    let ubuf = Vbo::from_data(uvs.as_slice(), hgl::StaticDraw);
    let ibuf = Ebo::from_indices(inds.as_slice());

    vbuf.bind();

    vao.enable_attrib(&prog, "pos", gl::FLOAT, 3, 0, 0);

    nbuf.bind();

    vao.enable_attrib(&prog, "norm", gl::FLOAT, 3, 0, 0);

    ubuf.bind();

    vao.enable_attrib(&prog, "uv", gl::FLOAT, 2, 0, 0);

    ibuf.bind();

    println!("There were {} verts, {} indices", verts.len(), inds.len());

    let img = png::load_png(&Path::new("tmonkey.png")).unwrap();

    if img.color_type != png::RGBA8 { fail!("Bad PNG type"); }

    let ii = hgl::texture::ImageInfo::new().pixel_format(hgl::texture::pixel::RGBA).pixel_type(hgl::texture::pixel::UNSIGNED_BYTE)
                                           .width(img.width as gl::types::GLint).height(img.height as gl::types::GLint);

    let tex = hgl::Texture::new(hgl::texture::Texture2D, ii, img.pixels.as_slice().as_ptr());
    tex.gen_mipmaps();
    tex.filter(hgl::texture::NearestMipmapNearest);
    tex.wrap(hgl::texture::Repeat);
    tex.activate(0);

    let (mut rx, mut ry) = (0f32, 0f32);

    let set_model = |rot: Vector3<gl::types::GLfloat>, trans: Vector3<gl::types::GLfloat>| {
        let mut m = Transform3D::translate(trans.x, trans.y, trans.z).to_matrix4();
        if rot.x != 0.0 {
            m = m.mul_m(&cgmath::matrix::Matrix3::from_angle_x(cgmath::angle::Rad{s: rot.x}).to_matrix4());
        }
        if rot.y != 0.0 {
            m = m.mul_m(&cgmath::matrix::Matrix3::from_angle_y(cgmath::angle::Rad{s: rot.y}).to_matrix4());
        }
        if rot.z != 0.0 {
            m = m.mul_m(&cgmath::matrix::Matrix3::from_angle_z(cgmath::angle::Rad{s: rot.z}).to_matrix4());
        }
        set_mat(prog.uniform("mat_m"), &m);
    };

    let a: AffineMatrix3<f32> = Transform::look_at(&Point3::<f32>::new(0.0, 0.0, 0.0), &Point3::<f32>::new(0.0, 0.0, -1.0), &Vector3::<f32>::new(0.0, 1.0, 0.0));
    set_mat(prog.uniform("mat_v"),  &a.to_matrix4());
    set_mat(prog.uniform("mat_p"), &cgmath::projection::perspective(cgmath::angle::Deg{s: 60f32}, (640.0/480.0), 0.1, 100.0));
    set_model(Vector3::new(rx, ry, 0.0), Vector3::new(0.0f32, 0.0f32, -5.0f32));

    gl::ClearColor(0.41, 0.0, 0.54, 1.0);

    while !window.should_close() {
        glfw.poll_events();

        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::SizeEvent(w, h) => {
                    set_mat(prog.uniform("mat_p"), &cgmath::projection::perspective(cgmath::angle::Deg{s: 60f32}, (w as f32/h as f32), 0.1, 100.0));
                    gl::Viewport(0, 0, w, h);
                }
                glfw::KeyEvent(key, _, action, _) => {
                    if action == glfw::Press || action == glfw::Repeat {
                        match key {
                            glfw::KeyW => rx += 0.5,
                            glfw::KeyS => rx -= 0.5,
                            glfw::KeyA => ry += 0.5,
                            glfw::KeyD => ry -= 0.5,
                            _ => { }
                        }
                        set_model(Vector3::new(rx, ry, 0.0), Vector3::new(0.0f32, 0.0f32, -5.0f32));
                        println!("x: {} y: {}", rx, ry);
                    }
                }
                _ => {}
            }
        }

        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        vao.draw_elements(hgl::Triangles, 0, inds.len() as gl::types::GLint);

        window.swap_buffers();

    }
}

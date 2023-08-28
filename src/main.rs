use glad_gl::gl;
use glfw::{Action, Context, Key, WindowHint};
use shader::Shader;

mod buffer;
mod shader;

fn main() {
    let width: i32 = 800;
    let height: i32 = 600;
    let title = env!("CARGO_PKG_NAME");

    // initialize glfw library
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).expect("failed to initialize glfw library");

    glfw.window_hint(WindowHint::ContextVersion(3, 3));
    glfw.window_hint(WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    // Create a windowed mode window and its OpenGL context
    let (mut window, events) = glfw
        .create_window(
            width as u32,
            height as u32,
            title,
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window.");

    // Make the window's context current
    window.make_current();
    window.set_key_polling(true);

    // load opengl functions
    gl::load(|s| window.get_proc_address(s));

    // shader compile
    let vertex_shader = Shader::gen(shader::ShaderKind::VertexShader);
    vertex_shader.source_str(VERTEX_SHADER_SOURCE);
    vertex_shader.compile().unwrap();

    let fragment_shader = Shader::gen(shader::ShaderKind::FragmentShader);
    fragment_shader.source_str(FRAGMENT_SHADER_SOURCE);
    fragment_shader.compile().unwrap();

    unsafe {
        gl::ClearColor(1.0f32, 1.0f32, 0.4f32, 0.7f32);
    }

    // Loop until the user closes the window
    while !window.should_close() {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        // Swap front and back buffers
        window.swap_buffers();

        // Poll for and process events
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            println!("{:?}", event);
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true)
                }
                _ => {}
            }
        }
    }
}

const VERTEX_SHADER_SOURCE: &'static str = "#version 330 core
layout (location = 0) in vec3 aPos;

void main()
{
    gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
}\0";

const FRAGMENT_SHADER_SOURCE: &'static str = "#version 330 core
out vec4 FragColor;

void main()
{
    FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
}\0";

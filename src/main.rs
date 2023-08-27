use glad_gl::gl;
use glfw::{Action, Context, Key, WindowHint};

mod shader;
mod buffer;

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

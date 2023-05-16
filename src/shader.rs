use std::ptr::null;

use glad_gl::gl;

pub trait Shader {
    fn id(&self) -> u32;

    fn compile<T: AsRef<str>>(&self, src: T) -> Result<(), String> {
        let src: *const u8 = src.as_ref().as_ptr();
        let mut success: i32 = 0;
        let mut log_len: i32 = 0;
        let mut alog_len: i32 = 0;
        let mut log_info: Vec<u8>;

        unsafe {
            gl::ShaderSource(self.id(), 1, &src.cast(), null());
            gl::CompileShader(self.id());
            gl::GetShaderiv(self.id(), gl::COMPILE_STATUS, &mut success);

            if success != gl::TRUE.into() {
                gl::GetShaderiv(self.id(), gl::INFO_LOG_LENGTH, &mut log_len);
                log_info = Vec::with_capacity(log_len as usize);
                log_info.set_len(log_len as usize);
                gl::GetShaderInfoLog(
                    self.id(),
                    log_len,
                    &mut alog_len,
                    log_info.as_mut_ptr().cast(),
                );

                return Err(String::from_utf8_unchecked(log_info));
            }
        }

        Ok(())
    }
}

pub struct VertexShader {
    id: u32,
}

impl VertexShader {
    pub fn new() -> Self {
        let id;
        unsafe {
            id = gl::CreateShader(gl::VERTEX_SHADER);
        }
        Self { id }
    }
}

impl Shader for VertexShader {
    fn id(&self) -> u32 {
        self.id
    }
}

impl Drop for VertexShader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

pub struct FragmentShader {
    id: u32,
}

impl FragmentShader {
    pub fn new() -> Self {
        let id;
        unsafe {
            id = gl::CreateShader(gl::FRAGMENT_SHADER);
        }
        Self { id }
    }
}

impl Shader for FragmentShader {
    fn id(&self) -> u32 {
        self.id
    }
}

impl Drop for FragmentShader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

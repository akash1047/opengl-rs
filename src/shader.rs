use std::ptr::null;

use glad_gl::gl;

#[repr(u32)]
pub enum ShaderKind {
    VertexShader = gl::VERTEX_SHADER,
    FragmentShader = gl::FRAGMENT_SHADER,
}

pub struct Shader {
    id: u32,
}

impl Shader {
    pub fn gen(kind: ShaderKind) -> Self {
        let id: u32 = unsafe { gl::CreateShader(kind as u32) };
        if id == 0 {
            panic!("Failed to create shader.");
        }
        Self { id }
    }

    pub fn source_str(&self, source: &'_ str) {
        unsafe { gl::ShaderSource(self.id, 1, &source.as_ptr().cast(), null()) }
    }

    pub fn compile(&self) -> Result<(), String> {
        unsafe {
            gl::CompileShader(self.id);
            let mut success: i32 = 0;
            gl::GetShaderiv(self.id, gl::COMPILE_STATUS, &mut success);

            if success == gl::FALSE.into() {
                let mut log_size: i32 = 0;
                gl::GetShaderiv(self.id, gl::INFO_LOG_LENGTH, &mut log_size);
                let mut info_log: Vec<u8> = Vec::with_capacity(log_size as usize);
                info_log.set_len(log_size as usize);
                gl::GetShaderInfoLog(
                    self.id,
                    info_log.capacity() as i32,
                    &mut log_size,
                    info_log.as_mut_ptr().cast(),
                );
                return Err(String::from_utf8_unchecked(info_log));
            }
        }

        Ok(())
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { gl::DeleteShader(self.id) }
    }
}

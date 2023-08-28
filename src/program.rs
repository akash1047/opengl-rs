use std::ptr::null_mut;

use glad_gl::gl;

use crate::shader::Shader;

pub struct Program {
    id: u32,
}

impl Program {
    pub fn new() -> Self {
        let id: u32 = unsafe { gl::CreateProgram() };
        if id == 0 {
            panic!("Failed to create opengl Program.");
        }
        Self { id }
    }

    pub fn attach(&self, shader: &Shader) {
        unsafe { gl::AttachShader(self.id, shader.id) }
    }

    pub fn link(&self) -> Result<(), String> {
        unsafe {
            gl::LinkProgram(self.id);

            let mut success: i32 = 0;
            gl::GetProgramiv(self.id, gl::LINK_STATUS, &mut success);

            if success == gl::FALSE.into() {
                let mut log_size: i32 = 0;
                gl::GetProgramiv(self.id, gl::INFO_LOG_LENGTH, &mut log_size);
                let mut log_info: Vec<u8> = Vec::with_capacity(log_size as usize);
                gl::GetProgramInfoLog(
                    self.id,
                    log_info.capacity() as i32,
                    null_mut(),
                    log_info.as_mut_ptr().cast(),
                );
                log_info.set_len(log_size as usize);
                return Err(String::from_utf8_unchecked(log_info));
            }
        }

        Ok(())
    }

    pub fn use_program(&self) {
        unsafe { gl::UseProgram(self.id) }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.id) }
    }
}

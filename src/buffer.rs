use glad_gl::gl;

pub struct Buffer {
    pub ids: Vec<u32>,
}

impl Buffer {
    pub fn gen(size: usize) -> Self {
        let mut ids = vec![0; size];
        unsafe {
            gl::GenBuffers(size as i32, ids.as_mut_ptr());
        }
        Self { ids }
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe { gl::DeleteBuffers(self.ids.len() as i32, self.ids.as_mut_ptr()) }
    }
}

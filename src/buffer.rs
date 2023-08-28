use glad_gl::gl;

pub struct ArrayBuffer {
    id: u32,
}

impl ArrayBuffer {
    pub fn gen() -> Self {
        let mut id: u32 = 0;
        unsafe { gl::GenBuffers(1, &mut id) }
        Self { id }
    }

    pub fn bind(&self) {
        unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, self.id) }
    } 

    pub fn buffer_data<T: Sized>(&self, data: &[T]) {
        unsafe { gl::BufferData(gl::ARRAY_BUFFER, data.len() as isize, data.as_ptr().cast(), gl::STATIC_DRAW) }
    }
}

impl Drop for ArrayBuffer {
    fn drop(&mut self) {
        unsafe { gl::DeleteBuffers(1, &self.id) }
    }
}

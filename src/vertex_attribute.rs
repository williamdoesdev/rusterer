use std::mem;

pub struct VertexAttribute {
    pub dtype: u32,
    pub bytes: i32,
    pub size: i32,
    pub normalized: bool,
    pub offset: i32,
}

pub trait PushLayoutAttribute {
    fn push_attribute<T: BufferDataType>(&mut self, size: i32) -> &mut Self;
}

impl PushLayoutAttribute for Vec<VertexAttribute> {
    fn push_attribute<T: BufferDataType>(&mut self, size: i32) -> &mut Self {
        let new_attribute = VertexAttribute {
            dtype: T::GL_TYPE,
            bytes: T::BYTES,
            size: size,
            normalized: false,
            offset: self.stride(),
        };

        self.push(new_attribute);

        return self
    }
}

pub trait GetStride {
    fn stride(&self) -> i32;
}

impl GetStride for Vec<VertexAttribute> {
    fn stride(&self) -> i32 {
        return self.iter().map(|attr| attr.size * attr.bytes).sum()
    }
}

pub trait BufferDataType {
    const GL_TYPE: u32;
    const BYTES: i32;
}

impl BufferDataType for f32 {
    const GL_TYPE: u32 = glow::FLOAT;
    const BYTES: i32 = std::mem::size_of::<f32>() as i32;
}

// Other types...
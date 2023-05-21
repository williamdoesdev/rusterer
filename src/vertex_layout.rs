pub struct VertexLayout {
    pub attributes: Vec<LayoutAttribute>,
    pub stride: i32
}

impl VertexLayout {
    pub fn new(attributes: Option<Vec<LayoutAttribute>>) -> Self {
        match attributes {
            Some(attrs) => {
                let stride = attrs.iter().map(|attr| attr.components * std::mem::size_of::<u32>() as i32).sum();
                VertexLayout {
                    attributes: attrs,
                    stride,
                }
            }
            None => {
                VertexLayout {
                    attributes: Vec::new(),
                    stride: 0,
                }
            }
        }
    }
    

    pub fn push<T: BufferDataType>(&mut self, components: i32) {
        let attr = LayoutAttribute {
            dtype: T::GL_TYPE,
            components: components,
            normalized: false,
            ptr: self.stride
        };
        self.attributes.push(attr);

        self.stride += components * std::mem::size_of::<T>() as i32;
    }
}

pub struct LayoutAttribute {
    pub dtype: u32,
    pub components: i32,
    pub normalized: bool,
    pub ptr: i32
}



trait BufferDataType {
    const GL_TYPE: u32;
}

impl BufferDataType for f32 {
    const GL_TYPE: u32 = glow::FLOAT;
}

// More types...
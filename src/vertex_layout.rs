pub struct VertexLayout {
    attributes: Vec<LayoutAttribute>,
    stride: u32
}

impl VertexLayout {
    impl VertexLayout {
        pub fn new(attributes: Option<Vec<LayoutAttribute>>) -> Self {
            match attributes {
                Some(attrs) => {
                    let stride = attrs.iter().map(|attr| attr.count * std::mem::size_of::<u32>() as u32).sum();
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
    }
    

    pub fn push<T: BufferDataType>(&mut self) {
        let attr = LayoutAttribute {
            dtype: T::GL_TYPE,
            count: T::COMPONENTS,
            normalized: false,
            ptr: self.stride
        };
        self.attributes.push(attr);

        self.stride += T::COMPONENTS * std::mem::size_of::<T>() as u32;
    }
}

struct LayoutAttribute {
    dtype: u32,
    count: u32,
    normalized: bool,
    ptr: u32
}



trait BufferDataType {
    const GL_TYPE: u32;
    const COMPONENTS: u32;
}

impl BufferDataType for f32 {
    const GL_TYPE: u32 = glow::FLOAT;
    const COMPONENTS: u32 = 1;
}

impl BufferDataType for [f32; 2] {
    const GL_TYPE: u32 = glow::FLOAT;
    const COMPONENTS: u32 = 2;
}

impl BufferDataType for [f32; 3] {
    const GL_TYPE: u32 = glow::FLOAT;
    const COMPONENTS: u32 = 3;
}

impl BufferDataType for [f32; 4] {
    const GL_TYPE: u32 = glow::FLOAT;
    const COMPONENTS: u32 = 4;
}
// More types...
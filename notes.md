# Vertex Array Object Layout
```
┌──────── Vertex 1 ────────┐┌──────── Vertex 2 ────────┐
▊▊ Attrib 1 ▊▊▊▊ Attrib 2 ▊▊▊▊ Attrib 1 ▊▊▊▊ Attrib 2 ▊▊
└─ Index 1 ──┘└─ Index 2 ──┘└─ Index 1 ──┘└─ Index 2 ──┘
└───────── Stride ─────────┘└───────── Stride ─────────┘
```
<b>Index</b>: The index of an attribute within a vertex

<b>Size</b>: The number of components for an attribute. i.e. a 2D coordinate would have a size of 2

<b>Normalized</b>: Whether or not values should be normalized to between 0.0 and 1.0

<b>Stride</b>: The distance in bytes between two vertices, including all attributes

<b>Pointer</b>: The offset of each attribute in bytes within a specific vertex
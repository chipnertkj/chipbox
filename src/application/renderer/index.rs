#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Index(u16);

impl Index {
    pub const fn describe() -> wgpu::IndexFormat {
        wgpu::IndexFormat::Uint16
    }
}

use surfman::{Device, Context, Connection, ContextAttributeFlags, ContextAttributes, GLVersion};
use std::cell::RefCell;
use skia_safe::gpu::DirectContext;

thread_local!(static GL_CONTEXT: RefCell<Option<OpenGL>> = RefCell::new(None));

pub struct OpenGL {
    device:Device,
    context:Context
}

#[cfg(target_os = "macos")]
impl OpenGL {
    fn init() -> bool {
        GL_CONTEXT.with(|cell| {
            let mut local_ctx = cell.borrow_mut();
            if local_ctx.is_none(){
                if let Some(ctx) = OpenGL::new() {
                    local_ctx.replace(ctx);
                    true
                } else {
                    false
                }
            } else {
                true
            }
        })
    }

    pub fn supported() -> bool {
        Self::init()
    }

    pub fn new() -> Option<Self> {
        let connection = Connection::new().ok()?;
        let adapter = connection.create_hardware_adapter().ok()?;
        let mut device = connection.create_device(&adapter).ok()?;
        let context_attributes = ContextAttributes {
            version: GLVersion::new(3, 3),
            flags: ContextAttributeFlags::empty(),
        };
        let context_descriptor = device
            .create_context_descriptor(&context_attributes)
            .ok()?;
        let context = device.create_context(&context_descriptor, None).ok()?;
        device.make_context_current(&context).ok()?;
        gl::load_with(|symbol_name| device.get_proc_address(&context, symbol_name));

        Some(OpenGL{device, context})
    }

    pub fn direct_context() -> Option<DirectContext> {
        Self::init();
        DirectContext::new_gl(None, None)
    }

}

impl Drop for OpenGL {
    fn drop(&mut self) {
        self.device.destroy_context(&mut self.context).unwrap();
    }
}

#[cfg(not(target_os = "macos"))]
impl OpenGL {
    pub fn new() -> Option<self> { None }
    pub fn supported() -> bool { false }
    pub fn direct_context() -> Option<DirectContext> { None }
}

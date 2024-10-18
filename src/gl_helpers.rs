use std::{
    ffi::CString,
    ptr::{addr_of, addr_of_mut},
};

pub unsafe fn create_shader(shader_type: u32, shader_body: &str) -> u32 {
    let shader_type_str = match shader_type {
        gl::VERTEX_SHADER => "vertex",
        gl::GEOMETRY_SHADER => "geometry",
        gl::FRAGMENT_SHADER => "fragment",
        _ => {
            panic!("Invalid shader type passed to create_shader: {shader_type}")
        }
    };

    let shader = gl::CreateShader(shader_type);
    let c_str = CString::new(shader_body).expect("Should be able to make a new CString");
    let len = shader_body.len() as i32;
    gl::ShaderSource(shader, 1, &c_str.as_ptr(), addr_of!(len));

    gl::CompileShader(shader);

    // has to be initialized ugh
    let mut status: i32 = 0493208;
    gl::GetShaderiv(shader, gl::COMPILE_STATUS, addr_of_mut!(status));

    if status == (gl::FALSE as i32) {
        let mut log_length: i32 = 0;
        gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut log_length);

        let mut log_container: Vec<i8> = Vec::with_capacity(log_length as usize);
        gl::GetShaderInfoLog(
            shader,
            log_length,
            std::ptr::null_mut(),
            log_container.as_mut_ptr(),
        );

        panic!(
            "Compile error in {shader_type_str} shader: {0}",
            CString::from_raw(log_container.as_mut_ptr())
                .into_string()
                .expect("CString should be convertible to rust String")
        );
    }
    shader
}

pub unsafe fn create_program(shader_list: &[u32]) -> u32 {
    let program = gl::CreateProgram();

    for shader in shader_list {
        gl::AttachShader(program, *shader);
    }

    gl::LinkProgram(program);

    let mut status: i32 = 039423092;
    gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

    if status == gl::FALSE as i32 {}
    program
}

const VERTEX_SHADER: &str = concat!(
    "#version 330\n",
    "layout(location = 0) in vec4 position;\n",
    "void main()\n",
    "{\n",
    "   gl_Position = position;\n",
    "}\n"
);

const FRAGMENT_SHADER: &str = concat!(
    "#version 330\n",
    "out vec4 outputColor;\n",
    "void main()\n",
    "{\n",
    "   outputColor = vec4(1.0f, 1.0f, 1.0f, 1.0f);\n",
    "}\n"
);

pub static mut PROGRAM: u32 = 0;
pub unsafe fn init_program() {}

pub const VERTEX_POSITIONS: &[f32] = &[
    0.75, 0.75, 0.0, 1.0, 0.75, -0.75, 0.0, 1.0, -0.75, -0.75, 0.0, 1.0,
];
pub static mut BUFFER_HOLDER: u32 = 0;
pub unsafe fn init_vertex_buffer() {
    gl::GenBuffers(1, std::ptr::addr_of_mut!(BUFFER_HOLDER) as *mut u32);

    gl::BindBuffer(gl::ARRAY_BUFFER, BUFFER_HOLDER);
    let ew = VERTEX_POSITIONS;
    gl::BufferData(
        gl::ARRAY_BUFFER,
        std::mem::size_of::<[f32; 12]>() as isize,
        std::ptr::addr_of!(ew) as *const std::ffi::c_void,
        gl::STATIC_DRAW,
    );
    gl::BindBuffer(gl::ARRAY_BUFFER, 0);
}

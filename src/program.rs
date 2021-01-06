/**
 * A wrapper that stores a programs render pipeline and references to required data
 */
pub struct Program {
    frag_shader: &Str,
    vertex_shader: &Str,
}

/**
 * Handles the creation and management of GPU programs and their resources
 */
// impl Program {
//     pub fn new(vertex_shader: &str, frag_shader: &str) -> {
//         Self { frag_shader, vertex_shader }
//     }
// }

/**
 * Generic interface
 */
pub trait Bufferable: Sized {
    fn as_bytes(&self) -> &[u8];

    fn set_program_defaults(&mut self, _selected: usize) {}
}

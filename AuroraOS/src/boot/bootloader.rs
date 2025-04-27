use kernel::Kernel;
// Implementación del bootloader de AuroraOS
pub struct Bootloader;
impl Bootloader {
    // Carga del kernel en memoria
    pub fn load_kernel(&mut self) -> Kernel {
        Kernel::new()
    }
    // Salto al kernel
    pub fn jump_to_kernel(&mut self, kernel: Kernel) {
        kernel.init();
        kernel.run();
    }
}
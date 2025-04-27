use autograd::Autograd;
use matrix_ops::MatrixOps;
use rest_server::RestServer;
// Implementación del kernel de AuroraOS
pub struct Kernel {
    autograd: Autograd,
    matrix_ops: MatrixOps,
    rest_server: RestServer,
}
impl Kernel {
    pub fn new() -> Self {
        Kernel {
            autograd: Autograd::new(Vec::new()),
            matrix_ops: MatrixOps,
            rest_server: RestServer,
        }
    }
    // Inicialización del kernel
    pub fn init(&mut self) {
        self.autograd.init();
        self.matrix_ops.init();
        self.rest_server.init();
    }
    // Ejecución del kernel
    pub fn run(&mut self) {
        self.rest_server.run();
    }
}
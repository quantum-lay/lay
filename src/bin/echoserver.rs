use laysur::Laysur;
use laysur::gates::{XGate, YGate, ZGate, CXGate};

struct PrintQasmDriver {
}

impl Laysur for PrintQasmDriver {
    fn initialize(&mut self) {
        println!("OPENQASM 2.0;");
    }
}

impl XGate for PrintQasmDriver {
    fn x(&mut self, q: u32) {
        println!("x [{}];", q);
    }
}

impl YGate for PrintQasmDriver {
    fn y(&mut self, q: u32) {
        println!("y [{}];", q);
    }
}

impl ZGate for PrintQasmDriver {
    fn z(&mut self, q: u32) {
        println!("z [{}];", q);
    }
}

impl CXGate for PrintQasmDriver {
    fn cx(&mut self, c: u32, t: u32) {
        println!("cx [{}, {}];", c, t);
    }
}

struct TransparentLayer<T> {
    base: T,
}

impl<T> TransparentLayer<T> {
    fn new(base: T) -> Self {
        Self { base }
    }
}

impl<T: Laysur> Laysur for TransparentLayer<T> {
    fn initialize(&mut self) {
        self.base.initialize();
    }
}

impl<T: XGate> XGate for TransparentLayer<T> {
    fn x(&mut self, q: u32) {
        self.base.x(q)
    }
}

impl<T: YGate> YGate for TransparentLayer<T> {
    fn y(&mut self, q: u32) {
        self.base.y(q)
    }
}

impl<T: ZGate> ZGate for TransparentLayer<T> {
    fn z(&mut self, q: u32) {
        self.base.z(q)
    }
}

fn main() {
    let mut cli = TransparentLayer::new(PrintQasmDriver{});
    cli.initialize();
    cli.x(0);
    cli.y(0);
}

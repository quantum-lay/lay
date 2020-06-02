use laysur::Laysur;
use laysur::gates::{XGate, YGate, ZGate, CXGate};

struct EchoDriver {
}

impl Laysur for EchoDriver {
    fn initialize(&mut self) {
        println!("initialize()");
    }
}

impl XGate for EchoDriver {
    fn x(&mut self, q: u32) {
        println!("x({})", q);
    }
}

impl YGate for EchoDriver {
    fn y(&mut self, q: u32) {
        println!("y({})", q);
    }
}

impl ZGate for EchoDriver {
    fn z(&mut self, q: u32) {
        println!("z({})", q);
    }
}

impl CXGate for EchoDriver {
    fn cx(&mut self, c: u32, t: u32) {
        println!("cx({}, {})", c, t);
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
    let mut cli = TransparentLayer::new(TransparentLayer::new(EchoDriver{}));
    cli.initialize();
    cli.x(0);
    cli.y(0);
}

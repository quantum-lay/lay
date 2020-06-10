use lay::Layer;
use lay::gates::{CliffordGate};

struct EchoDriver {
}

impl Layer for EchoDriver {
    type Receive = ();
    fn initialize(&mut self) {
        println!("initialize()");
    }
    fn receive(&mut self) -> () {
    }
}

impl CliffordGate for EchoDriver {
    fn x(&mut self, q: u32) {
        println!("x({})", q);
    }
    fn y(&mut self, q: u32) {
        println!("y({})", q);
    }
    fn z(&mut self, q: u32) {
        println!("z({})", q);
    }
    fn h(&mut self, q: u32) {
        println!("h({})", q);
    }
    fn s(&mut self, q: u32) {
        println!("s({})", q);
    }
    fn sdg(&mut self, q: u32) {
        println!("sdg({})", q);
    }
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

impl<T: Layer> Layer for TransparentLayer<T> {
    type Receive = ();
    fn initialize(&mut self) {
        self.base.initialize();
    }
    fn receive(&mut self) -> () {
    }
}

impl<T: CliffordGate> CliffordGate for TransparentLayer<T> {
    fn x(&mut self, q: u32) {
        self.base.x(q)
    }
    fn y(&mut self, q: u32) {
        self.base.y(q)
    }
    fn z(&mut self, q: u32) {
        self.base.z(q)
    }
    fn h(&mut self, q: u32) {
        self.base.h(q)
    }
    fn s(&mut self, q: u32) {
        self.base.s(q)
    }
    fn sdg(&mut self, q: u32) {
        self.base.sdg(q)
    }
    fn cx(&mut self, c: u32, t: u32) {
        self.base.cx(c, t)
    }
}

fn main() {
    let mut cli = TransparentLayer::new(TransparentLayer::new(EchoDriver{}));
    cli.initialize();
    cli.x(0);
    cli.y(0);
}

use lay::Layer;
use lay::gates::{CliffordGate};

struct EchoDriver {
}

impl Layer for EchoDriver {
    type Receive = ();
    type Qubit = u32;
    type Slot = u32;
    fn initialize(&mut self) {
        println!("initialize()");
    }
    fn receive(&mut self) -> () {
        ()
    }
    fn measure(&mut self, q: u32, slot: u32) {
        println!("m({}) -> slot({})", q, slot);
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
    type Receive = T::Receive;
    type Qubit = T::Qubit;
    type Slot = T::Slot;
    fn initialize(&mut self) {
        self.base.initialize();
    }
    fn send(&mut self) {
        self.base.send();
    }
    fn receive(&mut self) -> Self::Receive {
        self.base.receive()
    }
    fn measure(&mut self, q: Self::Qubit, slot: Self::Slot) {
        self.base.measure(q, slot)
    }
}

impl<T: CliffordGate> CliffordGate for TransparentLayer<T> {
    fn x(&mut self, q: Self::Qubit) {
        self.base.x(q)
    }
    fn y(&mut self, q: Self::Qubit) {
        self.base.y(q)
    }
    fn z(&mut self, q: Self::Qubit) {
        self.base.z(q)
    }
    fn h(&mut self, q: Self::Qubit) {
        self.base.h(q)
    }
    fn s(&mut self, q: Self::Qubit) {
        self.base.s(q)
    }
    fn sdg(&mut self, q: Self::Qubit) {
        self.base.sdg(q)
    }
    fn cx(&mut self, c: Self::Qubit, t: Self::Qubit) {
        self.base.cx(c, t)
    }
}

fn main() {
    let mut cli = TransparentLayer::new(TransparentLayer::new(EchoDriver{}));
    cli.initialize();
    cli.x(0);
    cli.y(0);
    cli.measure(0, 0);
    cli.send();
    cli.receive();
}

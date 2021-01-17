use std::marker::PhantomData;
use crate::{Layer, Measured, operations::Operation, gates::{PauliGate, HGate, SGate, TGate, CXGate}};

pub trait Converter<Q1, Q2, S1, S2> {
    fn qconv(q: Q1) -> Q2;
    fn sconv(s: S1) -> S2;
}

pub struct QubitSlotConvertLayer<L: Layer, Q, S, C>
    where C: Converter<Q, L::Qubit, S, L::Slot>
{
    layer: L,
    phantom: PhantomData<(C, Q, S)>,
}

impl<L: Layer, Q, S, C> Layer for QubitSlotConvertLayer<L, Q, S, C>
    where C: Converter<Q, L::Qubit, S, L::Slot>
{
    type Operation = ConvOp<L, Q, S, C>;
    type Qubit = Q;
    type Slot = S;
    type Buffer = ConvBuf<Self, L, C>;
    type Requested = L::Requested;
    type Response = L::Response;

    fn make_buffer(&self) -> Self::Buffer {
        ConvBuf(self.layer.make_buffer(), PhantomData)
    }

    fn send(&mut self, ops: &[Self::Operation]) -> Self::Requested {
        self.layer.send(
            unsafe { std::mem::transmute(ops) }
        )
    }

    fn receive(&mut self, buf: &mut Self::Buffer) -> Self::Response {
        self.layer.receive(&mut buf.0)
    }

    fn send_receive(&mut self, ops: &[Self::Operation], buf: &mut Self::Buffer) -> Self::Response {
        self.layer.send_receive(
            unsafe { std::mem::transmute(ops) },
            &mut buf.0
        )
    }
}

impl<L: Layer + PauliGate, Q, S, C> PauliGate for QubitSlotConvertLayer<L, Q, S, C>
    where C: Converter<Q, L::Qubit, S, L::Slot> {}
impl<L: Layer + PauliGate + HGate, Q, S, C> HGate for QubitSlotConvertLayer<L, Q, S, C>
    where C: Converter<Q, L::Qubit, S, L::Slot> {}


#[repr(transparent)]
struct ConvOp<L: Layer, Q, S, C> (L::Operation, PhantomData<(L, Q, S, C)>);
impl<L: Layer, Q, S, C> Operation<QubitSlotConvertLayer<L, Q, S, C>> for ConvOp<L, Q, S, C>
        where C: Converter<Q, L::Qubit, S, L::Slot>,
              QubitSlotConvertLayer<L, Q, S, C>: Layer,
{
    fn initialize() -> Self {
        ConvOp(L::Operation::initialize(), PhantomData)
    }

    fn measure(q: Q, s: S) -> Self {
        ConvOp(L::Operation::measure(C::qconv(q), C::sconv(s)), PhantomData)
    }

    fn x(q: <QubitSlotConvertLayer<L, Q, S, C> as Layer>::Qubit) -> Self
            where QubitSlotConvertLayer<L, Q, S, C>: PauliGate,
                  L: Layer, C: Converter<Q, L::Qubit, S, L::Slot>,
    {
        ConvOp(L::Operation::x(C::qconv(q)), PhantomData)
    }

    fn y(q: Q) -> Self where QubitSlotConvertLayer<L, Q, S, C>: PauliGate {
        ConvOp(L::Operation::y(C::qconv(q)), PhantomData)
    }

    fn z(q: Q) -> Self where QubitSlotConvertLayer<L, Q, S, C>: PauliGate {
        ConvOp(L::Operation::z(C::qconv(q)), PhantomData)
    }

    fn h(q: Q) -> Self where QubitSlotConvertLayer<L, Q, S, C>: HGate {
        ConvOp(L::Operation::h(C::qconv(q)), PhantomData)
    }

    fn s(q: Q) -> Self where QubitSlotConvertLayer<L, Q, S, C>: SGate {
        ConvOp(L::Operation::s(C::qconv(q)), PhantomData)
    }

    fn sdg(q: Q) -> Self where QubitSlotConvertLayer<L, Q, S, C>: SGate {
        ConvOp(L::Operation::sdg(C::qconv(q)), PhantomData)
    }

    fn t(q: Q) -> Self where QubitSlotConvertLayer<L, Q, S, C>: TGate {
        ConvOp(L::Operation::t(C::qconv(q)), PhantomData)
    }

    fn tdg(q: Q) -> Self where QubitSlotConvertLayer<L, Q, S, C>: TGate {
        ConvOp(L::Operation::tdg(C::qconv(q)), PhantomData)
    }

    fn cx(c: Q, t: Q) -> Self
    where QubitSlotConvertLayer<L, Q, S, C>: CXGate {
        ConvOp(L::Operation::cx(C::qconv(c), C::qconv(t)), PhantomData)
    }
}

struct ConvBuf<Conv, L: Layer, C> (L::Buffer, PhantomData<(Conv, C)>);
impl<Conv: Layer, L: Layer, C> Measured for ConvBuf<Conv, L, C>
        where C: Converter<Conv::Qubit, L::Qubit, Conv::Slot, L::Slot>
{
    type Slot = Conv::Slot;

    fn get(&self, n: Self::Slot) -> bool {
        self.0.get(C::sconv(n))
    }
}

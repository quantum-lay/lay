use std::marker::PhantomData;
use crate::{Layer, Measured, operations::{Operation, PauliOperation, HOperation, SOperation, TOperation, CXOperation}, gates::{PauliGate, HGate, SGate, TGate, CXGate}};

pub trait Converter<Q1, Q2, S1, S2> {
    fn qconv(q: Q1) -> Q2;
    fn sconv(s: S1) -> S2;
}

#[derive(Debug)]
pub struct QubitSlotConvertLayer<L: Layer, Q, S, C>
    where C: Converter<Q, L::Qubit, S, L::Slot>
{
    layer: L,
    phantom: PhantomData<(C, Q, S)>,
}

impl<L: Layer, Q, S, C> QubitSlotConvertLayer<L, Q, S, C>
        where C: Converter<Q, L::Qubit, S, L::Slot> {
    pub fn new(layer: L) -> Self {
        QubitSlotConvertLayer { layer, phantom: PhantomData }
    }
}

impl<L: Layer, Q, S, C> Layer for QubitSlotConvertLayer<L, Q, S, C>
    where C: Converter<Q, L::Qubit, S, L::Slot>
{
    type Operation = QubitSlotConvertOperation<L, Q, S, C>;
    type Qubit = Q;
    type Slot = S;
    type Buffer = QubitSlotConvertLayerBuffer<Self, L, C>;
    type Requested = L::Requested;
    type Response = L::Response;

    fn make_buffer(&self) -> Self::Buffer {
        QubitSlotConvertLayerBuffer(self.layer.make_buffer(), PhantomData)
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
    where L: PauliGate, C: Converter<Q, L::Qubit, S, L::Slot> {}
impl<L: Layer + HGate, Q, S, C> HGate for QubitSlotConvertLayer<L, Q, S, C>
    where C: Converter<Q, L::Qubit, S, L::Slot> {}
impl<L: Layer + SGate, Q, S, C> SGate for QubitSlotConvertLayer<L, Q, S, C>
    where C: Converter<Q, L::Qubit, S, L::Slot> {}
impl<L: Layer + TGate, Q, S, C> TGate for QubitSlotConvertLayer<L, Q, S, C>
    where C: Converter<Q, L::Qubit, S, L::Slot> {}
impl<L: Layer + CXGate, Q, S, C> CXGate for QubitSlotConvertLayer<L, Q, S, C>
    where C: Converter<Q, L::Qubit, S, L::Slot> {}

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QubitSlotConvertOperation<L: Layer, Q, S, C> (L::Operation, std::marker::PhantomData<(Q, S, C)>);

impl<L: Layer, Q, S, C> QubitSlotConvertOperation<L, Q, S, C>
    where C: Converter<Q, L::Qubit, S, L::Slot>
{
    fn new(op: L::Operation) -> Self {
        Self(op, PhantomData)
    }
}

impl<L: Layer, Q, S, C> Operation<QubitSlotConvertLayer<L, Q, S, C>> for QubitSlotConvertOperation<L, Q, S, C>
    where C: Converter<Q, L::Qubit, S, L::Slot>,
          L::Operation: Operation<L>,
{
    fn initialize() -> Self {
        Self::new(L::Operation::initialize())
    }

    fn measure(q: Q, s: S) -> Self {
        Self::new(L::Operation::measure(C::qconv(q), C::sconv(s)))
    }
}

impl<L, Q, S, C> PauliOperation<QubitSlotConvertLayer<L, Q, S, C>> for QubitSlotConvertOperation<L, Q, S, C>
    where L: Layer + PauliGate,
          C: Converter<Q, L::Qubit, S, L::Slot>,
          <L as Layer>::Operation: PauliOperation<L>,
{
    fn x(q: Q) -> Self {
        Self::new(L::Operation::x(C::qconv(q)))
    }

    fn y(q: Q) -> Self {
        Self::new(L::Operation::y(C::qconv(q)))
    }

    fn z(q: Q) -> Self {
        Self::new(L::Operation::z(C::qconv(q)))
    }
}

impl<L, Q, S, C> HOperation<QubitSlotConvertLayer<L, Q, S, C>> for QubitSlotConvertOperation<L, Q, S, C>
    where L: Layer + HGate,
          C: Converter<Q, L::Qubit, S, L::Slot>,
          <L as Layer>::Operation: HOperation<L>,
{
    fn h(q: Q) -> Self {
        Self::new(L::Operation::h(C::qconv(q)))
    }
}

impl<L, Q, S, C> SOperation<QubitSlotConvertLayer<L, Q, S, C>> for QubitSlotConvertOperation<L, Q, S, C>
    where L: Layer + SGate,
          C: Converter<Q, L::Qubit, S, L::Slot>,
          <L as Layer>::Operation: SOperation<L>,
{
    fn s(q: Q) -> Self {
        Self::new(L::Operation::s(C::qconv(q)))
    }

    fn sdg(q: Q) -> Self {
        Self::new(L::Operation::sdg(C::qconv(q)))
    }
}

impl<L, Q, S, C> TOperation<QubitSlotConvertLayer<L, Q, S, C>> for QubitSlotConvertOperation<L, Q, S, C>
    where L: Layer + TGate,
          C: Converter<Q, L::Qubit, S, L::Slot>,
          <L as Layer>::Operation: TOperation<L>,
{
    fn t(q: Q) -> Self {
        Self::new(L::Operation::t(C::qconv(q)))
    }

    fn tdg(q: Q) -> Self {
        Self::new(L::Operation::tdg(C::qconv(q)))
    }
}

impl<L, Q, S, C> CXOperation<QubitSlotConvertLayer<L, Q, S, C>> for QubitSlotConvertOperation<L, Q, S, C>
    where L: Layer + CXGate,
          C: Converter<Q, L::Qubit, S, L::Slot>,
          <L as Layer>::Operation: CXOperation<L>,
{
    fn cx(c: Q, t: Q) -> Self {
        Self::new(L::Operation::cx(C::qconv(c), C::qconv(t)))
    }
}

#[repr(transparent)]
pub struct QubitSlotConvertLayerBuffer<Conv, L: Layer, C> (L::Buffer, PhantomData<(Conv, C)>);

impl<Conv: Layer, L: Layer, C> Measured for QubitSlotConvertLayerBuffer<Conv, L, C>
        where C: Converter<Conv::Qubit, L::Qubit, Conv::Slot, L::Slot>
{
    type Slot = Conv::Slot;

    fn get(&self, n: Self::Slot) -> bool {
        self.0.get(C::sconv(n))
    }
}

pub struct SerializationLayerConverterU32<const W: u32>();
impl<const W: u32> Converter<(u32, u32), u32, (u32, u32), u32> for SerializationLayerConverterU32<W> {
    fn qconv(q: (u32, u32)) -> u32 { 
        q.0 + q.1 * W
    }

    fn sconv(s: (u32, u32)) -> u32 { 
        s.0 + s.1 * W
    }
}

pub type SerializationLayer<L, const W: u32> = QubitSlotConvertLayer<L, (u32, u32), (u32, u32), SerializationLayerConverterU32<W>>;

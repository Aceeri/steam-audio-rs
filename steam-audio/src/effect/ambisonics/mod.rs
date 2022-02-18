pub mod decode;
pub mod encode;

// How many channels the higher order ambisonic has.
//
// 0 -> 1
// 1 -> 4
// 2 -> 9
// ...
pub fn ambisonic_order_channels(order: u8) -> usize {
    (order as usize + 1) * (order as usize + 1)
}

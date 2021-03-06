use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register"] pub cr: CR,
    #[doc = "0x04 - software trigger register"] pub swtrigr: SWTRIGR,
    #[doc = "0x08 - channel1 12-bit right-aligned data holding register"] pub dhr12r1: DHR12R1,
    #[doc = "0x0c - channel1 12-bit left aligned data holding register"] pub dhr12l1: DHR12L1,
    #[doc = "0x10 - channel1 8-bit right aligned data holding register"] pub dhr8r1: DHR8R1,
    #[doc = "0x14 - DAC channel2 12-bit right-aligned data holding register"] pub dhr12r2: DHR12R2,
    #[doc = "0x18 - DAC channel2 12-bit left-aligned data holding register"] pub dhr12l2: DHR12L2,
    #[doc = "0x1c - DAC channel2 8-bit right-aligned data holding register"] pub dhr8r2: DHR8R2,
    #[doc = "0x20 - DHR12RD"] pub dhr12rd: DHR12RD,
    #[doc = "0x24 - Dual DAC 12-bit left-aligned data holding register"] pub dhr12ld: DHR12LD,
    #[doc = "0x28 - Dual DAC 8-bit right-aligned data holding register"] pub dhr8rd: DHR8RD,
    #[doc = "0x2c - channel1 data output register"] pub dor1: DOR1,
    #[doc = "0x30 - DAC channel2 data output register"] pub dor2: DOR2,
    #[doc = "0x34 - status register"] pub sr: SR,
}
#[doc = "control register"]
pub struct CR {
    register: VolatileCell<u32>,
}
#[doc = "control register"]
pub mod cr;
#[doc = "software trigger register"]
pub struct SWTRIGR {
    register: VolatileCell<u32>,
}
#[doc = "software trigger register"]
pub mod swtrigr;
#[doc = "channel1 12-bit right-aligned data holding register"]
pub struct DHR12R1 {
    register: VolatileCell<u32>,
}
#[doc = "channel1 12-bit right-aligned data holding register"]
pub mod dhr12r1;
#[doc = "channel1 12-bit left aligned data holding register"]
pub struct DHR12L1 {
    register: VolatileCell<u32>,
}
#[doc = "channel1 12-bit left aligned data holding register"]
pub mod dhr12l1;
#[doc = "channel1 8-bit right aligned data holding register"]
pub struct DHR8R1 {
    register: VolatileCell<u32>,
}
#[doc = "channel1 8-bit right aligned data holding register"]
pub mod dhr8r1;
#[doc = "channel1 data output register"]
pub struct DOR1 {
    register: VolatileCell<u32>,
}
#[doc = "channel1 data output register"]
pub mod dor1;
#[doc = "status register"]
pub struct SR {
    register: VolatileCell<u32>,
}
#[doc = "status register"]
pub mod sr;
#[doc = "DAC channel2 12-bit right-aligned data holding register"]
pub struct DHR12R2 {
    register: VolatileCell<u32>,
}
#[doc = "DAC channel2 12-bit right-aligned data holding register"]
pub mod dhr12r2;
#[doc = "DAC channel2 12-bit left-aligned data holding register"]
pub struct DHR12L2 {
    register: VolatileCell<u32>,
}
#[doc = "DAC channel2 12-bit left-aligned data holding register"]
pub mod dhr12l2;
#[doc = "DAC channel2 8-bit right-aligned data holding register"]
pub struct DHR8R2 {
    register: VolatileCell<u32>,
}
#[doc = "DAC channel2 8-bit right-aligned data holding register"]
pub mod dhr8r2;
#[doc = "DHR12RD"]
pub struct DHR12RD {
    register: VolatileCell<u32>,
}
#[doc = "DHR12RD"]
pub mod dhr12rd;
#[doc = "Dual DAC 12-bit left-aligned data holding register"]
pub struct DHR12LD {
    register: VolatileCell<u32>,
}
#[doc = "Dual DAC 12-bit left-aligned data holding register"]
pub mod dhr12ld;
#[doc = "Dual DAC 8-bit right-aligned data holding register"]
pub struct DHR8RD {
    register: VolatileCell<u32>,
}
#[doc = "Dual DAC 8-bit right-aligned data holding register"]
pub mod dhr8rd;
#[doc = "DAC channel2 data output register"]
pub struct DOR2 {
    register: VolatileCell<u32>,
}
#[doc = "DAC channel2 data output register"]
pub mod dor2;

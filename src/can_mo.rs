#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    mo: [Mo; 64],
}
impl RegisterBlock {
    #[doc = "0x00..0x800 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo(&self, n: usize) -> &Mo {
        &self.mo[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x800 - Message Object Registers"]
    #[inline(always)]
    pub fn mo_iter(&self) -> impl Iterator<Item = &Mo> {
        self.mo.iter()
    }
}
#[doc = "Message Object Registers"]
pub use self::mo::Mo;
#[doc = r"Cluster"]
#[doc = "Message Object Registers"]
pub mod mo;

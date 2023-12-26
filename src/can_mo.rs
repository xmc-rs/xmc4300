#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    mo: [MO; 64],
}
impl RegisterBlock {
    #[doc = "0x00..0x800 - Message Object Registers"]
    #[inline(always)]
    pub const fn mo(&self, n: usize) -> &MO {
        &self.mo[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x800 - Message Object Registers"]
    #[inline(always)]
    pub fn mo_iter(&self) -> impl Iterator<Item = &MO> {
        self.mo.iter()
    }
}
#[doc = "Message Object Registers"]
pub use self::mo::MO;
#[doc = r"Cluster"]
#[doc = "Message Object Registers"]
pub mod mo;

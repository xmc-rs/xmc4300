#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x800 - Message Object Registers"]
    pub mo: [MO; 64],
}
#[doc = "Message Object Registers"]
pub use self::mo::MO;
#[doc = r"Cluster"]
#[doc = "Message Object Registers"]
pub mod mo;

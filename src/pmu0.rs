#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    id: ID,
}
impl RegisterBlock {
    #[doc = "0x00 - PMU0 Identification Register"]
    #[inline(always)]
    pub const fn id(&self) -> &ID {
        &self.id
    }
}
#[doc = "ID (r) register accessor: PMU0 Identification Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`id::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id`]
module"]
pub type ID = crate::Reg<id::ID_SPEC>;
#[doc = "PMU0 Identification Register"]
pub mod id;

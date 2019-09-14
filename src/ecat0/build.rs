#[doc = "Reader of register BUILD"]
pub type R = crate::R<u16, super::BUILD>;
#[doc = "Reader of field `BUILD`"]
pub type BUILD_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Actual build of EtherCAT controller"]
    #[inline(always)]
    pub fn build(&self) -> BUILD_R {
        BUILD_R::new((self.bits & 0xffff) as u16)
    }
}

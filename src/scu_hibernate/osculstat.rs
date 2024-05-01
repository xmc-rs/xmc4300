#[doc = "Register `OSCULSTAT` reader"]
pub type R = crate::R<OsculstatSpec>;
#[doc = "Field `X1D` reader - XTAL1 Data Value"]
pub type X1dR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - XTAL1 Data Value"]
    #[inline(always)]
    pub fn x1d(&self) -> X1dR {
        X1dR::new((self.bits & 1) != 0)
    }
}
#[doc = "OSC_ULP Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osculstat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OsculstatSpec;
impl crate::RegisterSpec for OsculstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`osculstat::R`](R) reader structure"]
impl crate::Readable for OsculstatSpec {}
#[doc = "`reset()` method sets OSCULSTAT to value 0"]
impl crate::Resettable for OsculstatSpec {
    const RESET_VALUE: u32 = 0;
}

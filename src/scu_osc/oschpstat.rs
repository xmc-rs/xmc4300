#[doc = "Register `OSCHPSTAT` reader"]
pub type R = crate::R<OschpstatSpec>;
#[doc = "Field `X1D` reader - XTAL1 Data Value"]
pub type X1dR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - XTAL1 Data Value"]
    #[inline(always)]
    pub fn x1d(&self) -> X1dR {
        X1dR::new((self.bits & 1) != 0)
    }
}
#[doc = "OSC_HP Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oschpstat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OschpstatSpec;
impl crate::RegisterSpec for OschpstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`oschpstat::R`](R) reader structure"]
impl crate::Readable for OschpstatSpec {}
#[doc = "`reset()` method sets OSCHPSTAT to value 0"]
impl crate::Resettable for OschpstatSpec {
    const RESET_VALUE: u32 = 0;
}

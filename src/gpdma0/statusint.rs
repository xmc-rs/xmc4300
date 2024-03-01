#[doc = "Register `STATUSINT` reader"]
pub type R = crate::R<StatusintSpec>;
#[doc = "Field `TFR` reader - OR of the contents of STATUSTFR register"]
pub type TfrR = crate::BitReader;
#[doc = "Field `BLOCK` reader - OR of the contents of STATUSBLOCK register"]
pub type BlockR = crate::BitReader;
#[doc = "Field `SRCT` reader - OR of the contents of STATUSSRCTRAN register"]
pub type SrctR = crate::BitReader;
#[doc = "Field `DSTT` reader - OR of the contents of STATUSDSTTRAN register"]
pub type DsttR = crate::BitReader;
#[doc = "Field `ERR` reader - OR of the contents of STATUSERR register"]
pub type ErrR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - OR of the contents of STATUSTFR register"]
    #[inline(always)]
    pub fn tfr(&self) -> TfrR {
        TfrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OR of the contents of STATUSBLOCK register"]
    #[inline(always)]
    pub fn block(&self) -> BlockR {
        BlockR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - OR of the contents of STATUSSRCTRAN register"]
    #[inline(always)]
    pub fn srct(&self) -> SrctR {
        SrctR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OR of the contents of STATUSDSTTRAN register"]
    #[inline(always)]
    pub fn dstt(&self) -> DsttR {
        DsttR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - OR of the contents of STATUSERR register"]
    #[inline(always)]
    pub fn err(&self) -> ErrR {
        ErrR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "Combined Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`statusint::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusintSpec;
impl crate::RegisterSpec for StatusintSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`statusint::R`](R) reader structure"]
impl crate::Readable for StatusintSpec {}
#[doc = "`reset()` method sets STATUSINT to value 0"]
impl crate::Resettable for StatusintSpec {
    const RESET_VALUE: u32 = 0;
}

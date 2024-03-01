#[doc = "Register `STSSR` reader"]
pub type R = crate::R<StssrSpec>;
#[doc = "Field `SPSE` reader - Periodic Seconds Service Request Status after Masking"]
pub type SpseR = crate::BitReader;
#[doc = "Field `SPMI` reader - Periodic Minutes Service Request Status after Masking"]
pub type SpmiR = crate::BitReader;
#[doc = "Field `SPHO` reader - Periodic Hours Service Request Status after Masking"]
pub type SphoR = crate::BitReader;
#[doc = "Field `SPDA` reader - Periodic Days Service Request Status after Masking"]
pub type SpdaR = crate::BitReader;
#[doc = "Field `SPMO` reader - Periodic Months Service Request Status after Masking"]
pub type SpmoR = crate::BitReader;
#[doc = "Field `SPYE` reader - Periodic Years Service Request Status after Masking"]
pub type SpyeR = crate::BitReader;
#[doc = "Field `SAI` reader - Alarm Service Request Status after Masking"]
pub type SaiR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Periodic Seconds Service Request Status after Masking"]
    #[inline(always)]
    pub fn spse(&self) -> SpseR {
        SpseR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Periodic Minutes Service Request Status after Masking"]
    #[inline(always)]
    pub fn spmi(&self) -> SpmiR {
        SpmiR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Periodic Hours Service Request Status after Masking"]
    #[inline(always)]
    pub fn spho(&self) -> SphoR {
        SphoR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Periodic Days Service Request Status after Masking"]
    #[inline(always)]
    pub fn spda(&self) -> SpdaR {
        SpdaR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Periodic Months Service Request Status after Masking"]
    #[inline(always)]
    pub fn spmo(&self) -> SpmoR {
        SpmoR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Periodic Years Service Request Status after Masking"]
    #[inline(always)]
    pub fn spye(&self) -> SpyeR {
        SpyeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Alarm Service Request Status after Masking"]
    #[inline(always)]
    pub fn sai(&self) -> SaiR {
        SaiR::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "RTC Service Request Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stssr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StssrSpec;
impl crate::RegisterSpec for StssrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stssr::R`](R) reader structure"]
impl crate::Readable for StssrSpec {}
#[doc = "`reset()` method sets STSSR to value 0"]
impl crate::Resettable for StssrSpec {
    const RESET_VALUE: u32 = 0;
}

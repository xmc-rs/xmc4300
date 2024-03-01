#[doc = "Register `RAWSTAT` reader"]
pub type R = crate::R<RawstatSpec>;
#[doc = "Field `RPSE` reader - Raw Periodic Seconds Service Request"]
pub type RpseR = crate::BitReader;
#[doc = "Field `RPMI` reader - Raw Periodic Minutes Service Request"]
pub type RpmiR = crate::BitReader;
#[doc = "Field `RPHO` reader - Raw Periodic Hours Service Request"]
pub type RphoR = crate::BitReader;
#[doc = "Field `RPDA` reader - Raw Periodic Days Service Request"]
pub type RpdaR = crate::BitReader;
#[doc = "Field `RPMO` reader - Raw Periodic Months Service Request"]
pub type RpmoR = crate::BitReader;
#[doc = "Field `RPYE` reader - Raw Periodic Years Service Request"]
pub type RpyeR = crate::BitReader;
#[doc = "Field `RAI` reader - Raw Alarm Service Request"]
pub type RaiR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Raw Periodic Seconds Service Request"]
    #[inline(always)]
    pub fn rpse(&self) -> RpseR {
        RpseR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Raw Periodic Minutes Service Request"]
    #[inline(always)]
    pub fn rpmi(&self) -> RpmiR {
        RpmiR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Raw Periodic Hours Service Request"]
    #[inline(always)]
    pub fn rpho(&self) -> RphoR {
        RphoR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Raw Periodic Days Service Request"]
    #[inline(always)]
    pub fn rpda(&self) -> RpdaR {
        RpdaR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Raw Periodic Months Service Request"]
    #[inline(always)]
    pub fn rpmo(&self) -> RpmoR {
        RpmoR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Raw Periodic Years Service Request"]
    #[inline(always)]
    pub fn rpye(&self) -> RpyeR {
        RpyeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Raw Alarm Service Request"]
    #[inline(always)]
    pub fn rai(&self) -> RaiR {
        RaiR::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "RTC Raw Service Request Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rawstat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RawstatSpec;
impl crate::RegisterSpec for RawstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rawstat::R`](R) reader structure"]
impl crate::Readable for RawstatSpec {}
#[doc = "`reset()` method sets RAWSTAT to value 0"]
impl crate::Resettable for RawstatSpec {
    const RESET_VALUE: u32 = 0;
}

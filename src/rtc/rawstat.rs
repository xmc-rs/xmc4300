#[doc = "Register `RAWSTAT` reader"]
pub type R = crate::R<RAWSTAT_SPEC>;
#[doc = "Field `RPSE` reader - Raw Periodic Seconds Service Request"]
pub type RPSE_R = crate::BitReader;
#[doc = "Field `RPMI` reader - Raw Periodic Minutes Service Request"]
pub type RPMI_R = crate::BitReader;
#[doc = "Field `RPHO` reader - Raw Periodic Hours Service Request"]
pub type RPHO_R = crate::BitReader;
#[doc = "Field `RPDA` reader - Raw Periodic Days Service Request"]
pub type RPDA_R = crate::BitReader;
#[doc = "Field `RPMO` reader - Raw Periodic Months Service Request"]
pub type RPMO_R = crate::BitReader;
#[doc = "Field `RPYE` reader - Raw Periodic Years Service Request"]
pub type RPYE_R = crate::BitReader;
#[doc = "Field `RAI` reader - Raw Alarm Service Request"]
pub type RAI_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Raw Periodic Seconds Service Request"]
    #[inline(always)]
    pub fn rpse(&self) -> RPSE_R {
        RPSE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Raw Periodic Minutes Service Request"]
    #[inline(always)]
    pub fn rpmi(&self) -> RPMI_R {
        RPMI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Raw Periodic Hours Service Request"]
    #[inline(always)]
    pub fn rpho(&self) -> RPHO_R {
        RPHO_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Raw Periodic Days Service Request"]
    #[inline(always)]
    pub fn rpda(&self) -> RPDA_R {
        RPDA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Raw Periodic Months Service Request"]
    #[inline(always)]
    pub fn rpmo(&self) -> RPMO_R {
        RPMO_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Raw Periodic Years Service Request"]
    #[inline(always)]
    pub fn rpye(&self) -> RPYE_R {
        RPYE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Raw Alarm Service Request"]
    #[inline(always)]
    pub fn rai(&self) -> RAI_R {
        RAI_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "RTC Raw Service Request Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rawstat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RAWSTAT_SPEC;
impl crate::RegisterSpec for RAWSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rawstat::R`](R) reader structure"]
impl crate::Readable for RAWSTAT_SPEC {}
#[doc = "`reset()` method sets RAWSTAT to value 0"]
impl crate::Resettable for RAWSTAT_SPEC {
    const RESET_VALUE: u32 = 0;
}

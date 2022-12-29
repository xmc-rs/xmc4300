#[doc = "Register `STSSR` reader"]
pub struct R(crate::R<STSSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STSSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STSSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STSSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SPSE` reader - Periodic Seconds Service Request Status after Masking"]
pub type SPSE_R = crate::BitReader<bool>;
#[doc = "Field `SPMI` reader - Periodic Minutes Service Request Status after Masking"]
pub type SPMI_R = crate::BitReader<bool>;
#[doc = "Field `SPHO` reader - Periodic Hours Service Request Status after Masking"]
pub type SPHO_R = crate::BitReader<bool>;
#[doc = "Field `SPDA` reader - Periodic Days Service Request Status after Masking"]
pub type SPDA_R = crate::BitReader<bool>;
#[doc = "Field `SPMO` reader - Periodic Months Service Request Status after Masking"]
pub type SPMO_R = crate::BitReader<bool>;
#[doc = "Field `SPYE` reader - Periodic Years Service Request Status after Masking"]
pub type SPYE_R = crate::BitReader<bool>;
#[doc = "Field `SAI` reader - Alarm Service Request Status after Masking"]
pub type SAI_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Periodic Seconds Service Request Status after Masking"]
    #[inline(always)]
    pub fn spse(&self) -> SPSE_R {
        SPSE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Periodic Minutes Service Request Status after Masking"]
    #[inline(always)]
    pub fn spmi(&self) -> SPMI_R {
        SPMI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Periodic Hours Service Request Status after Masking"]
    #[inline(always)]
    pub fn spho(&self) -> SPHO_R {
        SPHO_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Periodic Days Service Request Status after Masking"]
    #[inline(always)]
    pub fn spda(&self) -> SPDA_R {
        SPDA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Periodic Months Service Request Status after Masking"]
    #[inline(always)]
    pub fn spmo(&self) -> SPMO_R {
        SPMO_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Periodic Years Service Request Status after Masking"]
    #[inline(always)]
    pub fn spye(&self) -> SPYE_R {
        SPYE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Alarm Service Request Status after Masking"]
    #[inline(always)]
    pub fn sai(&self) -> SAI_R {
        SAI_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "RTC Service Request Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stssr](index.html) module"]
pub struct STSSR_SPEC;
impl crate::RegisterSpec for STSSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stssr::R](R) reader structure"]
impl crate::Readable for STSSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STSSR to value 0"]
impl crate::Resettable for STSSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

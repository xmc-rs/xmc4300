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
pub struct SPSE_R(crate::FieldReader<bool, bool>);
impl SPSE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPSE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPMI` reader - Periodic Minutes Service Request Status after Masking"]
pub struct SPMI_R(crate::FieldReader<bool, bool>);
impl SPMI_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPMI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPMI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPHO` reader - Periodic Hours Service Request Status after Masking"]
pub struct SPHO_R(crate::FieldReader<bool, bool>);
impl SPHO_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPHO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPHO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPDA` reader - Periodic Days Service Request Status after Masking"]
pub struct SPDA_R(crate::FieldReader<bool, bool>);
impl SPDA_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPDA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPDA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPMO` reader - Periodic Months Service Request Status after Masking"]
pub struct SPMO_R(crate::FieldReader<bool, bool>);
impl SPMO_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPMO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPMO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPYE` reader - Periodic Years Service Request Status after Masking"]
pub struct SPYE_R(crate::FieldReader<bool, bool>);
impl SPYE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPYE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPYE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAI` reader - Alarm Service Request Status after Masking"]
pub struct SAI_R(crate::FieldReader<bool, bool>);
impl SAI_R {
    pub(crate) fn new(bits: bool) -> Self {
        SAI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Periodic Seconds Service Request Status after Masking"]
    #[inline(always)]
    pub fn spse(&self) -> SPSE_R {
        SPSE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Periodic Minutes Service Request Status after Masking"]
    #[inline(always)]
    pub fn spmi(&self) -> SPMI_R {
        SPMI_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Periodic Hours Service Request Status after Masking"]
    #[inline(always)]
    pub fn spho(&self) -> SPHO_R {
        SPHO_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Periodic Days Service Request Status after Masking"]
    #[inline(always)]
    pub fn spda(&self) -> SPDA_R {
        SPDA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Periodic Months Service Request Status after Masking"]
    #[inline(always)]
    pub fn spmo(&self) -> SPMO_R {
        SPMO_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Periodic Years Service Request Status after Masking"]
    #[inline(always)]
    pub fn spye(&self) -> SPYE_R {
        SPYE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Alarm Service Request Status after Masking"]
    #[inline(always)]
    pub fn sai(&self) -> SAI_R {
        SAI_R::new(((self.bits >> 8) & 0x01) != 0)
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
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

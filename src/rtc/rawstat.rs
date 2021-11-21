#[doc = "Register `RAWSTAT` reader"]
pub struct R(crate::R<RAWSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAWSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAWSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAWSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RPSE` reader - Raw Periodic Seconds Service Request"]
pub struct RPSE_R(crate::FieldReader<bool, bool>);
impl RPSE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RPSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPSE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RPMI` reader - Raw Periodic Minutes Service Request"]
pub struct RPMI_R(crate::FieldReader<bool, bool>);
impl RPMI_R {
    pub(crate) fn new(bits: bool) -> Self {
        RPMI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPMI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RPHO` reader - Raw Periodic Hours Service Request"]
pub struct RPHO_R(crate::FieldReader<bool, bool>);
impl RPHO_R {
    pub(crate) fn new(bits: bool) -> Self {
        RPHO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPHO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RPDA` reader - Raw Periodic Days Service Request"]
pub struct RPDA_R(crate::FieldReader<bool, bool>);
impl RPDA_R {
    pub(crate) fn new(bits: bool) -> Self {
        RPDA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPDA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RPMO` reader - Raw Periodic Months Service Request"]
pub struct RPMO_R(crate::FieldReader<bool, bool>);
impl RPMO_R {
    pub(crate) fn new(bits: bool) -> Self {
        RPMO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPMO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RPYE` reader - Raw Periodic Years Service Request"]
pub struct RPYE_R(crate::FieldReader<bool, bool>);
impl RPYE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RPYE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPYE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAI` reader - Raw Alarm Service Request"]
pub struct RAI_R(crate::FieldReader<bool, bool>);
impl RAI_R {
    pub(crate) fn new(bits: bool) -> Self {
        RAI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Raw Periodic Seconds Service Request"]
    #[inline(always)]
    pub fn rpse(&self) -> RPSE_R {
        RPSE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Raw Periodic Minutes Service Request"]
    #[inline(always)]
    pub fn rpmi(&self) -> RPMI_R {
        RPMI_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Raw Periodic Hours Service Request"]
    #[inline(always)]
    pub fn rpho(&self) -> RPHO_R {
        RPHO_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Raw Periodic Days Service Request"]
    #[inline(always)]
    pub fn rpda(&self) -> RPDA_R {
        RPDA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Raw Periodic Months Service Request"]
    #[inline(always)]
    pub fn rpmo(&self) -> RPMO_R {
        RPMO_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Raw Periodic Years Service Request"]
    #[inline(always)]
    pub fn rpye(&self) -> RPYE_R {
        RPYE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Raw Alarm Service Request"]
    #[inline(always)]
    pub fn rai(&self) -> RAI_R {
        RAI_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
#[doc = "RTC Raw Service Request Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rawstat](index.html) module"]
pub struct RAWSTAT_SPEC;
impl crate::RegisterSpec for RAWSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rawstat::R](R) reader structure"]
impl crate::Readable for RAWSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RAWSTAT to value 0"]
impl crate::Resettable for RAWSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

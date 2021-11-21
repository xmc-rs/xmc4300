#[doc = "Register `QBUR0` reader"]
pub struct R(crate::R<QBUR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QBUR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QBUR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QBUR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `REQCHNR` reader - Request Channel Number"]
pub struct REQCHNR_R(crate::FieldReader<u8, u8>);
impl REQCHNR_R {
    pub(crate) fn new(bits: u8) -> Self {
        REQCHNR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REQCHNR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RF` reader - Refill"]
pub struct RF_R(crate::FieldReader<bool, bool>);
impl RF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENSI` reader - Enable Source Interrupt"]
pub struct ENSI_R(crate::FieldReader<bool, bool>);
impl ENSI_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENSI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENSI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTR` reader - External Trigger"]
pub struct EXTR_R(crate::FieldReader<bool, bool>);
impl EXTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Request Channel Number Valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum V_A {
    #[doc = "0: Backup register not valid"]
    VALUE1 = 0,
    #[doc = "1: Backup register contains a valid entry. This will be requested before a valid entry in queue register 0 (stage 0) will be requested."]
    VALUE2 = 1,
}
impl From<V_A> for bool {
    #[inline(always)]
    fn from(variant: V_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `V` reader - Request Channel Number Valid"]
pub struct V_R(crate::FieldReader<bool, V_A>);
impl V_R {
    pub(crate) fn new(bits: bool) -> Self {
        V_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> V_A {
        match self.bits {
            false => V_A::VALUE1,
            true => V_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == V_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == V_A::VALUE2
    }
}
impl core::ops::Deref for V_R {
    type Target = crate::FieldReader<bool, V_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:4 - Request Channel Number"]
    #[inline(always)]
    pub fn reqchnr(&self) -> REQCHNR_R {
        REQCHNR_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Refill"]
    #[inline(always)]
    pub fn rf(&self) -> RF_R {
        RF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable Source Interrupt"]
    #[inline(always)]
    pub fn ensi(&self) -> ENSI_R {
        ENSI_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - External Trigger"]
    #[inline(always)]
    pub fn extr(&self) -> EXTR_R {
        EXTR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Request Channel Number Valid"]
    #[inline(always)]
    pub fn v(&self) -> V_R {
        V_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
#[doc = "Queue 0 Backup Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qbur0](index.html) module"]
pub struct QBUR0_SPEC;
impl crate::RegisterSpec for QBUR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [qbur0::R](R) reader structure"]
impl crate::Readable for QBUR0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets QBUR0 to value 0"]
impl crate::Resettable for QBUR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

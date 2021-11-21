#[doc = "Register `SLOT_INT_STATUS` reader"]
pub struct R(crate::R<SLOT_INT_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLOT_INT_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLOT_INT_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLOT_INT_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Interrupt Signal for Card Slot\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SLOT_INT_STATUS_A {
    #[doc = "0: Slot 1"]
    VALUE1 = 0,
}
impl From<SLOT_INT_STATUS_A> for u8 {
    #[inline(always)]
    fn from(variant: SLOT_INT_STATUS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SLOT_INT_STATUS` reader - Interrupt Signal for Card Slot"]
pub struct SLOT_INT_STATUS_R(crate::FieldReader<u8, SLOT_INT_STATUS_A>);
impl SLOT_INT_STATUS_R {
    pub(crate) fn new(bits: u8) -> Self {
        SLOT_INT_STATUS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SLOT_INT_STATUS_A> {
        match self.bits {
            0 => Some(SLOT_INT_STATUS_A::VALUE1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == SLOT_INT_STATUS_A::VALUE1
    }
}
impl core::ops::Deref for SLOT_INT_STATUS_R {
    type Target = crate::FieldReader<u8, SLOT_INT_STATUS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Interrupt Signal for Card Slot"]
    #[inline(always)]
    pub fn slot_int_status(&self) -> SLOT_INT_STATUS_R {
        SLOT_INT_STATUS_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Slot Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slot_int_status](index.html) module"]
pub struct SLOT_INT_STATUS_SPEC;
impl crate::RegisterSpec for SLOT_INT_STATUS_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [slot_int_status::R](R) reader structure"]
impl crate::Readable for SLOT_INT_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SLOT_INT_STATUS to value 0"]
impl crate::Resettable for SLOT_INT_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

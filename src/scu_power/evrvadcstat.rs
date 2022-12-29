#[doc = "Register `EVRVADCSTAT` reader"]
pub struct R(crate::R<EVRVADCSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVRVADCSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVRVADCSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVRVADCSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VADC13V` reader - VADC 1.3 V Conversion Result"]
pub type VADC13V_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VADC33V` reader - VADC 3.3 V Conversion Result"]
pub type VADC33V_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - VADC 1.3 V Conversion Result"]
    #[inline(always)]
    pub fn vadc13v(&self) -> VADC13V_R {
        VADC13V_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - VADC 3.3 V Conversion Result"]
    #[inline(always)]
    pub fn vadc33v(&self) -> VADC33V_R {
        VADC33V_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "EVR VADC Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evrvadcstat](index.html) module"]
pub struct EVRVADCSTAT_SPEC;
impl crate::RegisterSpec for EVRVADCSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [evrvadcstat::R](R) reader structure"]
impl crate::Readable for EVRVADCSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EVRVADCSTAT to value 0"]
impl crate::Resettable for EVRVADCSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

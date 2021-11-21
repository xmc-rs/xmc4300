#[doc = "Register `FMMU_L_STOP_BIT` reader"]
pub struct R(crate::R<FMMU_L_STOP_BIT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMMU_L_STOP_BIT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMMU_L_STOP_BIT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMMU_L_STOP_BIT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `L_STOP_BIT` reader - Last logical bit that shall be mapped"]
pub struct L_STOP_BIT_R(crate::FieldReader<u8, u8>);
impl L_STOP_BIT_R {
    pub(crate) fn new(bits: u8) -> Self {
        L_STOP_BIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for L_STOP_BIT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:2 - Last logical bit that shall be mapped"]
    #[inline(always)]
    pub fn l_stop_bit(&self) -> L_STOP_BIT_R {
        L_STOP_BIT_R::new((self.bits & 0x07) as u8)
    }
}
#[doc = "Stop bit FMMU 0 in logical address space\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmmu_l_stop_bit](index.html) module"]
pub struct FMMU_L_STOP_BIT_SPEC;
impl crate::RegisterSpec for FMMU_L_STOP_BIT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [fmmu_l_stop_bit::R](R) reader structure"]
impl crate::Readable for FMMU_L_STOP_BIT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FMMU_L_STOP_BIT to value 0"]
impl crate::Resettable for FMMU_L_STOP_BIT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

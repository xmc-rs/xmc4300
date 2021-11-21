#[doc = "Register `DC_PDI_START_EV_TIME` reader"]
pub struct R(crate::R<DC_PDI_START_EV_TIME_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DC_PDI_START_EV_TIME_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DC_PDI_START_EV_TIME_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DC_PDI_START_EV_TIME_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PDI_START_EV_TIME` reader - Register captures local time when at least one SyncManager asserts an PDI buffer start event"]
pub struct PDI_START_EV_TIME_R(crate::FieldReader<u32, u32>);
impl PDI_START_EV_TIME_R {
    pub(crate) fn new(bits: u32) -> Self {
        PDI_START_EV_TIME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDI_START_EV_TIME_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Register captures local time when at least one SyncManager asserts an PDI buffer start event"]
    #[inline(always)]
    pub fn pdi_start_ev_time(&self) -> PDI_START_EV_TIME_R {
        PDI_START_EV_TIME_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "PDI Buffer Start Event Time\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dc_pdi_start_ev_time](index.html) module"]
pub struct DC_PDI_START_EV_TIME_SPEC;
impl crate::RegisterSpec for DC_PDI_START_EV_TIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dc_pdi_start_ev_time::R](R) reader structure"]
impl crate::Readable for DC_PDI_START_EV_TIME_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DC_PDI_START_EV_TIME to value 0"]
impl crate::Resettable for DC_PDI_START_EV_TIME_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

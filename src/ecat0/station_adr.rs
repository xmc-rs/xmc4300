#[doc = "Register `STATION_ADR` reader"]
pub struct R(crate::R<STATION_ADR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATION_ADR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATION_ADR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATION_ADR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NODE_ADDR` reader - Address used for node addressing (FPxx commands)"]
pub struct NODE_ADDR_R(crate::FieldReader<u16, u16>);
impl NODE_ADDR_R {
    pub(crate) fn new(bits: u16) -> Self {
        NODE_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NODE_ADDR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Address used for node addressing (FPxx commands)"]
    #[inline(always)]
    pub fn node_addr(&self) -> NODE_ADDR_R {
        NODE_ADDR_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Configured Station Address\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [station_adr](index.html) module"]
pub struct STATION_ADR_SPEC;
impl crate::RegisterSpec for STATION_ADR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [station_adr::R](R) reader structure"]
impl crate::Readable for STATION_ADR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATION_ADR to value 0"]
impl crate::Resettable for STATION_ADR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

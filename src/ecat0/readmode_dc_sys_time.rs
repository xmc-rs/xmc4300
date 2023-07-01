#[doc = "Register `DC_SYS_TIME[%s]` reader"]
pub struct R(crate::R<READMODE_DC_SYS_TIME_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<READMODE_DC_SYS_TIME_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<READMODE_DC_SYS_TIME_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<READMODE_DC_SYS_TIME_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `READ_ACCESS` reader - Read access"]
pub type READ_ACCESS_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Read access"]
    #[inline(always)]
    pub fn read_access(&self) -> READ_ACCESS_R {
        READ_ACCESS_R::new(self.bits)
    }
}
#[doc = "System Time read access\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [readmode_dc_sys_time](index.html) module"]
pub struct READMODE_DC_SYS_TIME_SPEC;
impl crate::RegisterSpec for READMODE_DC_SYS_TIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [readmode_dc_sys_time::R](R) reader structure"]
impl crate::Readable for READMODE_DC_SYS_TIME_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DC_SYS_TIME[%s]
to value 0"]
impl crate::Resettable for READMODE_DC_SYS_TIME_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `DC_SYS_TIME[%s]` reader"]
pub type R = crate::R<READMODE_DC_SYS_TIME_SPEC>;
#[doc = "Field `READ_ACCESS` reader - Read access"]
pub type READ_ACCESS_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Read access"]
    #[inline(always)]
    pub fn read_access(&self) -> READ_ACCESS_R {
        READ_ACCESS_R::new(self.bits)
    }
}
#[doc = "System Time read access\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`readmode_dc_sys_time::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct READMODE_DC_SYS_TIME_SPEC;
impl crate::RegisterSpec for READMODE_DC_SYS_TIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`readmode_dc_sys_time::R`](R) reader structure"]
impl crate::Readable for READMODE_DC_SYS_TIME_SPEC {}
#[doc = "`reset()` method sets DC_SYS_TIME[%s]
to value 0"]
impl crate::Resettable for READMODE_DC_SYS_TIME_SPEC {
    const RESET_VALUE: u32 = 0;
}

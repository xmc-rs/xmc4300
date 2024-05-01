#[doc = "Register `DC_SYS_TIME[%s]` reader"]
pub type R = crate::R<ReadmodeDcSysTimeSpec>;
#[doc = "Field `READ_ACCESS` reader - Read access"]
pub type ReadAccessR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Read access"]
    #[inline(always)]
    pub fn read_access(&self) -> ReadAccessR {
        ReadAccessR::new(self.bits)
    }
}
#[doc = "System Time read access\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`readmode_dc_sys_time::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ReadmodeDcSysTimeSpec;
impl crate::RegisterSpec for ReadmodeDcSysTimeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`readmode_dc_sys_time::R`](R) reader structure"]
impl crate::Readable for ReadmodeDcSysTimeSpec {}
#[doc = "`reset()` method sets DC_SYS_TIME[%s]
to value 0"]
impl crate::Resettable for ReadmodeDcSysTimeSpec {
    const RESET_VALUE: u32 = 0;
}

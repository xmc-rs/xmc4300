#[doc = "Register `BUILD` reader"]
pub type R = crate::R<BuildSpec>;
#[doc = "Field `BUILD` reader - Actual build of EtherCAT controller"]
pub type BuildR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Actual build of EtherCAT controller"]
    #[inline(always)]
    pub fn build(&self) -> BuildR {
        BuildR::new(self.bits)
    }
}
#[doc = "Build Version\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`build::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BuildSpec;
impl crate::RegisterSpec for BuildSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`build::R`](R) reader structure"]
impl crate::Readable for BuildSpec {}
#[doc = "`reset()` method sets BUILD to value 0x01"]
impl crate::Resettable for BuildSpec {
    const RESET_VALUE: u16 = 0x01;
}

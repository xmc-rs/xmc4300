#[doc = "Register `VERSION` reader"]
pub type R = crate::R<VersionSpec>;
#[doc = "Field `SNPSVER` reader - Synopsys-defined Version (3.7)"]
pub type SnpsverR = crate::FieldReader;
#[doc = "Field `USERVER` reader - User-defined Version (Configured with the coreConsultant)"]
pub type UserverR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Synopsys-defined Version (3.7)"]
    #[inline(always)]
    pub fn snpsver(&self) -> SnpsverR {
        SnpsverR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - User-defined Version (Configured with the coreConsultant)"]
    #[inline(always)]
    pub fn userver(&self) -> UserverR {
        UserverR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "Version Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`version::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VersionSpec;
impl crate::RegisterSpec for VersionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`version::R`](R) reader structure"]
impl crate::Readable for VersionSpec {}
#[doc = "`reset()` method sets VERSION to value 0x1037"]
impl crate::Resettable for VersionSpec {
    const RESET_VALUE: u32 = 0x1037;
}

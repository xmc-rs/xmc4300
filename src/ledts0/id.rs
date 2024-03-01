#[doc = "Register `ID` reader"]
pub type R = crate::R<IdSpec>;
#[doc = "Field `MOD_REV` reader - Module Revision Number"]
pub type ModRevR = crate::FieldReader;
#[doc = "Field `MOD_TYPE` reader - Module Type"]
pub type ModTypeR = crate::FieldReader;
#[doc = "Field `MOD_NUMBER` reader - Module Number Value"]
pub type ModNumberR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:7 - Module Revision Number"]
    #[inline(always)]
    pub fn mod_rev(&self) -> ModRevR {
        ModRevR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Module Type"]
    #[inline(always)]
    pub fn mod_type(&self) -> ModTypeR {
        ModTypeR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - Module Number Value"]
    #[inline(always)]
    pub fn mod_number(&self) -> ModNumberR {
        ModNumberR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Module Identification Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`id::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdSpec;
impl crate::RegisterSpec for IdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`id::R`](R) reader structure"]
impl crate::Readable for IdSpec {}
#[doc = "`reset()` method sets ID to value 0x00ab_c000"]
impl crate::Resettable for IdSpec {
    const RESET_VALUE: u32 = 0x00ab_c000;
}

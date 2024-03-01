#[doc = "Register `ID` reader"]
pub type R = crate::R<IdSpec>;
#[doc = "Field `MODR` reader - Module Revision"]
pub type ModrR = crate::FieldReader;
#[doc = "Field `MODT` reader - Module Type"]
pub type ModtR = crate::FieldReader;
#[doc = "Field `MODN` reader - Module Number"]
pub type ModnR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:7 - Module Revision"]
    #[inline(always)]
    pub fn modr(&self) -> ModrR {
        ModrR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Module Type"]
    #[inline(always)]
    pub fn modt(&self) -> ModtR {
        ModtR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - Module Number"]
    #[inline(always)]
    pub fn modn(&self) -> ModnR {
        ModnR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Module Identification Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`id::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdSpec;
impl crate::RegisterSpec for IdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`id::R`](R) reader structure"]
impl crate::Readable for IdSpec {}
#[doc = "`reset()` method sets ID to value 0x00a5_c000"]
impl crate::Resettable for IdSpec {
    const RESET_VALUE: u32 = 0x00a5_c000;
}

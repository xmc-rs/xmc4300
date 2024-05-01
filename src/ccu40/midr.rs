#[doc = "Register `MIDR` reader"]
pub type R = crate::R<MidrSpec>;
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
#[doc = "Module Identification\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`midr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MidrSpec;
impl crate::RegisterSpec for MidrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`midr::R`](R) reader structure"]
impl crate::Readable for MidrSpec {}
#[doc = "`reset()` method sets MIDR to value 0x00a6_c000"]
impl crate::Resettable for MidrSpec {
    const RESET_VALUE: u32 = 0x00a6_c000;
}

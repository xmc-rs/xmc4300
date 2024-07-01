#[doc = "Register `MIDR` reader"]
pub type R = crate::R<MIDR_SPEC>;
#[doc = "Field `MODR` reader - Module Revision"]
pub type MODR_R = crate::FieldReader;
#[doc = "Field `MODT` reader - Module Type"]
pub type MODT_R = crate::FieldReader;
#[doc = "Field `MODN` reader - Module Number"]
pub type MODN_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:7 - Module Revision"]
    #[inline(always)]
    pub fn modr(&self) -> MODR_R {
        MODR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Module Type"]
    #[inline(always)]
    pub fn modt(&self) -> MODT_R {
        MODT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - Module Number"]
    #[inline(always)]
    pub fn modn(&self) -> MODN_R {
        MODN_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Module Identification\n\nYou can [`read`](crate::Reg::read) this register and get [`midr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MIDR_SPEC;
impl crate::RegisterSpec for MIDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`midr::R`](R) reader structure"]
impl crate::Readable for MIDR_SPEC {}
#[doc = "`reset()` method sets MIDR to value 0x00a7_c000"]
impl crate::Resettable for MIDR_SPEC {
    const RESET_VALUE: u32 = 0x00a7_c000;
}

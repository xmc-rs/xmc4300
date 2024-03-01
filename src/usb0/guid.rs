#[doc = "Register `GUID` reader"]
pub type R = crate::R<GuidSpec>;
#[doc = "Register `GUID` writer"]
pub type W = crate::W<GuidSpec>;
#[doc = "Field `MOD_REV` reader - Module Revision"]
pub type ModRevR = crate::FieldReader;
#[doc = "Field `MOD_REV` writer - Module Revision"]
pub type ModRevW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MOD_TYPE` reader - Module Type"]
pub type ModTypeR = crate::FieldReader;
#[doc = "Field `MOD_TYPE` writer - Module Type"]
pub type ModTypeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MOD_NUMBER` reader - Module Number"]
pub type ModNumberR = crate::FieldReader<u16>;
#[doc = "Field `MOD_NUMBER` writer - Module Number"]
pub type ModNumberW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:7 - Module Revision"]
    #[inline(always)]
    pub fn mod_rev(&self) -> ModRevR {
        ModRevR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Module Type"]
    #[inline(always)]
    pub fn mod_type(&self) -> ModTypeR {
        ModTypeR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - Module Number"]
    #[inline(always)]
    pub fn mod_number(&self) -> ModNumberR {
        ModNumberR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - Module Revision"]
    #[inline(always)]
    #[must_use]
    pub fn mod_rev(&mut self) -> ModRevW<GuidSpec> {
        ModRevW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Module Type"]
    #[inline(always)]
    #[must_use]
    pub fn mod_type(&mut self) -> ModTypeW<GuidSpec> {
        ModTypeW::new(self, 8)
    }
    #[doc = "Bits 16:31 - Module Number"]
    #[inline(always)]
    #[must_use]
    pub fn mod_number(&mut self) -> ModNumberW<GuidSpec> {
        ModNumberW::new(self, 16)
    }
}
#[doc = "USB Module Identification Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`guid::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`guid::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GuidSpec;
impl crate::RegisterSpec for GuidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`guid::R`](R) reader structure"]
impl crate::Readable for GuidSpec {}
#[doc = "`write(|w| ..)` method takes [`guid::W`](W) writer structure"]
impl crate::Writable for GuidSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GUID to value 0x00ae_c000"]
impl crate::Resettable for GuidSpec {
    const RESET_VALUE: u32 = 0x00ae_c000;
}

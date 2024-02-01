#[doc = "Register `GUID` reader"]
pub type R = crate::R<GUID_SPEC>;
#[doc = "Register `GUID` writer"]
pub type W = crate::W<GUID_SPEC>;
#[doc = "Field `MOD_REV` reader - Module Revision"]
pub type MOD_REV_R = crate::FieldReader;
#[doc = "Field `MOD_REV` writer - Module Revision"]
pub type MOD_REV_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MOD_TYPE` reader - Module Type"]
pub type MOD_TYPE_R = crate::FieldReader;
#[doc = "Field `MOD_TYPE` writer - Module Type"]
pub type MOD_TYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MOD_NUMBER` reader - Module Number"]
pub type MOD_NUMBER_R = crate::FieldReader<u16>;
#[doc = "Field `MOD_NUMBER` writer - Module Number"]
pub type MOD_NUMBER_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:7 - Module Revision"]
    #[inline(always)]
    pub fn mod_rev(&self) -> MOD_REV_R {
        MOD_REV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Module Type"]
    #[inline(always)]
    pub fn mod_type(&self) -> MOD_TYPE_R {
        MOD_TYPE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - Module Number"]
    #[inline(always)]
    pub fn mod_number(&self) -> MOD_NUMBER_R {
        MOD_NUMBER_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - Module Revision"]
    #[inline(always)]
    #[must_use]
    pub fn mod_rev(&mut self) -> MOD_REV_W<GUID_SPEC> {
        MOD_REV_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Module Type"]
    #[inline(always)]
    #[must_use]
    pub fn mod_type(&mut self) -> MOD_TYPE_W<GUID_SPEC> {
        MOD_TYPE_W::new(self, 8)
    }
    #[doc = "Bits 16:31 - Module Number"]
    #[inline(always)]
    #[must_use]
    pub fn mod_number(&mut self) -> MOD_NUMBER_W<GUID_SPEC> {
        MOD_NUMBER_W::new(self, 16)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "USB Module Identification Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`guid::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`guid::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GUID_SPEC;
impl crate::RegisterSpec for GUID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`guid::R`](R) reader structure"]
impl crate::Readable for GUID_SPEC {}
#[doc = "`write(|w| ..)` method takes [`guid::W`](W) writer structure"]
impl crate::Writable for GUID_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GUID to value 0x00ae_c000"]
impl crate::Resettable for GUID_SPEC {
    const RESET_VALUE: u32 = 0x00ae_c000;
}

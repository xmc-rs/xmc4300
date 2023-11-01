#[doc = "Register `ATIM1` reader"]
pub type R = crate::R<ATIM1_SPEC>;
#[doc = "Register `ATIM1` writer"]
pub type W = crate::W<ATIM1_SPEC>;
#[doc = "Field `AMO` reader - Alarm Month Compare Value"]
pub type AMO_R = crate::FieldReader;
#[doc = "Field `AMO` writer - Alarm Month Compare Value"]
pub type AMO_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `AYE` reader - Alarm Year Compare Value"]
pub type AYE_R = crate::FieldReader<u16>;
#[doc = "Field `AYE` writer - Alarm Year Compare Value"]
pub type AYE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 8:11 - Alarm Month Compare Value"]
    #[inline(always)]
    pub fn amo(&self) -> AMO_R {
        AMO_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - Alarm Year Compare Value"]
    #[inline(always)]
    pub fn aye(&self) -> AYE_R {
        AYE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 8:11 - Alarm Month Compare Value"]
    #[inline(always)]
    #[must_use]
    pub fn amo(&mut self) -> AMO_W<ATIM1_SPEC, 8> {
        AMO_W::new(self)
    }
    #[doc = "Bits 16:31 - Alarm Year Compare Value"]
    #[inline(always)]
    #[must_use]
    pub fn aye(&mut self) -> AYE_W<ATIM1_SPEC, 16> {
        AYE_W::new(self)
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
#[doc = "RTC Alarm Time Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atim1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atim1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ATIM1_SPEC;
impl crate::RegisterSpec for ATIM1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`atim1::R`](R) reader structure"]
impl crate::Readable for ATIM1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`atim1::W`](W) writer structure"]
impl crate::Writable for ATIM1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ATIM1 to value 0"]
impl crate::Resettable for ATIM1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

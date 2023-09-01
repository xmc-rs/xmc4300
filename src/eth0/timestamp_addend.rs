#[doc = "Register `TIMESTAMP_ADDEND` reader"]
pub type R = crate::R<TIMESTAMP_ADDEND_SPEC>;
#[doc = "Register `TIMESTAMP_ADDEND` writer"]
pub type W = crate::W<TIMESTAMP_ADDEND_SPEC>;
#[doc = "Field `TSAR` reader - Timestamp Addend Register"]
pub type TSAR_R = crate::FieldReader<u32>;
#[doc = "Field `TSAR` writer - Timestamp Addend Register"]
pub type TSAR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Timestamp Addend Register"]
    #[inline(always)]
    pub fn tsar(&self) -> TSAR_R {
        TSAR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Timestamp Addend Register"]
    #[inline(always)]
    #[must_use]
    pub fn tsar(&mut self) -> TSAR_W<TIMESTAMP_ADDEND_SPEC, 0> {
        TSAR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Timestamp Addend Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timestamp_addend::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timestamp_addend::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMESTAMP_ADDEND_SPEC;
impl crate::RegisterSpec for TIMESTAMP_ADDEND_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timestamp_addend::R`](R) reader structure"]
impl crate::Readable for TIMESTAMP_ADDEND_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timestamp_addend::W`](W) writer structure"]
impl crate::Writable for TIMESTAMP_ADDEND_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMESTAMP_ADDEND to value 0"]
impl crate::Resettable for TIMESTAMP_ADDEND_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

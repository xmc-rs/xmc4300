#[doc = "Register `BYP` reader"]
pub type R = crate::R<BYP_SPEC>;
#[doc = "Register `BYP` writer"]
pub type W = crate::W<BYP_SPEC>;
#[doc = "Field `BDATA` reader - Bypass Data"]
pub type BDATA_R = crate::FieldReader<u16>;
#[doc = "Field `BDATA` writer - Bypass Data"]
pub type BDATA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Bypass Data"]
    #[inline(always)]
    pub fn bdata(&self) -> BDATA_R {
        BDATA_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Bypass Data"]
    #[inline(always)]
    #[must_use]
    pub fn bdata(&mut self) -> BDATA_W<BYP_SPEC, 0> {
        BDATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Bypass Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`byp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`byp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BYP_SPEC;
impl crate::RegisterSpec for BYP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`byp::R`](R) reader structure"]
impl crate::Readable for BYP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`byp::W`](W) writer structure"]
impl crate::Writable for BYP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BYP to value 0"]
impl crate::Resettable for BYP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

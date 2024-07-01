#[doc = "Register `BYP` reader"]
pub type R = crate::R<BYP_SPEC>;
#[doc = "Register `BYP` writer"]
pub type W = crate::W<BYP_SPEC>;
#[doc = "Field `BDATA` reader - Bypass Data"]
pub type BDATA_R = crate::FieldReader<u16>;
#[doc = "Field `BDATA` writer - Bypass Data"]
pub type BDATA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
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
    pub fn bdata(&mut self) -> BDATA_W<BYP_SPEC> {
        BDATA_W::new(self, 0)
    }
}
#[doc = "Bypass Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`byp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`byp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BYP_SPEC;
impl crate::RegisterSpec for BYP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`byp::R`](R) reader structure"]
impl crate::Readable for BYP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`byp::W`](W) writer structure"]
impl crate::Writable for BYP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BYP to value 0"]
impl crate::Resettable for BYP_SPEC {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `MOFGPR` reader"]
pub type R = crate::R<MOFGPR_SPEC>;
#[doc = "Register `MOFGPR` writer"]
pub type W = crate::W<MOFGPR_SPEC>;
#[doc = "Field `BOT` reader - Bottom Pointer"]
pub type BOT_R = crate::FieldReader;
#[doc = "Field `BOT` writer - Bottom Pointer"]
pub type BOT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TOP` reader - Top Pointer"]
pub type TOP_R = crate::FieldReader;
#[doc = "Field `TOP` writer - Top Pointer"]
pub type TOP_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CUR` reader - Current Object Pointer"]
pub type CUR_R = crate::FieldReader;
#[doc = "Field `CUR` writer - Current Object Pointer"]
pub type CUR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SEL` reader - Object Select Pointer"]
pub type SEL_R = crate::FieldReader;
#[doc = "Field `SEL` writer - Object Select Pointer"]
pub type SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Bottom Pointer"]
    #[inline(always)]
    pub fn bot(&self) -> BOT_R {
        BOT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Top Pointer"]
    #[inline(always)]
    pub fn top(&self) -> TOP_R {
        TOP_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Current Object Pointer"]
    #[inline(always)]
    pub fn cur(&self) -> CUR_R {
        CUR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Object Select Pointer"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bottom Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn bot(&mut self) -> BOT_W<MOFGPR_SPEC> {
        BOT_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Top Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn top(&mut self) -> TOP_W<MOFGPR_SPEC> {
        TOP_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Current Object Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn cur(&mut self) -> CUR_W<MOFGPR_SPEC> {
        CUR_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Object Select Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SEL_W<MOFGPR_SPEC> {
        SEL_W::new(self, 24)
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
#[doc = "Message Object FIFO/Gateway Pointer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mofgpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mofgpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MOFGPR_SPEC;
impl crate::RegisterSpec for MOFGPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mofgpr::R`](R) reader structure"]
impl crate::Readable for MOFGPR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mofgpr::W`](W) writer structure"]
impl crate::Writable for MOFGPR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MOFGPR to value 0"]
impl crate::Resettable for MOFGPR_SPEC {
    const RESET_VALUE: u32 = 0;
}

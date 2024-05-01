#[doc = "Register `MOFGPR` reader"]
pub type R = crate::R<MofgprSpec>;
#[doc = "Register `MOFGPR` writer"]
pub type W = crate::W<MofgprSpec>;
#[doc = "Field `BOT` reader - Bottom Pointer"]
pub type BotR = crate::FieldReader;
#[doc = "Field `BOT` writer - Bottom Pointer"]
pub type BotW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TOP` reader - Top Pointer"]
pub type TopR = crate::FieldReader;
#[doc = "Field `TOP` writer - Top Pointer"]
pub type TopW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CUR` reader - Current Object Pointer"]
pub type CurR = crate::FieldReader;
#[doc = "Field `CUR` writer - Current Object Pointer"]
pub type CurW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SEL` reader - Object Select Pointer"]
pub type SelR = crate::FieldReader;
#[doc = "Field `SEL` writer - Object Select Pointer"]
pub type SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Bottom Pointer"]
    #[inline(always)]
    pub fn bot(&self) -> BotR {
        BotR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Top Pointer"]
    #[inline(always)]
    pub fn top(&self) -> TopR {
        TopR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Current Object Pointer"]
    #[inline(always)]
    pub fn cur(&self) -> CurR {
        CurR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Object Select Pointer"]
    #[inline(always)]
    pub fn sel(&self) -> SelR {
        SelR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bottom Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn bot(&mut self) -> BotW<MofgprSpec> {
        BotW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Top Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn top(&mut self) -> TopW<MofgprSpec> {
        TopW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Current Object Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn cur(&mut self) -> CurW<MofgprSpec> {
        CurW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Object Select Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SelW<MofgprSpec> {
        SelW::new(self, 24)
    }
}
#[doc = "Message Object FIFO/Gateway Pointer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mofgpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mofgpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MofgprSpec;
impl crate::RegisterSpec for MofgprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mofgpr::R`](R) reader structure"]
impl crate::Readable for MofgprSpec {}
#[doc = "`write(|w| ..)` method takes [`mofgpr::W`](W) writer structure"]
impl crate::Writable for MofgprSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MOFGPR to value 0"]
impl crate::Resettable for MofgprSpec {
    const RESET_VALUE: u32 = 0;
}

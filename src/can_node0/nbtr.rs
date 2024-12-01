#[doc = "Register `NBTR` reader"]
pub type R = crate::R<NBTR_SPEC>;
#[doc = "Register `NBTR` writer"]
pub type W = crate::W<NBTR_SPEC>;
#[doc = "Field `BRP` reader - Baud Rate Prescaler"]
pub type BRP_R = crate::FieldReader;
#[doc = "Field `BRP` writer - Baud Rate Prescaler"]
pub type BRP_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SJW` reader - (Re) Synchronization Jump Width"]
pub type SJW_R = crate::FieldReader;
#[doc = "Field `SJW` writer - (Re) Synchronization Jump Width"]
pub type SJW_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TSEG1` reader - Time Segment Before Sample Point"]
pub type TSEG1_R = crate::FieldReader;
#[doc = "Field `TSEG1` writer - Time Segment Before Sample Point"]
pub type TSEG1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TSEG2` reader - Time Segment After Sample Point"]
pub type TSEG2_R = crate::FieldReader;
#[doc = "Field `TSEG2` writer - Time Segment After Sample Point"]
pub type TSEG2_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Divide Prescaler Clock by 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIV8_A {
    #[doc = "0: A time quantum lasts (BRP+1) clock cycles."]
    VALUE1 = 0,
    #[doc = "1: A time quantum lasts 8 (BRP+1) clock cycles."]
    VALUE2 = 1,
}
impl From<DIV8_A> for bool {
    #[inline(always)]
    fn from(variant: DIV8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIV8` reader - Divide Prescaler Clock by 8"]
pub type DIV8_R = crate::BitReader<DIV8_A>;
impl DIV8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DIV8_A {
        match self.bits {
            false => DIV8_A::VALUE1,
            true => DIV8_A::VALUE2,
        }
    }
    #[doc = "A time quantum lasts (BRP+1) clock cycles."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DIV8_A::VALUE1
    }
    #[doc = "A time quantum lasts 8 (BRP+1) clock cycles."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DIV8_A::VALUE2
    }
}
#[doc = "Field `DIV8` writer - Divide Prescaler Clock by 8"]
pub type DIV8_W<'a, REG> = crate::BitWriter<'a, REG, DIV8_A>;
impl<'a, REG> DIV8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A time quantum lasts (BRP+1) clock cycles."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DIV8_A::VALUE1)
    }
    #[doc = "A time quantum lasts 8 (BRP+1) clock cycles."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DIV8_A::VALUE2)
    }
}
impl R {
    #[doc = "Bits 0:5 - Baud Rate Prescaler"]
    #[inline(always)]
    pub fn brp(&self) -> BRP_R {
        BRP_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - (Re) Synchronization Jump Width"]
    #[inline(always)]
    pub fn sjw(&self) -> SJW_R {
        SJW_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11 - Time Segment Before Sample Point"]
    #[inline(always)]
    pub fn tseg1(&self) -> TSEG1_R {
        TSEG1_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Time Segment After Sample Point"]
    #[inline(always)]
    pub fn tseg2(&self) -> TSEG2_R {
        TSEG2_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Divide Prescaler Clock by 8"]
    #[inline(always)]
    pub fn div8(&self) -> DIV8_R {
        DIV8_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Baud Rate Prescaler"]
    #[inline(always)]
    pub fn brp(&mut self) -> BRP_W<NBTR_SPEC> {
        BRP_W::new(self, 0)
    }
    #[doc = "Bits 6:7 - (Re) Synchronization Jump Width"]
    #[inline(always)]
    pub fn sjw(&mut self) -> SJW_W<NBTR_SPEC> {
        SJW_W::new(self, 6)
    }
    #[doc = "Bits 8:11 - Time Segment Before Sample Point"]
    #[inline(always)]
    pub fn tseg1(&mut self) -> TSEG1_W<NBTR_SPEC> {
        TSEG1_W::new(self, 8)
    }
    #[doc = "Bits 12:14 - Time Segment After Sample Point"]
    #[inline(always)]
    pub fn tseg2(&mut self) -> TSEG2_W<NBTR_SPEC> {
        TSEG2_W::new(self, 12)
    }
    #[doc = "Bit 15 - Divide Prescaler Clock by 8"]
    #[inline(always)]
    pub fn div8(&mut self) -> DIV8_W<NBTR_SPEC> {
        DIV8_W::new(self, 15)
    }
}
#[doc = "Node Bit Timing Register\n\nYou can [`read`](crate::Reg::read) this register and get [`nbtr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nbtr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NBTR_SPEC;
impl crate::RegisterSpec for NBTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nbtr::R`](R) reader structure"]
impl crate::Readable for NBTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`nbtr::W`](W) writer structure"]
impl crate::Writable for NBTR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NBTR to value 0"]
impl crate::Resettable for NBTR_SPEC {
    const RESET_VALUE: u32 = 0;
}

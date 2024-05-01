#[doc = "Register `NBTR` reader"]
pub type R = crate::R<NbtrSpec>;
#[doc = "Register `NBTR` writer"]
pub type W = crate::W<NbtrSpec>;
#[doc = "Field `BRP` reader - Baud Rate Prescaler"]
pub type BrpR = crate::FieldReader;
#[doc = "Field `BRP` writer - Baud Rate Prescaler"]
pub type BrpW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SJW` reader - (Re) Synchronization Jump Width"]
pub type SjwR = crate::FieldReader;
#[doc = "Field `SJW` writer - (Re) Synchronization Jump Width"]
pub type SjwW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TSEG1` reader - Time Segment Before Sample Point"]
pub type Tseg1R = crate::FieldReader;
#[doc = "Field `TSEG1` writer - Time Segment Before Sample Point"]
pub type Tseg1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TSEG2` reader - Time Segment After Sample Point"]
pub type Tseg2R = crate::FieldReader;
#[doc = "Field `TSEG2` writer - Time Segment After Sample Point"]
pub type Tseg2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Divide Prescaler Clock by 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Div8 {
    #[doc = "0: A time quantum lasts (BRP+1) clock cycles."]
    Value1 = 0,
    #[doc = "1: A time quantum lasts 8 (BRP+1) clock cycles."]
    Value2 = 1,
}
impl From<Div8> for bool {
    #[inline(always)]
    fn from(variant: Div8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIV8` reader - Divide Prescaler Clock by 8"]
pub type Div8R = crate::BitReader<Div8>;
impl Div8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Div8 {
        match self.bits {
            false => Div8::Value1,
            true => Div8::Value2,
        }
    }
    #[doc = "A time quantum lasts (BRP+1) clock cycles."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Div8::Value1
    }
    #[doc = "A time quantum lasts 8 (BRP+1) clock cycles."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Div8::Value2
    }
}
#[doc = "Field `DIV8` writer - Divide Prescaler Clock by 8"]
pub type Div8W<'a, REG> = crate::BitWriter<'a, REG, Div8>;
impl<'a, REG> Div8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A time quantum lasts (BRP+1) clock cycles."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Div8::Value1)
    }
    #[doc = "A time quantum lasts 8 (BRP+1) clock cycles."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Div8::Value2)
    }
}
impl R {
    #[doc = "Bits 0:5 - Baud Rate Prescaler"]
    #[inline(always)]
    pub fn brp(&self) -> BrpR {
        BrpR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - (Re) Synchronization Jump Width"]
    #[inline(always)]
    pub fn sjw(&self) -> SjwR {
        SjwR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11 - Time Segment Before Sample Point"]
    #[inline(always)]
    pub fn tseg1(&self) -> Tseg1R {
        Tseg1R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Time Segment After Sample Point"]
    #[inline(always)]
    pub fn tseg2(&self) -> Tseg2R {
        Tseg2R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Divide Prescaler Clock by 8"]
    #[inline(always)]
    pub fn div8(&self) -> Div8R {
        Div8R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Baud Rate Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn brp(&mut self) -> BrpW<NbtrSpec> {
        BrpW::new(self, 0)
    }
    #[doc = "Bits 6:7 - (Re) Synchronization Jump Width"]
    #[inline(always)]
    #[must_use]
    pub fn sjw(&mut self) -> SjwW<NbtrSpec> {
        SjwW::new(self, 6)
    }
    #[doc = "Bits 8:11 - Time Segment Before Sample Point"]
    #[inline(always)]
    #[must_use]
    pub fn tseg1(&mut self) -> Tseg1W<NbtrSpec> {
        Tseg1W::new(self, 8)
    }
    #[doc = "Bits 12:14 - Time Segment After Sample Point"]
    #[inline(always)]
    #[must_use]
    pub fn tseg2(&mut self) -> Tseg2W<NbtrSpec> {
        Tseg2W::new(self, 12)
    }
    #[doc = "Bit 15 - Divide Prescaler Clock by 8"]
    #[inline(always)]
    #[must_use]
    pub fn div8(&mut self) -> Div8W<NbtrSpec> {
        Div8W::new(self, 15)
    }
}
#[doc = "Node Bit Timing Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nbtr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nbtr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NbtrSpec;
impl crate::RegisterSpec for NbtrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nbtr::R`](R) reader structure"]
impl crate::Readable for NbtrSpec {}
#[doc = "`write(|w| ..)` method takes [`nbtr::W`](W) writer structure"]
impl crate::Writable for NbtrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NBTR to value 0"]
impl crate::Resettable for NbtrSpec {
    const RESET_VALUE: u32 = 0;
}

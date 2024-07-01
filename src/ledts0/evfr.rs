#[doc = "Register `EVFR` reader"]
pub type R = crate::R<EVFR_SPEC>;
#[doc = "Register `EVFR` writer"]
pub type W = crate::W<EVFR_SPEC>;
#[doc = "Field `TSF` reader - Time Slice Interrupt Flag"]
pub type TSF_R = crate::BitReader;
#[doc = "Field `TFF` reader - (Extended) Time Frame Interrupt Flag"]
pub type TFF_R = crate::BitReader;
#[doc = "Field `TPF` reader - Autoscan Time Period Interrupt Flag"]
pub type TPF_R = crate::BitReader;
#[doc = "TS-Counter Overflow Indication\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSCTROVF_A {
    #[doc = "0: No overflow has occurred."]
    VALUE1 = 0,
    #[doc = "1: The TS-counter has overflowed at least once."]
    VALUE2 = 1,
}
impl From<TSCTROVF_A> for bool {
    #[inline(always)]
    fn from(variant: TSCTROVF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSCTROVF` reader - TS-Counter Overflow Indication"]
pub type TSCTROVF_R = crate::BitReader<TSCTROVF_A>;
impl TSCTROVF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TSCTROVF_A {
        match self.bits {
            false => TSCTROVF_A::VALUE1,
            true => TSCTROVF_A::VALUE2,
        }
    }
    #[doc = "No overflow has occurred."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TSCTROVF_A::VALUE1
    }
    #[doc = "The TS-counter has overflowed at least once."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TSCTROVF_A::VALUE2
    }
}
#[doc = "Clear Time Slice Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSF_A {
    #[doc = "0: No action."]
    VALUE1 = 0,
    #[doc = "1: Bit TSF is cleared."]
    VALUE2 = 1,
}
impl From<CTSF_A> for bool {
    #[inline(always)]
    fn from(variant: CTSF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSF` writer - Clear Time Slice Interrupt Flag"]
pub type CTSF_W<'a, REG> = crate::BitWriter<'a, REG, CTSF_A>;
impl<'a, REG> CTSF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CTSF_A::VALUE1)
    }
    #[doc = "Bit TSF is cleared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CTSF_A::VALUE2)
    }
}
#[doc = "Clear (Extended) Time Frame Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTFF_A {
    #[doc = "0: No action."]
    VALUE1 = 0,
    #[doc = "1: Bit TFF is cleared."]
    VALUE2 = 1,
}
impl From<CTFF_A> for bool {
    #[inline(always)]
    fn from(variant: CTFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTFF` writer - Clear (Extended) Time Frame Interrupt Flag"]
pub type CTFF_W<'a, REG> = crate::BitWriter<'a, REG, CTFF_A>;
impl<'a, REG> CTFF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CTFF_A::VALUE1)
    }
    #[doc = "Bit TFF is cleared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CTFF_A::VALUE2)
    }
}
#[doc = "Clear Autoscan Time Period Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTPF_A {
    #[doc = "0: No action."]
    VALUE1 = 0,
    #[doc = "1: Bit TPF is cleared."]
    VALUE2 = 1,
}
impl From<CTPF_A> for bool {
    #[inline(always)]
    fn from(variant: CTPF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTPF` writer - Clear Autoscan Time Period Interrupt Flag"]
pub type CTPF_W<'a, REG> = crate::BitWriter<'a, REG, CTPF_A>;
impl<'a, REG> CTPF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CTPF_A::VALUE1)
    }
    #[doc = "Bit TPF is cleared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CTPF_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Time Slice Interrupt Flag"]
    #[inline(always)]
    pub fn tsf(&self) -> TSF_R {
        TSF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - (Extended) Time Frame Interrupt Flag"]
    #[inline(always)]
    pub fn tff(&self) -> TFF_R {
        TFF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Autoscan Time Period Interrupt Flag"]
    #[inline(always)]
    pub fn tpf(&self) -> TPF_R {
        TPF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TS-Counter Overflow Indication"]
    #[inline(always)]
    pub fn tsctrovf(&self) -> TSCTROVF_R {
        TSCTROVF_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Clear Time Slice Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ctsf(&mut self) -> CTSF_W<EVFR_SPEC> {
        CTSF_W::new(self, 16)
    }
    #[doc = "Bit 17 - Clear (Extended) Time Frame Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ctff(&mut self) -> CTFF_W<EVFR_SPEC> {
        CTFF_W::new(self, 17)
    }
    #[doc = "Bit 18 - Clear Autoscan Time Period Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ctpf(&mut self) -> CTPF_W<EVFR_SPEC> {
        CTPF_W::new(self, 18)
    }
}
#[doc = "Event Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`evfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EVFR_SPEC;
impl crate::RegisterSpec for EVFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evfr::R`](R) reader structure"]
impl crate::Readable for EVFR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`evfr::W`](W) writer structure"]
impl crate::Writable for EVFR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVFR to value 0"]
impl crate::Resettable for EVFR_SPEC {
    const RESET_VALUE: u32 = 0;
}

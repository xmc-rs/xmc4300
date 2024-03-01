#[doc = "Register `EVFR` reader"]
pub type R = crate::R<EvfrSpec>;
#[doc = "Register `EVFR` writer"]
pub type W = crate::W<EvfrSpec>;
#[doc = "Field `TSF` reader - Time Slice Interrupt Flag"]
pub type TsfR = crate::BitReader;
#[doc = "Field `TFF` reader - (Extended) Time Frame Interrupt Flag"]
pub type TffR = crate::BitReader;
#[doc = "Field `TPF` reader - Autoscan Time Period Interrupt Flag"]
pub type TpfR = crate::BitReader;
#[doc = "TS-Counter Overflow Indication\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tsctrovf {
    #[doc = "0: No overflow has occurred."]
    Value1 = 0,
    #[doc = "1: The TS-counter has overflowed at least once."]
    Value2 = 1,
}
impl From<Tsctrovf> for bool {
    #[inline(always)]
    fn from(variant: Tsctrovf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSCTROVF` reader - TS-Counter Overflow Indication"]
pub type TsctrovfR = crate::BitReader<Tsctrovf>;
impl TsctrovfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tsctrovf {
        match self.bits {
            false => Tsctrovf::Value1,
            true => Tsctrovf::Value2,
        }
    }
    #[doc = "No overflow has occurred."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Tsctrovf::Value1
    }
    #[doc = "The TS-counter has overflowed at least once."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Tsctrovf::Value2
    }
}
#[doc = "Clear Time Slice Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctsf {
    #[doc = "0: No action."]
    Value1 = 0,
    #[doc = "1: Bit TSF is cleared."]
    Value2 = 1,
}
impl From<Ctsf> for bool {
    #[inline(always)]
    fn from(variant: Ctsf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSF` writer - Clear Time Slice Interrupt Flag"]
pub type CtsfW<'a, REG> = crate::BitWriter<'a, REG, Ctsf>;
impl<'a, REG> CtsfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsf::Value1)
    }
    #[doc = "Bit TSF is cleared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsf::Value2)
    }
}
#[doc = "Clear (Extended) Time Frame Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctff {
    #[doc = "0: No action."]
    Value1 = 0,
    #[doc = "1: Bit TFF is cleared."]
    Value2 = 1,
}
impl From<Ctff> for bool {
    #[inline(always)]
    fn from(variant: Ctff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTFF` writer - Clear (Extended) Time Frame Interrupt Flag"]
pub type CtffW<'a, REG> = crate::BitWriter<'a, REG, Ctff>;
impl<'a, REG> CtffW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ctff::Value1)
    }
    #[doc = "Bit TFF is cleared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ctff::Value2)
    }
}
#[doc = "Clear Autoscan Time Period Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctpf {
    #[doc = "0: No action."]
    Value1 = 0,
    #[doc = "1: Bit TPF is cleared."]
    Value2 = 1,
}
impl From<Ctpf> for bool {
    #[inline(always)]
    fn from(variant: Ctpf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTPF` writer - Clear Autoscan Time Period Interrupt Flag"]
pub type CtpfW<'a, REG> = crate::BitWriter<'a, REG, Ctpf>;
impl<'a, REG> CtpfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ctpf::Value1)
    }
    #[doc = "Bit TPF is cleared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ctpf::Value2)
    }
}
impl R {
    #[doc = "Bit 0 - Time Slice Interrupt Flag"]
    #[inline(always)]
    pub fn tsf(&self) -> TsfR {
        TsfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - (Extended) Time Frame Interrupt Flag"]
    #[inline(always)]
    pub fn tff(&self) -> TffR {
        TffR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Autoscan Time Period Interrupt Flag"]
    #[inline(always)]
    pub fn tpf(&self) -> TpfR {
        TpfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TS-Counter Overflow Indication"]
    #[inline(always)]
    pub fn tsctrovf(&self) -> TsctrovfR {
        TsctrovfR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Clear Time Slice Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ctsf(&mut self) -> CtsfW<EvfrSpec> {
        CtsfW::new(self, 16)
    }
    #[doc = "Bit 17 - Clear (Extended) Time Frame Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ctff(&mut self) -> CtffW<EvfrSpec> {
        CtffW::new(self, 17)
    }
    #[doc = "Bit 18 - Clear Autoscan Time Period Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ctpf(&mut self) -> CtpfW<EvfrSpec> {
        CtpfW::new(self, 18)
    }
}
#[doc = "Event Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evfr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evfr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvfrSpec;
impl crate::RegisterSpec for EvfrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evfr::R`](R) reader structure"]
impl crate::Readable for EvfrSpec {}
#[doc = "`write(|w| ..)` method takes [`evfr::W`](W) writer structure"]
impl crate::Writable for EvfrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVFR to value 0"]
impl crate::Resettable for EvfrSpec {
    const RESET_VALUE: u32 = 0;
}

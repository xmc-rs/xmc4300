#[doc = "Register `SYNCTR` reader"]
pub type R = crate::R<SynctrSpec>;
#[doc = "Register `SYNCTR` writer"]
pub type W = crate::W<SynctrSpec>;
#[doc = "Start Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Stsel {
    #[doc = "0: Kernel is synchronization master: Use own bitfield GxARBCFG.ANONC"]
    Value1 = 0,
    #[doc = "1: Kernel is synchronization slave: Control information from input CI1"]
    Value2 = 1,
    #[doc = "2: Kernel is synchronization slave: Control information from input CI2"]
    Value3 = 2,
    #[doc = "3: Kernel is synchronization slave: Control information from input CI3"]
    Value4 = 3,
}
impl From<Stsel> for u8 {
    #[inline(always)]
    fn from(variant: Stsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Stsel {
    type Ux = u8;
}
impl crate::IsEnum for Stsel {}
#[doc = "Field `STSEL` reader - Start Selection"]
pub type StselR = crate::FieldReader<Stsel>;
impl StselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stsel {
        match self.bits {
            0 => Stsel::Value1,
            1 => Stsel::Value2,
            2 => Stsel::Value3,
            3 => Stsel::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Kernel is synchronization master: Use own bitfield GxARBCFG.ANONC"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Stsel::Value1
    }
    #[doc = "Kernel is synchronization slave: Control information from input CI1"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Stsel::Value2
    }
    #[doc = "Kernel is synchronization slave: Control information from input CI2"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Stsel::Value3
    }
    #[doc = "Kernel is synchronization slave: Control information from input CI3"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Stsel::Value4
    }
}
#[doc = "Field `STSEL` writer - Start Selection"]
pub type StselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Stsel, crate::Safe>;
impl<'a, REG> StselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Kernel is synchronization master: Use own bitfield GxARBCFG.ANONC"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Stsel::Value1)
    }
    #[doc = "Kernel is synchronization slave: Control information from input CI1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Stsel::Value2)
    }
    #[doc = "Kernel is synchronization slave: Control information from input CI2"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Stsel::Value3)
    }
    #[doc = "Kernel is synchronization slave: Control information from input CI3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Stsel::Value4)
    }
}
#[doc = "Evaluate Ready Input Rx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Evalr1 {
    #[doc = "0: No ready input control"]
    Value1 = 0,
    #[doc = "1: Ready input Rx is considered for the start of a parallel conversion of this conversion group"]
    Value2 = 1,
}
impl From<Evalr1> for bool {
    #[inline(always)]
    fn from(variant: Evalr1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVALR1` reader - Evaluate Ready Input Rx"]
pub type Evalr1R = crate::BitReader<Evalr1>;
impl Evalr1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Evalr1 {
        match self.bits {
            false => Evalr1::Value1,
            true => Evalr1::Value2,
        }
    }
    #[doc = "No ready input control"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Evalr1::Value1
    }
    #[doc = "Ready input Rx is considered for the start of a parallel conversion of this conversion group"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Evalr1::Value2
    }
}
#[doc = "Field `EVALR1` writer - Evaluate Ready Input Rx"]
pub type Evalr1W<'a, REG> = crate::BitWriter<'a, REG, Evalr1>;
impl<'a, REG> Evalr1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No ready input control"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Evalr1::Value1)
    }
    #[doc = "Ready input Rx is considered for the start of a parallel conversion of this conversion group"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Evalr1::Value2)
    }
}
#[doc = "Evaluate Ready Input Rx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Evalr2 {
    #[doc = "0: No ready input control"]
    Value1 = 0,
    #[doc = "1: Ready input Rx is considered for the start of a parallel conversion of this conversion group"]
    Value2 = 1,
}
impl From<Evalr2> for bool {
    #[inline(always)]
    fn from(variant: Evalr2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVALR2` reader - Evaluate Ready Input Rx"]
pub type Evalr2R = crate::BitReader<Evalr2>;
impl Evalr2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Evalr2 {
        match self.bits {
            false => Evalr2::Value1,
            true => Evalr2::Value2,
        }
    }
    #[doc = "No ready input control"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Evalr2::Value1
    }
    #[doc = "Ready input Rx is considered for the start of a parallel conversion of this conversion group"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Evalr2::Value2
    }
}
#[doc = "Field `EVALR2` writer - Evaluate Ready Input Rx"]
pub type Evalr2W<'a, REG> = crate::BitWriter<'a, REG, Evalr2>;
impl<'a, REG> Evalr2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No ready input control"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Evalr2::Value1)
    }
    #[doc = "Ready input Rx is considered for the start of a parallel conversion of this conversion group"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Evalr2::Value2)
    }
}
#[doc = "Evaluate Ready Input Rx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Evalr3 {
    #[doc = "0: No ready input control"]
    Value1 = 0,
    #[doc = "1: Ready input Rx is considered for the start of a parallel conversion of this conversion group"]
    Value2 = 1,
}
impl From<Evalr3> for bool {
    #[inline(always)]
    fn from(variant: Evalr3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVALR3` reader - Evaluate Ready Input Rx"]
pub type Evalr3R = crate::BitReader<Evalr3>;
impl Evalr3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Evalr3 {
        match self.bits {
            false => Evalr3::Value1,
            true => Evalr3::Value2,
        }
    }
    #[doc = "No ready input control"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Evalr3::Value1
    }
    #[doc = "Ready input Rx is considered for the start of a parallel conversion of this conversion group"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Evalr3::Value2
    }
}
#[doc = "Field `EVALR3` writer - Evaluate Ready Input Rx"]
pub type Evalr3W<'a, REG> = crate::BitWriter<'a, REG, Evalr3>;
impl<'a, REG> Evalr3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No ready input control"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Evalr3::Value1)
    }
    #[doc = "Ready input Rx is considered for the start of a parallel conversion of this conversion group"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Evalr3::Value2)
    }
}
impl R {
    #[doc = "Bits 0:1 - Start Selection"]
    #[inline(always)]
    pub fn stsel(&self) -> StselR {
        StselR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - Evaluate Ready Input Rx"]
    #[inline(always)]
    pub fn evalr1(&self) -> Evalr1R {
        Evalr1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Evaluate Ready Input Rx"]
    #[inline(always)]
    pub fn evalr2(&self) -> Evalr2R {
        Evalr2R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Evaluate Ready Input Rx"]
    #[inline(always)]
    pub fn evalr3(&self) -> Evalr3R {
        Evalr3R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Start Selection"]
    #[inline(always)]
    #[must_use]
    pub fn stsel(&mut self) -> StselW<SynctrSpec> {
        StselW::new(self, 0)
    }
    #[doc = "Bit 4 - Evaluate Ready Input Rx"]
    #[inline(always)]
    #[must_use]
    pub fn evalr1(&mut self) -> Evalr1W<SynctrSpec> {
        Evalr1W::new(self, 4)
    }
    #[doc = "Bit 5 - Evaluate Ready Input Rx"]
    #[inline(always)]
    #[must_use]
    pub fn evalr2(&mut self) -> Evalr2W<SynctrSpec> {
        Evalr2W::new(self, 5)
    }
    #[doc = "Bit 6 - Evaluate Ready Input Rx"]
    #[inline(always)]
    #[must_use]
    pub fn evalr3(&mut self) -> Evalr3W<SynctrSpec> {
        Evalr3W::new(self, 6)
    }
}
#[doc = "Synchronization Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`synctr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`synctr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SynctrSpec;
impl crate::RegisterSpec for SynctrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`synctr::R`](R) reader structure"]
impl crate::Readable for SynctrSpec {}
#[doc = "`write(|w| ..)` method takes [`synctr::W`](W) writer structure"]
impl crate::Writable for SynctrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYNCTR to value 0"]
impl crate::Resettable for SynctrSpec {
    const RESET_VALUE: u32 = 0;
}

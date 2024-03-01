#[doc = "Register `ICSR` reader"]
pub type R = crate::R<IcsrSpec>;
#[doc = "Register `ICSR` writer"]
pub type W = crate::W<IcsrSpec>;
#[doc = "Active exception number\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Vectactive {
    #[doc = "0: Thread mode"]
    Value1 = 0,
}
impl From<Vectactive> for u16 {
    #[inline(always)]
    fn from(variant: Vectactive) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Vectactive {
    type Ux = u16;
}
#[doc = "Field `VECTACTIVE` reader - Active exception number"]
pub type VectactiveR = crate::FieldReader<Vectactive>;
impl VectactiveR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Vectactive> {
        match self.bits {
            0 => Some(Vectactive::Value1),
            _ => None,
        }
    }
    #[doc = "Thread mode"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Vectactive::Value1
    }
}
#[doc = "Return to Base\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rettobase {
    #[doc = "0: there are preempted active exceptions to execute"]
    Value1 = 0,
    #[doc = "1: there are no active exceptions, or the currently-executing exception is the only active exception."]
    Value2 = 1,
}
impl From<Rettobase> for bool {
    #[inline(always)]
    fn from(variant: Rettobase) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RETTOBASE` reader - Return to Base"]
pub type RettobaseR = crate::BitReader<Rettobase>;
impl RettobaseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rettobase {
        match self.bits {
            false => Rettobase::Value1,
            true => Rettobase::Value2,
        }
    }
    #[doc = "there are preempted active exceptions to execute"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rettobase::Value1
    }
    #[doc = "there are no active exceptions, or the currently-executing exception is the only active exception."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rettobase::Value2
    }
}
#[doc = "Vector Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Vectpending {
    #[doc = "0: no pending exceptions"]
    Value1 = 0,
}
impl From<Vectpending> for u8 {
    #[inline(always)]
    fn from(variant: Vectpending) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Vectpending {
    type Ux = u8;
}
#[doc = "Field `VECTPENDING` reader - Vector Pending"]
pub type VectpendingR = crate::FieldReader<Vectpending>;
impl VectpendingR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Vectpending> {
        match self.bits {
            0 => Some(Vectpending::Value1),
            _ => None,
        }
    }
    #[doc = "no pending exceptions"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Vectpending::Value1
    }
}
#[doc = "Interrupt pending flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Isrpending {
    #[doc = "0: interrupt not pending"]
    Value1 = 0,
    #[doc = "1: interrupt pending."]
    Value2 = 1,
}
impl From<Isrpending> for bool {
    #[inline(always)]
    fn from(variant: Isrpending) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISRPENDING` reader - Interrupt pending flag"]
pub type IsrpendingR = crate::BitReader<Isrpending>;
impl IsrpendingR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Isrpending {
        match self.bits {
            false => Isrpending::Value1,
            true => Isrpending::Value2,
        }
    }
    #[doc = "interrupt not pending"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Isrpending::Value1
    }
    #[doc = "interrupt pending."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Isrpending::Value2
    }
}
#[doc = "SysTick exception clear-pending bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pendstclr {
    #[doc = "0: no effect"]
    Value1 = 0,
    #[doc = "1: removes the pending state from the SysTick exception."]
    Value2 = 1,
}
impl From<Pendstclr> for bool {
    #[inline(always)]
    fn from(variant: Pendstclr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PENDSTCLR` writer - SysTick exception clear-pending bit"]
pub type PendstclrW<'a, REG> = crate::BitWriter<'a, REG, Pendstclr>;
impl<'a, REG> PendstclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Pendstclr::Value1)
    }
    #[doc = "removes the pending state from the SysTick exception."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Pendstclr::Value2)
    }
}
#[doc = "SysTick exception set-pending bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pendstset {
    #[doc = "0: no effect"]
    Value1 = 0,
    #[doc = "1: changes SysTick exception state to pending."]
    Value2 = 1,
}
impl From<Pendstset> for bool {
    #[inline(always)]
    fn from(variant: Pendstset) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PENDSTSET` reader - SysTick exception set-pending bit"]
pub type PendstsetR = crate::BitReader<Pendstset>;
impl PendstsetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pendstset {
        match self.bits {
            false => Pendstset::Value1,
            true => Pendstset::Value2,
        }
    }
    #[doc = "no effect"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pendstset::Value1
    }
    #[doc = "changes SysTick exception state to pending."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pendstset::Value2
    }
}
#[doc = "Field `PENDSTSET` writer - SysTick exception set-pending bit"]
pub type PendstsetW<'a, REG> = crate::BitWriter<'a, REG, Pendstset>;
impl<'a, REG> PendstsetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Pendstset::Value1)
    }
    #[doc = "changes SysTick exception state to pending."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Pendstset::Value2)
    }
}
#[doc = "PendSV clear-pending bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pendsvclr {
    #[doc = "0: no effect"]
    Value1 = 0,
    #[doc = "1: removes the pending state from the PendSV exception."]
    Value2 = 1,
}
impl From<Pendsvclr> for bool {
    #[inline(always)]
    fn from(variant: Pendsvclr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PENDSVCLR` writer - PendSV clear-pending bit"]
pub type PendsvclrW<'a, REG> = crate::BitWriter<'a, REG, Pendsvclr>;
impl<'a, REG> PendsvclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Pendsvclr::Value1)
    }
    #[doc = "removes the pending state from the PendSV exception."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Pendsvclr::Value2)
    }
}
#[doc = "Field `PENDSVSET` reader - PendSV set-pending bit: 0b0=no effect, 0b1=changes PendSV exception state to pending., 0b0=PendSV exception is not pending, 0b1=PendSV exception is pending.,"]
pub type PendsvsetR = crate::BitReader;
#[doc = "Field `PENDSVSET` writer - PendSV set-pending bit: 0b0=no effect, 0b1=changes PendSV exception state to pending., 0b0=PendSV exception is not pending, 0b1=PendSV exception is pending.,"]
pub type PendsvsetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NMIPENDSET` reader - NMI set-pending bit: 0b0=no effect, 0b1=changes NMI exception state to pending., 0b0=NMI exception is not pending, 0b1=NMI exception is pending.,"]
pub type NmipendsetR = crate::BitReader;
#[doc = "Field `NMIPENDSET` writer - NMI set-pending bit: 0b0=no effect, 0b1=changes NMI exception state to pending., 0b0=NMI exception is not pending, 0b1=NMI exception is pending.,"]
pub type NmipendsetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - Active exception number"]
    #[inline(always)]
    pub fn vectactive(&self) -> VectactiveR {
        VectactiveR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 11 - Return to Base"]
    #[inline(always)]
    pub fn rettobase(&self) -> RettobaseR {
        RettobaseR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:17 - Vector Pending"]
    #[inline(always)]
    pub fn vectpending(&self) -> VectpendingR {
        VectpendingR::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - Interrupt pending flag"]
    #[inline(always)]
    pub fn isrpending(&self) -> IsrpendingR {
        IsrpendingR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 26 - SysTick exception set-pending bit"]
    #[inline(always)]
    pub fn pendstset(&self) -> PendstsetR {
        PendstsetR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - PendSV set-pending bit: 0b0=no effect, 0b1=changes PendSV exception state to pending., 0b0=PendSV exception is not pending, 0b1=PendSV exception is pending.,"]
    #[inline(always)]
    pub fn pendsvset(&self) -> PendsvsetR {
        PendsvsetR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 31 - NMI set-pending bit: 0b0=no effect, 0b1=changes NMI exception state to pending., 0b0=NMI exception is not pending, 0b1=NMI exception is pending.,"]
    #[inline(always)]
    pub fn nmipendset(&self) -> NmipendsetR {
        NmipendsetR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 25 - SysTick exception clear-pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn pendstclr(&mut self) -> PendstclrW<IcsrSpec> {
        PendstclrW::new(self, 25)
    }
    #[doc = "Bit 26 - SysTick exception set-pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn pendstset(&mut self) -> PendstsetW<IcsrSpec> {
        PendstsetW::new(self, 26)
    }
    #[doc = "Bit 27 - PendSV clear-pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn pendsvclr(&mut self) -> PendsvclrW<IcsrSpec> {
        PendsvclrW::new(self, 27)
    }
    #[doc = "Bit 28 - PendSV set-pending bit: 0b0=no effect, 0b1=changes PendSV exception state to pending., 0b0=PendSV exception is not pending, 0b1=PendSV exception is pending.,"]
    #[inline(always)]
    #[must_use]
    pub fn pendsvset(&mut self) -> PendsvsetW<IcsrSpec> {
        PendsvsetW::new(self, 28)
    }
    #[doc = "Bit 31 - NMI set-pending bit: 0b0=no effect, 0b1=changes NMI exception state to pending., 0b0=NMI exception is not pending, 0b1=NMI exception is pending.,"]
    #[inline(always)]
    #[must_use]
    pub fn nmipendset(&mut self) -> NmipendsetW<IcsrSpec> {
        NmipendsetW::new(self, 31)
    }
}
#[doc = "Interrupt Control and State Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcsrSpec;
impl crate::RegisterSpec for IcsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icsr::R`](R) reader structure"]
impl crate::Readable for IcsrSpec {}
#[doc = "`write(|w| ..)` method takes [`icsr::W`](W) writer structure"]
impl crate::Writable for IcsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICSR to value 0"]
impl crate::Resettable for IcsrSpec {
    const RESET_VALUE: u32 = 0;
}

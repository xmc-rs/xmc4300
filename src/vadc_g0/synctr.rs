#[doc = "Register `SYNCTR` reader"]
pub type R = crate::R<SYNCTR_SPEC>;
#[doc = "Register `SYNCTR` writer"]
pub type W = crate::W<SYNCTR_SPEC>;
#[doc = "Field `STSEL` reader - Start Selection"]
pub type STSEL_R = crate::FieldReader<STSEL_A>;
#[doc = "Start Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STSEL_A {
    #[doc = "0: Kernel is synchronization master: Use own bitfield GxARBCFG.ANONC"]
    VALUE1 = 0,
    #[doc = "1: Kernel is synchronization slave: Control information from input CI1"]
    VALUE2 = 1,
    #[doc = "2: Kernel is synchronization slave: Control information from input CI2"]
    VALUE3 = 2,
    #[doc = "3: Kernel is synchronization slave: Control information from input CI3"]
    VALUE4 = 3,
}
impl From<STSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: STSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for STSEL_A {
    type Ux = u8;
}
impl STSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STSEL_A {
        match self.bits {
            0 => STSEL_A::VALUE1,
            1 => STSEL_A::VALUE2,
            2 => STSEL_A::VALUE3,
            3 => STSEL_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Kernel is synchronization master: Use own bitfield GxARBCFG.ANONC"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == STSEL_A::VALUE1
    }
    #[doc = "Kernel is synchronization slave: Control information from input CI1"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == STSEL_A::VALUE2
    }
    #[doc = "Kernel is synchronization slave: Control information from input CI2"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == STSEL_A::VALUE3
    }
    #[doc = "Kernel is synchronization slave: Control information from input CI3"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == STSEL_A::VALUE4
    }
}
#[doc = "Field `STSEL` writer - Start Selection"]
pub type STSEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, STSEL_A>;
impl<'a, REG> STSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Kernel is synchronization master: Use own bitfield GxARBCFG.ANONC"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(STSEL_A::VALUE1)
    }
    #[doc = "Kernel is synchronization slave: Control information from input CI1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(STSEL_A::VALUE2)
    }
    #[doc = "Kernel is synchronization slave: Control information from input CI2"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(STSEL_A::VALUE3)
    }
    #[doc = "Kernel is synchronization slave: Control information from input CI3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(STSEL_A::VALUE4)
    }
}
#[doc = "Field `EVALR1` reader - Evaluate Ready Input Rx"]
pub type EVALR1_R = crate::BitReader<EVALR1_A>;
#[doc = "Evaluate Ready Input Rx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EVALR1_A {
    #[doc = "0: No ready input control"]
    VALUE1 = 0,
    #[doc = "1: Ready input Rx is considered for the start of a parallel conversion of this conversion group"]
    VALUE2 = 1,
}
impl From<EVALR1_A> for bool {
    #[inline(always)]
    fn from(variant: EVALR1_A) -> Self {
        variant as u8 != 0
    }
}
impl EVALR1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EVALR1_A {
        match self.bits {
            false => EVALR1_A::VALUE1,
            true => EVALR1_A::VALUE2,
        }
    }
    #[doc = "No ready input control"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EVALR1_A::VALUE1
    }
    #[doc = "Ready input Rx is considered for the start of a parallel conversion of this conversion group"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EVALR1_A::VALUE2
    }
}
#[doc = "Field `EVALR1` writer - Evaluate Ready Input Rx"]
pub type EVALR1_W<'a, REG> = crate::BitWriter<'a, REG, EVALR1_A>;
impl<'a, REG> EVALR1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No ready input control"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(EVALR1_A::VALUE1)
    }
    #[doc = "Ready input Rx is considered for the start of a parallel conversion of this conversion group"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(EVALR1_A::VALUE2)
    }
}
#[doc = "Field `EVALR2` reader - Evaluate Ready Input Rx"]
pub type EVALR2_R = crate::BitReader<EVALR2_A>;
#[doc = "Evaluate Ready Input Rx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EVALR2_A {
    #[doc = "0: No ready input control"]
    VALUE1 = 0,
    #[doc = "1: Ready input Rx is considered for the start of a parallel conversion of this conversion group"]
    VALUE2 = 1,
}
impl From<EVALR2_A> for bool {
    #[inline(always)]
    fn from(variant: EVALR2_A) -> Self {
        variant as u8 != 0
    }
}
impl EVALR2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EVALR2_A {
        match self.bits {
            false => EVALR2_A::VALUE1,
            true => EVALR2_A::VALUE2,
        }
    }
    #[doc = "No ready input control"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EVALR2_A::VALUE1
    }
    #[doc = "Ready input Rx is considered for the start of a parallel conversion of this conversion group"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EVALR2_A::VALUE2
    }
}
#[doc = "Field `EVALR2` writer - Evaluate Ready Input Rx"]
pub type EVALR2_W<'a, REG> = crate::BitWriter<'a, REG, EVALR2_A>;
impl<'a, REG> EVALR2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No ready input control"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(EVALR2_A::VALUE1)
    }
    #[doc = "Ready input Rx is considered for the start of a parallel conversion of this conversion group"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(EVALR2_A::VALUE2)
    }
}
#[doc = "Field `EVALR3` reader - Evaluate Ready Input Rx"]
pub type EVALR3_R = crate::BitReader<EVALR3_A>;
#[doc = "Evaluate Ready Input Rx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EVALR3_A {
    #[doc = "0: No ready input control"]
    VALUE1 = 0,
    #[doc = "1: Ready input Rx is considered for the start of a parallel conversion of this conversion group"]
    VALUE2 = 1,
}
impl From<EVALR3_A> for bool {
    #[inline(always)]
    fn from(variant: EVALR3_A) -> Self {
        variant as u8 != 0
    }
}
impl EVALR3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EVALR3_A {
        match self.bits {
            false => EVALR3_A::VALUE1,
            true => EVALR3_A::VALUE2,
        }
    }
    #[doc = "No ready input control"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EVALR3_A::VALUE1
    }
    #[doc = "Ready input Rx is considered for the start of a parallel conversion of this conversion group"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EVALR3_A::VALUE2
    }
}
#[doc = "Field `EVALR3` writer - Evaluate Ready Input Rx"]
pub type EVALR3_W<'a, REG> = crate::BitWriter<'a, REG, EVALR3_A>;
impl<'a, REG> EVALR3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No ready input control"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(EVALR3_A::VALUE1)
    }
    #[doc = "Ready input Rx is considered for the start of a parallel conversion of this conversion group"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(EVALR3_A::VALUE2)
    }
}
impl R {
    #[doc = "Bits 0:1 - Start Selection"]
    #[inline(always)]
    pub fn stsel(&self) -> STSEL_R {
        STSEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - Evaluate Ready Input Rx"]
    #[inline(always)]
    pub fn evalr1(&self) -> EVALR1_R {
        EVALR1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Evaluate Ready Input Rx"]
    #[inline(always)]
    pub fn evalr2(&self) -> EVALR2_R {
        EVALR2_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Evaluate Ready Input Rx"]
    #[inline(always)]
    pub fn evalr3(&self) -> EVALR3_R {
        EVALR3_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Start Selection"]
    #[inline(always)]
    #[must_use]
    pub fn stsel(&mut self) -> STSEL_W<SYNCTR_SPEC> {
        STSEL_W::new(self, 0)
    }
    #[doc = "Bit 4 - Evaluate Ready Input Rx"]
    #[inline(always)]
    #[must_use]
    pub fn evalr1(&mut self) -> EVALR1_W<SYNCTR_SPEC> {
        EVALR1_W::new(self, 4)
    }
    #[doc = "Bit 5 - Evaluate Ready Input Rx"]
    #[inline(always)]
    #[must_use]
    pub fn evalr2(&mut self) -> EVALR2_W<SYNCTR_SPEC> {
        EVALR2_W::new(self, 5)
    }
    #[doc = "Bit 6 - Evaluate Ready Input Rx"]
    #[inline(always)]
    #[must_use]
    pub fn evalr3(&mut self) -> EVALR3_W<SYNCTR_SPEC> {
        EVALR3_W::new(self, 6)
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
#[doc = "Synchronization Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`synctr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`synctr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYNCTR_SPEC;
impl crate::RegisterSpec for SYNCTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`synctr::R`](R) reader structure"]
impl crate::Readable for SYNCTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`synctr::W`](W) writer structure"]
impl crate::Writable for SYNCTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYNCTR to value 0"]
impl crate::Resettable for SYNCTR_SPEC {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `CLOCK_CTRL` reader"]
pub type R = crate::R<CLOCK_CTRL_SPEC>;
#[doc = "Register `CLOCK_CTRL` writer"]
pub type W = crate::W<CLOCK_CTRL_SPEC>;
#[doc = "Field `INTERNAL_CLOCK_EN` reader - Internal Clock Enable"]
pub type INTERNAL_CLOCK_EN_R = crate::BitReader<INTERNAL_CLOCK_EN_A>;
#[doc = "Internal Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTERNAL_CLOCK_EN_A {
    #[doc = "0: Stop"]
    VALUE1 = 0,
    #[doc = "1: Oscillate"]
    VALUE2 = 1,
}
impl From<INTERNAL_CLOCK_EN_A> for bool {
    #[inline(always)]
    fn from(variant: INTERNAL_CLOCK_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl INTERNAL_CLOCK_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INTERNAL_CLOCK_EN_A {
        match self.bits {
            false => INTERNAL_CLOCK_EN_A::VALUE1,
            true => INTERNAL_CLOCK_EN_A::VALUE2,
        }
    }
    #[doc = "Stop"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == INTERNAL_CLOCK_EN_A::VALUE1
    }
    #[doc = "Oscillate"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == INTERNAL_CLOCK_EN_A::VALUE2
    }
}
#[doc = "Field `INTERNAL_CLOCK_EN` writer - Internal Clock Enable"]
pub type INTERNAL_CLOCK_EN_W<'a, REG> = crate::BitWriter<'a, REG, INTERNAL_CLOCK_EN_A>;
impl<'a, REG> INTERNAL_CLOCK_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Stop"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(INTERNAL_CLOCK_EN_A::VALUE1)
    }
    #[doc = "Oscillate"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(INTERNAL_CLOCK_EN_A::VALUE2)
    }
}
#[doc = "Field `INTERNAL_CLOCK_STABLE` reader - Internal Clock Stable"]
pub type INTERNAL_CLOCK_STABLE_R = crate::BitReader<INTERNAL_CLOCK_STABLE_A>;
#[doc = "Internal Clock Stable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTERNAL_CLOCK_STABLE_A {
    #[doc = "0: Not Ready"]
    VALUE1 = 0,
    #[doc = "1: Ready"]
    VALUE2 = 1,
}
impl From<INTERNAL_CLOCK_STABLE_A> for bool {
    #[inline(always)]
    fn from(variant: INTERNAL_CLOCK_STABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl INTERNAL_CLOCK_STABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INTERNAL_CLOCK_STABLE_A {
        match self.bits {
            false => INTERNAL_CLOCK_STABLE_A::VALUE1,
            true => INTERNAL_CLOCK_STABLE_A::VALUE2,
        }
    }
    #[doc = "Not Ready"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == INTERNAL_CLOCK_STABLE_A::VALUE1
    }
    #[doc = "Ready"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == INTERNAL_CLOCK_STABLE_A::VALUE2
    }
}
#[doc = "Field `SDCLOCK_EN` reader - SD Clock Enable"]
pub type SDCLOCK_EN_R = crate::BitReader<SDCLOCK_EN_A>;
#[doc = "SD Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDCLOCK_EN_A {
    #[doc = "0: Disable"]
    VALUE1 = 0,
    #[doc = "1: Enable"]
    VALUE2 = 1,
}
impl From<SDCLOCK_EN_A> for bool {
    #[inline(always)]
    fn from(variant: SDCLOCK_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl SDCLOCK_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SDCLOCK_EN_A {
        match self.bits {
            false => SDCLOCK_EN_A::VALUE1,
            true => SDCLOCK_EN_A::VALUE2,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SDCLOCK_EN_A::VALUE1
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SDCLOCK_EN_A::VALUE2
    }
}
#[doc = "Field `SDCLOCK_EN` writer - SD Clock Enable"]
pub type SDCLOCK_EN_W<'a, REG> = crate::BitWriter<'a, REG, SDCLOCK_EN_A>;
impl<'a, REG> SDCLOCK_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SDCLOCK_EN_A::VALUE1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SDCLOCK_EN_A::VALUE2)
    }
}
#[doc = "Field `SDCLK_FREQ_SEL` reader - SDCLK Frequency Select"]
pub type SDCLK_FREQ_SEL_R = crate::FieldReader<SDCLK_FREQ_SEL_A>;
#[doc = "SDCLK Frequency Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SDCLK_FREQ_SEL_A {
    #[doc = "0: base clock(10MHz-63MHz)"]
    VALUE1 = 0,
    #[doc = "1: base clock divided by 2"]
    VALUE2 = 1,
    #[doc = "16: base clock divided by 32"]
    VALUE3 = 16,
    #[doc = "2: base clock divided by 4"]
    VALUE4 = 2,
    #[doc = "4: base clock divided by 8"]
    VALUE5 = 4,
    #[doc = "8: base clock divided by 16"]
    VALUE6 = 8,
    #[doc = "32: base clock divided by 64"]
    VALUE7 = 32,
    #[doc = "64: base clock divided by 128"]
    VALUE8 = 64,
    #[doc = "128: base clock divided by 256"]
    VALUE9 = 128,
}
impl From<SDCLK_FREQ_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SDCLK_FREQ_SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SDCLK_FREQ_SEL_A {
    type Ux = u8;
}
impl SDCLK_FREQ_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SDCLK_FREQ_SEL_A> {
        match self.bits {
            0 => Some(SDCLK_FREQ_SEL_A::VALUE1),
            1 => Some(SDCLK_FREQ_SEL_A::VALUE2),
            16 => Some(SDCLK_FREQ_SEL_A::VALUE3),
            2 => Some(SDCLK_FREQ_SEL_A::VALUE4),
            4 => Some(SDCLK_FREQ_SEL_A::VALUE5),
            8 => Some(SDCLK_FREQ_SEL_A::VALUE6),
            32 => Some(SDCLK_FREQ_SEL_A::VALUE7),
            64 => Some(SDCLK_FREQ_SEL_A::VALUE8),
            128 => Some(SDCLK_FREQ_SEL_A::VALUE9),
            _ => None,
        }
    }
    #[doc = "base clock(10MHz-63MHz)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SDCLK_FREQ_SEL_A::VALUE1
    }
    #[doc = "base clock divided by 2"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SDCLK_FREQ_SEL_A::VALUE2
    }
    #[doc = "base clock divided by 32"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == SDCLK_FREQ_SEL_A::VALUE3
    }
    #[doc = "base clock divided by 4"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == SDCLK_FREQ_SEL_A::VALUE4
    }
    #[doc = "base clock divided by 8"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == SDCLK_FREQ_SEL_A::VALUE5
    }
    #[doc = "base clock divided by 16"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == SDCLK_FREQ_SEL_A::VALUE6
    }
    #[doc = "base clock divided by 64"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == SDCLK_FREQ_SEL_A::VALUE7
    }
    #[doc = "base clock divided by 128"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == SDCLK_FREQ_SEL_A::VALUE8
    }
    #[doc = "base clock divided by 256"]
    #[inline(always)]
    pub fn is_value9(&self) -> bool {
        *self == SDCLK_FREQ_SEL_A::VALUE9
    }
}
#[doc = "Field `SDCLK_FREQ_SEL` writer - SDCLK Frequency Select"]
pub type SDCLK_FREQ_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 8, SDCLK_FREQ_SEL_A>;
impl<'a, REG> SDCLK_FREQ_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "base clock(10MHz-63MHz)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SDCLK_FREQ_SEL_A::VALUE1)
    }
    #[doc = "base clock divided by 2"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SDCLK_FREQ_SEL_A::VALUE2)
    }
    #[doc = "base clock divided by 32"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(SDCLK_FREQ_SEL_A::VALUE3)
    }
    #[doc = "base clock divided by 4"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(SDCLK_FREQ_SEL_A::VALUE4)
    }
    #[doc = "base clock divided by 8"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(SDCLK_FREQ_SEL_A::VALUE5)
    }
    #[doc = "base clock divided by 16"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(SDCLK_FREQ_SEL_A::VALUE6)
    }
    #[doc = "base clock divided by 64"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut crate::W<REG> {
        self.variant(SDCLK_FREQ_SEL_A::VALUE7)
    }
    #[doc = "base clock divided by 128"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut crate::W<REG> {
        self.variant(SDCLK_FREQ_SEL_A::VALUE8)
    }
    #[doc = "base clock divided by 256"]
    #[inline(always)]
    pub fn value9(self) -> &'a mut crate::W<REG> {
        self.variant(SDCLK_FREQ_SEL_A::VALUE9)
    }
}
impl R {
    #[doc = "Bit 0 - Internal Clock Enable"]
    #[inline(always)]
    pub fn internal_clock_en(&self) -> INTERNAL_CLOCK_EN_R {
        INTERNAL_CLOCK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Internal Clock Stable"]
    #[inline(always)]
    pub fn internal_clock_stable(&self) -> INTERNAL_CLOCK_STABLE_R {
        INTERNAL_CLOCK_STABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SD Clock Enable"]
    #[inline(always)]
    pub fn sdclock_en(&self) -> SDCLOCK_EN_R {
        SDCLOCK_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:15 - SDCLK Frequency Select"]
    #[inline(always)]
    pub fn sdclk_freq_sel(&self) -> SDCLK_FREQ_SEL_R {
        SDCLK_FREQ_SEL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Internal Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn internal_clock_en(&mut self) -> INTERNAL_CLOCK_EN_W<CLOCK_CTRL_SPEC> {
        INTERNAL_CLOCK_EN_W::new(self, 0)
    }
    #[doc = "Bit 2 - SD Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sdclock_en(&mut self) -> SDCLOCK_EN_W<CLOCK_CTRL_SPEC> {
        SDCLOCK_EN_W::new(self, 2)
    }
    #[doc = "Bits 8:15 - SDCLK Frequency Select"]
    #[inline(always)]
    #[must_use]
    pub fn sdclk_freq_sel(&mut self) -> SDCLK_FREQ_SEL_W<CLOCK_CTRL_SPEC> {
        SDCLK_FREQ_SEL_W::new(self, 8)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Clock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clock_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLOCK_CTRL_SPEC;
impl crate::RegisterSpec for CLOCK_CTRL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`clock_ctrl::R`](R) reader structure"]
impl crate::Readable for CLOCK_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clock_ctrl::W`](W) writer structure"]
impl crate::Writable for CLOCK_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLOCK_CTRL to value 0"]
impl crate::Resettable for CLOCK_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

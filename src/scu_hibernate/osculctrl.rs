#[doc = "Register `OSCULCTRL` reader"]
pub type R = crate::R<OSCULCTRL_SPEC>;
#[doc = "Register `OSCULCTRL` writer"]
pub type W = crate::W<OSCULCTRL_SPEC>;
#[doc = "XTAL1 Data General Purpose Input Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum X1DEN_A {
    #[doc = "0: Data input inactivated, power down"]
    CONST_0 = 0,
    #[doc = "1: Data input active"]
    CONST_1 = 1,
}
impl From<X1DEN_A> for bool {
    #[inline(always)]
    fn from(variant: X1DEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `X1DEN` reader - XTAL1 Data General Purpose Input Enable"]
pub type X1DEN_R = crate::BitReader<X1DEN_A>;
impl X1DEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> X1DEN_A {
        match self.bits {
            false => X1DEN_A::CONST_0,
            true => X1DEN_A::CONST_1,
        }
    }
    #[doc = "Data input inactivated, power down"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == X1DEN_A::CONST_0
    }
    #[doc = "Data input active"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == X1DEN_A::CONST_1
    }
}
#[doc = "Field `X1DEN` writer - XTAL1 Data General Purpose Input Enable"]
pub type X1DEN_W<'a, REG> = crate::BitWriter<'a, REG, X1DEN_A>;
impl<'a, REG> X1DEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data input inactivated, power down"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(X1DEN_A::CONST_0)
    }
    #[doc = "Data input active"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(X1DEN_A::CONST_1)
    }
}
#[doc = "Oscillator Mode\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Oscillator is enabled, in operation"]
    CONST_00 = 0,
    #[doc = "1: Oscillator is enabled, in bypass mode"]
    CONST_01 = 1,
    #[doc = "2: Oscillator in power down"]
    CONST_10 = 2,
    #[doc = "3: Oscillator in power down, can be used as GPI"]
    CONST_11 = 3,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE_A {
    type Ux = u8;
}
impl crate::IsEnum for MODE_A {}
#[doc = "Field `MODE` reader - Oscillator Mode"]
pub type MODE_R = crate::FieldReader<MODE_A>;
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODE_A {
        match self.bits {
            0 => MODE_A::CONST_00,
            1 => MODE_A::CONST_01,
            2 => MODE_A::CONST_10,
            3 => MODE_A::CONST_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Oscillator is enabled, in operation"]
    #[inline(always)]
    pub fn is_const_00(&self) -> bool {
        *self == MODE_A::CONST_00
    }
    #[doc = "Oscillator is enabled, in bypass mode"]
    #[inline(always)]
    pub fn is_const_01(&self) -> bool {
        *self == MODE_A::CONST_01
    }
    #[doc = "Oscillator in power down"]
    #[inline(always)]
    pub fn is_const_10(&self) -> bool {
        *self == MODE_A::CONST_10
    }
    #[doc = "Oscillator in power down, can be used as GPI"]
    #[inline(always)]
    pub fn is_const_11(&self) -> bool {
        *self == MODE_A::CONST_11
    }
}
#[doc = "Field `MODE` writer - Oscillator Mode"]
pub type MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MODE_A, crate::Safe>;
impl<'a, REG> MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Oscillator is enabled, in operation"]
    #[inline(always)]
    pub fn const_00(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::CONST_00)
    }
    #[doc = "Oscillator is enabled, in bypass mode"]
    #[inline(always)]
    pub fn const_01(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::CONST_01)
    }
    #[doc = "Oscillator in power down"]
    #[inline(always)]
    pub fn const_10(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::CONST_10)
    }
    #[doc = "Oscillator in power down, can be used as GPI"]
    #[inline(always)]
    pub fn const_11(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::CONST_11)
    }
}
impl R {
    #[doc = "Bit 0 - XTAL1 Data General Purpose Input Enable"]
    #[inline(always)]
    pub fn x1den(&self) -> X1DEN_R {
        X1DEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5 - Oscillator Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - XTAL1 Data General Purpose Input Enable"]
    #[inline(always)]
    pub fn x1den(&mut self) -> X1DEN_W<OSCULCTRL_SPEC> {
        X1DEN_W::new(self, 0)
    }
    #[doc = "Bits 4:5 - Oscillator Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W<OSCULCTRL_SPEC> {
        MODE_W::new(self, 4)
    }
}
#[doc = "OSC_ULP Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`osculctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`osculctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OSCULCTRL_SPEC;
impl crate::RegisterSpec for OSCULCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`osculctrl::R`](R) reader structure"]
impl crate::Readable for OSCULCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`osculctrl::W`](W) writer structure"]
impl crate::Writable for OSCULCTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OSCULCTRL to value 0x20"]
impl crate::Resettable for OSCULCTRL_SPEC {
    const RESET_VALUE: u32 = 0x20;
}

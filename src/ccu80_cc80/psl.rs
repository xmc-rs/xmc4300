#[doc = "Register `PSL` reader"]
pub type R = crate::R<PSL_SPEC>;
#[doc = "Register `PSL` writer"]
pub type W = crate::W<PSL_SPEC>;
#[doc = "Field `PSL11` reader - Output Passive Level for CCU8x.OUTy0"]
pub type PSL11_R = crate::BitReader<PSL11_A>;
#[doc = "Output Passive Level for CCU8x.OUTy0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSL11_A {
    #[doc = "0: Passive Level is LOW"]
    VALUE1 = 0,
    #[doc = "1: Passive Level is HIGH"]
    VALUE2 = 1,
}
impl From<PSL11_A> for bool {
    #[inline(always)]
    fn from(variant: PSL11_A) -> Self {
        variant as u8 != 0
    }
}
impl PSL11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSL11_A {
        match self.bits {
            false => PSL11_A::VALUE1,
            true => PSL11_A::VALUE2,
        }
    }
    #[doc = "Passive Level is LOW"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PSL11_A::VALUE1
    }
    #[doc = "Passive Level is HIGH"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PSL11_A::VALUE2
    }
}
#[doc = "Field `PSL11` writer - Output Passive Level for CCU8x.OUTy0"]
pub type PSL11_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PSL11_A>;
impl<'a, REG, const O: u8> PSL11_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Passive Level is LOW"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PSL11_A::VALUE1)
    }
    #[doc = "Passive Level is HIGH"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PSL11_A::VALUE2)
    }
}
#[doc = "Field `PSL12` reader - Output Passive Level for CCU8x.OUTy1"]
pub type PSL12_R = crate::BitReader<PSL12_A>;
#[doc = "Output Passive Level for CCU8x.OUTy1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSL12_A {
    #[doc = "0: Passive Level is LOW"]
    VALUE1 = 0,
    #[doc = "1: Passive Level is HIGH"]
    VALUE2 = 1,
}
impl From<PSL12_A> for bool {
    #[inline(always)]
    fn from(variant: PSL12_A) -> Self {
        variant as u8 != 0
    }
}
impl PSL12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSL12_A {
        match self.bits {
            false => PSL12_A::VALUE1,
            true => PSL12_A::VALUE2,
        }
    }
    #[doc = "Passive Level is LOW"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PSL12_A::VALUE1
    }
    #[doc = "Passive Level is HIGH"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PSL12_A::VALUE2
    }
}
#[doc = "Field `PSL12` writer - Output Passive Level for CCU8x.OUTy1"]
pub type PSL12_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PSL12_A>;
impl<'a, REG, const O: u8> PSL12_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Passive Level is LOW"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PSL12_A::VALUE1)
    }
    #[doc = "Passive Level is HIGH"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PSL12_A::VALUE2)
    }
}
#[doc = "Field `PSL21` reader - Output Passive Level for CCU8x.OUTy2"]
pub type PSL21_R = crate::BitReader<PSL21_A>;
#[doc = "Output Passive Level for CCU8x.OUTy2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSL21_A {
    #[doc = "0: Passive Level is LOW"]
    VALUE1 = 0,
    #[doc = "1: Passive Level is HIGH"]
    VALUE2 = 1,
}
impl From<PSL21_A> for bool {
    #[inline(always)]
    fn from(variant: PSL21_A) -> Self {
        variant as u8 != 0
    }
}
impl PSL21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSL21_A {
        match self.bits {
            false => PSL21_A::VALUE1,
            true => PSL21_A::VALUE2,
        }
    }
    #[doc = "Passive Level is LOW"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PSL21_A::VALUE1
    }
    #[doc = "Passive Level is HIGH"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PSL21_A::VALUE2
    }
}
#[doc = "Field `PSL21` writer - Output Passive Level for CCU8x.OUTy2"]
pub type PSL21_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PSL21_A>;
impl<'a, REG, const O: u8> PSL21_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Passive Level is LOW"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PSL21_A::VALUE1)
    }
    #[doc = "Passive Level is HIGH"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PSL21_A::VALUE2)
    }
}
#[doc = "Field `PSL22` reader - Output Passive Level for CCU8x.OUTy3"]
pub type PSL22_R = crate::BitReader<PSL22_A>;
#[doc = "Output Passive Level for CCU8x.OUTy3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSL22_A {
    #[doc = "0: Passive Level is LOW"]
    VALUE1 = 0,
    #[doc = "1: Passive Level is HIGH"]
    VALUE2 = 1,
}
impl From<PSL22_A> for bool {
    #[inline(always)]
    fn from(variant: PSL22_A) -> Self {
        variant as u8 != 0
    }
}
impl PSL22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSL22_A {
        match self.bits {
            false => PSL22_A::VALUE1,
            true => PSL22_A::VALUE2,
        }
    }
    #[doc = "Passive Level is LOW"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PSL22_A::VALUE1
    }
    #[doc = "Passive Level is HIGH"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PSL22_A::VALUE2
    }
}
#[doc = "Field `PSL22` writer - Output Passive Level for CCU8x.OUTy3"]
pub type PSL22_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PSL22_A>;
impl<'a, REG, const O: u8> PSL22_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Passive Level is LOW"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PSL22_A::VALUE1)
    }
    #[doc = "Passive Level is HIGH"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PSL22_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Output Passive Level for CCU8x.OUTy0"]
    #[inline(always)]
    pub fn psl11(&self) -> PSL11_R {
        PSL11_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Output Passive Level for CCU8x.OUTy1"]
    #[inline(always)]
    pub fn psl12(&self) -> PSL12_R {
        PSL12_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Output Passive Level for CCU8x.OUTy2"]
    #[inline(always)]
    pub fn psl21(&self) -> PSL21_R {
        PSL21_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Output Passive Level for CCU8x.OUTy3"]
    #[inline(always)]
    pub fn psl22(&self) -> PSL22_R {
        PSL22_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Output Passive Level for CCU8x.OUTy0"]
    #[inline(always)]
    #[must_use]
    pub fn psl11(&mut self) -> PSL11_W<PSL_SPEC, 0> {
        PSL11_W::new(self)
    }
    #[doc = "Bit 1 - Output Passive Level for CCU8x.OUTy1"]
    #[inline(always)]
    #[must_use]
    pub fn psl12(&mut self) -> PSL12_W<PSL_SPEC, 1> {
        PSL12_W::new(self)
    }
    #[doc = "Bit 2 - Output Passive Level for CCU8x.OUTy2"]
    #[inline(always)]
    #[must_use]
    pub fn psl21(&mut self) -> PSL21_W<PSL_SPEC, 2> {
        PSL21_W::new(self)
    }
    #[doc = "Bit 3 - Output Passive Level for CCU8x.OUTy3"]
    #[inline(always)]
    #[must_use]
    pub fn psl22(&mut self) -> PSL22_W<PSL_SPEC, 3> {
        PSL22_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Passive Level Config\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PSL_SPEC;
impl crate::RegisterSpec for PSL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psl::R`](R) reader structure"]
impl crate::Readable for PSL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`psl::W`](W) writer structure"]
impl crate::Writable for PSL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PSL to value 0"]
impl crate::Resettable for PSL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

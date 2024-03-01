#[doc = "Register `PSL` reader"]
pub type R = crate::R<PslSpec>;
#[doc = "Register `PSL` writer"]
pub type W = crate::W<PslSpec>;
#[doc = "Output Passive Level for CCU8x.OUTy0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psl11 {
    #[doc = "0: Passive Level is LOW"]
    Value1 = 0,
    #[doc = "1: Passive Level is HIGH"]
    Value2 = 1,
}
impl From<Psl11> for bool {
    #[inline(always)]
    fn from(variant: Psl11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PSL11` reader - Output Passive Level for CCU8x.OUTy0"]
pub type Psl11R = crate::BitReader<Psl11>;
impl Psl11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Psl11 {
        match self.bits {
            false => Psl11::Value1,
            true => Psl11::Value2,
        }
    }
    #[doc = "Passive Level is LOW"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Psl11::Value1
    }
    #[doc = "Passive Level is HIGH"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Psl11::Value2
    }
}
#[doc = "Field `PSL11` writer - Output Passive Level for CCU8x.OUTy0"]
pub type Psl11W<'a, REG> = crate::BitWriter<'a, REG, Psl11>;
impl<'a, REG> Psl11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Passive Level is LOW"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Psl11::Value1)
    }
    #[doc = "Passive Level is HIGH"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Psl11::Value2)
    }
}
#[doc = "Output Passive Level for CCU8x.OUTy1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psl12 {
    #[doc = "0: Passive Level is LOW"]
    Value1 = 0,
    #[doc = "1: Passive Level is HIGH"]
    Value2 = 1,
}
impl From<Psl12> for bool {
    #[inline(always)]
    fn from(variant: Psl12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PSL12` reader - Output Passive Level for CCU8x.OUTy1"]
pub type Psl12R = crate::BitReader<Psl12>;
impl Psl12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Psl12 {
        match self.bits {
            false => Psl12::Value1,
            true => Psl12::Value2,
        }
    }
    #[doc = "Passive Level is LOW"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Psl12::Value1
    }
    #[doc = "Passive Level is HIGH"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Psl12::Value2
    }
}
#[doc = "Field `PSL12` writer - Output Passive Level for CCU8x.OUTy1"]
pub type Psl12W<'a, REG> = crate::BitWriter<'a, REG, Psl12>;
impl<'a, REG> Psl12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Passive Level is LOW"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Psl12::Value1)
    }
    #[doc = "Passive Level is HIGH"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Psl12::Value2)
    }
}
#[doc = "Output Passive Level for CCU8x.OUTy2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psl21 {
    #[doc = "0: Passive Level is LOW"]
    Value1 = 0,
    #[doc = "1: Passive Level is HIGH"]
    Value2 = 1,
}
impl From<Psl21> for bool {
    #[inline(always)]
    fn from(variant: Psl21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PSL21` reader - Output Passive Level for CCU8x.OUTy2"]
pub type Psl21R = crate::BitReader<Psl21>;
impl Psl21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Psl21 {
        match self.bits {
            false => Psl21::Value1,
            true => Psl21::Value2,
        }
    }
    #[doc = "Passive Level is LOW"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Psl21::Value1
    }
    #[doc = "Passive Level is HIGH"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Psl21::Value2
    }
}
#[doc = "Field `PSL21` writer - Output Passive Level for CCU8x.OUTy2"]
pub type Psl21W<'a, REG> = crate::BitWriter<'a, REG, Psl21>;
impl<'a, REG> Psl21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Passive Level is LOW"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Psl21::Value1)
    }
    #[doc = "Passive Level is HIGH"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Psl21::Value2)
    }
}
#[doc = "Output Passive Level for CCU8x.OUTy3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psl22 {
    #[doc = "0: Passive Level is LOW"]
    Value1 = 0,
    #[doc = "1: Passive Level is HIGH"]
    Value2 = 1,
}
impl From<Psl22> for bool {
    #[inline(always)]
    fn from(variant: Psl22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PSL22` reader - Output Passive Level for CCU8x.OUTy3"]
pub type Psl22R = crate::BitReader<Psl22>;
impl Psl22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Psl22 {
        match self.bits {
            false => Psl22::Value1,
            true => Psl22::Value2,
        }
    }
    #[doc = "Passive Level is LOW"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Psl22::Value1
    }
    #[doc = "Passive Level is HIGH"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Psl22::Value2
    }
}
#[doc = "Field `PSL22` writer - Output Passive Level for CCU8x.OUTy3"]
pub type Psl22W<'a, REG> = crate::BitWriter<'a, REG, Psl22>;
impl<'a, REG> Psl22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Passive Level is LOW"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Psl22::Value1)
    }
    #[doc = "Passive Level is HIGH"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Psl22::Value2)
    }
}
impl R {
    #[doc = "Bit 0 - Output Passive Level for CCU8x.OUTy0"]
    #[inline(always)]
    pub fn psl11(&self) -> Psl11R {
        Psl11R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Output Passive Level for CCU8x.OUTy1"]
    #[inline(always)]
    pub fn psl12(&self) -> Psl12R {
        Psl12R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Output Passive Level for CCU8x.OUTy2"]
    #[inline(always)]
    pub fn psl21(&self) -> Psl21R {
        Psl21R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Output Passive Level for CCU8x.OUTy3"]
    #[inline(always)]
    pub fn psl22(&self) -> Psl22R {
        Psl22R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Output Passive Level for CCU8x.OUTy0"]
    #[inline(always)]
    #[must_use]
    pub fn psl11(&mut self) -> Psl11W<PslSpec> {
        Psl11W::new(self, 0)
    }
    #[doc = "Bit 1 - Output Passive Level for CCU8x.OUTy1"]
    #[inline(always)]
    #[must_use]
    pub fn psl12(&mut self) -> Psl12W<PslSpec> {
        Psl12W::new(self, 1)
    }
    #[doc = "Bit 2 - Output Passive Level for CCU8x.OUTy2"]
    #[inline(always)]
    #[must_use]
    pub fn psl21(&mut self) -> Psl21W<PslSpec> {
        Psl21W::new(self, 2)
    }
    #[doc = "Bit 3 - Output Passive Level for CCU8x.OUTy3"]
    #[inline(always)]
    #[must_use]
    pub fn psl22(&mut self) -> Psl22W<PslSpec> {
        Psl22W::new(self, 3)
    }
}
#[doc = "Passive Level Config\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PslSpec;
impl crate::RegisterSpec for PslSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psl::R`](R) reader structure"]
impl crate::Readable for PslSpec {}
#[doc = "`write(|w| ..)` method takes [`psl::W`](W) writer structure"]
impl crate::Writable for PslSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSL to value 0"]
impl crate::Resettable for PslSpec {
    const RESET_VALUE: u32 = 0;
}

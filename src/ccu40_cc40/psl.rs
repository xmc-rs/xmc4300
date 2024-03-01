#[doc = "Register `PSL` reader"]
pub type R = crate::R<PslSpec>;
#[doc = "Register `PSL` writer"]
pub type W = crate::W<PslSpec>;
#[doc = "Output Passive Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psl {
    #[doc = "0: Passive Level is LOW"]
    Value1 = 0,
    #[doc = "1: Passive Level is HIGH"]
    Value2 = 1,
}
impl From<Psl> for bool {
    #[inline(always)]
    fn from(variant: Psl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PSL` reader - Output Passive Level"]
pub type PslR = crate::BitReader<Psl>;
impl PslR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Psl {
        match self.bits {
            false => Psl::Value1,
            true => Psl::Value2,
        }
    }
    #[doc = "Passive Level is LOW"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Psl::Value1
    }
    #[doc = "Passive Level is HIGH"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Psl::Value2
    }
}
#[doc = "Field `PSL` writer - Output Passive Level"]
pub type PslW<'a, REG> = crate::BitWriter<'a, REG, Psl>;
impl<'a, REG> PslW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Passive Level is LOW"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Psl::Value1)
    }
    #[doc = "Passive Level is HIGH"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Psl::Value2)
    }
}
impl R {
    #[doc = "Bit 0 - Output Passive Level"]
    #[inline(always)]
    pub fn psl(&self) -> PslR {
        PslR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Output Passive Level"]
    #[inline(always)]
    #[must_use]
    pub fn psl(&mut self) -> PslW<PslSpec> {
        PslW::new(self, 0)
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

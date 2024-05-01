#[doc = "Register `STS` reader"]
pub type R = crate::R<StsSpec>;
#[doc = "Register `STS` writer"]
pub type W = crate::W<StsSpec>;
#[doc = "Bufferable Write Access Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Werr {
    #[doc = "0: no write error occurred."]
    Value1 = 0,
    #[doc = "1: write error occurred, interrupt request is pending."]
    Value2 = 1,
}
impl From<Werr> for bool {
    #[inline(always)]
    fn from(variant: Werr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WERR` reader - Bufferable Write Access Error"]
pub type WerrR = crate::BitReader<Werr>;
impl WerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Werr {
        match self.bits {
            false => Werr::Value1,
            true => Werr::Value2,
        }
    }
    #[doc = "no write error occurred."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Werr::Value1
    }
    #[doc = "write error occurred, interrupt request is pending."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Werr::Value2
    }
}
#[doc = "Field `WERR` writer - Bufferable Write Access Error"]
pub type WerrW<'a, REG> = crate::BitWriter<'a, REG, Werr>;
impl<'a, REG> WerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no write error occurred."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Werr::Value1)
    }
    #[doc = "write error occurred, interrupt request is pending."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Werr::Value2)
    }
}
impl R {
    #[doc = "Bit 0 - Bufferable Write Access Error"]
    #[inline(always)]
    pub fn werr(&self) -> WerrR {
        WerrR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bufferable Write Access Error"]
    #[inline(always)]
    #[must_use]
    pub fn werr(&mut self) -> WerrW<StsSpec> {
        WerrW::new(self, 0)
    }
}
#[doc = "Peripheral Bridge Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StsSpec;
impl crate::RegisterSpec for StsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sts::R`](R) reader structure"]
impl crate::Readable for StsSpec {}
#[doc = "`write(|w| ..)` method takes [`sts::W`](W) writer structure"]
impl crate::Writable for StsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STS to value 0"]
impl crate::Resettable for StsSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `SDMMCDEL` reader"]
pub type R = crate::R<SdmmcdelSpec>;
#[doc = "Register `SDMMCDEL` writer"]
pub type W = crate::W<SdmmcdelSpec>;
#[doc = "Enable delay on the CMD/DAT out lines\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tapen {
    #[doc = "0: Disabled"]
    Value1 = 0,
    #[doc = "1: Enabled"]
    Value2 = 1,
}
impl From<Tapen> for bool {
    #[inline(always)]
    fn from(variant: Tapen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAPEN` reader - Enable delay on the CMD/DAT out lines"]
pub type TapenR = crate::BitReader<Tapen>;
impl TapenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tapen {
        match self.bits {
            false => Tapen::Value1,
            true => Tapen::Value2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Tapen::Value1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Tapen::Value2
    }
}
#[doc = "Field `TAPEN` writer - Enable delay on the CMD/DAT out lines"]
pub type TapenW<'a, REG> = crate::BitWriter<'a, REG, Tapen>;
impl<'a, REG> TapenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Tapen::Value1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Tapen::Value2)
    }
}
#[doc = "Field `TAPDEL` reader - Number of Delay Elements Select"]
pub type TapdelR = crate::FieldReader;
#[doc = "Field `TAPDEL` writer - Number of Delay Elements Select"]
pub type TapdelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Enable delay on the CMD/DAT out lines"]
    #[inline(always)]
    pub fn tapen(&self) -> TapenR {
        TapenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:7 - Number of Delay Elements Select"]
    #[inline(always)]
    pub fn tapdel(&self) -> TapdelR {
        TapdelR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable delay on the CMD/DAT out lines"]
    #[inline(always)]
    #[must_use]
    pub fn tapen(&mut self) -> TapenW<SdmmcdelSpec> {
        TapenW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Number of Delay Elements Select"]
    #[inline(always)]
    #[must_use]
    pub fn tapdel(&mut self) -> TapdelW<SdmmcdelSpec> {
        TapdelW::new(self, 4)
    }
}
#[doc = "SD-MMC Delay Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmcdel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmcdel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdmmcdelSpec;
impl crate::RegisterSpec for SdmmcdelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdmmcdel::R`](R) reader structure"]
impl crate::Readable for SdmmcdelSpec {}
#[doc = "`write(|w| ..)` method takes [`sdmmcdel::W`](W) writer structure"]
impl crate::Writable for SdmmcdelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDMMCDEL to value 0"]
impl crate::Resettable for SdmmcdelSpec {
    const RESET_VALUE: u32 = 0;
}

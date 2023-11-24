#[doc = "Register `OCS` reader"]
pub type R = crate::R<OCS_SPEC>;
#[doc = "Register `OCS` writer"]
pub type W = crate::W<OCS_SPEC>;
#[doc = "Field `TGS` reader - Trigger Set for OTGB0/1"]
pub type TGS_R = crate::FieldReader<TGS_A>;
#[doc = "Trigger Set for OTGB0/1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TGS_A {
    #[doc = "0: No Trigger Set output"]
    VALUE1 = 0,
    #[doc = "1: Trigger Set 1: TS16_SSIG, input sample signals"]
    VALUE2 = 1,
}
impl From<TGS_A> for u8 {
    #[inline(always)]
    fn from(variant: TGS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TGS_A {
    type Ux = u8;
}
impl TGS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TGS_A> {
        match self.bits {
            0 => Some(TGS_A::VALUE1),
            1 => Some(TGS_A::VALUE2),
            _ => None,
        }
    }
    #[doc = "No Trigger Set output"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TGS_A::VALUE1
    }
    #[doc = "Trigger Set 1: TS16_SSIG, input sample signals"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TGS_A::VALUE2
    }
}
#[doc = "Field `TGS` writer - Trigger Set for OTGB0/1"]
pub type TGS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, TGS_A>;
impl<'a, REG> TGS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Trigger Set output"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(TGS_A::VALUE1)
    }
    #[doc = "Trigger Set 1: TS16_SSIG, input sample signals"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(TGS_A::VALUE2)
    }
}
#[doc = "Field `TGB` reader - OTGB0/1 Bus Select"]
pub type TGB_R = crate::BitReader<TGB_A>;
#[doc = "OTGB0/1 Bus Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TGB_A {
    #[doc = "0: Trigger Set is output on OTGB0"]
    VALUE1 = 0,
    #[doc = "1: Trigger Set is output on OTGB1"]
    VALUE2 = 1,
}
impl From<TGB_A> for bool {
    #[inline(always)]
    fn from(variant: TGB_A) -> Self {
        variant as u8 != 0
    }
}
impl TGB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TGB_A {
        match self.bits {
            false => TGB_A::VALUE1,
            true => TGB_A::VALUE2,
        }
    }
    #[doc = "Trigger Set is output on OTGB0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TGB_A::VALUE1
    }
    #[doc = "Trigger Set is output on OTGB1"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TGB_A::VALUE2
    }
}
#[doc = "Field `TGB` writer - OTGB0/1 Bus Select"]
pub type TGB_W<'a, REG> = crate::BitWriter<'a, REG, TGB_A>;
impl<'a, REG> TGB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger Set is output on OTGB0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(TGB_A::VALUE1)
    }
    #[doc = "Trigger Set is output on OTGB1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(TGB_A::VALUE2)
    }
}
#[doc = "Field `TG_P` writer - TGS, TGB Write Protection"]
pub type TG_P_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUS` reader - OCDS Suspend Control"]
pub type SUS_R = crate::FieldReader<SUS_A>;
#[doc = "OCDS Suspend Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SUS_A {
    #[doc = "0: Will not suspend"]
    VALUE1 = 0,
    #[doc = "1: Hard suspend: Clock is switched off immediately."]
    VALUE2 = 1,
    #[doc = "2: Soft suspend mode 0: Stop conversions after the currently running one is completed and its result has been stored. No change for the arbiter."]
    VALUE3 = 2,
    #[doc = "3: Soft suspend mode 1: Stop conversions after the currently running one is completed and its result has been stored. Stop arbiter after the current arbitration round."]
    VALUE4 = 3,
}
impl From<SUS_A> for u8 {
    #[inline(always)]
    fn from(variant: SUS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SUS_A {
    type Ux = u8;
}
impl SUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SUS_A> {
        match self.bits {
            0 => Some(SUS_A::VALUE1),
            1 => Some(SUS_A::VALUE2),
            2 => Some(SUS_A::VALUE3),
            3 => Some(SUS_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Will not suspend"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SUS_A::VALUE1
    }
    #[doc = "Hard suspend: Clock is switched off immediately."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SUS_A::VALUE2
    }
    #[doc = "Soft suspend mode 0: Stop conversions after the currently running one is completed and its result has been stored. No change for the arbiter."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == SUS_A::VALUE3
    }
    #[doc = "Soft suspend mode 1: Stop conversions after the currently running one is completed and its result has been stored. Stop arbiter after the current arbitration round."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == SUS_A::VALUE4
    }
}
#[doc = "Field `SUS` writer - OCDS Suspend Control"]
pub type SUS_W<'a, REG> = crate::FieldWriter<'a, REG, 4, SUS_A>;
impl<'a, REG> SUS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Will not suspend"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SUS_A::VALUE1)
    }
    #[doc = "Hard suspend: Clock is switched off immediately."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SUS_A::VALUE2)
    }
    #[doc = "Soft suspend mode 0: Stop conversions after the currently running one is completed and its result has been stored. No change for the arbiter."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(SUS_A::VALUE3)
    }
    #[doc = "Soft suspend mode 1: Stop conversions after the currently running one is completed and its result has been stored. Stop arbiter after the current arbitration round."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(SUS_A::VALUE4)
    }
}
#[doc = "Field `SUS_P` writer - SUS Write Protection"]
pub type SUS_P_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUSSTA` reader - Suspend State"]
pub type SUSSTA_R = crate::BitReader<SUSSTA_A>;
#[doc = "Suspend State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUSSTA_A {
    #[doc = "0: Module is not (yet) suspended"]
    VALUE1 = 0,
    #[doc = "1: Module is suspended"]
    VALUE2 = 1,
}
impl From<SUSSTA_A> for bool {
    #[inline(always)]
    fn from(variant: SUSSTA_A) -> Self {
        variant as u8 != 0
    }
}
impl SUSSTA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SUSSTA_A {
        match self.bits {
            false => SUSSTA_A::VALUE1,
            true => SUSSTA_A::VALUE2,
        }
    }
    #[doc = "Module is not (yet) suspended"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SUSSTA_A::VALUE1
    }
    #[doc = "Module is suspended"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SUSSTA_A::VALUE2
    }
}
impl R {
    #[doc = "Bits 0:1 - Trigger Set for OTGB0/1"]
    #[inline(always)]
    pub fn tgs(&self) -> TGS_R {
        TGS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - OTGB0/1 Bus Select"]
    #[inline(always)]
    pub fn tgb(&self) -> TGB_R {
        TGB_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 24:27 - OCDS Suspend Control"]
    #[inline(always)]
    pub fn sus(&self) -> SUS_R {
        SUS_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 29 - Suspend State"]
    #[inline(always)]
    pub fn sussta(&self) -> SUSSTA_R {
        SUSSTA_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Trigger Set for OTGB0/1"]
    #[inline(always)]
    #[must_use]
    pub fn tgs(&mut self) -> TGS_W<OCS_SPEC> {
        TGS_W::new(self, 0)
    }
    #[doc = "Bit 2 - OTGB0/1 Bus Select"]
    #[inline(always)]
    #[must_use]
    pub fn tgb(&mut self) -> TGB_W<OCS_SPEC> {
        TGB_W::new(self, 2)
    }
    #[doc = "Bit 3 - TGS, TGB Write Protection"]
    #[inline(always)]
    #[must_use]
    pub fn tg_p(&mut self) -> TG_P_W<OCS_SPEC> {
        TG_P_W::new(self, 3)
    }
    #[doc = "Bits 24:27 - OCDS Suspend Control"]
    #[inline(always)]
    #[must_use]
    pub fn sus(&mut self) -> SUS_W<OCS_SPEC> {
        SUS_W::new(self, 24)
    }
    #[doc = "Bit 28 - SUS Write Protection"]
    #[inline(always)]
    #[must_use]
    pub fn sus_p(&mut self) -> SUS_P_W<OCS_SPEC> {
        SUS_P_W::new(self, 28)
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
#[doc = "OCDS Control and Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OCS_SPEC;
impl crate::RegisterSpec for OCS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ocs::R`](R) reader structure"]
impl crate::Readable for OCS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ocs::W`](W) writer structure"]
impl crate::Writable for OCS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OCS to value 0"]
impl crate::Resettable for OCS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

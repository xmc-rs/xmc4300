#[doc = "Register `DC_ACT` reader"]
pub type R = crate::R<DcActSpec>;
#[doc = "Register `DC_ACT` writer"]
pub type W = crate::W<DcActSpec>;
#[doc = "Sync Out Unit activation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SyncOut {
    #[doc = "0: Deactivated"]
    Value1 = 0,
    #[doc = "1: Activated"]
    Value2 = 1,
}
impl From<SyncOut> for bool {
    #[inline(always)]
    fn from(variant: SyncOut) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYNC_OUT` reader - Sync Out Unit activation"]
pub type SyncOutR = crate::BitReader<SyncOut>;
impl SyncOutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SyncOut {
        match self.bits {
            false => SyncOut::Value1,
            true => SyncOut::Value2,
        }
    }
    #[doc = "Deactivated"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SyncOut::Value1
    }
    #[doc = "Activated"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SyncOut::Value2
    }
}
#[doc = "Field `SYNC_OUT` writer - Sync Out Unit activation"]
pub type SyncOutW<'a, REG> = crate::BitWriter<'a, REG, SyncOut>;
impl<'a, REG> SyncOutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Deactivated"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SyncOut::Value1)
    }
    #[doc = "Activated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SyncOut::Value2)
    }
}
#[doc = "SYNC0 generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sync0 {
    #[doc = "0: Deactivated"]
    Value1 = 0,
    #[doc = "1: SYNC0 pulse is generated"]
    Value2 = 1,
}
impl From<Sync0> for bool {
    #[inline(always)]
    fn from(variant: Sync0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYNC_0` reader - SYNC0 generation"]
pub type Sync0R = crate::BitReader<Sync0>;
impl Sync0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sync0 {
        match self.bits {
            false => Sync0::Value1,
            true => Sync0::Value2,
        }
    }
    #[doc = "Deactivated"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Sync0::Value1
    }
    #[doc = "SYNC0 pulse is generated"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Sync0::Value2
    }
}
#[doc = "Field `SYNC_0` writer - SYNC0 generation"]
pub type Sync0W<'a, REG> = crate::BitWriter<'a, REG, Sync0>;
impl<'a, REG> Sync0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Deactivated"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Sync0::Value1)
    }
    #[doc = "SYNC0 pulse is generated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Sync0::Value2)
    }
}
#[doc = "SYNC1 generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sync1 {
    #[doc = "0: Deactivated"]
    Value1 = 0,
    #[doc = "1: SYNC1 pulse is generated"]
    Value2 = 1,
}
impl From<Sync1> for bool {
    #[inline(always)]
    fn from(variant: Sync1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYNC_1` reader - SYNC1 generation"]
pub type Sync1R = crate::BitReader<Sync1>;
impl Sync1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sync1 {
        match self.bits {
            false => Sync1::Value1,
            true => Sync1::Value2,
        }
    }
    #[doc = "Deactivated"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Sync1::Value1
    }
    #[doc = "SYNC1 pulse is generated"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Sync1::Value2
    }
}
#[doc = "Field `SYNC_1` writer - SYNC1 generation"]
pub type Sync1W<'a, REG> = crate::BitWriter<'a, REG, Sync1>;
impl<'a, REG> Sync1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Deactivated"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Sync1::Value1)
    }
    #[doc = "SYNC1 pulse is generated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Sync1::Value2)
    }
}
impl R {
    #[doc = "Bit 0 - Sync Out Unit activation"]
    #[inline(always)]
    pub fn sync_out(&self) -> SyncOutR {
        SyncOutR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SYNC0 generation"]
    #[inline(always)]
    pub fn sync_0(&self) -> Sync0R {
        Sync0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SYNC1 generation"]
    #[inline(always)]
    pub fn sync_1(&self) -> Sync1R {
        Sync1R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sync Out Unit activation"]
    #[inline(always)]
    #[must_use]
    pub fn sync_out(&mut self) -> SyncOutW<DcActSpec> {
        SyncOutW::new(self, 0)
    }
    #[doc = "Bit 1 - SYNC0 generation"]
    #[inline(always)]
    #[must_use]
    pub fn sync_0(&mut self) -> Sync0W<DcActSpec> {
        Sync0W::new(self, 1)
    }
    #[doc = "Bit 2 - SYNC1 generation"]
    #[inline(always)]
    #[must_use]
    pub fn sync_1(&mut self) -> Sync1W<DcActSpec> {
        Sync1W::new(self, 2)
    }
}
#[doc = "Activation register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc_act::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dc_act::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcActSpec;
impl crate::RegisterSpec for DcActSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dc_act::R`](R) reader structure"]
impl crate::Readable for DcActSpec {}
#[doc = "`write(|w| ..)` method takes [`dc_act::W`](W) writer structure"]
impl crate::Writable for DcActSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DC_ACT to value 0"]
impl crate::Resettable for DcActSpec {
    const RESET_VALUE: u8 = 0;
}

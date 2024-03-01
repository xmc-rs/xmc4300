#[doc = "Register `HFIR` reader"]
pub type R = crate::R<HfirSpec>;
#[doc = "Register `HFIR` writer"]
pub type W = crate::W<HfirSpec>;
#[doc = "Field `FrInt` reader - Frame Interval"]
pub type FrIntR = crate::FieldReader<u16>;
#[doc = "Field `FrInt` writer - Frame Interval"]
pub type FrIntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Reload Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HfirrldCtrl {
    #[doc = "0: HFIR cannot be reloaded dynamically"]
    Value1 = 0,
    #[doc = "1: HFIR can be dynamically reloaded during runtime"]
    Value2 = 1,
}
impl From<HfirrldCtrl> for bool {
    #[inline(always)]
    fn from(variant: HfirrldCtrl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HFIRRldCtrl` reader - Reload Control"]
pub type HfirrldCtrlR = crate::BitReader<HfirrldCtrl>;
impl HfirrldCtrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HfirrldCtrl {
        match self.bits {
            false => HfirrldCtrl::Value1,
            true => HfirrldCtrl::Value2,
        }
    }
    #[doc = "HFIR cannot be reloaded dynamically"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HfirrldCtrl::Value1
    }
    #[doc = "HFIR can be dynamically reloaded during runtime"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HfirrldCtrl::Value2
    }
}
#[doc = "Field `HFIRRldCtrl` writer - Reload Control"]
pub type HfirrldCtrlW<'a, REG> = crate::BitWriter<'a, REG, HfirrldCtrl>;
impl<'a, REG> HfirrldCtrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HFIR cannot be reloaded dynamically"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(HfirrldCtrl::Value1)
    }
    #[doc = "HFIR can be dynamically reloaded during runtime"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(HfirrldCtrl::Value2)
    }
}
impl R {
    #[doc = "Bits 0:15 - Frame Interval"]
    #[inline(always)]
    pub fn fr_int(&self) -> FrIntR {
        FrIntR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Reload Control"]
    #[inline(always)]
    pub fn hfirrld_ctrl(&self) -> HfirrldCtrlR {
        HfirrldCtrlR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Frame Interval"]
    #[inline(always)]
    #[must_use]
    pub fn fr_int(&mut self) -> FrIntW<HfirSpec> {
        FrIntW::new(self, 0)
    }
    #[doc = "Bit 16 - Reload Control"]
    #[inline(always)]
    #[must_use]
    pub fn hfirrld_ctrl(&mut self) -> HfirrldCtrlW<HfirSpec> {
        HfirrldCtrlW::new(self, 16)
    }
}
#[doc = "Host Frame Interval Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HfirSpec;
impl crate::RegisterSpec for HfirSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfir::R`](R) reader structure"]
impl crate::Readable for HfirSpec {}
#[doc = "`write(|w| ..)` method takes [`hfir::W`](W) writer structure"]
impl crate::Writable for HfirSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HFIR to value 0xea60"]
impl crate::Resettable for HfirSpec {
    const RESET_VALUE: u32 = 0xea60;
}

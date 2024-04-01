#[doc = "Register `BRSCTRL` reader"]
pub type R = crate::R<BrsctrlSpec>;
#[doc = "Register `BRSCTRL` writer"]
pub type W = crate::W<BrsctrlSpec>;
#[doc = "Source-specific Result Register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Srcresreg {
    #[doc = "0: Use GxCHCTRy.RESREG to select a group result register"]
    Value1 = 0,
    #[doc = "1: Store result in group result register GxRES1"]
    Value2 = 1,
    #[doc = "15: Store result in group result register GxRES15"]
    Value3 = 15,
}
impl From<Srcresreg> for u8 {
    #[inline(always)]
    fn from(variant: Srcresreg) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Srcresreg {
    type Ux = u8;
}
impl crate::IsEnum for Srcresreg {}
#[doc = "Field `SRCRESREG` reader - Source-specific Result Register"]
pub type SrcresregR = crate::FieldReader<Srcresreg>;
impl SrcresregR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Srcresreg> {
        match self.bits {
            0 => Some(Srcresreg::Value1),
            1 => Some(Srcresreg::Value2),
            15 => Some(Srcresreg::Value3),
            _ => None,
        }
    }
    #[doc = "Use GxCHCTRy.RESREG to select a group result register"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Srcresreg::Value1
    }
    #[doc = "Store result in group result register GxRES1"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Srcresreg::Value2
    }
    #[doc = "Store result in group result register GxRES15"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Srcresreg::Value3
    }
}
#[doc = "Field `SRCRESREG` writer - Source-specific Result Register"]
pub type SrcresregW<'a, REG> = crate::FieldWriter<'a, REG, 4, Srcresreg>;
impl<'a, REG> SrcresregW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Use GxCHCTRy.RESREG to select a group result register"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Srcresreg::Value1)
    }
    #[doc = "Store result in group result register GxRES1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Srcresreg::Value2)
    }
    #[doc = "Store result in group result register GxRES15"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Srcresreg::Value3)
    }
}
#[doc = "Field `XTSEL` reader - External Trigger Input Selection"]
pub type XtselR = crate::FieldReader;
#[doc = "Field `XTSEL` writer - External Trigger Input Selection"]
pub type XtselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `XTLVL` reader - External Trigger Level"]
pub type XtlvlR = crate::BitReader;
#[doc = "Trigger Operating Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Xtmode {
    #[doc = "0: No external trigger"]
    Value1 = 0,
    #[doc = "1: Trigger event upon a falling edge"]
    Value2 = 1,
    #[doc = "2: Trigger event upon a rising edge"]
    Value3 = 2,
    #[doc = "3: Trigger event upon any edge"]
    Value4 = 3,
}
impl From<Xtmode> for u8 {
    #[inline(always)]
    fn from(variant: Xtmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Xtmode {
    type Ux = u8;
}
impl crate::IsEnum for Xtmode {}
#[doc = "Field `XTMODE` reader - Trigger Operating Mode"]
pub type XtmodeR = crate::FieldReader<Xtmode>;
impl XtmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Xtmode {
        match self.bits {
            0 => Xtmode::Value1,
            1 => Xtmode::Value2,
            2 => Xtmode::Value3,
            3 => Xtmode::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "No external trigger"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Xtmode::Value1
    }
    #[doc = "Trigger event upon a falling edge"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Xtmode::Value2
    }
    #[doc = "Trigger event upon a rising edge"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Xtmode::Value3
    }
    #[doc = "Trigger event upon any edge"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Xtmode::Value4
    }
}
#[doc = "Field `XTMODE` writer - Trigger Operating Mode"]
pub type XtmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Xtmode, crate::Safe>;
impl<'a, REG> XtmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No external trigger"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Xtmode::Value1)
    }
    #[doc = "Trigger event upon a falling edge"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Xtmode::Value2)
    }
    #[doc = "Trigger event upon a rising edge"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Xtmode::Value3)
    }
    #[doc = "Trigger event upon any edge"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Xtmode::Value4)
    }
}
#[doc = "Write Control for Trigger Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Xtwc {
    #[doc = "0: No write access to trigger configuration"]
    Value1 = 0,
    #[doc = "1: Bitfields XTMODE and XTSEL can be written"]
    Value2 = 1,
}
impl From<Xtwc> for bool {
    #[inline(always)]
    fn from(variant: Xtwc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XTWC` writer - Write Control for Trigger Configuration"]
pub type XtwcW<'a, REG> = crate::BitWriter<'a, REG, Xtwc>;
impl<'a, REG> XtwcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No write access to trigger configuration"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Xtwc::Value1)
    }
    #[doc = "Bitfields XTMODE and XTSEL can be written"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Xtwc::Value2)
    }
}
#[doc = "Field `GTSEL` reader - Gate Input Selection"]
pub type GtselR = crate::FieldReader;
#[doc = "Field `GTSEL` writer - Gate Input Selection"]
pub type GtselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `GTLVL` reader - Gate Input Level"]
pub type GtlvlR = crate::BitReader;
#[doc = "Write Control for Gate Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gtwc {
    #[doc = "0: No write access to gate configuration"]
    Value1 = 0,
    #[doc = "1: Bitfield GTSEL can be written"]
    Value2 = 1,
}
impl From<Gtwc> for bool {
    #[inline(always)]
    fn from(variant: Gtwc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GTWC` writer - Write Control for Gate Configuration"]
pub type GtwcW<'a, REG> = crate::BitWriter<'a, REG, Gtwc>;
impl<'a, REG> GtwcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No write access to gate configuration"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Gtwc::Value1)
    }
    #[doc = "Bitfield GTSEL can be written"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Gtwc::Value2)
    }
}
impl R {
    #[doc = "Bits 0:3 - Source-specific Result Register"]
    #[inline(always)]
    pub fn srcresreg(&self) -> SrcresregR {
        SrcresregR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - External Trigger Input Selection"]
    #[inline(always)]
    pub fn xtsel(&self) -> XtselR {
        XtselR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - External Trigger Level"]
    #[inline(always)]
    pub fn xtlvl(&self) -> XtlvlR {
        XtlvlR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - Trigger Operating Mode"]
    #[inline(always)]
    pub fn xtmode(&self) -> XtmodeR {
        XtmodeR::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bits 16:19 - Gate Input Selection"]
    #[inline(always)]
    pub fn gtsel(&self) -> GtselR {
        GtselR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Gate Input Level"]
    #[inline(always)]
    pub fn gtlvl(&self) -> GtlvlR {
        GtlvlR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Source-specific Result Register"]
    #[inline(always)]
    #[must_use]
    pub fn srcresreg(&mut self) -> SrcresregW<BrsctrlSpec> {
        SrcresregW::new(self, 0)
    }
    #[doc = "Bits 8:11 - External Trigger Input Selection"]
    #[inline(always)]
    #[must_use]
    pub fn xtsel(&mut self) -> XtselW<BrsctrlSpec> {
        XtselW::new(self, 8)
    }
    #[doc = "Bits 13:14 - Trigger Operating Mode"]
    #[inline(always)]
    #[must_use]
    pub fn xtmode(&mut self) -> XtmodeW<BrsctrlSpec> {
        XtmodeW::new(self, 13)
    }
    #[doc = "Bit 15 - Write Control for Trigger Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn xtwc(&mut self) -> XtwcW<BrsctrlSpec> {
        XtwcW::new(self, 15)
    }
    #[doc = "Bits 16:19 - Gate Input Selection"]
    #[inline(always)]
    #[must_use]
    pub fn gtsel(&mut self) -> GtselW<BrsctrlSpec> {
        GtselW::new(self, 16)
    }
    #[doc = "Bit 23 - Write Control for Gate Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn gtwc(&mut self) -> GtwcW<BrsctrlSpec> {
        GtwcW::new(self, 23)
    }
}
#[doc = "Background Request Source Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`brsctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`brsctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BrsctrlSpec;
impl crate::RegisterSpec for BrsctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`brsctrl::R`](R) reader structure"]
impl crate::Readable for BrsctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`brsctrl::W`](W) writer structure"]
impl crate::Writable for BrsctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BRSCTRL to value 0"]
impl crate::Resettable for BrsctrlSpec {
    const RESET_VALUE: u32 = 0;
}

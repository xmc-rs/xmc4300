#[doc = "Register `ASCTRL` reader"]
pub struct R(crate::R<ASCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ASCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ASCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ASCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ASCTRL` writer"]
pub struct W(crate::W<ASCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ASCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<ASCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ASCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRCRESREG` reader - Source-specific Result Register"]
pub type SRCRESREG_R = crate::FieldReader<u8, SRCRESREG_A>;
#[doc = "Source-specific Result Register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SRCRESREG_A {
    #[doc = "0: Use GxCHCTRy.RESREG to select a group result register"]
    VALUE1 = 0,
    #[doc = "1: Store result in group result register GxRES1"]
    VALUE2 = 1,
    #[doc = "15: Store result in group result register GxRES15"]
    VALUE3 = 15,
}
impl From<SRCRESREG_A> for u8 {
    #[inline(always)]
    fn from(variant: SRCRESREG_A) -> Self {
        variant as _
    }
}
impl SRCRESREG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SRCRESREG_A> {
        match self.bits {
            0 => Some(SRCRESREG_A::VALUE1),
            1 => Some(SRCRESREG_A::VALUE2),
            15 => Some(SRCRESREG_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SRCRESREG_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SRCRESREG_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == SRCRESREG_A::VALUE3
    }
}
#[doc = "Field `SRCRESREG` writer - Source-specific Result Register"]
pub type SRCRESREG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ASCTRL_SPEC, u8, SRCRESREG_A, 4, O>;
impl<'a, const O: u8> SRCRESREG_W<'a, O> {
    #[doc = "Use GxCHCTRy.RESREG to select a group result register"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SRCRESREG_A::VALUE1)
    }
    #[doc = "Store result in group result register GxRES1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SRCRESREG_A::VALUE2)
    }
    #[doc = "Store result in group result register GxRES15"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(SRCRESREG_A::VALUE3)
    }
}
#[doc = "Field `XTSEL` reader - External Trigger Input Selection"]
pub type XTSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `XTSEL` writer - External Trigger Input Selection"]
pub type XTSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ASCTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `XTLVL` reader - External Trigger Level"]
pub type XTLVL_R = crate::BitReader<bool>;
#[doc = "Field `XTMODE` reader - Trigger Operating Mode"]
pub type XTMODE_R = crate::FieldReader<u8, XTMODE_A>;
#[doc = "Trigger Operating Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum XTMODE_A {
    #[doc = "0: No external trigger"]
    VALUE1 = 0,
    #[doc = "1: Trigger event upon a falling edge"]
    VALUE2 = 1,
    #[doc = "2: Trigger event upon a rising edge"]
    VALUE3 = 2,
    #[doc = "3: Trigger event upon any edge"]
    VALUE4 = 3,
}
impl From<XTMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: XTMODE_A) -> Self {
        variant as _
    }
}
impl XTMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XTMODE_A {
        match self.bits {
            0 => XTMODE_A::VALUE1,
            1 => XTMODE_A::VALUE2,
            2 => XTMODE_A::VALUE3,
            3 => XTMODE_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == XTMODE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == XTMODE_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == XTMODE_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == XTMODE_A::VALUE4
    }
}
#[doc = "Field `XTMODE` writer - Trigger Operating Mode"]
pub type XTMODE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, ASCTRL_SPEC, u8, XTMODE_A, 2, O>;
impl<'a, const O: u8> XTMODE_W<'a, O> {
    #[doc = "No external trigger"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(XTMODE_A::VALUE1)
    }
    #[doc = "Trigger event upon a falling edge"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(XTMODE_A::VALUE2)
    }
    #[doc = "Trigger event upon a rising edge"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(XTMODE_A::VALUE3)
    }
    #[doc = "Trigger event upon any edge"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(XTMODE_A::VALUE4)
    }
}
#[doc = "Write Control for Trigger Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XTWC_AW {
    #[doc = "0: No write access to trigger configuration"]
    VALUE1 = 0,
    #[doc = "1: Bitfields XTMODE and XTSEL can be written"]
    VALUE2 = 1,
}
impl From<XTWC_AW> for bool {
    #[inline(always)]
    fn from(variant: XTWC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XTWC` writer - Write Control for Trigger Configuration"]
pub type XTWC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASCTRL_SPEC, XTWC_AW, O>;
impl<'a, const O: u8> XTWC_W<'a, O> {
    #[doc = "No write access to trigger configuration"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(XTWC_AW::VALUE1)
    }
    #[doc = "Bitfields XTMODE and XTSEL can be written"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(XTWC_AW::VALUE2)
    }
}
#[doc = "Field `GTSEL` reader - Gate Input Selection"]
pub type GTSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GTSEL` writer - Gate Input Selection"]
pub type GTSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ASCTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `GTLVL` reader - Gate Input Level"]
pub type GTLVL_R = crate::BitReader<bool>;
#[doc = "Write Control for Gate Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GTWC_AW {
    #[doc = "0: No write access to gate configuration"]
    VALUE1 = 0,
    #[doc = "1: Bitfield GTSEL can be written"]
    VALUE2 = 1,
}
impl From<GTWC_AW> for bool {
    #[inline(always)]
    fn from(variant: GTWC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GTWC` writer - Write Control for Gate Configuration"]
pub type GTWC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASCTRL_SPEC, GTWC_AW, O>;
impl<'a, const O: u8> GTWC_W<'a, O> {
    #[doc = "No write access to gate configuration"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(GTWC_AW::VALUE1)
    }
    #[doc = "Bitfield GTSEL can be written"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(GTWC_AW::VALUE2)
    }
}
#[doc = "Field `TMEN` reader - Timer Mode Enable"]
pub type TMEN_R = crate::BitReader<TMEN_A>;
#[doc = "Timer Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TMEN_A {
    #[doc = "0: No timer mode: standard gating mechanism can be used"]
    VALUE1 = 0,
    #[doc = "1: Timer mode for equidistant sampling enabled: standard gating mechanism must be disabled"]
    VALUE2 = 1,
}
impl From<TMEN_A> for bool {
    #[inline(always)]
    fn from(variant: TMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMEN_A {
        match self.bits {
            false => TMEN_A::VALUE1,
            true => TMEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TMEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TMEN_A::VALUE2
    }
}
#[doc = "Field `TMEN` writer - Timer Mode Enable"]
pub type TMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASCTRL_SPEC, TMEN_A, O>;
impl<'a, const O: u8> TMEN_W<'a, O> {
    #[doc = "No timer mode: standard gating mechanism can be used"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TMEN_A::VALUE1)
    }
    #[doc = "Timer mode for equidistant sampling enabled: standard gating mechanism must be disabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TMEN_A::VALUE2)
    }
}
#[doc = "Write Control for Timer Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TMWC_AW {
    #[doc = "0: No write access to timer mode"]
    VALUE1 = 0,
    #[doc = "1: Bitfield TMEN can be written"]
    VALUE2 = 1,
}
impl From<TMWC_AW> for bool {
    #[inline(always)]
    fn from(variant: TMWC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMWC` writer - Write Control for Timer Mode"]
pub type TMWC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASCTRL_SPEC, TMWC_AW, O>;
impl<'a, const O: u8> TMWC_W<'a, O> {
    #[doc = "No write access to timer mode"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TMWC_AW::VALUE1)
    }
    #[doc = "Bitfield TMEN can be written"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TMWC_AW::VALUE2)
    }
}
impl R {
    #[doc = "Bits 0:3 - Source-specific Result Register"]
    #[inline(always)]
    pub fn srcresreg(&self) -> SRCRESREG_R {
        SRCRESREG_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - External Trigger Input Selection"]
    #[inline(always)]
    pub fn xtsel(&self) -> XTSEL_R {
        XTSEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - External Trigger Level"]
    #[inline(always)]
    pub fn xtlvl(&self) -> XTLVL_R {
        XTLVL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - Trigger Operating Mode"]
    #[inline(always)]
    pub fn xtmode(&self) -> XTMODE_R {
        XTMODE_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bits 16:19 - Gate Input Selection"]
    #[inline(always)]
    pub fn gtsel(&self) -> GTSEL_R {
        GTSEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Gate Input Level"]
    #[inline(always)]
    pub fn gtlvl(&self) -> GTLVL_R {
        GTLVL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 28 - Timer Mode Enable"]
    #[inline(always)]
    pub fn tmen(&self) -> TMEN_R {
        TMEN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Source-specific Result Register"]
    #[inline(always)]
    #[must_use]
    pub fn srcresreg(&mut self) -> SRCRESREG_W<0> {
        SRCRESREG_W::new(self)
    }
    #[doc = "Bits 8:11 - External Trigger Input Selection"]
    #[inline(always)]
    #[must_use]
    pub fn xtsel(&mut self) -> XTSEL_W<8> {
        XTSEL_W::new(self)
    }
    #[doc = "Bits 13:14 - Trigger Operating Mode"]
    #[inline(always)]
    #[must_use]
    pub fn xtmode(&mut self) -> XTMODE_W<13> {
        XTMODE_W::new(self)
    }
    #[doc = "Bit 15 - Write Control for Trigger Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn xtwc(&mut self) -> XTWC_W<15> {
        XTWC_W::new(self)
    }
    #[doc = "Bits 16:19 - Gate Input Selection"]
    #[inline(always)]
    #[must_use]
    pub fn gtsel(&mut self) -> GTSEL_W<16> {
        GTSEL_W::new(self)
    }
    #[doc = "Bit 23 - Write Control for Gate Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn gtwc(&mut self) -> GTWC_W<23> {
        GTWC_W::new(self)
    }
    #[doc = "Bit 28 - Timer Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmen(&mut self) -> TMEN_W<28> {
        TMEN_W::new(self)
    }
    #[doc = "Bit 31 - Write Control for Timer Mode"]
    #[inline(always)]
    #[must_use]
    pub fn tmwc(&mut self) -> TMWC_W<31> {
        TMWC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Autoscan Source Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [asctrl](index.html) module"]
pub struct ASCTRL_SPEC;
impl crate::RegisterSpec for ASCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [asctrl::R](R) reader structure"]
impl crate::Readable for ASCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [asctrl::W](W) writer structure"]
impl crate::Writable for ASCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ASCTRL to value 0"]
impl crate::Resettable for ASCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

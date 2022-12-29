#[doc = "Register `PRSET1` writer"]
pub struct W(crate::W<PRSET1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRSET1_SPEC>;
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
impl From<crate::W<PRSET1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRSET1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "LEDTS Reset Assert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LEDTSCU0RS_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Assert reset"]
    CONST_1 = 1,
}
impl From<LEDTSCU0RS_AW> for bool {
    #[inline(always)]
    fn from(variant: LEDTSCU0RS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LEDTSCU0RS` writer - LEDTS Reset Assert"]
pub type LEDTSCU0RS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSET1_SPEC, LEDTSCU0RS_AW, O>;
impl<'a, const O: u8> LEDTSCU0RS_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(LEDTSCU0RS_AW::CONST_0)
    }
    #[doc = "Assert reset"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(LEDTSCU0RS_AW::CONST_1)
    }
}
#[doc = "MultiCAN Reset Assert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCAN0RS_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Assert reset"]
    CONST_1 = 1,
}
impl From<MCAN0RS_AW> for bool {
    #[inline(always)]
    fn from(variant: MCAN0RS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCAN0RS` writer - MultiCAN Reset Assert"]
pub type MCAN0RS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSET1_SPEC, MCAN0RS_AW, O>;
impl<'a, const O: u8> MCAN0RS_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(MCAN0RS_AW::CONST_0)
    }
    #[doc = "Assert reset"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(MCAN0RS_AW::CONST_1)
    }
}
#[doc = "DAC Reset Assert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DACRS_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Assert reset"]
    CONST_1 = 1,
}
impl From<DACRS_AW> for bool {
    #[inline(always)]
    fn from(variant: DACRS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DACRS` writer - DAC Reset Assert"]
pub type DACRS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSET1_SPEC, DACRS_AW, O>;
impl<'a, const O: u8> DACRS_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(DACRS_AW::CONST_0)
    }
    #[doc = "Assert reset"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(DACRS_AW::CONST_1)
    }
}
#[doc = "MMC Interface Reset Assert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MMCIRS_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Assert reset"]
    CONST_1 = 1,
}
impl From<MMCIRS_AW> for bool {
    #[inline(always)]
    fn from(variant: MMCIRS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MMCIRS` writer - MMC Interface Reset Assert"]
pub type MMCIRS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSET1_SPEC, MMCIRS_AW, O>;
impl<'a, const O: u8> MMCIRS_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(MMCIRS_AW::CONST_0)
    }
    #[doc = "Assert reset"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(MMCIRS_AW::CONST_1)
    }
}
#[doc = "USIC1 Reset Assert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USIC1RS_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Assert reset"]
    CONST_1 = 1,
}
impl From<USIC1RS_AW> for bool {
    #[inline(always)]
    fn from(variant: USIC1RS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USIC1RS` writer - USIC1 Reset Assert"]
pub type USIC1RS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSET1_SPEC, USIC1RS_AW, O>;
impl<'a, const O: u8> USIC1RS_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(USIC1RS_AW::CONST_0)
    }
    #[doc = "Assert reset"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(USIC1RS_AW::CONST_1)
    }
}
#[doc = "PORTS Reset Assert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PPORTSRS_AW {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: Assert reset"]
    CONST_1 = 1,
}
impl From<PPORTSRS_AW> for bool {
    #[inline(always)]
    fn from(variant: PPORTSRS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PPORTSRS` writer - PORTS Reset Assert"]
pub type PPORTSRS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSET1_SPEC, PPORTSRS_AW, O>;
impl<'a, const O: u8> PPORTSRS_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PPORTSRS_AW::CONST_0)
    }
    #[doc = "Assert reset"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PPORTSRS_AW::CONST_1)
    }
}
impl W {
    #[doc = "Bit 3 - LEDTS Reset Assert"]
    #[inline(always)]
    #[must_use]
    pub fn ledtscu0rs(&mut self) -> LEDTSCU0RS_W<3> {
        LEDTSCU0RS_W::new(self)
    }
    #[doc = "Bit 4 - MultiCAN Reset Assert"]
    #[inline(always)]
    #[must_use]
    pub fn mcan0rs(&mut self) -> MCAN0RS_W<4> {
        MCAN0RS_W::new(self)
    }
    #[doc = "Bit 5 - DAC Reset Assert"]
    #[inline(always)]
    #[must_use]
    pub fn dacrs(&mut self) -> DACRS_W<5> {
        DACRS_W::new(self)
    }
    #[doc = "Bit 6 - MMC Interface Reset Assert"]
    #[inline(always)]
    #[must_use]
    pub fn mmcirs(&mut self) -> MMCIRS_W<6> {
        MMCIRS_W::new(self)
    }
    #[doc = "Bit 7 - USIC1 Reset Assert"]
    #[inline(always)]
    #[must_use]
    pub fn usic1rs(&mut self) -> USIC1RS_W<7> {
        USIC1RS_W::new(self)
    }
    #[doc = "Bit 9 - PORTS Reset Assert"]
    #[inline(always)]
    #[must_use]
    pub fn pportsrs(&mut self) -> PPORTSRS_W<9> {
        PPORTSRS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCU Peripheral 1 Reset Set\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prset1](index.html) module"]
pub struct PRSET1_SPEC;
impl crate::RegisterSpec for PRSET1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [prset1::W](W) writer structure"]
impl crate::Writable for PRSET1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRSET1 to value 0"]
impl crate::Resettable for PRSET1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

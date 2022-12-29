#[doc = "Register `CLEARSRCTRAN` writer"]
pub struct W(crate::W<CLEARSRCTRAN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLEARSRCTRAN_SPEC>;
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
impl From<crate::W<CLEARSRCTRAN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLEARSRCTRAN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Clear Interrupt Status and Raw Status for channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH0_AW {
    #[doc = "0: no effect"]
    VALUE1 = 0,
    #[doc = "1: clear status"]
    VALUE2 = 1,
}
impl From<CH0_AW> for bool {
    #[inline(always)]
    fn from(variant: CH0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0` writer - Clear Interrupt Status and Raw Status for channel 0"]
pub type CH0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLEARSRCTRAN_SPEC, CH0_AW, O>;
impl<'a, const O: u8> CH0_W<'a, O> {
    #[doc = "no effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH0_AW::VALUE1)
    }
    #[doc = "clear status"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH0_AW::VALUE2)
    }
}
#[doc = "Clear Interrupt Status and Raw Status for channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH1_AW {
    #[doc = "0: no effect"]
    VALUE1 = 0,
    #[doc = "1: clear status"]
    VALUE2 = 1,
}
impl From<CH1_AW> for bool {
    #[inline(always)]
    fn from(variant: CH1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH1` writer - Clear Interrupt Status and Raw Status for channel 1"]
pub type CH1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLEARSRCTRAN_SPEC, CH1_AW, O>;
impl<'a, const O: u8> CH1_W<'a, O> {
    #[doc = "no effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH1_AW::VALUE1)
    }
    #[doc = "clear status"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH1_AW::VALUE2)
    }
}
#[doc = "Clear Interrupt Status and Raw Status for channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH2_AW {
    #[doc = "0: no effect"]
    VALUE1 = 0,
    #[doc = "1: clear status"]
    VALUE2 = 1,
}
impl From<CH2_AW> for bool {
    #[inline(always)]
    fn from(variant: CH2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH2` writer - Clear Interrupt Status and Raw Status for channel 2"]
pub type CH2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLEARSRCTRAN_SPEC, CH2_AW, O>;
impl<'a, const O: u8> CH2_W<'a, O> {
    #[doc = "no effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH2_AW::VALUE1)
    }
    #[doc = "clear status"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH2_AW::VALUE2)
    }
}
#[doc = "Clear Interrupt Status and Raw Status for channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH3_AW {
    #[doc = "0: no effect"]
    VALUE1 = 0,
    #[doc = "1: clear status"]
    VALUE2 = 1,
}
impl From<CH3_AW> for bool {
    #[inline(always)]
    fn from(variant: CH3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH3` writer - Clear Interrupt Status and Raw Status for channel 3"]
pub type CH3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLEARSRCTRAN_SPEC, CH3_AW, O>;
impl<'a, const O: u8> CH3_W<'a, O> {
    #[doc = "no effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH3_AW::VALUE1)
    }
    #[doc = "clear status"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH3_AW::VALUE2)
    }
}
#[doc = "Clear Interrupt Status and Raw Status for channel 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH4_AW {
    #[doc = "0: no effect"]
    VALUE1 = 0,
    #[doc = "1: clear status"]
    VALUE2 = 1,
}
impl From<CH4_AW> for bool {
    #[inline(always)]
    fn from(variant: CH4_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH4` writer - Clear Interrupt Status and Raw Status for channel 4"]
pub type CH4_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLEARSRCTRAN_SPEC, CH4_AW, O>;
impl<'a, const O: u8> CH4_W<'a, O> {
    #[doc = "no effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH4_AW::VALUE1)
    }
    #[doc = "clear status"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH4_AW::VALUE2)
    }
}
#[doc = "Clear Interrupt Status and Raw Status for channel 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH5_AW {
    #[doc = "0: no effect"]
    VALUE1 = 0,
    #[doc = "1: clear status"]
    VALUE2 = 1,
}
impl From<CH5_AW> for bool {
    #[inline(always)]
    fn from(variant: CH5_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH5` writer - Clear Interrupt Status and Raw Status for channel 5"]
pub type CH5_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLEARSRCTRAN_SPEC, CH5_AW, O>;
impl<'a, const O: u8> CH5_W<'a, O> {
    #[doc = "no effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH5_AW::VALUE1)
    }
    #[doc = "clear status"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH5_AW::VALUE2)
    }
}
#[doc = "Clear Interrupt Status and Raw Status for channel 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH6_AW {
    #[doc = "0: no effect"]
    VALUE1 = 0,
    #[doc = "1: clear status"]
    VALUE2 = 1,
}
impl From<CH6_AW> for bool {
    #[inline(always)]
    fn from(variant: CH6_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH6` writer - Clear Interrupt Status and Raw Status for channel 6"]
pub type CH6_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLEARSRCTRAN_SPEC, CH6_AW, O>;
impl<'a, const O: u8> CH6_W<'a, O> {
    #[doc = "no effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH6_AW::VALUE1)
    }
    #[doc = "clear status"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH6_AW::VALUE2)
    }
}
#[doc = "Clear Interrupt Status and Raw Status for channel 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH7_AW {
    #[doc = "0: no effect"]
    VALUE1 = 0,
    #[doc = "1: clear status"]
    VALUE2 = 1,
}
impl From<CH7_AW> for bool {
    #[inline(always)]
    fn from(variant: CH7_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH7` writer - Clear Interrupt Status and Raw Status for channel 7"]
pub type CH7_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLEARSRCTRAN_SPEC, CH7_AW, O>;
impl<'a, const O: u8> CH7_W<'a, O> {
    #[doc = "no effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH7_AW::VALUE1)
    }
    #[doc = "clear status"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH7_AW::VALUE2)
    }
}
impl W {
    #[doc = "Bit 0 - Clear Interrupt Status and Raw Status for channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn ch0(&mut self) -> CH0_W<0> {
        CH0_W::new(self)
    }
    #[doc = "Bit 1 - Clear Interrupt Status and Raw Status for channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ch1(&mut self) -> CH1_W<1> {
        CH1_W::new(self)
    }
    #[doc = "Bit 2 - Clear Interrupt Status and Raw Status for channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn ch2(&mut self) -> CH2_W<2> {
        CH2_W::new(self)
    }
    #[doc = "Bit 3 - Clear Interrupt Status and Raw Status for channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn ch3(&mut self) -> CH3_W<3> {
        CH3_W::new(self)
    }
    #[doc = "Bit 4 - Clear Interrupt Status and Raw Status for channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn ch4(&mut self) -> CH4_W<4> {
        CH4_W::new(self)
    }
    #[doc = "Bit 5 - Clear Interrupt Status and Raw Status for channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn ch5(&mut self) -> CH5_W<5> {
        CH5_W::new(self)
    }
    #[doc = "Bit 6 - Clear Interrupt Status and Raw Status for channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn ch6(&mut self) -> CH6_W<6> {
        CH6_W::new(self)
    }
    #[doc = "Bit 7 - Clear Interrupt Status and Raw Status for channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn ch7(&mut self) -> CH7_W<7> {
        CH7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IntSrcTran Status\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clearsrctran](index.html) module"]
pub struct CLEARSRCTRAN_SPEC;
impl crate::RegisterSpec for CLEARSRCTRAN_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [clearsrctran::W](W) writer structure"]
impl crate::Writable for CLEARSRCTRAN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLEARSRCTRAN to value 0"]
impl crate::Resettable for CLEARSRCTRAN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

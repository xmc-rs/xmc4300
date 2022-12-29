#[doc = "Register `REQSRCREG` reader"]
pub struct R(crate::R<REQSRCREG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REQSRCREG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REQSRCREG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REQSRCREG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REQSRCREG` writer"]
pub struct W(crate::W<REQSRCREG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REQSRCREG_SPEC>;
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
impl From<crate::W<REQSRCREG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REQSRCREG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0` reader - Source request for channel 0"]
pub type CH0_R = crate::BitReader<bool>;
#[doc = "Field `CH0` writer - Source request for channel 0"]
pub type CH0_W<'a, const O: u8> = crate::BitWriter<'a, u32, REQSRCREG_SPEC, bool, O>;
#[doc = "Field `CH1` reader - Source request for channel 1"]
pub type CH1_R = crate::BitReader<bool>;
#[doc = "Field `CH1` writer - Source request for channel 1"]
pub type CH1_W<'a, const O: u8> = crate::BitWriter<'a, u32, REQSRCREG_SPEC, bool, O>;
#[doc = "Field `CH2` reader - Source request for channel 2"]
pub type CH2_R = crate::BitReader<bool>;
#[doc = "Field `CH2` writer - Source request for channel 2"]
pub type CH2_W<'a, const O: u8> = crate::BitWriter<'a, u32, REQSRCREG_SPEC, bool, O>;
#[doc = "Field `CH3` reader - Source request for channel 3"]
pub type CH3_R = crate::BitReader<bool>;
#[doc = "Field `CH3` writer - Source request for channel 3"]
pub type CH3_W<'a, const O: u8> = crate::BitWriter<'a, u32, REQSRCREG_SPEC, bool, O>;
#[doc = "Field `CH4` reader - Source request for channel 4"]
pub type CH4_R = crate::BitReader<bool>;
#[doc = "Field `CH4` writer - Source request for channel 4"]
pub type CH4_W<'a, const O: u8> = crate::BitWriter<'a, u32, REQSRCREG_SPEC, bool, O>;
#[doc = "Field `CH5` reader - Source request for channel 5"]
pub type CH5_R = crate::BitReader<bool>;
#[doc = "Field `CH5` writer - Source request for channel 5"]
pub type CH5_W<'a, const O: u8> = crate::BitWriter<'a, u32, REQSRCREG_SPEC, bool, O>;
#[doc = "Field `CH6` reader - Source request for channel 6"]
pub type CH6_R = crate::BitReader<bool>;
#[doc = "Field `CH6` writer - Source request for channel 6"]
pub type CH6_W<'a, const O: u8> = crate::BitWriter<'a, u32, REQSRCREG_SPEC, bool, O>;
#[doc = "Field `CH7` reader - Source request for channel 7"]
pub type CH7_R = crate::BitReader<bool>;
#[doc = "Field `CH7` writer - Source request for channel 7"]
pub type CH7_W<'a, const O: u8> = crate::BitWriter<'a, u32, REQSRCREG_SPEC, bool, O>;
#[doc = "Source request write enable for channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WE_CH0_AW {
    #[doc = "0: write disabled"]
    VALUE1 = 0,
    #[doc = "1: write enabled"]
    VALUE2 = 1,
}
impl From<WE_CH0_AW> for bool {
    #[inline(always)]
    fn from(variant: WE_CH0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WE_CH0` writer - Source request write enable for channel 0"]
pub type WE_CH0_W<'a, const O: u8> = crate::BitWriter<'a, u32, REQSRCREG_SPEC, WE_CH0_AW, O>;
impl<'a, const O: u8> WE_CH0_W<'a, O> {
    #[doc = "write disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(WE_CH0_AW::VALUE1)
    }
    #[doc = "write enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(WE_CH0_AW::VALUE2)
    }
}
#[doc = "Source request write enable for channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WE_CH1_AW {
    #[doc = "0: write disabled"]
    VALUE1 = 0,
    #[doc = "1: write enabled"]
    VALUE2 = 1,
}
impl From<WE_CH1_AW> for bool {
    #[inline(always)]
    fn from(variant: WE_CH1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WE_CH1` writer - Source request write enable for channel 1"]
pub type WE_CH1_W<'a, const O: u8> = crate::BitWriter<'a, u32, REQSRCREG_SPEC, WE_CH1_AW, O>;
impl<'a, const O: u8> WE_CH1_W<'a, O> {
    #[doc = "write disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(WE_CH1_AW::VALUE1)
    }
    #[doc = "write enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(WE_CH1_AW::VALUE2)
    }
}
#[doc = "Source request write enable for channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WE_CH2_AW {
    #[doc = "0: write disabled"]
    VALUE1 = 0,
    #[doc = "1: write enabled"]
    VALUE2 = 1,
}
impl From<WE_CH2_AW> for bool {
    #[inline(always)]
    fn from(variant: WE_CH2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WE_CH2` writer - Source request write enable for channel 2"]
pub type WE_CH2_W<'a, const O: u8> = crate::BitWriter<'a, u32, REQSRCREG_SPEC, WE_CH2_AW, O>;
impl<'a, const O: u8> WE_CH2_W<'a, O> {
    #[doc = "write disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(WE_CH2_AW::VALUE1)
    }
    #[doc = "write enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(WE_CH2_AW::VALUE2)
    }
}
#[doc = "Source request write enable for channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WE_CH3_AW {
    #[doc = "0: write disabled"]
    VALUE1 = 0,
    #[doc = "1: write enabled"]
    VALUE2 = 1,
}
impl From<WE_CH3_AW> for bool {
    #[inline(always)]
    fn from(variant: WE_CH3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WE_CH3` writer - Source request write enable for channel 3"]
pub type WE_CH3_W<'a, const O: u8> = crate::BitWriter<'a, u32, REQSRCREG_SPEC, WE_CH3_AW, O>;
impl<'a, const O: u8> WE_CH3_W<'a, O> {
    #[doc = "write disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(WE_CH3_AW::VALUE1)
    }
    #[doc = "write enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(WE_CH3_AW::VALUE2)
    }
}
#[doc = "Source request write enable for channel 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WE_CH4_AW {
    #[doc = "0: write disabled"]
    VALUE1 = 0,
    #[doc = "1: write enabled"]
    VALUE2 = 1,
}
impl From<WE_CH4_AW> for bool {
    #[inline(always)]
    fn from(variant: WE_CH4_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WE_CH4` writer - Source request write enable for channel 4"]
pub type WE_CH4_W<'a, const O: u8> = crate::BitWriter<'a, u32, REQSRCREG_SPEC, WE_CH4_AW, O>;
impl<'a, const O: u8> WE_CH4_W<'a, O> {
    #[doc = "write disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(WE_CH4_AW::VALUE1)
    }
    #[doc = "write enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(WE_CH4_AW::VALUE2)
    }
}
#[doc = "Source request write enable for channel 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WE_CH5_AW {
    #[doc = "0: write disabled"]
    VALUE1 = 0,
    #[doc = "1: write enabled"]
    VALUE2 = 1,
}
impl From<WE_CH5_AW> for bool {
    #[inline(always)]
    fn from(variant: WE_CH5_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WE_CH5` writer - Source request write enable for channel 5"]
pub type WE_CH5_W<'a, const O: u8> = crate::BitWriter<'a, u32, REQSRCREG_SPEC, WE_CH5_AW, O>;
impl<'a, const O: u8> WE_CH5_W<'a, O> {
    #[doc = "write disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(WE_CH5_AW::VALUE1)
    }
    #[doc = "write enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(WE_CH5_AW::VALUE2)
    }
}
#[doc = "Source request write enable for channel 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WE_CH6_AW {
    #[doc = "0: write disabled"]
    VALUE1 = 0,
    #[doc = "1: write enabled"]
    VALUE2 = 1,
}
impl From<WE_CH6_AW> for bool {
    #[inline(always)]
    fn from(variant: WE_CH6_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WE_CH6` writer - Source request write enable for channel 6"]
pub type WE_CH6_W<'a, const O: u8> = crate::BitWriter<'a, u32, REQSRCREG_SPEC, WE_CH6_AW, O>;
impl<'a, const O: u8> WE_CH6_W<'a, O> {
    #[doc = "write disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(WE_CH6_AW::VALUE1)
    }
    #[doc = "write enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(WE_CH6_AW::VALUE2)
    }
}
#[doc = "Source request write enable for channel 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WE_CH7_AW {
    #[doc = "0: write disabled"]
    VALUE1 = 0,
    #[doc = "1: write enabled"]
    VALUE2 = 1,
}
impl From<WE_CH7_AW> for bool {
    #[inline(always)]
    fn from(variant: WE_CH7_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WE_CH7` writer - Source request write enable for channel 7"]
pub type WE_CH7_W<'a, const O: u8> = crate::BitWriter<'a, u32, REQSRCREG_SPEC, WE_CH7_AW, O>;
impl<'a, const O: u8> WE_CH7_W<'a, O> {
    #[doc = "write disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(WE_CH7_AW::VALUE1)
    }
    #[doc = "write enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(WE_CH7_AW::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Source request for channel 0"]
    #[inline(always)]
    pub fn ch0(&self) -> CH0_R {
        CH0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Source request for channel 1"]
    #[inline(always)]
    pub fn ch1(&self) -> CH1_R {
        CH1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Source request for channel 2"]
    #[inline(always)]
    pub fn ch2(&self) -> CH2_R {
        CH2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Source request for channel 3"]
    #[inline(always)]
    pub fn ch3(&self) -> CH3_R {
        CH3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Source request for channel 4"]
    #[inline(always)]
    pub fn ch4(&self) -> CH4_R {
        CH4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Source request for channel 5"]
    #[inline(always)]
    pub fn ch5(&self) -> CH5_R {
        CH5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Source request for channel 6"]
    #[inline(always)]
    pub fn ch6(&self) -> CH6_R {
        CH6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Source request for channel 7"]
    #[inline(always)]
    pub fn ch7(&self) -> CH7_R {
        CH7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Source request for channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn ch0(&mut self) -> CH0_W<0> {
        CH0_W::new(self)
    }
    #[doc = "Bit 1 - Source request for channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ch1(&mut self) -> CH1_W<1> {
        CH1_W::new(self)
    }
    #[doc = "Bit 2 - Source request for channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn ch2(&mut self) -> CH2_W<2> {
        CH2_W::new(self)
    }
    #[doc = "Bit 3 - Source request for channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn ch3(&mut self) -> CH3_W<3> {
        CH3_W::new(self)
    }
    #[doc = "Bit 4 - Source request for channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn ch4(&mut self) -> CH4_W<4> {
        CH4_W::new(self)
    }
    #[doc = "Bit 5 - Source request for channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn ch5(&mut self) -> CH5_W<5> {
        CH5_W::new(self)
    }
    #[doc = "Bit 6 - Source request for channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn ch6(&mut self) -> CH6_W<6> {
        CH6_W::new(self)
    }
    #[doc = "Bit 7 - Source request for channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn ch7(&mut self) -> CH7_W<7> {
        CH7_W::new(self)
    }
    #[doc = "Bit 8 - Source request write enable for channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn we_ch0(&mut self) -> WE_CH0_W<8> {
        WE_CH0_W::new(self)
    }
    #[doc = "Bit 9 - Source request write enable for channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn we_ch1(&mut self) -> WE_CH1_W<9> {
        WE_CH1_W::new(self)
    }
    #[doc = "Bit 10 - Source request write enable for channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn we_ch2(&mut self) -> WE_CH2_W<10> {
        WE_CH2_W::new(self)
    }
    #[doc = "Bit 11 - Source request write enable for channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn we_ch3(&mut self) -> WE_CH3_W<11> {
        WE_CH3_W::new(self)
    }
    #[doc = "Bit 12 - Source request write enable for channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn we_ch4(&mut self) -> WE_CH4_W<12> {
        WE_CH4_W::new(self)
    }
    #[doc = "Bit 13 - Source request write enable for channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn we_ch5(&mut self) -> WE_CH5_W<13> {
        WE_CH5_W::new(self)
    }
    #[doc = "Bit 14 - Source request write enable for channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn we_ch6(&mut self) -> WE_CH6_W<14> {
        WE_CH6_W::new(self)
    }
    #[doc = "Bit 15 - Source request write enable for channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn we_ch7(&mut self) -> WE_CH7_W<15> {
        WE_CH7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Source Software Transaction Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reqsrcreg](index.html) module"]
pub struct REQSRCREG_SPEC;
impl crate::RegisterSpec for REQSRCREG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reqsrcreg::R](R) reader structure"]
impl crate::Readable for REQSRCREG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reqsrcreg::W](W) writer structure"]
impl crate::Writable for REQSRCREG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REQSRCREG to value 0"]
impl crate::Resettable for REQSRCREG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

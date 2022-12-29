#[doc = "Register `REFCLR` writer"]
pub struct W(crate::W<REFCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REFCLR_SPEC>;
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
impl From<crate::W<REFCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REFCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Clear Result Event for Result Register 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REV0_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear the result event flag in GxREFLAG"]
    VALUE2 = 1,
}
impl From<REV0_AW> for bool {
    #[inline(always)]
    fn from(variant: REV0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV0` writer - Clear Result Event for Result Register 0"]
pub type REV0_W<'a, const O: u8> = crate::BitWriter<'a, u32, REFCLR_SPEC, REV0_AW, O>;
impl<'a, const O: u8> REV0_W<'a, O> {
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV0_AW::VALUE1)
    }
    #[doc = "Clear the result event flag in GxREFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV0_AW::VALUE2)
    }
}
#[doc = "Clear Result Event for Result Register 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REV1_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear the result event flag in GxREFLAG"]
    VALUE2 = 1,
}
impl From<REV1_AW> for bool {
    #[inline(always)]
    fn from(variant: REV1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV1` writer - Clear Result Event for Result Register 1"]
pub type REV1_W<'a, const O: u8> = crate::BitWriter<'a, u32, REFCLR_SPEC, REV1_AW, O>;
impl<'a, const O: u8> REV1_W<'a, O> {
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV1_AW::VALUE1)
    }
    #[doc = "Clear the result event flag in GxREFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV1_AW::VALUE2)
    }
}
#[doc = "Clear Result Event for Result Register 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REV2_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear the result event flag in GxREFLAG"]
    VALUE2 = 1,
}
impl From<REV2_AW> for bool {
    #[inline(always)]
    fn from(variant: REV2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV2` writer - Clear Result Event for Result Register 2"]
pub type REV2_W<'a, const O: u8> = crate::BitWriter<'a, u32, REFCLR_SPEC, REV2_AW, O>;
impl<'a, const O: u8> REV2_W<'a, O> {
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV2_AW::VALUE1)
    }
    #[doc = "Clear the result event flag in GxREFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV2_AW::VALUE2)
    }
}
#[doc = "Clear Result Event for Result Register 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REV3_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear the result event flag in GxREFLAG"]
    VALUE2 = 1,
}
impl From<REV3_AW> for bool {
    #[inline(always)]
    fn from(variant: REV3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV3` writer - Clear Result Event for Result Register 3"]
pub type REV3_W<'a, const O: u8> = crate::BitWriter<'a, u32, REFCLR_SPEC, REV3_AW, O>;
impl<'a, const O: u8> REV3_W<'a, O> {
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV3_AW::VALUE1)
    }
    #[doc = "Clear the result event flag in GxREFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV3_AW::VALUE2)
    }
}
#[doc = "Clear Result Event for Result Register 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REV4_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear the result event flag in GxREFLAG"]
    VALUE2 = 1,
}
impl From<REV4_AW> for bool {
    #[inline(always)]
    fn from(variant: REV4_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV4` writer - Clear Result Event for Result Register 4"]
pub type REV4_W<'a, const O: u8> = crate::BitWriter<'a, u32, REFCLR_SPEC, REV4_AW, O>;
impl<'a, const O: u8> REV4_W<'a, O> {
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV4_AW::VALUE1)
    }
    #[doc = "Clear the result event flag in GxREFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV4_AW::VALUE2)
    }
}
#[doc = "Clear Result Event for Result Register 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REV5_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear the result event flag in GxREFLAG"]
    VALUE2 = 1,
}
impl From<REV5_AW> for bool {
    #[inline(always)]
    fn from(variant: REV5_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV5` writer - Clear Result Event for Result Register 5"]
pub type REV5_W<'a, const O: u8> = crate::BitWriter<'a, u32, REFCLR_SPEC, REV5_AW, O>;
impl<'a, const O: u8> REV5_W<'a, O> {
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV5_AW::VALUE1)
    }
    #[doc = "Clear the result event flag in GxREFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV5_AW::VALUE2)
    }
}
#[doc = "Clear Result Event for Result Register 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REV6_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear the result event flag in GxREFLAG"]
    VALUE2 = 1,
}
impl From<REV6_AW> for bool {
    #[inline(always)]
    fn from(variant: REV6_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV6` writer - Clear Result Event for Result Register 6"]
pub type REV6_W<'a, const O: u8> = crate::BitWriter<'a, u32, REFCLR_SPEC, REV6_AW, O>;
impl<'a, const O: u8> REV6_W<'a, O> {
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV6_AW::VALUE1)
    }
    #[doc = "Clear the result event flag in GxREFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV6_AW::VALUE2)
    }
}
#[doc = "Clear Result Event for Result Register 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REV7_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear the result event flag in GxREFLAG"]
    VALUE2 = 1,
}
impl From<REV7_AW> for bool {
    #[inline(always)]
    fn from(variant: REV7_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV7` writer - Clear Result Event for Result Register 7"]
pub type REV7_W<'a, const O: u8> = crate::BitWriter<'a, u32, REFCLR_SPEC, REV7_AW, O>;
impl<'a, const O: u8> REV7_W<'a, O> {
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV7_AW::VALUE1)
    }
    #[doc = "Clear the result event flag in GxREFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV7_AW::VALUE2)
    }
}
#[doc = "Clear Result Event for Result Register 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REV8_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear the result event flag in GxREFLAG"]
    VALUE2 = 1,
}
impl From<REV8_AW> for bool {
    #[inline(always)]
    fn from(variant: REV8_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV8` writer - Clear Result Event for Result Register 8"]
pub type REV8_W<'a, const O: u8> = crate::BitWriter<'a, u32, REFCLR_SPEC, REV8_AW, O>;
impl<'a, const O: u8> REV8_W<'a, O> {
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV8_AW::VALUE1)
    }
    #[doc = "Clear the result event flag in GxREFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV8_AW::VALUE2)
    }
}
#[doc = "Clear Result Event for Result Register 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REV9_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear the result event flag in GxREFLAG"]
    VALUE2 = 1,
}
impl From<REV9_AW> for bool {
    #[inline(always)]
    fn from(variant: REV9_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV9` writer - Clear Result Event for Result Register 9"]
pub type REV9_W<'a, const O: u8> = crate::BitWriter<'a, u32, REFCLR_SPEC, REV9_AW, O>;
impl<'a, const O: u8> REV9_W<'a, O> {
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV9_AW::VALUE1)
    }
    #[doc = "Clear the result event flag in GxREFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV9_AW::VALUE2)
    }
}
#[doc = "Clear Result Event for Result Register 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REV10_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear the result event flag in GxREFLAG"]
    VALUE2 = 1,
}
impl From<REV10_AW> for bool {
    #[inline(always)]
    fn from(variant: REV10_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV10` writer - Clear Result Event for Result Register 10"]
pub type REV10_W<'a, const O: u8> = crate::BitWriter<'a, u32, REFCLR_SPEC, REV10_AW, O>;
impl<'a, const O: u8> REV10_W<'a, O> {
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV10_AW::VALUE1)
    }
    #[doc = "Clear the result event flag in GxREFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV10_AW::VALUE2)
    }
}
#[doc = "Clear Result Event for Result Register 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REV11_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear the result event flag in GxREFLAG"]
    VALUE2 = 1,
}
impl From<REV11_AW> for bool {
    #[inline(always)]
    fn from(variant: REV11_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV11` writer - Clear Result Event for Result Register 11"]
pub type REV11_W<'a, const O: u8> = crate::BitWriter<'a, u32, REFCLR_SPEC, REV11_AW, O>;
impl<'a, const O: u8> REV11_W<'a, O> {
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV11_AW::VALUE1)
    }
    #[doc = "Clear the result event flag in GxREFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV11_AW::VALUE2)
    }
}
#[doc = "Clear Result Event for Result Register 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REV12_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear the result event flag in GxREFLAG"]
    VALUE2 = 1,
}
impl From<REV12_AW> for bool {
    #[inline(always)]
    fn from(variant: REV12_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV12` writer - Clear Result Event for Result Register 12"]
pub type REV12_W<'a, const O: u8> = crate::BitWriter<'a, u32, REFCLR_SPEC, REV12_AW, O>;
impl<'a, const O: u8> REV12_W<'a, O> {
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV12_AW::VALUE1)
    }
    #[doc = "Clear the result event flag in GxREFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV12_AW::VALUE2)
    }
}
#[doc = "Clear Result Event for Result Register 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REV13_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear the result event flag in GxREFLAG"]
    VALUE2 = 1,
}
impl From<REV13_AW> for bool {
    #[inline(always)]
    fn from(variant: REV13_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV13` writer - Clear Result Event for Result Register 13"]
pub type REV13_W<'a, const O: u8> = crate::BitWriter<'a, u32, REFCLR_SPEC, REV13_AW, O>;
impl<'a, const O: u8> REV13_W<'a, O> {
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV13_AW::VALUE1)
    }
    #[doc = "Clear the result event flag in GxREFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV13_AW::VALUE2)
    }
}
#[doc = "Clear Result Event for Result Register 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REV14_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear the result event flag in GxREFLAG"]
    VALUE2 = 1,
}
impl From<REV14_AW> for bool {
    #[inline(always)]
    fn from(variant: REV14_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV14` writer - Clear Result Event for Result Register 14"]
pub type REV14_W<'a, const O: u8> = crate::BitWriter<'a, u32, REFCLR_SPEC, REV14_AW, O>;
impl<'a, const O: u8> REV14_W<'a, O> {
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV14_AW::VALUE1)
    }
    #[doc = "Clear the result event flag in GxREFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV14_AW::VALUE2)
    }
}
#[doc = "Clear Result Event for Result Register 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REV15_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear the result event flag in GxREFLAG"]
    VALUE2 = 1,
}
impl From<REV15_AW> for bool {
    #[inline(always)]
    fn from(variant: REV15_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV15` writer - Clear Result Event for Result Register 15"]
pub type REV15_W<'a, const O: u8> = crate::BitWriter<'a, u32, REFCLR_SPEC, REV15_AW, O>;
impl<'a, const O: u8> REV15_W<'a, O> {
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(REV15_AW::VALUE1)
    }
    #[doc = "Clear the result event flag in GxREFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(REV15_AW::VALUE2)
    }
}
impl W {
    #[doc = "Bit 0 - Clear Result Event for Result Register 0"]
    #[inline(always)]
    #[must_use]
    pub fn rev0(&mut self) -> REV0_W<0> {
        REV0_W::new(self)
    }
    #[doc = "Bit 1 - Clear Result Event for Result Register 1"]
    #[inline(always)]
    #[must_use]
    pub fn rev1(&mut self) -> REV1_W<1> {
        REV1_W::new(self)
    }
    #[doc = "Bit 2 - Clear Result Event for Result Register 2"]
    #[inline(always)]
    #[must_use]
    pub fn rev2(&mut self) -> REV2_W<2> {
        REV2_W::new(self)
    }
    #[doc = "Bit 3 - Clear Result Event for Result Register 3"]
    #[inline(always)]
    #[must_use]
    pub fn rev3(&mut self) -> REV3_W<3> {
        REV3_W::new(self)
    }
    #[doc = "Bit 4 - Clear Result Event for Result Register 4"]
    #[inline(always)]
    #[must_use]
    pub fn rev4(&mut self) -> REV4_W<4> {
        REV4_W::new(self)
    }
    #[doc = "Bit 5 - Clear Result Event for Result Register 5"]
    #[inline(always)]
    #[must_use]
    pub fn rev5(&mut self) -> REV5_W<5> {
        REV5_W::new(self)
    }
    #[doc = "Bit 6 - Clear Result Event for Result Register 6"]
    #[inline(always)]
    #[must_use]
    pub fn rev6(&mut self) -> REV6_W<6> {
        REV6_W::new(self)
    }
    #[doc = "Bit 7 - Clear Result Event for Result Register 7"]
    #[inline(always)]
    #[must_use]
    pub fn rev7(&mut self) -> REV7_W<7> {
        REV7_W::new(self)
    }
    #[doc = "Bit 8 - Clear Result Event for Result Register 8"]
    #[inline(always)]
    #[must_use]
    pub fn rev8(&mut self) -> REV8_W<8> {
        REV8_W::new(self)
    }
    #[doc = "Bit 9 - Clear Result Event for Result Register 9"]
    #[inline(always)]
    #[must_use]
    pub fn rev9(&mut self) -> REV9_W<9> {
        REV9_W::new(self)
    }
    #[doc = "Bit 10 - Clear Result Event for Result Register 10"]
    #[inline(always)]
    #[must_use]
    pub fn rev10(&mut self) -> REV10_W<10> {
        REV10_W::new(self)
    }
    #[doc = "Bit 11 - Clear Result Event for Result Register 11"]
    #[inline(always)]
    #[must_use]
    pub fn rev11(&mut self) -> REV11_W<11> {
        REV11_W::new(self)
    }
    #[doc = "Bit 12 - Clear Result Event for Result Register 12"]
    #[inline(always)]
    #[must_use]
    pub fn rev12(&mut self) -> REV12_W<12> {
        REV12_W::new(self)
    }
    #[doc = "Bit 13 - Clear Result Event for Result Register 13"]
    #[inline(always)]
    #[must_use]
    pub fn rev13(&mut self) -> REV13_W<13> {
        REV13_W::new(self)
    }
    #[doc = "Bit 14 - Clear Result Event for Result Register 14"]
    #[inline(always)]
    #[must_use]
    pub fn rev14(&mut self) -> REV14_W<14> {
        REV14_W::new(self)
    }
    #[doc = "Bit 15 - Clear Result Event for Result Register 15"]
    #[inline(always)]
    #[must_use]
    pub fn rev15(&mut self) -> REV15_W<15> {
        REV15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Result Event Flag Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [refclr](index.html) module"]
pub struct REFCLR_SPEC;
impl crate::RegisterSpec for REFCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [refclr::W](W) writer structure"]
impl crate::Writable for REFCLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REFCLR to value 0"]
impl crate::Resettable for REFCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

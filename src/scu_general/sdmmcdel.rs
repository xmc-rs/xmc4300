#[doc = "Register `SDMMCDEL` reader"]
pub struct R(crate::R<SDMMCDEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDMMCDEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDMMCDEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDMMCDEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDMMCDEL` writer"]
pub struct W(crate::W<SDMMCDEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDMMCDEL_SPEC>;
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
impl From<crate::W<SDMMCDEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDMMCDEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Enable delay on the CMD/DAT out lines\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAPEN_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<TAPEN_A> for bool {
    #[inline(always)]
    fn from(variant: TAPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAPEN` reader - Enable delay on the CMD/DAT out lines"]
pub struct TAPEN_R(crate::FieldReader<bool, TAPEN_A>);
impl TAPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAPEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAPEN_A {
        match self.bits {
            false => TAPEN_A::VALUE1,
            true => TAPEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == TAPEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == TAPEN_A::VALUE2
    }
}
impl core::ops::Deref for TAPEN_R {
    type Target = crate::FieldReader<bool, TAPEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAPEN` writer - Enable delay on the CMD/DAT out lines"]
pub struct TAPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TAPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TAPEN_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TAPEN_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `TAPDEL` reader - Number of Delay Elements Select"]
pub struct TAPDEL_R(crate::FieldReader<u8, u8>);
impl TAPDEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        TAPDEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAPDEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAPDEL` writer - Number of Delay Elements Select"]
pub struct TAPDEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TAPDEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable delay on the CMD/DAT out lines"]
    #[inline(always)]
    pub fn tapen(&self) -> TAPEN_R {
        TAPEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - Number of Delay Elements Select"]
    #[inline(always)]
    pub fn tapdel(&self) -> TAPDEL_R {
        TAPDEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable delay on the CMD/DAT out lines"]
    #[inline(always)]
    pub fn tapen(&mut self) -> TAPEN_W {
        TAPEN_W { w: self }
    }
    #[doc = "Bits 4:7 - Number of Delay Elements Select"]
    #[inline(always)]
    pub fn tapdel(&mut self) -> TAPDEL_W {
        TAPDEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SD-MMC Delay Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmcdel](index.html) module"]
pub struct SDMMCDEL_SPEC;
impl crate::RegisterSpec for SDMMCDEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdmmcdel::R](R) reader structure"]
impl crate::Readable for SDMMCDEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdmmcdel::W](W) writer structure"]
impl crate::Writable for SDMMCDEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SDMMCDEL to value 0"]
impl crate::Resettable for SDMMCDEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

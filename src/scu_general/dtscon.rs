#[doc = "Register `DTSCON` reader"]
pub struct R(crate::R<DTSCON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTSCON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTSCON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTSCON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DTSCON` writer"]
pub struct W(crate::W<DTSCON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DTSCON_SPEC>;
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
impl From<crate::W<DTSCON_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DTSCON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWD` reader - Sensor Power Down"]
pub type PWD_R = crate::BitReader<PWD_A>;
#[doc = "Sensor Power Down\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWD_A {
    #[doc = "0: The DTS is powered"]
    CONST_0 = 0,
    #[doc = "1: The DTS is not powered"]
    CONST_1 = 1,
}
impl From<PWD_A> for bool {
    #[inline(always)]
    fn from(variant: PWD_A) -> Self {
        variant as u8 != 0
    }
}
impl PWD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWD_A {
        match self.bits {
            false => PWD_A::CONST_0,
            true => PWD_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PWD_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PWD_A::CONST_1
    }
}
#[doc = "Field `PWD` writer - Sensor Power Down"]
pub type PWD_W<'a, const O: u8> = crate::BitWriter<'a, DTSCON_SPEC, O, PWD_A>;
impl<'a, const O: u8> PWD_W<'a, O> {
    #[doc = "The DTS is powered"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PWD_A::CONST_0)
    }
    #[doc = "The DTS is not powered"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PWD_A::CONST_1)
    }
}
#[doc = "Sensor Measurement Start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum START_AW {
    #[doc = "0: No DTS measurement is started"]
    CONST_0 = 0,
    #[doc = "1: DTS measurement is started"]
    CONST_1 = 1,
}
impl From<START_AW> for bool {
    #[inline(always)]
    fn from(variant: START_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `START` writer - Sensor Measurement Start"]
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, DTSCON_SPEC, O, START_AW>;
impl<'a, const O: u8> START_W<'a, O> {
    #[doc = "No DTS measurement is started"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(START_AW::CONST_0)
    }
    #[doc = "DTS measurement is started"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(START_AW::CONST_1)
    }
}
#[doc = "Field `OFFSET` reader - Offset Calibration Value"]
pub type OFFSET_R = crate::FieldReader;
#[doc = "Field `OFFSET` writer - Offset Calibration Value"]
pub type OFFSET_W<'a, const O: u8> = crate::FieldWriter<'a, DTSCON_SPEC, 7, O>;
#[doc = "Field `GAIN` reader - Gain Calibration Value"]
pub type GAIN_R = crate::FieldReader;
#[doc = "Field `GAIN` writer - Gain Calibration Value"]
pub type GAIN_W<'a, const O: u8> = crate::FieldWriter<'a, DTSCON_SPEC, 6, O>;
#[doc = "Field `REFTRIM` reader - Reference Trim Calibration Value"]
pub type REFTRIM_R = crate::FieldReader;
#[doc = "Field `REFTRIM` writer - Reference Trim Calibration Value"]
pub type REFTRIM_W<'a, const O: u8> = crate::FieldWriter<'a, DTSCON_SPEC, 3, O>;
#[doc = "Field `BGTRIM` reader - Bandgap Trim Calibration Value"]
pub type BGTRIM_R = crate::FieldReader;
#[doc = "Field `BGTRIM` writer - Bandgap Trim Calibration Value"]
pub type BGTRIM_W<'a, const O: u8> = crate::FieldWriter<'a, DTSCON_SPEC, 4, O>;
impl R {
    #[doc = "Bit 0 - Sensor Power Down"]
    #[inline(always)]
    pub fn pwd(&self) -> PWD_R {
        PWD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:10 - Offset Calibration Value"]
    #[inline(always)]
    pub fn offset(&self) -> OFFSET_R {
        OFFSET_R::new(((self.bits >> 4) & 0x7f) as u8)
    }
    #[doc = "Bits 11:16 - Gain Calibration Value"]
    #[inline(always)]
    pub fn gain(&self) -> GAIN_R {
        GAIN_R::new(((self.bits >> 11) & 0x3f) as u8)
    }
    #[doc = "Bits 17:19 - Reference Trim Calibration Value"]
    #[inline(always)]
    pub fn reftrim(&self) -> REFTRIM_R {
        REFTRIM_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:23 - Bandgap Trim Calibration Value"]
    #[inline(always)]
    pub fn bgtrim(&self) -> BGTRIM_R {
        BGTRIM_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Sensor Power Down"]
    #[inline(always)]
    #[must_use]
    pub fn pwd(&mut self) -> PWD_W<0> {
        PWD_W::new(self)
    }
    #[doc = "Bit 1 - Sensor Measurement Start"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<1> {
        START_W::new(self)
    }
    #[doc = "Bits 4:10 - Offset Calibration Value"]
    #[inline(always)]
    #[must_use]
    pub fn offset(&mut self) -> OFFSET_W<4> {
        OFFSET_W::new(self)
    }
    #[doc = "Bits 11:16 - Gain Calibration Value"]
    #[inline(always)]
    #[must_use]
    pub fn gain(&mut self) -> GAIN_W<11> {
        GAIN_W::new(self)
    }
    #[doc = "Bits 17:19 - Reference Trim Calibration Value"]
    #[inline(always)]
    #[must_use]
    pub fn reftrim(&mut self) -> REFTRIM_W<17> {
        REFTRIM_W::new(self)
    }
    #[doc = "Bits 20:23 - Bandgap Trim Calibration Value"]
    #[inline(always)]
    #[must_use]
    pub fn bgtrim(&mut self) -> BGTRIM_W<20> {
        BGTRIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Die Temperature Sensor Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtscon](index.html) module"]
pub struct DTSCON_SPEC;
impl crate::RegisterSpec for DTSCON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dtscon::R](R) reader structure"]
impl crate::Readable for DTSCON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dtscon::W](W) writer structure"]
impl crate::Writable for DTSCON_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DTSCON to value 0x01"]
impl crate::Resettable for DTSCON_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}

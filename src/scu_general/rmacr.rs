#[doc = "Register `RMACR` reader"]
pub struct R(crate::R<RMACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RMACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RMACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RMACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RMACR` writer"]
pub struct W(crate::W<RMACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RMACR_SPEC>;
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
impl From<crate::W<RMACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RMACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Hibernate Retention Memory Register Update Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDWR_A {
    #[doc = "0: transfer data from Retention Memory in Hibernate domain to RMDATA register"]
    CONST_0 = 0,
    #[doc = "1: transfer data from RMDATA into Retention Memory in Hibernate domain"]
    CONST_1 = 1,
}
impl From<RDWR_A> for bool {
    #[inline(always)]
    fn from(variant: RDWR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDWR` reader - Hibernate Retention Memory Register Update Control"]
pub struct RDWR_R(crate::FieldReader<bool, RDWR_A>);
impl RDWR_R {
    pub(crate) fn new(bits: bool) -> Self {
        RDWR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDWR_A {
        match self.bits {
            false => RDWR_A::CONST_0,
            true => RDWR_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        **self == RDWR_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        **self == RDWR_A::CONST_1
    }
}
impl core::ops::Deref for RDWR_R {
    type Target = crate::FieldReader<bool, RDWR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RDWR` writer - Hibernate Retention Memory Register Update Control"]
pub struct RDWR_W<'a> {
    w: &'a mut W,
}
impl<'a> RDWR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RDWR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "transfer data from Retention Memory in Hibernate domain to RMDATA register"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(RDWR_A::CONST_0)
    }
    #[doc = "transfer data from RMDATA into Retention Memory in Hibernate domain"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(RDWR_A::CONST_1)
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
#[doc = "Field `ADDR` reader - Hibernate Retention Memory Register Address Select"]
pub struct ADDR_R(crate::FieldReader<u8, u8>);
impl ADDR_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDR` writer - Hibernate Retention Memory Register Address Select"]
pub struct ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Hibernate Retention Memory Register Update Control"]
    #[inline(always)]
    pub fn rdwr(&self) -> RDWR_R {
        RDWR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - Hibernate Retention Memory Register Address Select"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Hibernate Retention Memory Register Update Control"]
    #[inline(always)]
    pub fn rdwr(&mut self) -> RDWR_W {
        RDWR_W { w: self }
    }
    #[doc = "Bits 16:19 - Hibernate Retention Memory Register Address Select"]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W {
        ADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Retention Memory Access Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmacr](index.html) module"]
pub struct RMACR_SPEC;
impl crate::RegisterSpec for RMACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rmacr::R](R) reader structure"]
impl crate::Readable for RMACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rmacr::W](W) writer structure"]
impl crate::Writable for RMACR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RMACR to value 0"]
impl crate::Resettable for RMACR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

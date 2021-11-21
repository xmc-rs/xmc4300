#[doc = "Register `CTLH` reader"]
pub struct R(crate::R<CTLH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTLH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTLH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTLH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTLH` writer"]
pub struct W(crate::W<CTLH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTLH_SPEC>;
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
impl From<crate::W<CTLH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTLH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DONE` reader - Done bit"]
pub struct DONE_R(crate::FieldReader<bool, bool>);
impl DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DONE` writer - Done bit"]
pub struct DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> DONE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `BLOCK_TS` reader - Block Transfer Size"]
pub struct BLOCK_TS_R(crate::FieldReader<u16, u16>);
impl BLOCK_TS_R {
    pub(crate) fn new(bits: u16) -> Self {
        BLOCK_TS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLOCK_TS_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLOCK_TS` writer - Block Transfer Size"]
pub struct BLOCK_TS_W<'a> {
    w: &'a mut W,
}
impl<'a> BLOCK_TS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bit 12 - Done bit"]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 0:11 - Block Transfer Size"]
    #[inline(always)]
    pub fn block_ts(&self) -> BLOCK_TS_R {
        BLOCK_TS_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 12 - Done bit"]
    #[inline(always)]
    pub fn done(&mut self) -> DONE_W {
        DONE_W { w: self }
    }
    #[doc = "Bits 0:11 - Block Transfer Size"]
    #[inline(always)]
    pub fn block_ts(&mut self) -> BLOCK_TS_W {
        BLOCK_TS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register High\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctlh](index.html) module"]
pub struct CTLH_SPEC;
impl crate::RegisterSpec for CTLH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctlh::R](R) reader structure"]
impl crate::Readable for CTLH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctlh::W](W) writer structure"]
impl crate::Writable for CTLH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTLH to value 0x02"]
impl crate::Resettable for CTLH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}

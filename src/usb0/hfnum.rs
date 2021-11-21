#[doc = "Register `HFNUM` reader"]
pub struct R(crate::R<HFNUM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HFNUM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HFNUM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HFNUM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HFNUM` writer"]
pub struct W(crate::W<HFNUM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HFNUM_SPEC>;
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
impl From<crate::W<HFNUM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HFNUM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FrNum` reader - Frame Number"]
pub struct FRNUM_R(crate::FieldReader<u16, u16>);
impl FRNUM_R {
    pub(crate) fn new(bits: u16) -> Self {
        FRNUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRNUM_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FrNum` writer - Frame Number"]
pub struct FRNUM_W<'a> {
    w: &'a mut W,
}
impl<'a> FRNUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `FrRem` reader - Frame Time Remaining"]
pub struct FRREM_R(crate::FieldReader<u16, u16>);
impl FRREM_R {
    pub(crate) fn new(bits: u16) -> Self {
        FRREM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRREM_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Frame Number"]
    #[inline(always)]
    pub fn fr_num(&self) -> FRNUM_R {
        FRNUM_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Frame Time Remaining"]
    #[inline(always)]
    pub fn fr_rem(&self) -> FRREM_R {
        FRREM_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Frame Number"]
    #[inline(always)]
    pub fn fr_num(&mut self) -> FRNUM_W {
        FRNUM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Frame Number/Frame Time Remaining Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfnum](index.html) module"]
pub struct HFNUM_SPEC;
impl crate::RegisterSpec for HFNUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hfnum::R](R) reader structure"]
impl crate::Readable for HFNUM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hfnum::W](W) writer structure"]
impl crate::Writable for HFNUM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HFNUM to value 0x3fff"]
impl crate::Resettable for HFNUM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3fff
    }
}

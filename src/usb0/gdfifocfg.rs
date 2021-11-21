#[doc = "Register `GDFIFOCFG` reader"]
pub struct R(crate::R<GDFIFOCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GDFIFOCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GDFIFOCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GDFIFOCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GDFIFOCFG` writer"]
pub struct W(crate::W<GDFIFOCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GDFIFOCFG_SPEC>;
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
impl From<crate::W<GDFIFOCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GDFIFOCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GDFIFOCfg` reader - GDFIFOCfg"]
pub struct GDFIFOCFG_R(crate::FieldReader<u16, u16>);
impl GDFIFOCFG_R {
    pub(crate) fn new(bits: u16) -> Self {
        GDFIFOCFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GDFIFOCFG_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GDFIFOCfg` writer - GDFIFOCfg"]
pub struct GDFIFOCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GDFIFOCFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `EPInfoBaseAddr` reader - EPInfoBaseAddr"]
pub struct EPINFOBASEADDR_R(crate::FieldReader<u16, u16>);
impl EPINFOBASEADDR_R {
    pub(crate) fn new(bits: u16) -> Self {
        EPINFOBASEADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPINFOBASEADDR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPInfoBaseAddr` writer - EPInfoBaseAddr"]
pub struct EPINFOBASEADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> EPINFOBASEADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - GDFIFOCfg"]
    #[inline(always)]
    pub fn gdfifocfg(&self) -> GDFIFOCFG_R {
        GDFIFOCFG_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - EPInfoBaseAddr"]
    #[inline(always)]
    pub fn epinfo_base_addr(&self) -> EPINFOBASEADDR_R {
        EPINFOBASEADDR_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - GDFIFOCfg"]
    #[inline(always)]
    pub fn gdfifocfg(&mut self) -> GDFIFOCFG_W {
        GDFIFOCFG_W { w: self }
    }
    #[doc = "Bits 16:31 - EPInfoBaseAddr"]
    #[inline(always)]
    pub fn epinfo_base_addr(&mut self) -> EPINFOBASEADDR_W {
        EPINFOBASEADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global DFIFO Software Config Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gdfifocfg](index.html) module"]
pub struct GDFIFOCFG_SPEC;
impl crate::RegisterSpec for GDFIFOCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gdfifocfg::R](R) reader structure"]
impl crate::Readable for GDFIFOCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gdfifocfg::W](W) writer structure"]
impl crate::Writable for GDFIFOCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GDFIFOCFG to value 0x027a_02b2"]
impl crate::Resettable for GDFIFOCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x027a_02b2
    }
}

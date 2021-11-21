#[doc = "Register `DC_SYS_TIME` writer"]
pub struct W(crate::W<WRITEMODE_DC_SYS_TIME_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WRITEMODE_DC_SYS_TIME_SPEC>;
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
impl From<crate::W<WRITEMODE_DC_SYS_TIME_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WRITEMODE_DC_SYS_TIME_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WRITE_ACCESS` writer - Write access"]
pub struct WRITE_ACCESS_W<'a> {
    w: &'a mut W,
}
impl<'a> WRITE_ACCESS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Write access"]
    #[inline(always)]
    pub fn write_access(&mut self) -> WRITE_ACCESS_W {
        WRITE_ACCESS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Time \\[WRITE Mode\\]\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [writemode_dc_sys_time](index.html) module"]
pub struct WRITEMODE_DC_SYS_TIME_SPEC;
impl crate::RegisterSpec for WRITEMODE_DC_SYS_TIME_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [writemode_dc_sys_time::W](W) writer structure"]
impl crate::Writable for WRITEMODE_DC_SYS_TIME_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DC_SYS_TIME to value 0"]
impl crate::Resettable for WRITEMODE_DC_SYS_TIME_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

#[doc = "Register `DC_SYS_TIME_DELAY` reader"]
pub struct R(crate::R<DC_SYS_TIME_DELAY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DC_SYS_TIME_DELAY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DC_SYS_TIME_DELAY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DC_SYS_TIME_DELAY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DC_SYS_TIME_DELAY` writer"]
pub struct W(crate::W<DC_SYS_TIME_DELAY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DC_SYS_TIME_DELAY_SPEC>;
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
impl From<crate::W<DC_SYS_TIME_DELAY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DC_SYS_TIME_DELAY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLK_DELAY` reader - Delay between Reference Clock and the ESC"]
pub type CLK_DELAY_R = crate::FieldReader<u32>;
#[doc = "Field `CLK_DELAY` writer - Delay between Reference Clock and the ESC"]
pub type CLK_DELAY_W<'a, const O: u8> = crate::FieldWriter<'a, DC_SYS_TIME_DELAY_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Delay between Reference Clock and the ESC"]
    #[inline(always)]
    pub fn clk_delay(&self) -> CLK_DELAY_R {
        CLK_DELAY_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Delay between Reference Clock and the ESC"]
    #[inline(always)]
    #[must_use]
    pub fn clk_delay(&mut self) -> CLK_DELAY_W<0> {
        CLK_DELAY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Time Delay\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dc_sys_time_delay](index.html) module"]
pub struct DC_SYS_TIME_DELAY_SPEC;
impl crate::RegisterSpec for DC_SYS_TIME_DELAY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dc_sys_time_delay::R](R) reader structure"]
impl crate::Readable for DC_SYS_TIME_DELAY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dc_sys_time_delay::W](W) writer structure"]
impl crate::Writable for DC_SYS_TIME_DELAY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DC_SYS_TIME_DELAY to value 0"]
impl crate::Resettable for DC_SYS_TIME_DELAY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

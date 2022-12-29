#[doc = "Register `GNPTXFSIZ_DEVICEMODE` reader"]
pub struct R(crate::R<GNPTXFSIZ_DEVICEMODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GNPTXFSIZ_DEVICEMODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GNPTXFSIZ_DEVICEMODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GNPTXFSIZ_DEVICEMODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GNPTXFSIZ_DEVICEMODE` writer"]
pub struct W(crate::W<GNPTXFSIZ_DEVICEMODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GNPTXFSIZ_DEVICEMODE_SPEC>;
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
impl From<crate::W<GNPTXFSIZ_DEVICEMODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GNPTXFSIZ_DEVICEMODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INEPTxF0StAddr` reader - IN Endpoint FIFO0 Transmit RAM Start Address"]
pub type INEPTX_F0ST_ADDR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `INEPTxF0StAddr` writer - IN Endpoint FIFO0 Transmit RAM Start Address"]
pub type INEPTX_F0ST_ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GNPTXFSIZ_DEVICEMODE_SPEC, u16, u16, 16, O>;
#[doc = "Field `INEPTxF0Dep` reader - IN Endpoint TxFIFO 0 Depth"]
pub type INEPTX_F0DEP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `INEPTxF0Dep` writer - IN Endpoint TxFIFO 0 Depth"]
pub type INEPTX_F0DEP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GNPTXFSIZ_DEVICEMODE_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - IN Endpoint FIFO0 Transmit RAM Start Address"]
    #[inline(always)]
    pub fn ineptx_f0st_addr(&self) -> INEPTX_F0ST_ADDR_R {
        INEPTX_F0ST_ADDR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - IN Endpoint TxFIFO 0 Depth"]
    #[inline(always)]
    pub fn ineptx_f0dep(&self) -> INEPTX_F0DEP_R {
        INEPTX_F0DEP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - IN Endpoint FIFO0 Transmit RAM Start Address"]
    #[inline(always)]
    #[must_use]
    pub fn ineptx_f0st_addr(&mut self) -> INEPTX_F0ST_ADDR_W<0> {
        INEPTX_F0ST_ADDR_W::new(self)
    }
    #[doc = "Bits 16:31 - IN Endpoint TxFIFO 0 Depth"]
    #[inline(always)]
    #[must_use]
    pub fn ineptx_f0dep(&mut self) -> INEPTX_F0DEP_W<16> {
        INEPTX_F0DEP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Non-Periodic Transmit FIFO Size Register \\[DEVICEMODE\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gnptxfsiz_devicemode](index.html) module"]
pub struct GNPTXFSIZ_DEVICEMODE_SPEC;
impl crate::RegisterSpec for GNPTXFSIZ_DEVICEMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gnptxfsiz_devicemode::R](R) reader structure"]
impl crate::Readable for GNPTXFSIZ_DEVICEMODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gnptxfsiz_devicemode::W](W) writer structure"]
impl crate::Writable for GNPTXFSIZ_DEVICEMODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GNPTXFSIZ_DEVICEMODE to value 0x0010_0000"]
impl crate::Resettable for GNPTXFSIZ_DEVICEMODE_SPEC {
    const RESET_VALUE: Self::Ux = 0x0010_0000;
}

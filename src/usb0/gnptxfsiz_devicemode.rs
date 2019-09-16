#[doc = "Reader of register GNPTXFSIZ_DEVICEMODE"]
pub type R = crate::R<u32, super::GNPTXFSIZ_DEVICEMODE>;
#[doc = "Writer for register GNPTXFSIZ_DEVICEMODE"]
pub type W = crate::W<u32, super::GNPTXFSIZ_DEVICEMODE>;
#[doc = "Register GNPTXFSIZ_DEVICEMODE `reset()`'s with value 0x0010_0000"]
impl crate::ResetValue for super::GNPTXFSIZ_DEVICEMODE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0010_0000
    }
}
#[doc = "Reader of field `INEPTxF0StAddr`"]
pub type INEPTXF0STADDR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `INEPTxF0StAddr`"]
pub struct INEPTXF0STADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> INEPTXF0STADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `INEPTxF0Dep`"]
pub type INEPTXF0DEP_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `INEPTxF0Dep`"]
pub struct INEPTXF0DEP_W<'a> {
    w: &'a mut W,
}
impl<'a> INEPTXF0DEP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - IN Endpoint FIFO0 Transmit RAM Start Address"]
    #[inline(always)]
    pub fn ineptx_f0st_addr(&self) -> INEPTXF0STADDR_R {
        INEPTXF0STADDR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - IN Endpoint TxFIFO 0 Depth"]
    #[inline(always)]
    pub fn ineptx_f0dep(&self) -> INEPTXF0DEP_R {
        INEPTXF0DEP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - IN Endpoint FIFO0 Transmit RAM Start Address"]
    #[inline(always)]
    pub fn ineptx_f0st_addr(&mut self) -> INEPTXF0STADDR_W {
        INEPTXF0STADDR_W { w: self }
    }
    #[doc = "Bits 16:31 - IN Endpoint TxFIFO 0 Depth"]
    #[inline(always)]
    pub fn ineptx_f0dep(&mut self) -> INEPTXF0DEP_W {
        INEPTXF0DEP_W { w: self }
    }
}

#[doc = "Reader of register GNPTXFSIZ_HOSTMODE"]
pub type R = crate::R<u32, super::GNPTXFSIZ_HOSTMODE>;
#[doc = "Writer for register GNPTXFSIZ_HOSTMODE"]
pub type W = crate::W<u32, super::GNPTXFSIZ_HOSTMODE>;
#[doc = "Register GNPTXFSIZ_HOSTMODE `reset()`'s with value 0x0010_011a"]
impl crate::ResetValue for super::GNPTXFSIZ_HOSTMODE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0010_011a
    }
}
#[doc = "Reader of field `NPTxFStAddr`"]
pub type NPTXFSTADDR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `NPTxFStAddr`"]
pub struct NPTXFSTADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> NPTXFSTADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `NPTxFDep`"]
pub type NPTXFDEP_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `NPTxFDep`"]
pub struct NPTXFDEP_W<'a> {
    w: &'a mut W,
}
impl<'a> NPTXFDEP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Non-periodic Transmit RAM Start Address"]
    #[inline(always)]
    pub fn nptx_fst_addr(&self) -> NPTXFSTADDR_R {
        NPTXFSTADDR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Non-periodic TxFIFO Depth"]
    #[inline(always)]
    pub fn nptx_fdep(&self) -> NPTXFDEP_R {
        NPTXFDEP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Non-periodic Transmit RAM Start Address"]
    #[inline(always)]
    pub fn nptx_fst_addr(&mut self) -> NPTXFSTADDR_W {
        NPTXFSTADDR_W { w: self }
    }
    #[doc = "Bits 16:31 - Non-periodic TxFIFO Depth"]
    #[inline(always)]
    pub fn nptx_fdep(&mut self) -> NPTXFDEP_W {
        NPTXFDEP_W { w: self }
    }
}

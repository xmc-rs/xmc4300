#[doc = "Reader of register HPTXFSIZ"]
pub type R = crate::R<u32, super::HPTXFSIZ>;
#[doc = "Writer for register HPTXFSIZ"]
pub type W = crate::W<u32, super::HPTXFSIZ>;
#[doc = "Register HPTXFSIZ `reset()`'s with value 0x0100_012a"]
impl crate::ResetValue for super::HPTXFSIZ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0100_012a
    }
}
#[doc = "Reader of field `PTxFStAddr`"]
pub type PTXFSTADDR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PTxFStAddr`"]
pub struct PTXFSTADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> PTXFSTADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `PTxFSize`"]
pub type PTXFSIZE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PTxFSize`"]
pub struct PTXFSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> PTXFSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Host Periodic TxFIFO Start Address"]
    #[inline(always)]
    pub fn ptx_fst_addr(&self) -> PTXFSTADDR_R {
        PTXFSTADDR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Host Periodic TxFIFO Depth"]
    #[inline(always)]
    pub fn ptx_fsize(&self) -> PTXFSIZE_R {
        PTXFSIZE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Host Periodic TxFIFO Start Address"]
    #[inline(always)]
    pub fn ptx_fst_addr(&mut self) -> PTXFSTADDR_W {
        PTXFSTADDR_W { w: self }
    }
    #[doc = "Bits 16:31 - Host Periodic TxFIFO Depth"]
    #[inline(always)]
    pub fn ptx_fsize(&mut self) -> PTXFSIZE_W {
        PTXFSIZE_W { w: self }
    }
}

#[doc = "Reader of register HCDMA_SCATGATHER"]
pub type R = crate::R<u32, super::HCDMA_SCATGATHER>;
#[doc = "Writer for register HCDMA_SCATGATHER"]
pub type W = crate::W<u32, super::HCDMA_SCATGATHER>;
#[doc = "Register HCDMA_SCATGATHER `reset()`'s with value 0"]
impl crate::ResetValue for super::HCDMA_SCATGATHER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Current Transfer Desc:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTD_A {
    #[doc = "0: 1 descriptor"]
    VALUE1,
    #[doc = "63: 64 descriptors"]
    VALUE2,
}
impl From<CTD_A> for u8 {
    #[inline(always)]
    fn from(variant: CTD_A) -> Self {
        match variant {
            CTD_A::VALUE1 => 0,
            CTD_A::VALUE2 => 63,
        }
    }
}
#[doc = "Reader of field `CTD`"]
pub type CTD_R = crate::R<u8, CTD_A>;
impl CTD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CTD_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CTD_A::VALUE1),
            63 => Val(CTD_A::VALUE2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CTD_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CTD_A::VALUE2
    }
}
#[doc = "Write proxy for field `CTD`"]
pub struct CTD_W<'a> {
    w: &'a mut W,
}
impl<'a> CTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1 descriptor"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CTD_A::VALUE1)
    }
    #[doc = "64 descriptors"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CTD_A::VALUE2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 3)) | (((value as u32) & 0x3f) << 3);
        self.w
    }
}
#[doc = "Reader of field `DMAAddr`"]
pub type DMAADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DMAAddr`"]
pub struct DMAADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x007f_ffff << 9)) | (((value as u32) & 0x007f_ffff) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:8 - Current Transfer Desc:"]
    #[inline(always)]
    pub fn ctd(&self) -> CTD_R {
        CTD_R::new(((self.bits >> 3) & 0x3f) as u8)
    }
    #[doc = "Bits 9:31 - DMA Address"]
    #[inline(always)]
    pub fn dmaaddr(&self) -> DMAADDR_R {
        DMAADDR_R::new(((self.bits >> 9) & 0x007f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 3:8 - Current Transfer Desc:"]
    #[inline(always)]
    pub fn ctd(&mut self) -> CTD_W {
        CTD_W { w: self }
    }
    #[doc = "Bits 9:31 - DMA Address"]
    #[inline(always)]
    pub fn dmaaddr(&mut self) -> DMAADDR_W {
        DMAADDR_W { w: self }
    }
}

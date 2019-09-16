#[doc = "Reader of register DSR"]
pub type R = crate::R<u32, super::DSR>;
#[doc = "Writer for register DSR"]
pub type W = crate::W<u32, super::DSR>;
#[doc = "Register DSR `reset()`'s with value 0"]
impl crate::ResetValue for super::DSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DSC`"]
pub type DSC_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DSC`"]
pub struct DSC_W<'a> {
    w: &'a mut W,
}
impl<'a> DSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 20)) | (((value as u32) & 0x0fff) << 20);
        self.w
    }
}
#[doc = "Reader of field `DSI`"]
pub type DSI_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DSI`"]
pub struct DSI_W<'a> {
    w: &'a mut W,
}
impl<'a> DSI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | ((value as u32) & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 20:31 - Destination scatter count"]
    #[inline(always)]
    pub fn dsc(&self) -> DSC_R {
        DSC_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
    #[doc = "Bits 0:19 - Destination scatter interval"]
    #[inline(always)]
    pub fn dsi(&self) -> DSI_R {
        DSI_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 20:31 - Destination scatter count"]
    #[inline(always)]
    pub fn dsc(&mut self) -> DSC_W {
        DSC_W { w: self }
    }
    #[doc = "Bits 0:19 - Destination scatter interval"]
    #[inline(always)]
    pub fn dsi(&mut self) -> DSI_W {
        DSI_W { w: self }
    }
}

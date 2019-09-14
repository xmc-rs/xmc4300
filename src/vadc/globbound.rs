#[doc = "Reader of register GLOBBOUND"]
pub type R = crate::R<u32, super::GLOBBOUND>;
#[doc = "Writer for register GLOBBOUND"]
pub type W = crate::W<u32, super::GLOBBOUND>;
#[doc = "Register GLOBBOUND `reset()`'s with value 0"]
impl crate::ResetValue for super::GLOBBOUND {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BOUNDARY0`"]
pub type BOUNDARY0_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BOUNDARY0`"]
pub struct BOUNDARY0_W<'a> {
    w: &'a mut W,
}
impl<'a> BOUNDARY0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
#[doc = "Reader of field `BOUNDARY1`"]
pub type BOUNDARY1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BOUNDARY1`"]
pub struct BOUNDARY1_W<'a> {
    w: &'a mut W,
}
impl<'a> BOUNDARY1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | (((value as u32) & 0x0fff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Boundary Value 0 for Limit Checking"]
    #[inline(always)]
    pub fn boundary0(&self) -> BOUNDARY0_R {
        BOUNDARY0_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Boundary Value 1 for Limit Checking"]
    #[inline(always)]
    pub fn boundary1(&self) -> BOUNDARY1_R {
        BOUNDARY1_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Boundary Value 0 for Limit Checking"]
    #[inline(always)]
    pub fn boundary0(&mut self) -> BOUNDARY0_W {
        BOUNDARY0_W { w: self }
    }
    #[doc = "Bits 16:27 - Boundary Value 1 for Limit Checking"]
    #[inline(always)]
    pub fn boundary1(&mut self) -> BOUNDARY1_W {
        BOUNDARY1_W { w: self }
    }
}

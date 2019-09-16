#[doc = "Reader of register IOCR12"]
pub type R = crate::R<u32, super::IOCR12>;
#[doc = "Writer for register IOCR12"]
pub type W = crate::W<u32, super::IOCR12>;
#[doc = "Register IOCR12 `reset()`'s with value 0"]
impl crate::ResetValue for super::IOCR12 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PC12`"]
pub type PC12_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PC12`"]
pub struct PC12_W<'a> {
    w: &'a mut W,
}
impl<'a> PC12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 3)) | (((value as u32) & 0x1f) << 3);
        self.w
    }
}
#[doc = "Reader of field `PC13`"]
pub type PC13_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PC13`"]
pub struct PC13_W<'a> {
    w: &'a mut W,
}
impl<'a> PC13_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 11)) | (((value as u32) & 0x1f) << 11);
        self.w
    }
}
#[doc = "Reader of field `PC14`"]
pub type PC14_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PC14`"]
pub struct PC14_W<'a> {
    w: &'a mut W,
}
impl<'a> PC14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 19)) | (((value as u32) & 0x1f) << 19);
        self.w
    }
}
#[doc = "Reader of field `PC15`"]
pub type PC15_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PC15`"]
pub struct PC15_W<'a> {
    w: &'a mut W,
}
impl<'a> PC15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 27)) | (((value as u32) & 0x1f) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:7 - Port Control for Port n Pin 12 to 15"]
    #[inline(always)]
    pub fn pc12(&self) -> PC12_R {
        PC12_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - Port Control for Port n Pin 12 to 15"]
    #[inline(always)]
    pub fn pc13(&self) -> PC13_R {
        PC13_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 19:23 - Port Control for Port n Pin 12 to 15"]
    #[inline(always)]
    pub fn pc14(&self) -> PC14_R {
        PC14_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 27:31 - Port Control for Port n Pin 12 to 15"]
    #[inline(always)]
    pub fn pc15(&self) -> PC15_R {
        PC15_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 3:7 - Port Control for Port n Pin 12 to 15"]
    #[inline(always)]
    pub fn pc12(&mut self) -> PC12_W {
        PC12_W { w: self }
    }
    #[doc = "Bits 11:15 - Port Control for Port n Pin 12 to 15"]
    #[inline(always)]
    pub fn pc13(&mut self) -> PC13_W {
        PC13_W { w: self }
    }
    #[doc = "Bits 19:23 - Port Control for Port n Pin 12 to 15"]
    #[inline(always)]
    pub fn pc14(&mut self) -> PC14_W {
        PC14_W { w: self }
    }
    #[doc = "Bits 27:31 - Port Control for Port n Pin 12 to 15"]
    #[inline(always)]
    pub fn pc15(&mut self) -> PC15_W {
        PC15_W { w: self }
    }
}

#[doc = "Reader of register IOCR4"]
pub type R = crate::R<u32, super::IOCR4>;
#[doc = "Writer for register IOCR4"]
pub type W = crate::W<u32, super::IOCR4>;
#[doc = "Register IOCR4 `reset()`'s with value 0"]
impl crate::ResetValue for super::IOCR4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PC4`"]
pub type PC4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PC4`"]
pub struct PC4_W<'a> {
    w: &'a mut W,
}
impl<'a> PC4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 3)) | (((value as u32) & 0x1f) << 3);
        self.w
    }
}
#[doc = "Reader of field `PC5`"]
pub type PC5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PC5`"]
pub struct PC5_W<'a> {
    w: &'a mut W,
}
impl<'a> PC5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 11)) | (((value as u32) & 0x1f) << 11);
        self.w
    }
}
#[doc = "Reader of field `PC6`"]
pub type PC6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PC6`"]
pub struct PC6_W<'a> {
    w: &'a mut W,
}
impl<'a> PC6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 19)) | (((value as u32) & 0x1f) << 19);
        self.w
    }
}
#[doc = "Reader of field `PC7`"]
pub type PC7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PC7`"]
pub struct PC7_W<'a> {
    w: &'a mut W,
}
impl<'a> PC7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 27)) | (((value as u32) & 0x1f) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:7 - Port Control for Port n Pin 4 to 7"]
    #[inline(always)]
    pub fn pc4(&self) -> PC4_R {
        PC4_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - Port Control for Port n Pin 4 to 7"]
    #[inline(always)]
    pub fn pc5(&self) -> PC5_R {
        PC5_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 19:23 - Port Control for Port n Pin 4 to 7"]
    #[inline(always)]
    pub fn pc6(&self) -> PC6_R {
        PC6_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 27:31 - Port Control for Port n Pin 4 to 7"]
    #[inline(always)]
    pub fn pc7(&self) -> PC7_R {
        PC7_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 3:7 - Port Control for Port n Pin 4 to 7"]
    #[inline(always)]
    pub fn pc4(&mut self) -> PC4_W {
        PC4_W { w: self }
    }
    #[doc = "Bits 11:15 - Port Control for Port n Pin 4 to 7"]
    #[inline(always)]
    pub fn pc5(&mut self) -> PC5_W {
        PC5_W { w: self }
    }
    #[doc = "Bits 19:23 - Port Control for Port n Pin 4 to 7"]
    #[inline(always)]
    pub fn pc6(&mut self) -> PC6_W {
        PC6_W { w: self }
    }
    #[doc = "Bits 27:31 - Port Control for Port n Pin 4 to 7"]
    #[inline(always)]
    pub fn pc7(&mut self) -> PC7_W {
        PC7_W { w: self }
    }
}

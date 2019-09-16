#[doc = "Reader of register IOCR8"]
pub type R = crate::R<u32, super::IOCR8>;
#[doc = "Writer for register IOCR8"]
pub type W = crate::W<u32, super::IOCR8>;
#[doc = "Register IOCR8 `reset()`'s with value 0"]
impl crate::ResetValue for super::IOCR8 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PC8`"]
pub type PC8_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PC8`"]
pub struct PC8_W<'a> {
    w: &'a mut W,
}
impl<'a> PC8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 3)) | (((value as u32) & 0x1f) << 3);
        self.w
    }
}
#[doc = "Reader of field `PC9`"]
pub type PC9_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PC9`"]
pub struct PC9_W<'a> {
    w: &'a mut W,
}
impl<'a> PC9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 11)) | (((value as u32) & 0x1f) << 11);
        self.w
    }
}
#[doc = "Reader of field `PC10`"]
pub type PC10_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PC10`"]
pub struct PC10_W<'a> {
    w: &'a mut W,
}
impl<'a> PC10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 19)) | (((value as u32) & 0x1f) << 19);
        self.w
    }
}
#[doc = "Reader of field `PC11`"]
pub type PC11_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PC11`"]
pub struct PC11_W<'a> {
    w: &'a mut W,
}
impl<'a> PC11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 27)) | (((value as u32) & 0x1f) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:7 - Port Control for Port n Pin 8 to 11"]
    #[inline(always)]
    pub fn pc8(&self) -> PC8_R {
        PC8_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - Port Control for Port n Pin 8 to 11"]
    #[inline(always)]
    pub fn pc9(&self) -> PC9_R {
        PC9_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 19:23 - Port Control for Port n Pin 8 to 11"]
    #[inline(always)]
    pub fn pc10(&self) -> PC10_R {
        PC10_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 27:31 - Port Control for Port n Pin 8 to 11"]
    #[inline(always)]
    pub fn pc11(&self) -> PC11_R {
        PC11_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 3:7 - Port Control for Port n Pin 8 to 11"]
    #[inline(always)]
    pub fn pc8(&mut self) -> PC8_W {
        PC8_W { w: self }
    }
    #[doc = "Bits 11:15 - Port Control for Port n Pin 8 to 11"]
    #[inline(always)]
    pub fn pc9(&mut self) -> PC9_W {
        PC9_W { w: self }
    }
    #[doc = "Bits 19:23 - Port Control for Port n Pin 8 to 11"]
    #[inline(always)]
    pub fn pc10(&mut self) -> PC10_W {
        PC10_W { w: self }
    }
    #[doc = "Bits 27:31 - Port Control for Port n Pin 8 to 11"]
    #[inline(always)]
    pub fn pc11(&mut self) -> PC11_W {
        PC11_W { w: self }
    }
}

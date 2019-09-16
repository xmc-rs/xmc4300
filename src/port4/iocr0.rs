#[doc = "Reader of register IOCR0"]
pub type R = crate::R<u32, super::IOCR0>;
#[doc = "Writer for register IOCR0"]
pub type W = crate::W<u32, super::IOCR0>;
#[doc = "Register IOCR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::IOCR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PC0`"]
pub type PC0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PC0`"]
pub struct PC0_W<'a> {
    w: &'a mut W,
}
impl<'a> PC0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 3)) | (((value as u32) & 0x1f) << 3);
        self.w
    }
}
#[doc = "Reader of field `PC1`"]
pub type PC1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PC1`"]
pub struct PC1_W<'a> {
    w: &'a mut W,
}
impl<'a> PC1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 11)) | (((value as u32) & 0x1f) << 11);
        self.w
    }
}
#[doc = "Reader of field `PC2`"]
pub type PC2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PC2`"]
pub struct PC2_W<'a> {
    w: &'a mut W,
}
impl<'a> PC2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 19)) | (((value as u32) & 0x1f) << 19);
        self.w
    }
}
#[doc = "Reader of field `PC3`"]
pub type PC3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PC3`"]
pub struct PC3_W<'a> {
    w: &'a mut W,
}
impl<'a> PC3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 27)) | (((value as u32) & 0x1f) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:7 - Port Control for Port n Pin 0 to 3"]
    #[inline(always)]
    pub fn pc0(&self) -> PC0_R {
        PC0_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - Port Control for Port n Pin 0 to 3"]
    #[inline(always)]
    pub fn pc1(&self) -> PC1_R {
        PC1_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 19:23 - Port Control for Port n Pin 0 to 3"]
    #[inline(always)]
    pub fn pc2(&self) -> PC2_R {
        PC2_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 27:31 - Port Control for Port n Pin 0 to 3"]
    #[inline(always)]
    pub fn pc3(&self) -> PC3_R {
        PC3_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 3:7 - Port Control for Port n Pin 0 to 3"]
    #[inline(always)]
    pub fn pc0(&mut self) -> PC0_W {
        PC0_W { w: self }
    }
    #[doc = "Bits 11:15 - Port Control for Port n Pin 0 to 3"]
    #[inline(always)]
    pub fn pc1(&mut self) -> PC1_W {
        PC1_W { w: self }
    }
    #[doc = "Bits 19:23 - Port Control for Port n Pin 0 to 3"]
    #[inline(always)]
    pub fn pc2(&mut self) -> PC2_W {
        PC2_W { w: self }
    }
    #[doc = "Bits 27:31 - Port Control for Port n Pin 0 to 3"]
    #[inline(always)]
    pub fn pc3(&mut self) -> PC3_W {
        PC3_W { w: self }
    }
}

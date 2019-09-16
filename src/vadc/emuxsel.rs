#[doc = "Reader of register EMUXSEL"]
pub type R = crate::R<u32, super::EMUXSEL>;
#[doc = "Writer for register EMUXSEL"]
pub type W = crate::W<u32, super::EMUXSEL>;
#[doc = "Register EMUXSEL `reset()`'s with value 0"]
impl crate::ResetValue for super::EMUXSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EMUXGRP0`"]
pub type EMUXGRP0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EMUXGRP0`"]
pub struct EMUXGRP0_W<'a> {
    w: &'a mut W,
}
impl<'a> EMUXGRP0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `EMUXGRP1`"]
pub type EMUXGRP1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EMUXGRP1`"]
pub struct EMUXGRP1_W<'a> {
    w: &'a mut W,
}
impl<'a> EMUXGRP1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - External Multiplexer Group for Interface x"]
    #[inline(always)]
    pub fn emuxgrp0(&self) -> EMUXGRP0_R {
        EMUXGRP0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - External Multiplexer Group for Interface x"]
    #[inline(always)]
    pub fn emuxgrp1(&self) -> EMUXGRP1_R {
        EMUXGRP1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - External Multiplexer Group for Interface x"]
    #[inline(always)]
    pub fn emuxgrp0(&mut self) -> EMUXGRP0_W {
        EMUXGRP0_W { w: self }
    }
    #[doc = "Bits 4:7 - External Multiplexer Group for Interface x"]
    #[inline(always)]
    pub fn emuxgrp1(&mut self) -> EMUXGRP1_W {
        EMUXGRP1_W { w: self }
    }
}

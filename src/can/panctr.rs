#[doc = "Reader of register PANCTR"]
pub type R = crate::R<u32, super::PANCTR>;
#[doc = "Writer for register PANCTR"]
pub type W = crate::W<u32, super::PANCTR>;
#[doc = "Register PANCTR `reset()`'s with value 0x0301"]
impl crate::ResetValue for super::PANCTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0301
    }
}
#[doc = "Reader of field `PANCMD`"]
pub type PANCMD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PANCMD`"]
pub struct PANCMD_W<'a> {
    w: &'a mut W,
}
impl<'a> PANCMD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Panel Busy Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSY_A {
    #[doc = "0: Panel has finished command and is ready to accept a new command."]
    VALUE1,
    #[doc = "1: Panel operation is in progress."]
    VALUE2,
}
impl From<BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY_A) -> Self {
        match variant {
            BUSY_A::VALUE1 => false,
            BUSY_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `BUSY`"]
pub type BUSY_R = crate::R<bool, BUSY_A>;
impl BUSY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY_A {
        match self.bits {
            false => BUSY_A::VALUE1,
            true => BUSY_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BUSY_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BUSY_A::VALUE2
    }
}
#[doc = "Result Busy Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RBUSY_A {
    #[doc = "0: No update of PANAR1 and PANAR2 is scheduled by the list controller."]
    VALUE1,
    #[doc = "1: A list command is running (BUSY = 1) that will write results to PANAR1 and PANAR2, but the results are not yet available."]
    VALUE2,
}
impl From<RBUSY_A> for bool {
    #[inline(always)]
    fn from(variant: RBUSY_A) -> Self {
        match variant {
            RBUSY_A::VALUE1 => false,
            RBUSY_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `RBUSY`"]
pub type RBUSY_R = crate::R<bool, RBUSY_A>;
impl RBUSY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RBUSY_A {
        match self.bits {
            false => RBUSY_A::VALUE1,
            true => RBUSY_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RBUSY_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RBUSY_A::VALUE2
    }
}
#[doc = "Reader of field `PANAR1`"]
pub type PANAR1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PANAR1`"]
pub struct PANAR1_W<'a> {
    w: &'a mut W,
}
impl<'a> PANAR1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `PANAR2`"]
pub type PANAR2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PANAR2`"]
pub struct PANAR2_W<'a> {
    w: &'a mut W,
}
impl<'a> PANAR2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Panel Command"]
    #[inline(always)]
    pub fn pancmd(&self) -> PANCMD_R {
        PANCMD_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Panel Busy Flag"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Result Busy Flag"]
    #[inline(always)]
    pub fn rbusy(&self) -> RBUSY_R {
        RBUSY_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - Panel Argument 1"]
    #[inline(always)]
    pub fn panar1(&self) -> PANAR1_R {
        PANAR1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Panel Argument 2"]
    #[inline(always)]
    pub fn panar2(&self) -> PANAR2_R {
        PANAR2_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Panel Command"]
    #[inline(always)]
    pub fn pancmd(&mut self) -> PANCMD_W {
        PANCMD_W { w: self }
    }
    #[doc = "Bits 16:23 - Panel Argument 1"]
    #[inline(always)]
    pub fn panar1(&mut self) -> PANAR1_W {
        PANAR1_W { w: self }
    }
    #[doc = "Bits 24:31 - Panel Argument 2"]
    #[inline(always)]
    pub fn panar2(&mut self) -> PANAR2_W {
        PANAR2_W { w: self }
    }
}

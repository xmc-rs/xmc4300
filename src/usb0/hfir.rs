#[doc = "Reader of register HFIR"]
pub type R = crate::R<u32, super::HFIR>;
#[doc = "Writer for register HFIR"]
pub type W = crate::W<u32, super::HFIR>;
#[doc = "Register HFIR `reset()`'s with value 0xea60"]
impl crate::ResetValue for super::HFIR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xea60
    }
}
#[doc = "Reader of field `FrInt`"]
pub type FRINT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `FrInt`"]
pub struct FRINT_W<'a> {
    w: &'a mut W,
}
impl<'a> FRINT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reload Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HFIRRLDCTRL_A {
    #[doc = "0: HFIR cannot be reloaded dynamically"]
    VALUE1,
    #[doc = "1: HFIR can be dynamically reloaded during runtime"]
    VALUE2,
}
impl From<HFIRRLDCTRL_A> for bool {
    #[inline(always)]
    fn from(variant: HFIRRLDCTRL_A) -> Self {
        match variant {
            HFIRRLDCTRL_A::VALUE1 => false,
            HFIRRLDCTRL_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `HFIRRldCtrl`"]
pub type HFIRRLDCTRL_R = crate::R<bool, HFIRRLDCTRL_A>;
impl HFIRRLDCTRL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HFIRRLDCTRL_A {
        match self.bits {
            false => HFIRRLDCTRL_A::VALUE1,
            true => HFIRRLDCTRL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HFIRRLDCTRL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HFIRRLDCTRL_A::VALUE2
    }
}
#[doc = "Write proxy for field `HFIRRldCtrl`"]
pub struct HFIRRLDCTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> HFIRRLDCTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HFIRRLDCTRL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "HFIR cannot be reloaded dynamically"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(HFIRRLDCTRL_A::VALUE1)
    }
    #[doc = "HFIR can be dynamically reloaded during runtime"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(HFIRRLDCTRL_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Frame Interval"]
    #[inline(always)]
    pub fn fr_int(&self) -> FRINT_R {
        FRINT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Reload Control"]
    #[inline(always)]
    pub fn hfirrld_ctrl(&self) -> HFIRRLDCTRL_R {
        HFIRRLDCTRL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Frame Interval"]
    #[inline(always)]
    pub fn fr_int(&mut self) -> FRINT_W {
        FRINT_W { w: self }
    }
    #[doc = "Bit 16 - Reload Control"]
    #[inline(always)]
    pub fn hfirrld_ctrl(&mut self) -> HFIRRLDCTRL_W {
        HFIRRLDCTRL_W { w: self }
    }
}

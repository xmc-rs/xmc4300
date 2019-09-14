#[doc = "Writer for register PWRSET"]
pub type W = crate::W<u32, super::PWRSET>;
#[doc = "Register PWRSET `reset()`'s with value 0"]
impl crate::ResetValue for super::PWRSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Set Hibernate Domain Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HIB_AW {
    #[doc = "0: No effect"]
    CONST_0,
    #[doc = "1: Enable Hibernate domain"]
    CONST_1,
}
impl From<HIB_AW> for bool {
    #[inline(always)]
    fn from(variant: HIB_AW) -> Self {
        match variant {
            HIB_AW::CONST_0 => false,
            HIB_AW::CONST_1 => true,
        }
    }
}
#[doc = "Write proxy for field `HIB`"]
pub struct HIB_W<'a> {
    w: &'a mut W,
}
impl<'a> HIB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HIB_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(HIB_AW::CONST_0)
    }
    #[doc = "Enable Hibernate domain"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(HIB_AW::CONST_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Set USB PHY Transceiver Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBPHYPDQ_AW {
    #[doc = "0: No effect"]
    CONST_0,
    #[doc = "1: Active"]
    CONST_1,
}
impl From<USBPHYPDQ_AW> for bool {
    #[inline(always)]
    fn from(variant: USBPHYPDQ_AW) -> Self {
        match variant {
            USBPHYPDQ_AW::CONST_0 => false,
            USBPHYPDQ_AW::CONST_1 => true,
        }
    }
}
#[doc = "Write proxy for field `USBPHYPDQ`"]
pub struct USBPHYPDQ_W<'a> {
    w: &'a mut W,
}
impl<'a> USBPHYPDQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBPHYPDQ_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(USBPHYPDQ_AW::CONST_0)
    }
    #[doc = "Active"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(USBPHYPDQ_AW::CONST_1)
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
#[doc = "Set USB On-The-Go Comparators Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBOTGEN_AW {
    #[doc = "0: No effect"]
    CONST_0,
    #[doc = "1: Active"]
    CONST_1,
}
impl From<USBOTGEN_AW> for bool {
    #[inline(always)]
    fn from(variant: USBOTGEN_AW) -> Self {
        match variant {
            USBOTGEN_AW::CONST_0 => false,
            USBOTGEN_AW::CONST_1 => true,
        }
    }
}
#[doc = "Write proxy for field `USBOTGEN`"]
pub struct USBOTGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USBOTGEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBOTGEN_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(USBOTGEN_AW::CONST_0)
    }
    #[doc = "Active"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(USBOTGEN_AW::CONST_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Set USB Weak Pull-Up at PADN Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBPUWQ_AW {
    #[doc = "0: No effect"]
    CONST_0,
    #[doc = "1: Pull-up not active"]
    CONST_1,
}
impl From<USBPUWQ_AW> for bool {
    #[inline(always)]
    fn from(variant: USBPUWQ_AW) -> Self {
        match variant {
            USBPUWQ_AW::CONST_0 => false,
            USBPUWQ_AW::CONST_1 => true,
        }
    }
}
#[doc = "Write proxy for field `USBPUWQ`"]
pub struct USBPUWQ_W<'a> {
    w: &'a mut W,
}
impl<'a> USBPUWQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBPUWQ_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(USBPUWQ_AW::CONST_0)
    }
    #[doc = "Pull-up not active"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(USBPUWQ_AW::CONST_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Set Hibernate Domain Enable"]
    #[inline(always)]
    pub fn hib(&mut self) -> HIB_W {
        HIB_W { w: self }
    }
    #[doc = "Bit 16 - Set USB PHY Transceiver Disable"]
    #[inline(always)]
    pub fn usbphypdq(&mut self) -> USBPHYPDQ_W {
        USBPHYPDQ_W { w: self }
    }
    #[doc = "Bit 17 - Set USB On-The-Go Comparators Enable"]
    #[inline(always)]
    pub fn usbotgen(&mut self) -> USBOTGEN_W {
        USBOTGEN_W { w: self }
    }
    #[doc = "Bit 18 - Set USB Weak Pull-Up at PADN Enable"]
    #[inline(always)]
    pub fn usbpuwq(&mut self) -> USBPUWQ_W {
        USBPUWQ_W { w: self }
    }
}

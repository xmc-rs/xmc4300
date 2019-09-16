#[doc = "Reader of register SM_PDI_CTR"]
pub type R = crate::R<u8, super::SM_PDI_CTR>;
#[doc = "Writer for register SM_PDI_CTR"]
pub type W = crate::W<u8, super::SM_PDI_CTR>;
#[doc = "Register SM_PDI_CTR `reset()`'s with value 0"]
impl crate::ResetValue for super::SM_PDI_CTR {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Deactivate SyncManager\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEACT_A {
    #[doc = "0: Read 0 for Normal operation, SyncManager activated, write 0 for Activate SyncManager"]
    VALUE1,
    #[doc = "1: Read 1 for SyncManager deactivated and reset SyncManager locks access to Memory area, write 1 for Request SyncManager deactivation"]
    VALUE2,
}
impl From<DEACT_A> for bool {
    #[inline(always)]
    fn from(variant: DEACT_A) -> Self {
        match variant {
            DEACT_A::VALUE1 => false,
            DEACT_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `DEACT`"]
pub type DEACT_R = crate::R<bool, DEACT_A>;
impl DEACT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEACT_A {
        match self.bits {
            false => DEACT_A::VALUE1,
            true => DEACT_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DEACT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DEACT_A::VALUE2
    }
}
#[doc = "Write proxy for field `DEACT`"]
pub struct DEACT_W<'a> {
    w: &'a mut W,
}
impl<'a> DEACT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DEACT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read 0 for Normal operation, SyncManager activated, write 0 for Activate SyncManager"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DEACT_A::VALUE1)
    }
    #[doc = "Read 1 for SyncManager deactivated and reset SyncManager locks access to Memory area, write 1 for Request SyncManager deactivation"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DEACT_A::VALUE2)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `REP_ACK`"]
pub type REP_ACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REP_ACK`"]
pub struct REP_ACK_W<'a> {
    w: &'a mut W,
}
impl<'a> REP_ACK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u8) & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Deactivate SyncManager"]
    #[inline(always)]
    pub fn deact(&self) -> DEACT_R {
        DEACT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Repeat Ack"]
    #[inline(always)]
    pub fn rep_ack(&self) -> REP_ACK_R {
        REP_ACK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Deactivate SyncManager"]
    #[inline(always)]
    pub fn deact(&mut self) -> DEACT_W {
        DEACT_W { w: self }
    }
    #[doc = "Bit 1 - Repeat Ack"]
    #[inline(always)]
    pub fn rep_ack(&mut self) -> REP_ACK_W {
        REP_ACK_W { w: self }
    }
}

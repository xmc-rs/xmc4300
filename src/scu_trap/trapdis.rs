#[doc = "Reader of register TRAPDIS"]
pub type R = crate::R<u32, super::TRAPDIS>;
#[doc = "Writer for register TRAPDIS"]
pub type W = crate::W<u32, super::TRAPDIS>;
#[doc = "Register TRAPDIS `reset()`'s with value 0x0001_01fd"]
impl crate::ResetValue for super::TRAPDIS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0001_01fd
    }
}
#[doc = "OSC_HP Oscillator Watchdog Trap Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOSCWDGT_A {
    #[doc = "0: Trap request enabled"]
    CONST_0,
    #[doc = "1: Trap request disabled"]
    CONST_1,
}
impl From<SOSCWDGT_A> for bool {
    #[inline(always)]
    fn from(variant: SOSCWDGT_A) -> Self {
        match variant {
            SOSCWDGT_A::CONST_0 => false,
            SOSCWDGT_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `SOSCWDGT`"]
pub type SOSCWDGT_R = crate::R<bool, SOSCWDGT_A>;
impl SOSCWDGT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOSCWDGT_A {
        match self.bits {
            false => SOSCWDGT_A::CONST_0,
            true => SOSCWDGT_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == SOSCWDGT_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == SOSCWDGT_A::CONST_1
    }
}
#[doc = "Write proxy for field `SOSCWDGT`"]
pub struct SOSCWDGT_W<'a> {
    w: &'a mut W,
}
impl<'a> SOSCWDGT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SOSCWDGT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Trap request enabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(SOSCWDGT_A::CONST_0)
    }
    #[doc = "Trap request disabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(SOSCWDGT_A::CONST_1)
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
#[doc = "System VCO Lock Trap Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SVCOLCKT_A {
    #[doc = "0: Trap request enabled"]
    CONST_0,
    #[doc = "1: Trap request disabled"]
    CONST_1,
}
impl From<SVCOLCKT_A> for bool {
    #[inline(always)]
    fn from(variant: SVCOLCKT_A) -> Self {
        match variant {
            SVCOLCKT_A::CONST_0 => false,
            SVCOLCKT_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `SVCOLCKT`"]
pub type SVCOLCKT_R = crate::R<bool, SVCOLCKT_A>;
impl SVCOLCKT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SVCOLCKT_A {
        match self.bits {
            false => SVCOLCKT_A::CONST_0,
            true => SVCOLCKT_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == SVCOLCKT_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == SVCOLCKT_A::CONST_1
    }
}
#[doc = "Write proxy for field `SVCOLCKT`"]
pub struct SVCOLCKT_W<'a> {
    w: &'a mut W,
}
impl<'a> SVCOLCKT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SVCOLCKT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Trap request enabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(SVCOLCKT_A::CONST_0)
    }
    #[doc = "Trap request disabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(SVCOLCKT_A::CONST_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "USB VCO Lock Trap Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UVCOLCKT_A {
    #[doc = "0: Trap request enabled"]
    CONST_0,
    #[doc = "1: Trap request disabled"]
    CONST_1,
}
impl From<UVCOLCKT_A> for bool {
    #[inline(always)]
    fn from(variant: UVCOLCKT_A) -> Self {
        match variant {
            UVCOLCKT_A::CONST_0 => false,
            UVCOLCKT_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `UVCOLCKT`"]
pub type UVCOLCKT_R = crate::R<bool, UVCOLCKT_A>;
impl UVCOLCKT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UVCOLCKT_A {
        match self.bits {
            false => UVCOLCKT_A::CONST_0,
            true => UVCOLCKT_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == UVCOLCKT_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == UVCOLCKT_A::CONST_1
    }
}
#[doc = "Write proxy for field `UVCOLCKT`"]
pub struct UVCOLCKT_W<'a> {
    w: &'a mut W,
}
impl<'a> UVCOLCKT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UVCOLCKT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Trap request enabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(UVCOLCKT_A::CONST_0)
    }
    #[doc = "Trap request disabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(UVCOLCKT_A::CONST_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Parity Error Trap Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PET_A {
    #[doc = "0: Trap request enabled"]
    CONST_0,
    #[doc = "1: Trap request disabled"]
    CONST_1,
}
impl From<PET_A> for bool {
    #[inline(always)]
    fn from(variant: PET_A) -> Self {
        match variant {
            PET_A::CONST_0 => false,
            PET_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `PET`"]
pub type PET_R = crate::R<bool, PET_A>;
impl PET_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PET_A {
        match self.bits {
            false => PET_A::CONST_0,
            true => PET_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PET_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PET_A::CONST_1
    }
}
#[doc = "Write proxy for field `PET`"]
pub struct PET_W<'a> {
    w: &'a mut W,
}
impl<'a> PET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PET_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Trap request enabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PET_A::CONST_0)
    }
    #[doc = "Trap request disabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PET_A::CONST_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Brown Out Trap Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRWNT_A {
    #[doc = "0: Trap request enabled"]
    CONST_0,
    #[doc = "1: Trap request disabled"]
    CONST_1,
}
impl From<BRWNT_A> for bool {
    #[inline(always)]
    fn from(variant: BRWNT_A) -> Self {
        match variant {
            BRWNT_A::CONST_0 => false,
            BRWNT_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `BRWNT`"]
pub type BRWNT_R = crate::R<bool, BRWNT_A>;
impl BRWNT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRWNT_A {
        match self.bits {
            false => BRWNT_A::CONST_0,
            true => BRWNT_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == BRWNT_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == BRWNT_A::CONST_1
    }
}
#[doc = "Write proxy for field `BRWNT`"]
pub struct BRWNT_W<'a> {
    w: &'a mut W,
}
impl<'a> BRWNT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BRWNT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Trap request enabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(BRWNT_A::CONST_0)
    }
    #[doc = "Trap request disabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(BRWNT_A::CONST_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "OSC_ULP Oscillator Watchdog Trap Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ULPWDGT_A {
    #[doc = "0: Trap request enabled"]
    CONST_0,
    #[doc = "1: Trap request disabled"]
    CONST_1,
}
impl From<ULPWDGT_A> for bool {
    #[inline(always)]
    fn from(variant: ULPWDGT_A) -> Self {
        match variant {
            ULPWDGT_A::CONST_0 => false,
            ULPWDGT_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `ULPWDGT`"]
pub type ULPWDGT_R = crate::R<bool, ULPWDGT_A>;
impl ULPWDGT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ULPWDGT_A {
        match self.bits {
            false => ULPWDGT_A::CONST_0,
            true => ULPWDGT_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == ULPWDGT_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == ULPWDGT_A::CONST_1
    }
}
#[doc = "Write proxy for field `ULPWDGT`"]
pub struct ULPWDGT_W<'a> {
    w: &'a mut W,
}
impl<'a> ULPWDGT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ULPWDGT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Trap request enabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(ULPWDGT_A::CONST_0)
    }
    #[doc = "Trap request disabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(ULPWDGT_A::CONST_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Peripheral Bridge 0 Trap Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BWERR0T_A {
    #[doc = "0: Trap request enabled"]
    CONST_0,
    #[doc = "1: Trap request disabled"]
    CONST_1,
}
impl From<BWERR0T_A> for bool {
    #[inline(always)]
    fn from(variant: BWERR0T_A) -> Self {
        match variant {
            BWERR0T_A::CONST_0 => false,
            BWERR0T_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `BWERR0T`"]
pub type BWERR0T_R = crate::R<bool, BWERR0T_A>;
impl BWERR0T_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BWERR0T_A {
        match self.bits {
            false => BWERR0T_A::CONST_0,
            true => BWERR0T_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == BWERR0T_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == BWERR0T_A::CONST_1
    }
}
#[doc = "Write proxy for field `BWERR0T`"]
pub struct BWERR0T_W<'a> {
    w: &'a mut W,
}
impl<'a> BWERR0T_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BWERR0T_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Trap request enabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(BWERR0T_A::CONST_0)
    }
    #[doc = "Trap request disabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(BWERR0T_A::CONST_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Peripheral Bridge 1 Trap Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BWERR1T_A {
    #[doc = "0: Trap request enabled"]
    CONST_0,
    #[doc = "1: Trap request disabled"]
    CONST_1,
}
impl From<BWERR1T_A> for bool {
    #[inline(always)]
    fn from(variant: BWERR1T_A) -> Self {
        match variant {
            BWERR1T_A::CONST_0 => false,
            BWERR1T_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `BWERR1T`"]
pub type BWERR1T_R = crate::R<bool, BWERR1T_A>;
impl BWERR1T_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BWERR1T_A {
        match self.bits {
            false => BWERR1T_A::CONST_0,
            true => BWERR1T_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == BWERR1T_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == BWERR1T_A::CONST_1
    }
}
#[doc = "Write proxy for field `BWERR1T`"]
pub struct BWERR1T_W<'a> {
    w: &'a mut W,
}
impl<'a> BWERR1T_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BWERR1T_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Trap request enabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(BWERR1T_A::CONST_0)
    }
    #[doc = "Trap request disabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(BWERR1T_A::CONST_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "EtherCat Reset 0 Trap Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECAT0RST_A {
    #[doc = "0: Trap request enabled"]
    CONST_0,
    #[doc = "1: Trap request disabled"]
    CONST_1,
}
impl From<ECAT0RST_A> for bool {
    #[inline(always)]
    fn from(variant: ECAT0RST_A) -> Self {
        match variant {
            ECAT0RST_A::CONST_0 => false,
            ECAT0RST_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `ECAT0RST`"]
pub type ECAT0RST_R = crate::R<bool, ECAT0RST_A>;
impl ECAT0RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECAT0RST_A {
        match self.bits {
            false => ECAT0RST_A::CONST_0,
            true => ECAT0RST_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == ECAT0RST_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == ECAT0RST_A::CONST_1
    }
}
#[doc = "Write proxy for field `ECAT0RST`"]
pub struct ECAT0RST_W<'a> {
    w: &'a mut W,
}
impl<'a> ECAT0RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ECAT0RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Trap request enabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(ECAT0RST_A::CONST_0)
    }
    #[doc = "Trap request disabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(ECAT0RST_A::CONST_1)
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
    #[doc = "Bit 0 - OSC_HP Oscillator Watchdog Trap Disable"]
    #[inline(always)]
    pub fn soscwdgt(&self) -> SOSCWDGT_R {
        SOSCWDGT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - System VCO Lock Trap Disable"]
    #[inline(always)]
    pub fn svcolckt(&self) -> SVCOLCKT_R {
        SVCOLCKT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - USB VCO Lock Trap Disable"]
    #[inline(always)]
    pub fn uvcolckt(&self) -> UVCOLCKT_R {
        UVCOLCKT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Parity Error Trap Disable"]
    #[inline(always)]
    pub fn pet(&self) -> PET_R {
        PET_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Brown Out Trap Disable"]
    #[inline(always)]
    pub fn brwnt(&self) -> BRWNT_R {
        BRWNT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - OSC_ULP Oscillator Watchdog Trap Disable"]
    #[inline(always)]
    pub fn ulpwdgt(&self) -> ULPWDGT_R {
        ULPWDGT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Peripheral Bridge 0 Trap Disable"]
    #[inline(always)]
    pub fn bwerr0t(&self) -> BWERR0T_R {
        BWERR0T_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Peripheral Bridge 1 Trap Disable"]
    #[inline(always)]
    pub fn bwerr1t(&self) -> BWERR1T_R {
        BWERR1T_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 16 - EtherCat Reset 0 Trap Disable"]
    #[inline(always)]
    pub fn ecat0rst(&self) -> ECAT0RST_R {
        ECAT0RST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OSC_HP Oscillator Watchdog Trap Disable"]
    #[inline(always)]
    pub fn soscwdgt(&mut self) -> SOSCWDGT_W {
        SOSCWDGT_W { w: self }
    }
    #[doc = "Bit 2 - System VCO Lock Trap Disable"]
    #[inline(always)]
    pub fn svcolckt(&mut self) -> SVCOLCKT_W {
        SVCOLCKT_W { w: self }
    }
    #[doc = "Bit 3 - USB VCO Lock Trap Disable"]
    #[inline(always)]
    pub fn uvcolckt(&mut self) -> UVCOLCKT_W {
        UVCOLCKT_W { w: self }
    }
    #[doc = "Bit 4 - Parity Error Trap Disable"]
    #[inline(always)]
    pub fn pet(&mut self) -> PET_W {
        PET_W { w: self }
    }
    #[doc = "Bit 5 - Brown Out Trap Disable"]
    #[inline(always)]
    pub fn brwnt(&mut self) -> BRWNT_W {
        BRWNT_W { w: self }
    }
    #[doc = "Bit 6 - OSC_ULP Oscillator Watchdog Trap Disable"]
    #[inline(always)]
    pub fn ulpwdgt(&mut self) -> ULPWDGT_W {
        ULPWDGT_W { w: self }
    }
    #[doc = "Bit 7 - Peripheral Bridge 0 Trap Disable"]
    #[inline(always)]
    pub fn bwerr0t(&mut self) -> BWERR0T_W {
        BWERR0T_W { w: self }
    }
    #[doc = "Bit 8 - Peripheral Bridge 1 Trap Disable"]
    #[inline(always)]
    pub fn bwerr1t(&mut self) -> BWERR1T_W {
        BWERR1T_W { w: self }
    }
    #[doc = "Bit 16 - EtherCat Reset 0 Trap Disable"]
    #[inline(always)]
    pub fn ecat0rst(&mut self) -> ECAT0RST_W {
        ECAT0RST_W { w: self }
    }
}

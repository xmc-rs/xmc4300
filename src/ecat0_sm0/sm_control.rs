#[doc = "Reader of register SM_CONTROL"]
pub type R = crate::R<u8, super::SM_CONTROL>;
#[doc = "Operation Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OP_MODE_A {
    #[doc = "0: Buffered (3 buffer mode)"]
    VALUE1,
    #[doc = "2: Mailbox (Single buffer mode)"]
    VALUE3,
}
impl From<OP_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: OP_MODE_A) -> Self {
        match variant {
            OP_MODE_A::VALUE1 => 0,
            OP_MODE_A::VALUE3 => 2,
        }
    }
}
#[doc = "Reader of field `OP_MODE`"]
pub type OP_MODE_R = crate::R<u8, OP_MODE_A>;
impl OP_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, OP_MODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(OP_MODE_A::VALUE1),
            2 => Val(OP_MODE_A::VALUE3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == OP_MODE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == OP_MODE_A::VALUE3
    }
}
#[doc = "Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIR_A {
    #[doc = "0: Read: ECAT read access, PDI write access"]
    VALUE1,
    #[doc = "1: Write: ECAT write access, PDI read access"]
    VALUE2,
}
impl From<DIR_A> for u8 {
    #[inline(always)]
    fn from(variant: DIR_A) -> Self {
        match variant {
            DIR_A::VALUE1 => 0,
            DIR_A::VALUE2 => 1,
        }
    }
}
#[doc = "Reader of field `DIR`"]
pub type DIR_R = crate::R<u8, DIR_A>;
impl DIR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DIR_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DIR_A::VALUE1),
            1 => Val(DIR_A::VALUE2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DIR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DIR_A::VALUE2
    }
}
#[doc = "Interrupt in ECAT Event Request Register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT_ECAT_A {
    #[doc = "0: Disabled"]
    VALUE1,
    #[doc = "1: Enabled"]
    VALUE2,
}
impl From<INT_ECAT_A> for bool {
    #[inline(always)]
    fn from(variant: INT_ECAT_A) -> Self {
        match variant {
            INT_ECAT_A::VALUE1 => false,
            INT_ECAT_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `INT_ECAT`"]
pub type INT_ECAT_R = crate::R<bool, INT_ECAT_A>;
impl INT_ECAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT_ECAT_A {
        match self.bits {
            false => INT_ECAT_A::VALUE1,
            true => INT_ECAT_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == INT_ECAT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == INT_ECAT_A::VALUE2
    }
}
#[doc = "Interrupt in PDI Event Request Register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT_PDI_A {
    #[doc = "0: Disabled"]
    VALUE1,
    #[doc = "1: Enabled"]
    VALUE2,
}
impl From<INT_PDI_A> for bool {
    #[inline(always)]
    fn from(variant: INT_PDI_A) -> Self {
        match variant {
            INT_PDI_A::VALUE1 => false,
            INT_PDI_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `INT_PDI`"]
pub type INT_PDI_R = crate::R<bool, INT_PDI_A>;
impl INT_PDI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT_PDI_A {
        match self.bits {
            false => INT_PDI_A::VALUE1,
            true => INT_PDI_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == INT_PDI_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == INT_PDI_A::VALUE2
    }
}
#[doc = "Watchdog Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WD_TRG_A {
    #[doc = "0: Disabled"]
    VALUE1,
    #[doc = "1: Enabled"]
    VALUE2,
}
impl From<WD_TRG_A> for bool {
    #[inline(always)]
    fn from(variant: WD_TRG_A) -> Self {
        match variant {
            WD_TRG_A::VALUE1 => false,
            WD_TRG_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `WD_TRG`"]
pub type WD_TRG_R = crate::R<bool, WD_TRG_A>;
impl WD_TRG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WD_TRG_A {
        match self.bits {
            false => WD_TRG_A::VALUE1,
            true => WD_TRG_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WD_TRG_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WD_TRG_A::VALUE2
    }
}
impl R {
    #[doc = "Bits 0:1 - Operation Mode"]
    #[inline(always)]
    pub fn op_mode(&self) -> OP_MODE_R {
        OP_MODE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Direction"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - Interrupt in ECAT Event Request Register"]
    #[inline(always)]
    pub fn int_ecat(&self) -> INT_ECAT_R {
        INT_ECAT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Interrupt in PDI Event Request Register"]
    #[inline(always)]
    pub fn int_pdi(&self) -> INT_PDI_R {
        INT_PDI_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Watchdog Trigger Enable"]
    #[inline(always)]
    pub fn wd_trg(&self) -> WD_TRG_R {
        WD_TRG_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}

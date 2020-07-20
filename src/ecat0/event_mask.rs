#[doc = "Reader of register EVENT_MASK"]
pub type R = crate::R<u16, super::EVENT_MASK>;
#[doc = "DC Latch event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DC_LE_MASK_A {
    #[doc = "0: Corresponding ECAT Event Request register bit is not mapped"]
    VALUE1 = 0,
    #[doc = "1: Corresponding ECAT Event Request register bit is mapped"]
    VALUE2 = 1,
}
impl From<DC_LE_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: DC_LE_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DC_LE_MASK`"]
pub type DC_LE_MASK_R = crate::R<bool, DC_LE_MASK_A>;
impl DC_LE_MASK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DC_LE_MASK_A {
        match self.bits {
            false => DC_LE_MASK_A::VALUE1,
            true => DC_LE_MASK_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DC_LE_MASK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DC_LE_MASK_A::VALUE2
    }
}
#[doc = "DL Status event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DL_SE_MASK_A {
    #[doc = "0: Corresponding ECAT Event Request register bit is not mapped"]
    VALUE1 = 0,
    #[doc = "1: Corresponding ECAT Event Request register bit is mapped"]
    VALUE2 = 1,
}
impl From<DL_SE_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: DL_SE_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DL_SE_MASK`"]
pub type DL_SE_MASK_R = crate::R<bool, DL_SE_MASK_A>;
impl DL_SE_MASK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DL_SE_MASK_A {
        match self.bits {
            false => DL_SE_MASK_A::VALUE1,
            true => DL_SE_MASK_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DL_SE_MASK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DL_SE_MASK_A::VALUE2
    }
}
#[doc = "AL Status event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AL_SE_MASK_A {
    #[doc = "0: Corresponding ECAT Event Request register bit is not mapped"]
    VALUE1 = 0,
    #[doc = "1: Corresponding ECAT Event Request register bit is mapped"]
    VALUE2 = 1,
}
impl From<AL_SE_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: AL_SE_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AL_SE_MASK`"]
pub type AL_SE_MASK_R = crate::R<bool, AL_SE_MASK_A>;
impl AL_SE_MASK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AL_SE_MASK_A {
        match self.bits {
            false => AL_SE_MASK_A::VALUE1,
            true => AL_SE_MASK_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == AL_SE_MASK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == AL_SE_MASK_A::VALUE2
    }
}
#[doc = "Mirrors values of each SyncManager Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MIR_0_MASK_A {
    #[doc = "0: Corresponding ECAT Event Request register bit is not mapped"]
    VALUE1 = 0,
    #[doc = "1: Corresponding ECAT Event Request register bit is mapped"]
    VALUE2 = 1,
}
impl From<MIR_0_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: MIR_0_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MIR_0_MASK`"]
pub type MIR_0_MASK_R = crate::R<bool, MIR_0_MASK_A>;
impl MIR_0_MASK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MIR_0_MASK_A {
        match self.bits {
            false => MIR_0_MASK_A::VALUE1,
            true => MIR_0_MASK_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MIR_0_MASK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MIR_0_MASK_A::VALUE2
    }
}
#[doc = "Mirrors values of each SyncManager Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MIR_1_MASK_A {
    #[doc = "0: Corresponding ECAT Event Request register bit is not mapped"]
    VALUE1 = 0,
    #[doc = "1: Corresponding ECAT Event Request register bit is mapped"]
    VALUE2 = 1,
}
impl From<MIR_1_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: MIR_1_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MIR_1_MASK`"]
pub type MIR_1_MASK_R = crate::R<bool, MIR_1_MASK_A>;
impl MIR_1_MASK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MIR_1_MASK_A {
        match self.bits {
            false => MIR_1_MASK_A::VALUE1,
            true => MIR_1_MASK_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MIR_1_MASK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MIR_1_MASK_A::VALUE2
    }
}
#[doc = "Mirrors values of each SyncManager Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MIR_2_MASK_A {
    #[doc = "0: Corresponding ECAT Event Request register bit is not mapped"]
    VALUE1 = 0,
    #[doc = "1: Corresponding ECAT Event Request register bit is mapped"]
    VALUE2 = 1,
}
impl From<MIR_2_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: MIR_2_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MIR_2_MASK`"]
pub type MIR_2_MASK_R = crate::R<bool, MIR_2_MASK_A>;
impl MIR_2_MASK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MIR_2_MASK_A {
        match self.bits {
            false => MIR_2_MASK_A::VALUE1,
            true => MIR_2_MASK_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MIR_2_MASK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MIR_2_MASK_A::VALUE2
    }
}
#[doc = "Mirrors values of each SyncManager Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MIR_3_MASK_A {
    #[doc = "0: Corresponding ECAT Event Request register bit is not mapped"]
    VALUE1 = 0,
    #[doc = "1: Corresponding ECAT Event Request register bit is mapped"]
    VALUE2 = 1,
}
impl From<MIR_3_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: MIR_3_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MIR_3_MASK`"]
pub type MIR_3_MASK_R = crate::R<bool, MIR_3_MASK_A>;
impl MIR_3_MASK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MIR_3_MASK_A {
        match self.bits {
            false => MIR_3_MASK_A::VALUE1,
            true => MIR_3_MASK_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MIR_3_MASK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MIR_3_MASK_A::VALUE2
    }
}
#[doc = "Mirrors values of each SyncManager Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MIR_4_MASK_A {
    #[doc = "0: Corresponding ECAT Event Request register bit is not mapped"]
    VALUE1 = 0,
    #[doc = "1: Corresponding ECAT Event Request register bit is mapped"]
    VALUE2 = 1,
}
impl From<MIR_4_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: MIR_4_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MIR_4_MASK`"]
pub type MIR_4_MASK_R = crate::R<bool, MIR_4_MASK_A>;
impl MIR_4_MASK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MIR_4_MASK_A {
        match self.bits {
            false => MIR_4_MASK_A::VALUE1,
            true => MIR_4_MASK_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MIR_4_MASK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MIR_4_MASK_A::VALUE2
    }
}
#[doc = "Mirrors values of each SyncManager Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MIR_5_MASK_A {
    #[doc = "0: Corresponding ECAT Event Request register bit is not mapped"]
    VALUE1 = 0,
    #[doc = "1: Corresponding ECAT Event Request register bit is mapped"]
    VALUE2 = 1,
}
impl From<MIR_5_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: MIR_5_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MIR_5_MASK`"]
pub type MIR_5_MASK_R = crate::R<bool, MIR_5_MASK_A>;
impl MIR_5_MASK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MIR_5_MASK_A {
        match self.bits {
            false => MIR_5_MASK_A::VALUE1,
            true => MIR_5_MASK_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MIR_5_MASK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MIR_5_MASK_A::VALUE2
    }
}
#[doc = "Mirrors values of each SyncManager Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MIR_6_MASK_A {
    #[doc = "0: Corresponding ECAT Event Request register bit is not mapped"]
    VALUE1 = 0,
    #[doc = "1: Corresponding ECAT Event Request register bit is mapped"]
    VALUE2 = 1,
}
impl From<MIR_6_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: MIR_6_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MIR_6_MASK`"]
pub type MIR_6_MASK_R = crate::R<bool, MIR_6_MASK_A>;
impl MIR_6_MASK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MIR_6_MASK_A {
        match self.bits {
            false => MIR_6_MASK_A::VALUE1,
            true => MIR_6_MASK_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MIR_6_MASK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MIR_6_MASK_A::VALUE2
    }
}
#[doc = "Mirrors values of each SyncManager Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MIR_7_MASK_A {
    #[doc = "0: Corresponding ECAT Event Request register bit is not mapped"]
    VALUE1 = 0,
    #[doc = "1: Corresponding ECAT Event Request register bit is mapped"]
    VALUE2 = 1,
}
impl From<MIR_7_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: MIR_7_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MIR_7_MASK`"]
pub type MIR_7_MASK_R = crate::R<bool, MIR_7_MASK_A>;
impl MIR_7_MASK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MIR_7_MASK_A {
        match self.bits {
            false => MIR_7_MASK_A::VALUE1,
            true => MIR_7_MASK_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MIR_7_MASK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MIR_7_MASK_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - DC Latch event"]
    #[inline(always)]
    pub fn dc_le_mask(&self) -> DC_LE_MASK_R {
        DC_LE_MASK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - DL Status event"]
    #[inline(always)]
    pub fn dl_se_mask(&self) -> DL_SE_MASK_R {
        DL_SE_MASK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - AL Status event"]
    #[inline(always)]
    pub fn al_se_mask(&self) -> AL_SE_MASK_R {
        AL_SE_MASK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Mirrors values of each SyncManager Status"]
    #[inline(always)]
    pub fn mir_0_mask(&self) -> MIR_0_MASK_R {
        MIR_0_MASK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Mirrors values of each SyncManager Status"]
    #[inline(always)]
    pub fn mir_1_mask(&self) -> MIR_1_MASK_R {
        MIR_1_MASK_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Mirrors values of each SyncManager Status"]
    #[inline(always)]
    pub fn mir_2_mask(&self) -> MIR_2_MASK_R {
        MIR_2_MASK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Mirrors values of each SyncManager Status"]
    #[inline(always)]
    pub fn mir_3_mask(&self) -> MIR_3_MASK_R {
        MIR_3_MASK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Mirrors values of each SyncManager Status"]
    #[inline(always)]
    pub fn mir_4_mask(&self) -> MIR_4_MASK_R {
        MIR_4_MASK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Mirrors values of each SyncManager Status"]
    #[inline(always)]
    pub fn mir_5_mask(&self) -> MIR_5_MASK_R {
        MIR_5_MASK_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Mirrors values of each SyncManager Status"]
    #[inline(always)]
    pub fn mir_6_mask(&self) -> MIR_6_MASK_R {
        MIR_6_MASK_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Mirrors values of each SyncManager Status"]
    #[inline(always)]
    pub fn mir_7_mask(&self) -> MIR_7_MASK_R {
        MIR_7_MASK_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}

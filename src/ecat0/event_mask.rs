#[doc = "Register `EVENT_MASK` reader"]
pub type R = crate::R<EventMaskSpec>;
#[doc = "DC Latch event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DcLeMask {
    #[doc = "0: Corresponding ECAT Event Request register bit is not mapped"]
    Value1 = 0,
    #[doc = "1: Corresponding ECAT Event Request register bit is mapped"]
    Value2 = 1,
}
impl From<DcLeMask> for bool {
    #[inline(always)]
    fn from(variant: DcLeMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DC_LE_MASK` reader - DC Latch event"]
pub type DcLeMaskR = crate::BitReader<DcLeMask>;
impl DcLeMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DcLeMask {
        match self.bits {
            false => DcLeMask::Value1,
            true => DcLeMask::Value2,
        }
    }
    #[doc = "Corresponding ECAT Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DcLeMask::Value1
    }
    #[doc = "Corresponding ECAT Event Request register bit is mapped"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DcLeMask::Value2
    }
}
#[doc = "DL Status event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DlSeMask {
    #[doc = "0: Corresponding ECAT Event Request register bit is not mapped"]
    Value1 = 0,
    #[doc = "1: Corresponding ECAT Event Request register bit is mapped"]
    Value2 = 1,
}
impl From<DlSeMask> for bool {
    #[inline(always)]
    fn from(variant: DlSeMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DL_SE_MASK` reader - DL Status event"]
pub type DlSeMaskR = crate::BitReader<DlSeMask>;
impl DlSeMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DlSeMask {
        match self.bits {
            false => DlSeMask::Value1,
            true => DlSeMask::Value2,
        }
    }
    #[doc = "Corresponding ECAT Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DlSeMask::Value1
    }
    #[doc = "Corresponding ECAT Event Request register bit is mapped"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DlSeMask::Value2
    }
}
#[doc = "AL Status event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AlSeMask {
    #[doc = "0: Corresponding ECAT Event Request register bit is not mapped"]
    Value1 = 0,
    #[doc = "1: Corresponding ECAT Event Request register bit is mapped"]
    Value2 = 1,
}
impl From<AlSeMask> for bool {
    #[inline(always)]
    fn from(variant: AlSeMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AL_SE_MASK` reader - AL Status event"]
pub type AlSeMaskR = crate::BitReader<AlSeMask>;
impl AlSeMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AlSeMask {
        match self.bits {
            false => AlSeMask::Value1,
            true => AlSeMask::Value2,
        }
    }
    #[doc = "Corresponding ECAT Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == AlSeMask::Value1
    }
    #[doc = "Corresponding ECAT Event Request register bit is mapped"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == AlSeMask::Value2
    }
}
#[doc = "Mirrors values of each SyncManager Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mir0Mask {
    #[doc = "0: Corresponding ECAT Event Request register bit is not mapped"]
    Value1 = 0,
    #[doc = "1: Corresponding ECAT Event Request register bit is mapped"]
    Value2 = 1,
}
impl From<Mir0Mask> for bool {
    #[inline(always)]
    fn from(variant: Mir0Mask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIR_0_MASK` reader - Mirrors values of each SyncManager Status"]
pub type Mir0MaskR = crate::BitReader<Mir0Mask>;
impl Mir0MaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mir0Mask {
        match self.bits {
            false => Mir0Mask::Value1,
            true => Mir0Mask::Value2,
        }
    }
    #[doc = "Corresponding ECAT Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Mir0Mask::Value1
    }
    #[doc = "Corresponding ECAT Event Request register bit is mapped"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Mir0Mask::Value2
    }
}
#[doc = "Mirrors values of each SyncManager Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mir1Mask {
    #[doc = "0: Corresponding ECAT Event Request register bit is not mapped"]
    Value1 = 0,
    #[doc = "1: Corresponding ECAT Event Request register bit is mapped"]
    Value2 = 1,
}
impl From<Mir1Mask> for bool {
    #[inline(always)]
    fn from(variant: Mir1Mask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIR_1_MASK` reader - Mirrors values of each SyncManager Status"]
pub type Mir1MaskR = crate::BitReader<Mir1Mask>;
impl Mir1MaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mir1Mask {
        match self.bits {
            false => Mir1Mask::Value1,
            true => Mir1Mask::Value2,
        }
    }
    #[doc = "Corresponding ECAT Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Mir1Mask::Value1
    }
    #[doc = "Corresponding ECAT Event Request register bit is mapped"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Mir1Mask::Value2
    }
}
#[doc = "Mirrors values of each SyncManager Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mir2Mask {
    #[doc = "0: Corresponding ECAT Event Request register bit is not mapped"]
    Value1 = 0,
    #[doc = "1: Corresponding ECAT Event Request register bit is mapped"]
    Value2 = 1,
}
impl From<Mir2Mask> for bool {
    #[inline(always)]
    fn from(variant: Mir2Mask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIR_2_MASK` reader - Mirrors values of each SyncManager Status"]
pub type Mir2MaskR = crate::BitReader<Mir2Mask>;
impl Mir2MaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mir2Mask {
        match self.bits {
            false => Mir2Mask::Value1,
            true => Mir2Mask::Value2,
        }
    }
    #[doc = "Corresponding ECAT Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Mir2Mask::Value1
    }
    #[doc = "Corresponding ECAT Event Request register bit is mapped"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Mir2Mask::Value2
    }
}
#[doc = "Mirrors values of each SyncManager Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mir3Mask {
    #[doc = "0: Corresponding ECAT Event Request register bit is not mapped"]
    Value1 = 0,
    #[doc = "1: Corresponding ECAT Event Request register bit is mapped"]
    Value2 = 1,
}
impl From<Mir3Mask> for bool {
    #[inline(always)]
    fn from(variant: Mir3Mask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIR_3_MASK` reader - Mirrors values of each SyncManager Status"]
pub type Mir3MaskR = crate::BitReader<Mir3Mask>;
impl Mir3MaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mir3Mask {
        match self.bits {
            false => Mir3Mask::Value1,
            true => Mir3Mask::Value2,
        }
    }
    #[doc = "Corresponding ECAT Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Mir3Mask::Value1
    }
    #[doc = "Corresponding ECAT Event Request register bit is mapped"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Mir3Mask::Value2
    }
}
#[doc = "Mirrors values of each SyncManager Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mir4Mask {
    #[doc = "0: Corresponding ECAT Event Request register bit is not mapped"]
    Value1 = 0,
    #[doc = "1: Corresponding ECAT Event Request register bit is mapped"]
    Value2 = 1,
}
impl From<Mir4Mask> for bool {
    #[inline(always)]
    fn from(variant: Mir4Mask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIR_4_MASK` reader - Mirrors values of each SyncManager Status"]
pub type Mir4MaskR = crate::BitReader<Mir4Mask>;
impl Mir4MaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mir4Mask {
        match self.bits {
            false => Mir4Mask::Value1,
            true => Mir4Mask::Value2,
        }
    }
    #[doc = "Corresponding ECAT Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Mir4Mask::Value1
    }
    #[doc = "Corresponding ECAT Event Request register bit is mapped"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Mir4Mask::Value2
    }
}
#[doc = "Mirrors values of each SyncManager Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mir5Mask {
    #[doc = "0: Corresponding ECAT Event Request register bit is not mapped"]
    Value1 = 0,
    #[doc = "1: Corresponding ECAT Event Request register bit is mapped"]
    Value2 = 1,
}
impl From<Mir5Mask> for bool {
    #[inline(always)]
    fn from(variant: Mir5Mask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIR_5_MASK` reader - Mirrors values of each SyncManager Status"]
pub type Mir5MaskR = crate::BitReader<Mir5Mask>;
impl Mir5MaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mir5Mask {
        match self.bits {
            false => Mir5Mask::Value1,
            true => Mir5Mask::Value2,
        }
    }
    #[doc = "Corresponding ECAT Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Mir5Mask::Value1
    }
    #[doc = "Corresponding ECAT Event Request register bit is mapped"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Mir5Mask::Value2
    }
}
#[doc = "Mirrors values of each SyncManager Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mir6Mask {
    #[doc = "0: Corresponding ECAT Event Request register bit is not mapped"]
    Value1 = 0,
    #[doc = "1: Corresponding ECAT Event Request register bit is mapped"]
    Value2 = 1,
}
impl From<Mir6Mask> for bool {
    #[inline(always)]
    fn from(variant: Mir6Mask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIR_6_MASK` reader - Mirrors values of each SyncManager Status"]
pub type Mir6MaskR = crate::BitReader<Mir6Mask>;
impl Mir6MaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mir6Mask {
        match self.bits {
            false => Mir6Mask::Value1,
            true => Mir6Mask::Value2,
        }
    }
    #[doc = "Corresponding ECAT Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Mir6Mask::Value1
    }
    #[doc = "Corresponding ECAT Event Request register bit is mapped"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Mir6Mask::Value2
    }
}
#[doc = "Mirrors values of each SyncManager Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mir7Mask {
    #[doc = "0: Corresponding ECAT Event Request register bit is not mapped"]
    Value1 = 0,
    #[doc = "1: Corresponding ECAT Event Request register bit is mapped"]
    Value2 = 1,
}
impl From<Mir7Mask> for bool {
    #[inline(always)]
    fn from(variant: Mir7Mask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIR_7_MASK` reader - Mirrors values of each SyncManager Status"]
pub type Mir7MaskR = crate::BitReader<Mir7Mask>;
impl Mir7MaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mir7Mask {
        match self.bits {
            false => Mir7Mask::Value1,
            true => Mir7Mask::Value2,
        }
    }
    #[doc = "Corresponding ECAT Event Request register bit is not mapped"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Mir7Mask::Value1
    }
    #[doc = "Corresponding ECAT Event Request register bit is mapped"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Mir7Mask::Value2
    }
}
impl R {
    #[doc = "Bit 0 - DC Latch event"]
    #[inline(always)]
    pub fn dc_le_mask(&self) -> DcLeMaskR {
        DcLeMaskR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - DL Status event"]
    #[inline(always)]
    pub fn dl_se_mask(&self) -> DlSeMaskR {
        DlSeMaskR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AL Status event"]
    #[inline(always)]
    pub fn al_se_mask(&self) -> AlSeMaskR {
        AlSeMaskR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Mirrors values of each SyncManager Status"]
    #[inline(always)]
    pub fn mir_0_mask(&self) -> Mir0MaskR {
        Mir0MaskR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Mirrors values of each SyncManager Status"]
    #[inline(always)]
    pub fn mir_1_mask(&self) -> Mir1MaskR {
        Mir1MaskR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Mirrors values of each SyncManager Status"]
    #[inline(always)]
    pub fn mir_2_mask(&self) -> Mir2MaskR {
        Mir2MaskR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Mirrors values of each SyncManager Status"]
    #[inline(always)]
    pub fn mir_3_mask(&self) -> Mir3MaskR {
        Mir3MaskR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Mirrors values of each SyncManager Status"]
    #[inline(always)]
    pub fn mir_4_mask(&self) -> Mir4MaskR {
        Mir4MaskR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Mirrors values of each SyncManager Status"]
    #[inline(always)]
    pub fn mir_5_mask(&self) -> Mir5MaskR {
        Mir5MaskR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Mirrors values of each SyncManager Status"]
    #[inline(always)]
    pub fn mir_6_mask(&self) -> Mir6MaskR {
        Mir6MaskR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Mirrors values of each SyncManager Status"]
    #[inline(always)]
    pub fn mir_7_mask(&self) -> Mir7MaskR {
        Mir7MaskR::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "ECAT Event Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`event_mask::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventMaskSpec;
impl crate::RegisterSpec for EventMaskSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`event_mask::R`](R) reader structure"]
impl crate::Readable for EventMaskSpec {}
#[doc = "`reset()` method sets EVENT_MASK to value 0"]
impl crate::Resettable for EventMaskSpec {
    const RESET_VALUE: u16 = 0;
}

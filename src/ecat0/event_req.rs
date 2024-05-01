#[doc = "Register `EVENT_REQ` reader"]
pub type R = crate::R<EventReqSpec>;
#[doc = "DC Latch event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DcLe {
    #[doc = "0: No change on DC Latch Inputs"]
    Value1 = 0,
    #[doc = "1: At least one change on DC Latch Inputs"]
    Value2 = 1,
}
impl From<DcLe> for bool {
    #[inline(always)]
    fn from(variant: DcLe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DC_LE` reader - DC Latch event"]
pub type DcLeR = crate::BitReader<DcLe>;
impl DcLeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DcLe {
        match self.bits {
            false => DcLe::Value1,
            true => DcLe::Value2,
        }
    }
    #[doc = "No change on DC Latch Inputs"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DcLe::Value1
    }
    #[doc = "At least one change on DC Latch Inputs"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DcLe::Value2
    }
}
#[doc = "DL Status event\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DlSe {
    #[doc = "0: No change in DL Status"]
    Value1 = 0,
    #[doc = "1: DL Status change"]
    Value2 = 1,
}
impl From<DlSe> for bool {
    #[inline(always)]
    fn from(variant: DlSe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DL_SE` reader - DL Status event"]
pub type DlSeR = crate::BitReader<DlSe>;
impl DlSeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DlSe {
        match self.bits {
            false => DlSe::Value1,
            true => DlSe::Value2,
        }
    }
    #[doc = "No change in DL Status"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DlSe::Value1
    }
    #[doc = "DL Status change"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DlSe::Value2
    }
}
#[doc = "AL Status event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AlSe {
    #[doc = "0: No change in AL Status"]
    Value1 = 0,
    #[doc = "1: AL Status change"]
    Value2 = 1,
}
impl From<AlSe> for bool {
    #[inline(always)]
    fn from(variant: AlSe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AL_SE` reader - AL Status event"]
pub type AlSeR = crate::BitReader<AlSe>;
impl AlSeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AlSe {
        match self.bits {
            false => AlSe::Value1,
            true => AlSe::Value2,
        }
    }
    #[doc = "No change in AL Status"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == AlSe::Value1
    }
    #[doc = "AL Status change"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == AlSe::Value2
    }
}
#[doc = "Mirrors values of each SyncManager Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mir0 {
    #[doc = "0: No Sync Channel 0 event"]
    Value1 = 0,
    #[doc = "1: Sync Channel 0 event pending"]
    Value2 = 1,
}
impl From<Mir0> for bool {
    #[inline(always)]
    fn from(variant: Mir0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIR_0` reader - Mirrors values of each SyncManager Status"]
pub type Mir0R = crate::BitReader<Mir0>;
impl Mir0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mir0 {
        match self.bits {
            false => Mir0::Value1,
            true => Mir0::Value2,
        }
    }
    #[doc = "No Sync Channel 0 event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Mir0::Value1
    }
    #[doc = "Sync Channel 0 event pending"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Mir0::Value2
    }
}
#[doc = "Mirrors values of each SyncManager Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mir1 {
    #[doc = "0: No Sync Channel 1 event"]
    Value1 = 0,
    #[doc = "1: Sync Channel 1 event pending"]
    Value2 = 1,
}
impl From<Mir1> for bool {
    #[inline(always)]
    fn from(variant: Mir1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIR_1` reader - Mirrors values of each SyncManager Status"]
pub type Mir1R = crate::BitReader<Mir1>;
impl Mir1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mir1 {
        match self.bits {
            false => Mir1::Value1,
            true => Mir1::Value2,
        }
    }
    #[doc = "No Sync Channel 1 event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Mir1::Value1
    }
    #[doc = "Sync Channel 1 event pending"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Mir1::Value2
    }
}
#[doc = "Mirrors values of each SyncManager Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mir2 {
    #[doc = "0: No Sync Channel 2 event"]
    Value1 = 0,
    #[doc = "1: Sync Channel 2 event pending"]
    Value2 = 1,
}
impl From<Mir2> for bool {
    #[inline(always)]
    fn from(variant: Mir2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIR_2` reader - Mirrors values of each SyncManager Status"]
pub type Mir2R = crate::BitReader<Mir2>;
impl Mir2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mir2 {
        match self.bits {
            false => Mir2::Value1,
            true => Mir2::Value2,
        }
    }
    #[doc = "No Sync Channel 2 event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Mir2::Value1
    }
    #[doc = "Sync Channel 2 event pending"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Mir2::Value2
    }
}
#[doc = "Mirrors values of each SyncManager Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mir3 {
    #[doc = "0: No Sync Channel 3 event"]
    Value1 = 0,
    #[doc = "1: Sync Channel 3event pending"]
    Value2 = 1,
}
impl From<Mir3> for bool {
    #[inline(always)]
    fn from(variant: Mir3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIR_3` reader - Mirrors values of each SyncManager Status"]
pub type Mir3R = crate::BitReader<Mir3>;
impl Mir3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mir3 {
        match self.bits {
            false => Mir3::Value1,
            true => Mir3::Value2,
        }
    }
    #[doc = "No Sync Channel 3 event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Mir3::Value1
    }
    #[doc = "Sync Channel 3event pending"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Mir3::Value2
    }
}
#[doc = "Mirrors values of each SyncManager Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mir4 {
    #[doc = "0: No Sync Channel 4 event"]
    Value1 = 0,
    #[doc = "1: Sync Channel 4 event pending"]
    Value2 = 1,
}
impl From<Mir4> for bool {
    #[inline(always)]
    fn from(variant: Mir4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIR_4` reader - Mirrors values of each SyncManager Status"]
pub type Mir4R = crate::BitReader<Mir4>;
impl Mir4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mir4 {
        match self.bits {
            false => Mir4::Value1,
            true => Mir4::Value2,
        }
    }
    #[doc = "No Sync Channel 4 event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Mir4::Value1
    }
    #[doc = "Sync Channel 4 event pending"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Mir4::Value2
    }
}
#[doc = "Mirrors values of each SyncManager Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mir5 {
    #[doc = "0: No Sync Channel 5 event"]
    Value1 = 0,
    #[doc = "1: Sync Channel 5 event pending"]
    Value2 = 1,
}
impl From<Mir5> for bool {
    #[inline(always)]
    fn from(variant: Mir5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIR_5` reader - Mirrors values of each SyncManager Status"]
pub type Mir5R = crate::BitReader<Mir5>;
impl Mir5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mir5 {
        match self.bits {
            false => Mir5::Value1,
            true => Mir5::Value2,
        }
    }
    #[doc = "No Sync Channel 5 event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Mir5::Value1
    }
    #[doc = "Sync Channel 5 event pending"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Mir5::Value2
    }
}
#[doc = "Mirrors values of each SyncManager Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mir6 {
    #[doc = "0: No Sync Channel 6 event"]
    Value1 = 0,
    #[doc = "1: Sync Channel 6 event pending"]
    Value2 = 1,
}
impl From<Mir6> for bool {
    #[inline(always)]
    fn from(variant: Mir6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIR_6` reader - Mirrors values of each SyncManager Status"]
pub type Mir6R = crate::BitReader<Mir6>;
impl Mir6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mir6 {
        match self.bits {
            false => Mir6::Value1,
            true => Mir6::Value2,
        }
    }
    #[doc = "No Sync Channel 6 event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Mir6::Value1
    }
    #[doc = "Sync Channel 6 event pending"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Mir6::Value2
    }
}
#[doc = "Mirrors values of each SyncManager Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mir7 {
    #[doc = "0: No Sync Channel 7 event"]
    Value1 = 0,
    #[doc = "1: Sync Channel 7 event pending"]
    Value2 = 1,
}
impl From<Mir7> for bool {
    #[inline(always)]
    fn from(variant: Mir7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIR_7` reader - Mirrors values of each SyncManager Status"]
pub type Mir7R = crate::BitReader<Mir7>;
impl Mir7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mir7 {
        match self.bits {
            false => Mir7::Value1,
            true => Mir7::Value2,
        }
    }
    #[doc = "No Sync Channel 7 event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Mir7::Value1
    }
    #[doc = "Sync Channel 7 event pending"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Mir7::Value2
    }
}
impl R {
    #[doc = "Bit 0 - DC Latch event"]
    #[inline(always)]
    pub fn dc_le(&self) -> DcLeR {
        DcLeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - DL Status event"]
    #[inline(always)]
    pub fn dl_se(&self) -> DlSeR {
        DlSeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AL Status event"]
    #[inline(always)]
    pub fn al_se(&self) -> AlSeR {
        AlSeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Mirrors values of each SyncManager Status"]
    #[inline(always)]
    pub fn mir_0(&self) -> Mir0R {
        Mir0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Mirrors values of each SyncManager Status"]
    #[inline(always)]
    pub fn mir_1(&self) -> Mir1R {
        Mir1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Mirrors values of each SyncManager Status"]
    #[inline(always)]
    pub fn mir_2(&self) -> Mir2R {
        Mir2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Mirrors values of each SyncManager Status"]
    #[inline(always)]
    pub fn mir_3(&self) -> Mir3R {
        Mir3R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Mirrors values of each SyncManager Status"]
    #[inline(always)]
    pub fn mir_4(&self) -> Mir4R {
        Mir4R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Mirrors values of each SyncManager Status"]
    #[inline(always)]
    pub fn mir_5(&self) -> Mir5R {
        Mir5R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Mirrors values of each SyncManager Status"]
    #[inline(always)]
    pub fn mir_6(&self) -> Mir6R {
        Mir6R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Mirrors values of each SyncManager Status"]
    #[inline(always)]
    pub fn mir_7(&self) -> Mir7R {
        Mir7R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "ECAT Event Request\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`event_req::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventReqSpec;
impl crate::RegisterSpec for EventReqSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`event_req::R`](R) reader structure"]
impl crate::Readable for EventReqSpec {}
#[doc = "`reset()` method sets EVENT_REQ to value 0x04"]
impl crate::Resettable for EventReqSpec {
    const RESET_VALUE: u16 = 0x04;
}

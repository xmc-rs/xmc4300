#[doc = "Register `EVENT_REQ` reader"]
pub type R = crate::R<EVENT_REQ_SPEC>;
#[doc = "DC Latch event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DC_LE_A {
    #[doc = "0: No change on DC Latch Inputs"]
    VALUE1 = 0,
    #[doc = "1: At least one change on DC Latch Inputs"]
    VALUE2 = 1,
}
impl From<DC_LE_A> for bool {
    #[inline(always)]
    fn from(variant: DC_LE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DC_LE` reader - DC Latch event"]
pub type DC_LE_R = crate::BitReader<DC_LE_A>;
impl DC_LE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DC_LE_A {
        match self.bits {
            false => DC_LE_A::VALUE1,
            true => DC_LE_A::VALUE2,
        }
    }
    #[doc = "No change on DC Latch Inputs"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DC_LE_A::VALUE1
    }
    #[doc = "At least one change on DC Latch Inputs"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DC_LE_A::VALUE2
    }
}
#[doc = "DL Status event\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DL_SE_A {
    #[doc = "0: No change in DL Status"]
    VALUE1 = 0,
    #[doc = "1: DL Status change"]
    VALUE2 = 1,
}
impl From<DL_SE_A> for bool {
    #[inline(always)]
    fn from(variant: DL_SE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DL_SE` reader - DL Status event"]
pub type DL_SE_R = crate::BitReader<DL_SE_A>;
impl DL_SE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DL_SE_A {
        match self.bits {
            false => DL_SE_A::VALUE1,
            true => DL_SE_A::VALUE2,
        }
    }
    #[doc = "No change in DL Status"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DL_SE_A::VALUE1
    }
    #[doc = "DL Status change"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DL_SE_A::VALUE2
    }
}
#[doc = "AL Status event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AL_SE_A {
    #[doc = "0: No change in AL Status"]
    VALUE1 = 0,
    #[doc = "1: AL Status change"]
    VALUE2 = 1,
}
impl From<AL_SE_A> for bool {
    #[inline(always)]
    fn from(variant: AL_SE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AL_SE` reader - AL Status event"]
pub type AL_SE_R = crate::BitReader<AL_SE_A>;
impl AL_SE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AL_SE_A {
        match self.bits {
            false => AL_SE_A::VALUE1,
            true => AL_SE_A::VALUE2,
        }
    }
    #[doc = "No change in AL Status"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == AL_SE_A::VALUE1
    }
    #[doc = "AL Status change"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == AL_SE_A::VALUE2
    }
}
#[doc = "Mirrors values of each SyncManager Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MIR_0_A {
    #[doc = "0: No Sync Channel 0 event"]
    VALUE1 = 0,
    #[doc = "1: Sync Channel 0 event pending"]
    VALUE2 = 1,
}
impl From<MIR_0_A> for bool {
    #[inline(always)]
    fn from(variant: MIR_0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIR_0` reader - Mirrors values of each SyncManager Status"]
pub type MIR_0_R = crate::BitReader<MIR_0_A>;
impl MIR_0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MIR_0_A {
        match self.bits {
            false => MIR_0_A::VALUE1,
            true => MIR_0_A::VALUE2,
        }
    }
    #[doc = "No Sync Channel 0 event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MIR_0_A::VALUE1
    }
    #[doc = "Sync Channel 0 event pending"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MIR_0_A::VALUE2
    }
}
#[doc = "Mirrors values of each SyncManager Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MIR_1_A {
    #[doc = "0: No Sync Channel 1 event"]
    VALUE1 = 0,
    #[doc = "1: Sync Channel 1 event pending"]
    VALUE2 = 1,
}
impl From<MIR_1_A> for bool {
    #[inline(always)]
    fn from(variant: MIR_1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIR_1` reader - Mirrors values of each SyncManager Status"]
pub type MIR_1_R = crate::BitReader<MIR_1_A>;
impl MIR_1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MIR_1_A {
        match self.bits {
            false => MIR_1_A::VALUE1,
            true => MIR_1_A::VALUE2,
        }
    }
    #[doc = "No Sync Channel 1 event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MIR_1_A::VALUE1
    }
    #[doc = "Sync Channel 1 event pending"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MIR_1_A::VALUE2
    }
}
#[doc = "Mirrors values of each SyncManager Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MIR_2_A {
    #[doc = "0: No Sync Channel 2 event"]
    VALUE1 = 0,
    #[doc = "1: Sync Channel 2 event pending"]
    VALUE2 = 1,
}
impl From<MIR_2_A> for bool {
    #[inline(always)]
    fn from(variant: MIR_2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIR_2` reader - Mirrors values of each SyncManager Status"]
pub type MIR_2_R = crate::BitReader<MIR_2_A>;
impl MIR_2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MIR_2_A {
        match self.bits {
            false => MIR_2_A::VALUE1,
            true => MIR_2_A::VALUE2,
        }
    }
    #[doc = "No Sync Channel 2 event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MIR_2_A::VALUE1
    }
    #[doc = "Sync Channel 2 event pending"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MIR_2_A::VALUE2
    }
}
#[doc = "Mirrors values of each SyncManager Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MIR_3_A {
    #[doc = "0: No Sync Channel 3 event"]
    VALUE1 = 0,
    #[doc = "1: Sync Channel 3event pending"]
    VALUE2 = 1,
}
impl From<MIR_3_A> for bool {
    #[inline(always)]
    fn from(variant: MIR_3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIR_3` reader - Mirrors values of each SyncManager Status"]
pub type MIR_3_R = crate::BitReader<MIR_3_A>;
impl MIR_3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MIR_3_A {
        match self.bits {
            false => MIR_3_A::VALUE1,
            true => MIR_3_A::VALUE2,
        }
    }
    #[doc = "No Sync Channel 3 event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MIR_3_A::VALUE1
    }
    #[doc = "Sync Channel 3event pending"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MIR_3_A::VALUE2
    }
}
#[doc = "Mirrors values of each SyncManager Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MIR_4_A {
    #[doc = "0: No Sync Channel 4 event"]
    VALUE1 = 0,
    #[doc = "1: Sync Channel 4 event pending"]
    VALUE2 = 1,
}
impl From<MIR_4_A> for bool {
    #[inline(always)]
    fn from(variant: MIR_4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIR_4` reader - Mirrors values of each SyncManager Status"]
pub type MIR_4_R = crate::BitReader<MIR_4_A>;
impl MIR_4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MIR_4_A {
        match self.bits {
            false => MIR_4_A::VALUE1,
            true => MIR_4_A::VALUE2,
        }
    }
    #[doc = "No Sync Channel 4 event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MIR_4_A::VALUE1
    }
    #[doc = "Sync Channel 4 event pending"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MIR_4_A::VALUE2
    }
}
#[doc = "Mirrors values of each SyncManager Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MIR_5_A {
    #[doc = "0: No Sync Channel 5 event"]
    VALUE1 = 0,
    #[doc = "1: Sync Channel 5 event pending"]
    VALUE2 = 1,
}
impl From<MIR_5_A> for bool {
    #[inline(always)]
    fn from(variant: MIR_5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIR_5` reader - Mirrors values of each SyncManager Status"]
pub type MIR_5_R = crate::BitReader<MIR_5_A>;
impl MIR_5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MIR_5_A {
        match self.bits {
            false => MIR_5_A::VALUE1,
            true => MIR_5_A::VALUE2,
        }
    }
    #[doc = "No Sync Channel 5 event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MIR_5_A::VALUE1
    }
    #[doc = "Sync Channel 5 event pending"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MIR_5_A::VALUE2
    }
}
#[doc = "Mirrors values of each SyncManager Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MIR_6_A {
    #[doc = "0: No Sync Channel 6 event"]
    VALUE1 = 0,
    #[doc = "1: Sync Channel 6 event pending"]
    VALUE2 = 1,
}
impl From<MIR_6_A> for bool {
    #[inline(always)]
    fn from(variant: MIR_6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIR_6` reader - Mirrors values of each SyncManager Status"]
pub type MIR_6_R = crate::BitReader<MIR_6_A>;
impl MIR_6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MIR_6_A {
        match self.bits {
            false => MIR_6_A::VALUE1,
            true => MIR_6_A::VALUE2,
        }
    }
    #[doc = "No Sync Channel 6 event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MIR_6_A::VALUE1
    }
    #[doc = "Sync Channel 6 event pending"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MIR_6_A::VALUE2
    }
}
#[doc = "Mirrors values of each SyncManager Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MIR_7_A {
    #[doc = "0: No Sync Channel 7 event"]
    VALUE1 = 0,
    #[doc = "1: Sync Channel 7 event pending"]
    VALUE2 = 1,
}
impl From<MIR_7_A> for bool {
    #[inline(always)]
    fn from(variant: MIR_7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIR_7` reader - Mirrors values of each SyncManager Status"]
pub type MIR_7_R = crate::BitReader<MIR_7_A>;
impl MIR_7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MIR_7_A {
        match self.bits {
            false => MIR_7_A::VALUE1,
            true => MIR_7_A::VALUE2,
        }
    }
    #[doc = "No Sync Channel 7 event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MIR_7_A::VALUE1
    }
    #[doc = "Sync Channel 7 event pending"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MIR_7_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - DC Latch event"]
    #[inline(always)]
    pub fn dc_le(&self) -> DC_LE_R {
        DC_LE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - DL Status event"]
    #[inline(always)]
    pub fn dl_se(&self) -> DL_SE_R {
        DL_SE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AL Status event"]
    #[inline(always)]
    pub fn al_se(&self) -> AL_SE_R {
        AL_SE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Mirrors values of each SyncManager Status"]
    #[inline(always)]
    pub fn mir_0(&self) -> MIR_0_R {
        MIR_0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Mirrors values of each SyncManager Status"]
    #[inline(always)]
    pub fn mir_1(&self) -> MIR_1_R {
        MIR_1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Mirrors values of each SyncManager Status"]
    #[inline(always)]
    pub fn mir_2(&self) -> MIR_2_R {
        MIR_2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Mirrors values of each SyncManager Status"]
    #[inline(always)]
    pub fn mir_3(&self) -> MIR_3_R {
        MIR_3_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Mirrors values of each SyncManager Status"]
    #[inline(always)]
    pub fn mir_4(&self) -> MIR_4_R {
        MIR_4_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Mirrors values of each SyncManager Status"]
    #[inline(always)]
    pub fn mir_5(&self) -> MIR_5_R {
        MIR_5_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Mirrors values of each SyncManager Status"]
    #[inline(always)]
    pub fn mir_6(&self) -> MIR_6_R {
        MIR_6_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Mirrors values of each SyncManager Status"]
    #[inline(always)]
    pub fn mir_7(&self) -> MIR_7_R {
        MIR_7_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "ECAT Event Request\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`event_req::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EVENT_REQ_SPEC;
impl crate::RegisterSpec for EVENT_REQ_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`event_req::R`](R) reader structure"]
impl crate::Readable for EVENT_REQ_SPEC {}
#[doc = "`reset()` method sets EVENT_REQ to value 0x04"]
impl crate::Resettable for EVENT_REQ_SPEC {
    const RESET_VALUE: u16 = 0x04;
}

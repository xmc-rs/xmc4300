#[doc = "Register `EVENT_REQ` reader"]
pub struct R(crate::R<EVENT_REQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVENT_REQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVENT_REQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVENT_REQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "DC Latch event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub struct DC_LE_R(crate::FieldReader<bool, DC_LE_A>);
impl DC_LE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DC_LE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DC_LE_A {
        match self.bits {
            false => DC_LE_A::VALUE1,
            true => DC_LE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == DC_LE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == DC_LE_A::VALUE2
    }
}
impl core::ops::Deref for DC_LE_R {
    type Target = crate::FieldReader<bool, DC_LE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "DL Status event\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub struct DL_SE_R(crate::FieldReader<bool, DL_SE_A>);
impl DL_SE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DL_SE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DL_SE_A {
        match self.bits {
            false => DL_SE_A::VALUE1,
            true => DL_SE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == DL_SE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == DL_SE_A::VALUE2
    }
}
impl core::ops::Deref for DL_SE_R {
    type Target = crate::FieldReader<bool, DL_SE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "AL Status event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub struct AL_SE_R(crate::FieldReader<bool, AL_SE_A>);
impl AL_SE_R {
    pub(crate) fn new(bits: bool) -> Self {
        AL_SE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AL_SE_A {
        match self.bits {
            false => AL_SE_A::VALUE1,
            true => AL_SE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == AL_SE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == AL_SE_A::VALUE2
    }
}
impl core::ops::Deref for AL_SE_R {
    type Target = crate::FieldReader<bool, AL_SE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Mirrors values of each SyncManager Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub struct MIR_0_R(crate::FieldReader<bool, MIR_0_A>);
impl MIR_0_R {
    pub(crate) fn new(bits: bool) -> Self {
        MIR_0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MIR_0_A {
        match self.bits {
            false => MIR_0_A::VALUE1,
            true => MIR_0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == MIR_0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == MIR_0_A::VALUE2
    }
}
impl core::ops::Deref for MIR_0_R {
    type Target = crate::FieldReader<bool, MIR_0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Mirrors values of each SyncManager Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub struct MIR_1_R(crate::FieldReader<bool, MIR_1_A>);
impl MIR_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        MIR_1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MIR_1_A {
        match self.bits {
            false => MIR_1_A::VALUE1,
            true => MIR_1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == MIR_1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == MIR_1_A::VALUE2
    }
}
impl core::ops::Deref for MIR_1_R {
    type Target = crate::FieldReader<bool, MIR_1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Mirrors values of each SyncManager Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub struct MIR_2_R(crate::FieldReader<bool, MIR_2_A>);
impl MIR_2_R {
    pub(crate) fn new(bits: bool) -> Self {
        MIR_2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MIR_2_A {
        match self.bits {
            false => MIR_2_A::VALUE1,
            true => MIR_2_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == MIR_2_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == MIR_2_A::VALUE2
    }
}
impl core::ops::Deref for MIR_2_R {
    type Target = crate::FieldReader<bool, MIR_2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Mirrors values of each SyncManager Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub struct MIR_3_R(crate::FieldReader<bool, MIR_3_A>);
impl MIR_3_R {
    pub(crate) fn new(bits: bool) -> Self {
        MIR_3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MIR_3_A {
        match self.bits {
            false => MIR_3_A::VALUE1,
            true => MIR_3_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == MIR_3_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == MIR_3_A::VALUE2
    }
}
impl core::ops::Deref for MIR_3_R {
    type Target = crate::FieldReader<bool, MIR_3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Mirrors values of each SyncManager Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub struct MIR_4_R(crate::FieldReader<bool, MIR_4_A>);
impl MIR_4_R {
    pub(crate) fn new(bits: bool) -> Self {
        MIR_4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MIR_4_A {
        match self.bits {
            false => MIR_4_A::VALUE1,
            true => MIR_4_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == MIR_4_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == MIR_4_A::VALUE2
    }
}
impl core::ops::Deref for MIR_4_R {
    type Target = crate::FieldReader<bool, MIR_4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Mirrors values of each SyncManager Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub struct MIR_5_R(crate::FieldReader<bool, MIR_5_A>);
impl MIR_5_R {
    pub(crate) fn new(bits: bool) -> Self {
        MIR_5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MIR_5_A {
        match self.bits {
            false => MIR_5_A::VALUE1,
            true => MIR_5_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == MIR_5_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == MIR_5_A::VALUE2
    }
}
impl core::ops::Deref for MIR_5_R {
    type Target = crate::FieldReader<bool, MIR_5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Mirrors values of each SyncManager Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub struct MIR_6_R(crate::FieldReader<bool, MIR_6_A>);
impl MIR_6_R {
    pub(crate) fn new(bits: bool) -> Self {
        MIR_6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MIR_6_A {
        match self.bits {
            false => MIR_6_A::VALUE1,
            true => MIR_6_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == MIR_6_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == MIR_6_A::VALUE2
    }
}
impl core::ops::Deref for MIR_6_R {
    type Target = crate::FieldReader<bool, MIR_6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Mirrors values of each SyncManager Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub struct MIR_7_R(crate::FieldReader<bool, MIR_7_A>);
impl MIR_7_R {
    pub(crate) fn new(bits: bool) -> Self {
        MIR_7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MIR_7_A {
        match self.bits {
            false => MIR_7_A::VALUE1,
            true => MIR_7_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == MIR_7_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == MIR_7_A::VALUE2
    }
}
impl core::ops::Deref for MIR_7_R {
    type Target = crate::FieldReader<bool, MIR_7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - DC Latch event"]
    #[inline(always)]
    pub fn dc_le(&self) -> DC_LE_R {
        DC_LE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - DL Status event"]
    #[inline(always)]
    pub fn dl_se(&self) -> DL_SE_R {
        DL_SE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - AL Status event"]
    #[inline(always)]
    pub fn al_se(&self) -> AL_SE_R {
        AL_SE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Mirrors values of each SyncManager Status"]
    #[inline(always)]
    pub fn mir_0(&self) -> MIR_0_R {
        MIR_0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Mirrors values of each SyncManager Status"]
    #[inline(always)]
    pub fn mir_1(&self) -> MIR_1_R {
        MIR_1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Mirrors values of each SyncManager Status"]
    #[inline(always)]
    pub fn mir_2(&self) -> MIR_2_R {
        MIR_2_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Mirrors values of each SyncManager Status"]
    #[inline(always)]
    pub fn mir_3(&self) -> MIR_3_R {
        MIR_3_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Mirrors values of each SyncManager Status"]
    #[inline(always)]
    pub fn mir_4(&self) -> MIR_4_R {
        MIR_4_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Mirrors values of each SyncManager Status"]
    #[inline(always)]
    pub fn mir_5(&self) -> MIR_5_R {
        MIR_5_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Mirrors values of each SyncManager Status"]
    #[inline(always)]
    pub fn mir_6(&self) -> MIR_6_R {
        MIR_6_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Mirrors values of each SyncManager Status"]
    #[inline(always)]
    pub fn mir_7(&self) -> MIR_7_R {
        MIR_7_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
#[doc = "ECAT Event Request\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [event_req](index.html) module"]
pub struct EVENT_REQ_SPEC;
impl crate::RegisterSpec for EVENT_REQ_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [event_req::R](R) reader structure"]
impl crate::Readable for EVENT_REQ_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EVENT_REQ to value 0x04"]
impl crate::Resettable for EVENT_REQ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x04
    }
}

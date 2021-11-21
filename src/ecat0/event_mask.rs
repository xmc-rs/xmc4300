#[doc = "Register `EVENT_MASK` reader"]
pub struct R(crate::R<EVENT_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVENT_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVENT_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVENT_MASK_SPEC>) -> Self {
        R(reader)
    }
}
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
#[doc = "Field `DC_LE_MASK` reader - DC Latch event"]
pub struct DC_LE_MASK_R(crate::FieldReader<bool, DC_LE_MASK_A>);
impl DC_LE_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        DC_LE_MASK_R(crate::FieldReader::new(bits))
    }
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
        **self == DC_LE_MASK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == DC_LE_MASK_A::VALUE2
    }
}
impl core::ops::Deref for DC_LE_MASK_R {
    type Target = crate::FieldReader<bool, DC_LE_MASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `DL_SE_MASK` reader - DL Status event"]
pub struct DL_SE_MASK_R(crate::FieldReader<bool, DL_SE_MASK_A>);
impl DL_SE_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        DL_SE_MASK_R(crate::FieldReader::new(bits))
    }
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
        **self == DL_SE_MASK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == DL_SE_MASK_A::VALUE2
    }
}
impl core::ops::Deref for DL_SE_MASK_R {
    type Target = crate::FieldReader<bool, DL_SE_MASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `AL_SE_MASK` reader - AL Status event"]
pub struct AL_SE_MASK_R(crate::FieldReader<bool, AL_SE_MASK_A>);
impl AL_SE_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        AL_SE_MASK_R(crate::FieldReader::new(bits))
    }
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
        **self == AL_SE_MASK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == AL_SE_MASK_A::VALUE2
    }
}
impl core::ops::Deref for AL_SE_MASK_R {
    type Target = crate::FieldReader<bool, AL_SE_MASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `MIR_0_MASK` reader - Mirrors values of each SyncManager Status"]
pub struct MIR_0_MASK_R(crate::FieldReader<bool, MIR_0_MASK_A>);
impl MIR_0_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        MIR_0_MASK_R(crate::FieldReader::new(bits))
    }
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
        **self == MIR_0_MASK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == MIR_0_MASK_A::VALUE2
    }
}
impl core::ops::Deref for MIR_0_MASK_R {
    type Target = crate::FieldReader<bool, MIR_0_MASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `MIR_1_MASK` reader - Mirrors values of each SyncManager Status"]
pub struct MIR_1_MASK_R(crate::FieldReader<bool, MIR_1_MASK_A>);
impl MIR_1_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        MIR_1_MASK_R(crate::FieldReader::new(bits))
    }
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
        **self == MIR_1_MASK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == MIR_1_MASK_A::VALUE2
    }
}
impl core::ops::Deref for MIR_1_MASK_R {
    type Target = crate::FieldReader<bool, MIR_1_MASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `MIR_2_MASK` reader - Mirrors values of each SyncManager Status"]
pub struct MIR_2_MASK_R(crate::FieldReader<bool, MIR_2_MASK_A>);
impl MIR_2_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        MIR_2_MASK_R(crate::FieldReader::new(bits))
    }
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
        **self == MIR_2_MASK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == MIR_2_MASK_A::VALUE2
    }
}
impl core::ops::Deref for MIR_2_MASK_R {
    type Target = crate::FieldReader<bool, MIR_2_MASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `MIR_3_MASK` reader - Mirrors values of each SyncManager Status"]
pub struct MIR_3_MASK_R(crate::FieldReader<bool, MIR_3_MASK_A>);
impl MIR_3_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        MIR_3_MASK_R(crate::FieldReader::new(bits))
    }
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
        **self == MIR_3_MASK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == MIR_3_MASK_A::VALUE2
    }
}
impl core::ops::Deref for MIR_3_MASK_R {
    type Target = crate::FieldReader<bool, MIR_3_MASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `MIR_4_MASK` reader - Mirrors values of each SyncManager Status"]
pub struct MIR_4_MASK_R(crate::FieldReader<bool, MIR_4_MASK_A>);
impl MIR_4_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        MIR_4_MASK_R(crate::FieldReader::new(bits))
    }
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
        **self == MIR_4_MASK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == MIR_4_MASK_A::VALUE2
    }
}
impl core::ops::Deref for MIR_4_MASK_R {
    type Target = crate::FieldReader<bool, MIR_4_MASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `MIR_5_MASK` reader - Mirrors values of each SyncManager Status"]
pub struct MIR_5_MASK_R(crate::FieldReader<bool, MIR_5_MASK_A>);
impl MIR_5_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        MIR_5_MASK_R(crate::FieldReader::new(bits))
    }
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
        **self == MIR_5_MASK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == MIR_5_MASK_A::VALUE2
    }
}
impl core::ops::Deref for MIR_5_MASK_R {
    type Target = crate::FieldReader<bool, MIR_5_MASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `MIR_6_MASK` reader - Mirrors values of each SyncManager Status"]
pub struct MIR_6_MASK_R(crate::FieldReader<bool, MIR_6_MASK_A>);
impl MIR_6_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        MIR_6_MASK_R(crate::FieldReader::new(bits))
    }
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
        **self == MIR_6_MASK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == MIR_6_MASK_A::VALUE2
    }
}
impl core::ops::Deref for MIR_6_MASK_R {
    type Target = crate::FieldReader<bool, MIR_6_MASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `MIR_7_MASK` reader - Mirrors values of each SyncManager Status"]
pub struct MIR_7_MASK_R(crate::FieldReader<bool, MIR_7_MASK_A>);
impl MIR_7_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        MIR_7_MASK_R(crate::FieldReader::new(bits))
    }
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
        **self == MIR_7_MASK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == MIR_7_MASK_A::VALUE2
    }
}
impl core::ops::Deref for MIR_7_MASK_R {
    type Target = crate::FieldReader<bool, MIR_7_MASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "ECAT Event Mask\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [event_mask](index.html) module"]
pub struct EVENT_MASK_SPEC;
impl crate::RegisterSpec for EVENT_MASK_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [event_mask::R](R) reader structure"]
impl crate::Readable for EVENT_MASK_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EVENT_MASK to value 0"]
impl crate::Resettable for EVENT_MASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

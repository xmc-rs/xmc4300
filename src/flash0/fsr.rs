#[doc = "Register `FSR` reader"]
pub struct R(crate::R<FSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Program Flash Busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PBUSY_A {
    #[doc = "0: PFLASH ready, not busy; PFLASH in read mode."]
    VALUE1 = 0,
    #[doc = "1: PFLASH busy; PFLASH not in read mode."]
    VALUE2 = 1,
}
impl From<PBUSY_A> for bool {
    #[inline(always)]
    fn from(variant: PBUSY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PBUSY` reader - Program Flash Busy"]
pub struct PBUSY_R(crate::FieldReader<bool, PBUSY_A>);
impl PBUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        PBUSY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PBUSY_A {
        match self.bits {
            false => PBUSY_A::VALUE1,
            true => PBUSY_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PBUSY_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PBUSY_A::VALUE2
    }
}
impl core::ops::Deref for PBUSY_R {
    type Target = crate::FieldReader<bool, PBUSY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FABUSY` reader - Flash Array Busy"]
pub struct FABUSY_R(crate::FieldReader<bool, bool>);
impl FABUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        FABUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FABUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Programming State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROG_A {
    #[doc = "0: There is no program operation requested or in progress or just finished."]
    VALUE1 = 0,
    #[doc = "1: Programming operation (write page) requested (from FIM) or in action or finished."]
    VALUE2 = 1,
}
impl From<PROG_A> for bool {
    #[inline(always)]
    fn from(variant: PROG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PROG` reader - Programming State"]
pub struct PROG_R(crate::FieldReader<bool, PROG_A>);
impl PROG_R {
    pub(crate) fn new(bits: bool) -> Self {
        PROG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PROG_A {
        match self.bits {
            false => PROG_A::VALUE1,
            true => PROG_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PROG_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PROG_A::VALUE2
    }
}
impl core::ops::Deref for PROG_R {
    type Target = crate::FieldReader<bool, PROG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Erase State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERASE_A {
    #[doc = "0: There is no erase operation requested or in progress or just finished"]
    VALUE1 = 0,
    #[doc = "1: Erase operation requested (from FIM) or in action or finished."]
    VALUE2 = 1,
}
impl From<ERASE_A> for bool {
    #[inline(always)]
    fn from(variant: ERASE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERASE` reader - Erase State"]
pub struct ERASE_R(crate::FieldReader<bool, ERASE_A>);
impl ERASE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERASE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERASE_A {
        match self.bits {
            false => ERASE_A::VALUE1,
            true => ERASE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == ERASE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == ERASE_A::VALUE2
    }
}
impl core::ops::Deref for ERASE_R {
    type Target = crate::FieldReader<bool, ERASE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Program Flash in Page Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PFPAGE_A {
    #[doc = "0: Program Flash not in page mode"]
    VALUE1 = 0,
    #[doc = "1: Program Flash in page mode; assembly buffer of PFLASH (256 byte) is in use (being filled up)"]
    VALUE2 = 1,
}
impl From<PFPAGE_A> for bool {
    #[inline(always)]
    fn from(variant: PFPAGE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PFPAGE` reader - Program Flash in Page Mode"]
pub struct PFPAGE_R(crate::FieldReader<bool, PFPAGE_A>);
impl PFPAGE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PFPAGE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PFPAGE_A {
        match self.bits {
            false => PFPAGE_A::VALUE1,
            true => PFPAGE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PFPAGE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PFPAGE_A::VALUE2
    }
}
impl core::ops::Deref for PFPAGE_R {
    type Target = crate::FieldReader<bool, PFPAGE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Program Flash Operation Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PFOPER_A {
    #[doc = "0: No operation error reported by Program Flash"]
    VALUE1 = 0,
    #[doc = "1: Flash array operation aborted, because of a Flash array failure, e.g. an ECC error in microcode."]
    VALUE2 = 1,
}
impl From<PFOPER_A> for bool {
    #[inline(always)]
    fn from(variant: PFOPER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PFOPER` reader - Program Flash Operation Error"]
pub struct PFOPER_R(crate::FieldReader<bool, PFOPER_A>);
impl PFOPER_R {
    pub(crate) fn new(bits: bool) -> Self {
        PFOPER_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PFOPER_A {
        match self.bits {
            false => PFOPER_A::VALUE1,
            true => PFOPER_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PFOPER_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PFOPER_A::VALUE2
    }
}
impl core::ops::Deref for PFOPER_R {
    type Target = crate::FieldReader<bool, PFOPER_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Command Sequence Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SQER_A {
    #[doc = "0: No sequence error"]
    VALUE1 = 0,
    #[doc = "1: Command state machine operation unsuccessful because of improper address or command sequence."]
    VALUE2 = 1,
}
impl From<SQER_A> for bool {
    #[inline(always)]
    fn from(variant: SQER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SQER` reader - Command Sequence Error"]
pub struct SQER_R(crate::FieldReader<bool, SQER_A>);
impl SQER_R {
    pub(crate) fn new(bits: bool) -> Self {
        SQER_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SQER_A {
        match self.bits {
            false => SQER_A::VALUE1,
            true => SQER_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == SQER_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SQER_A::VALUE2
    }
}
impl core::ops::Deref for SQER_R {
    type Target = crate::FieldReader<bool, SQER_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Protection Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROER_A {
    #[doc = "0: No protection error"]
    VALUE1 = 0,
    #[doc = "1: Protection error."]
    VALUE2 = 1,
}
impl From<PROER_A> for bool {
    #[inline(always)]
    fn from(variant: PROER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PROER` reader - Protection Error"]
pub struct PROER_R(crate::FieldReader<bool, PROER_A>);
impl PROER_R {
    pub(crate) fn new(bits: bool) -> Self {
        PROER_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PROER_A {
        match self.bits {
            false => PROER_A::VALUE1,
            true => PROER_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PROER_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PROER_A::VALUE2
    }
}
impl core::ops::Deref for PROER_R {
    type Target = crate::FieldReader<bool, PROER_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "PFLASH Single-Bit Error and Correction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PFSBER_A {
    #[doc = "0: No Single-Bit Error detected during read access to PFLASH"]
    VALUE1 = 0,
    #[doc = "1: Single-Bit Error detected and corrected"]
    VALUE2 = 1,
}
impl From<PFSBER_A> for bool {
    #[inline(always)]
    fn from(variant: PFSBER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PFSBER` reader - PFLASH Single-Bit Error and Correction"]
pub struct PFSBER_R(crate::FieldReader<bool, PFSBER_A>);
impl PFSBER_R {
    pub(crate) fn new(bits: bool) -> Self {
        PFSBER_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PFSBER_A {
        match self.bits {
            false => PFSBER_A::VALUE1,
            true => PFSBER_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PFSBER_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PFSBER_A::VALUE2
    }
}
impl core::ops::Deref for PFSBER_R {
    type Target = crate::FieldReader<bool, PFSBER_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "PFLASH Double-Bit Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PFDBER_A {
    #[doc = "0: No Double-Bit Error detected during read access to PFLASH"]
    VALUE1 = 0,
    #[doc = "1: Double-Bit Error detected in PFLASH"]
    VALUE2 = 1,
}
impl From<PFDBER_A> for bool {
    #[inline(always)]
    fn from(variant: PFDBER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PFDBER` reader - PFLASH Double-Bit Error"]
pub struct PFDBER_R(crate::FieldReader<bool, PFDBER_A>);
impl PFDBER_R {
    pub(crate) fn new(bits: bool) -> Self {
        PFDBER_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PFDBER_A {
        match self.bits {
            false => PFDBER_A::VALUE1,
            true => PFDBER_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PFDBER_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PFDBER_A::VALUE2
    }
}
impl core::ops::Deref for PFDBER_R {
    type Target = crate::FieldReader<bool, PFDBER_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Protection Installed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROIN_A {
    #[doc = "0: No protection is installed"]
    VALUE1 = 0,
    #[doc = "1: Read or/and write protection for one or more users is configured and correctly confirmed in the User Configuration Block(s)."]
    VALUE2 = 1,
}
impl From<PROIN_A> for bool {
    #[inline(always)]
    fn from(variant: PROIN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PROIN` reader - Protection Installed"]
pub struct PROIN_R(crate::FieldReader<bool, PROIN_A>);
impl PROIN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PROIN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PROIN_A {
        match self.bits {
            false => PROIN_A::VALUE1,
            true => PROIN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PROIN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PROIN_A::VALUE2
    }
}
impl core::ops::Deref for PROIN_R {
    type Target = crate::FieldReader<bool, PROIN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Read Protection Installed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RPROIN_A {
    #[doc = "0: No read protection installed"]
    VALUE1 = 0,
    #[doc = "1: Read protection and global write protection is configured and correctly confirmed in the User Configuration Block 0."]
    VALUE2 = 1,
}
impl From<RPROIN_A> for bool {
    #[inline(always)]
    fn from(variant: RPROIN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RPROIN` reader - Read Protection Installed"]
pub struct RPROIN_R(crate::FieldReader<bool, RPROIN_A>);
impl RPROIN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RPROIN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RPROIN_A {
        match self.bits {
            false => RPROIN_A::VALUE1,
            true => RPROIN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == RPROIN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == RPROIN_A::VALUE2
    }
}
impl core::ops::Deref for RPROIN_R {
    type Target = crate::FieldReader<bool, RPROIN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Read Protection Disable State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RPRODIS_A {
    #[doc = "0: Read protection (if installed) is not disabled"]
    VALUE1 = 0,
    #[doc = "1: Read and global write protection is temporarily disabled."]
    VALUE2 = 1,
}
impl From<RPRODIS_A> for bool {
    #[inline(always)]
    fn from(variant: RPRODIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RPRODIS` reader - Read Protection Disable State"]
pub struct RPRODIS_R(crate::FieldReader<bool, RPRODIS_A>);
impl RPRODIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RPRODIS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RPRODIS_A {
        match self.bits {
            false => RPRODIS_A::VALUE1,
            true => RPRODIS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == RPRODIS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == RPRODIS_A::VALUE2
    }
}
impl core::ops::Deref for RPRODIS_R {
    type Target = crate::FieldReader<bool, RPRODIS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Sector Write Protection Installed for User 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WPROIN0_A {
    #[doc = "0: No write protection installed for user 0"]
    VALUE1 = 0,
    #[doc = "1: Sector write protection for user 0 is configured and correctly confirmed in the User Configuration Block 0."]
    VALUE2 = 1,
}
impl From<WPROIN0_A> for bool {
    #[inline(always)]
    fn from(variant: WPROIN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WPROIN0` reader - Sector Write Protection Installed for User 0"]
pub struct WPROIN0_R(crate::FieldReader<bool, WPROIN0_A>);
impl WPROIN0_R {
    pub(crate) fn new(bits: bool) -> Self {
        WPROIN0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WPROIN0_A {
        match self.bits {
            false => WPROIN0_A::VALUE1,
            true => WPROIN0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == WPROIN0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == WPROIN0_A::VALUE2
    }
}
impl core::ops::Deref for WPROIN0_R {
    type Target = crate::FieldReader<bool, WPROIN0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Sector Write Protection Installed for User 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WPROIN1_A {
    #[doc = "0: No write protection installed for user 1"]
    VALUE1 = 0,
    #[doc = "1: Sector write protection for user 1 is configured and correctly confirmed in the User Configuration Block 1."]
    VALUE2 = 1,
}
impl From<WPROIN1_A> for bool {
    #[inline(always)]
    fn from(variant: WPROIN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WPROIN1` reader - Sector Write Protection Installed for User 1"]
pub struct WPROIN1_R(crate::FieldReader<bool, WPROIN1_A>);
impl WPROIN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        WPROIN1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WPROIN1_A {
        match self.bits {
            false => WPROIN1_A::VALUE1,
            true => WPROIN1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == WPROIN1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == WPROIN1_A::VALUE2
    }
}
impl core::ops::Deref for WPROIN1_R {
    type Target = crate::FieldReader<bool, WPROIN1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Sector OTP Protection Installed for User 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WPROIN2_A {
    #[doc = "0: No OTP write protection installed for user 2"]
    VALUE1 = 0,
    #[doc = "1: Sector OTP write protection with ROM functionality is configured and correctly confirmed in the UCB2. The protection is locked for ever."]
    VALUE2 = 1,
}
impl From<WPROIN2_A> for bool {
    #[inline(always)]
    fn from(variant: WPROIN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WPROIN2` reader - Sector OTP Protection Installed for User 2"]
pub struct WPROIN2_R(crate::FieldReader<bool, WPROIN2_A>);
impl WPROIN2_R {
    pub(crate) fn new(bits: bool) -> Self {
        WPROIN2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WPROIN2_A {
        match self.bits {
            false => WPROIN2_A::VALUE1,
            true => WPROIN2_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == WPROIN2_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == WPROIN2_A::VALUE2
    }
}
impl core::ops::Deref for WPROIN2_R {
    type Target = crate::FieldReader<bool, WPROIN2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Sector Write Protection Disabled for User 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WPRODIS0_A {
    #[doc = "0: All protected sectors of user 0 are locked if write protection is installed"]
    VALUE1 = 0,
    #[doc = "1: All write-protected sectors of user 0 are temporarily unlocked, if not coincidently locked by user 2 or via read protection."]
    VALUE2 = 1,
}
impl From<WPRODIS0_A> for bool {
    #[inline(always)]
    fn from(variant: WPRODIS0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WPRODIS0` reader - Sector Write Protection Disabled for User 0"]
pub struct WPRODIS0_R(crate::FieldReader<bool, WPRODIS0_A>);
impl WPRODIS0_R {
    pub(crate) fn new(bits: bool) -> Self {
        WPRODIS0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WPRODIS0_A {
        match self.bits {
            false => WPRODIS0_A::VALUE1,
            true => WPRODIS0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == WPRODIS0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == WPRODIS0_A::VALUE2
    }
}
impl core::ops::Deref for WPRODIS0_R {
    type Target = crate::FieldReader<bool, WPRODIS0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Sector Write Protection Disabled for User 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WPRODIS1_A {
    #[doc = "0: All protected sectors of user 1 are locked if write protection is installed"]
    VALUE1 = 0,
    #[doc = "1: All write-protected sectors of user 1 are temporarily unlocked, if not coincidently locked by user 0 or user 2 or via read protection."]
    VALUE2 = 1,
}
impl From<WPRODIS1_A> for bool {
    #[inline(always)]
    fn from(variant: WPRODIS1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WPRODIS1` reader - Sector Write Protection Disabled for User 1"]
pub struct WPRODIS1_R(crate::FieldReader<bool, WPRODIS1_A>);
impl WPRODIS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        WPRODIS1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WPRODIS1_A {
        match self.bits {
            false => WPRODIS1_A::VALUE1,
            true => WPRODIS1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == WPRODIS1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == WPRODIS1_A::VALUE2
    }
}
impl core::ops::Deref for WPRODIS1_R {
    type Target = crate::FieldReader<bool, WPRODIS1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Flash Sleep Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLM_A {
    #[doc = "0: Flash not in sleep mode"]
    VALUE1 = 0,
    #[doc = "1: Flash is in sleep or shut down mode"]
    VALUE2 = 1,
}
impl From<SLM_A> for bool {
    #[inline(always)]
    fn from(variant: SLM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLM` reader - Flash Sleep Mode"]
pub struct SLM_R(crate::FieldReader<bool, SLM_A>);
impl SLM_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLM_A {
        match self.bits {
            false => SLM_A::VALUE1,
            true => SLM_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == SLM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SLM_A::VALUE2
    }
}
impl core::ops::Deref for SLM_R {
    type Target = crate::FieldReader<bool, SLM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Verify Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VER_A {
    #[doc = "0: The page is correctly programmed or the sector correctly erased. All programmed or erased bits have full expected quality."]
    VALUE1 = 0,
    #[doc = "1: A program verify error or an erase verify error has been detected. Full quality (retention time) of all programmed (\"1\") or erased (\"0\") bits cannot be guaranteed."]
    VALUE2 = 1,
}
impl From<VER_A> for bool {
    #[inline(always)]
    fn from(variant: VER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VER` reader - Verify Error"]
pub struct VER_R(crate::FieldReader<bool, VER_A>);
impl VER_R {
    pub(crate) fn new(bits: bool) -> Self {
        VER_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VER_A {
        match self.bits {
            false => VER_A::VALUE1,
            true => VER_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == VER_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == VER_A::VALUE2
    }
}
impl core::ops::Deref for VER_R {
    type Target = crate::FieldReader<bool, VER_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Program Flash Busy"]
    #[inline(always)]
    pub fn pbusy(&self) -> PBUSY_R {
        PBUSY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Flash Array Busy"]
    #[inline(always)]
    pub fn fabusy(&self) -> FABUSY_R {
        FABUSY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Programming State"]
    #[inline(always)]
    pub fn prog(&self) -> PROG_R {
        PROG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Erase State"]
    #[inline(always)]
    pub fn erase(&self) -> ERASE_R {
        ERASE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Program Flash in Page Mode"]
    #[inline(always)]
    pub fn pfpage(&self) -> PFPAGE_R {
        PFPAGE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Program Flash Operation Error"]
    #[inline(always)]
    pub fn pfoper(&self) -> PFOPER_R {
        PFOPER_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Command Sequence Error"]
    #[inline(always)]
    pub fn sqer(&self) -> SQER_R {
        SQER_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Protection Error"]
    #[inline(always)]
    pub fn proer(&self) -> PROER_R {
        PROER_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - PFLASH Single-Bit Error and Correction"]
    #[inline(always)]
    pub fn pfsber(&self) -> PFSBER_R {
        PFSBER_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 14 - PFLASH Double-Bit Error"]
    #[inline(always)]
    pub fn pfdber(&self) -> PFDBER_R {
        PFDBER_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Protection Installed"]
    #[inline(always)]
    pub fn proin(&self) -> PROIN_R {
        PROIN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Read Protection Installed"]
    #[inline(always)]
    pub fn rproin(&self) -> RPROIN_R {
        RPROIN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Read Protection Disable State"]
    #[inline(always)]
    pub fn rprodis(&self) -> RPRODIS_R {
        RPRODIS_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Sector Write Protection Installed for User 0"]
    #[inline(always)]
    pub fn wproin0(&self) -> WPROIN0_R {
        WPROIN0_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Sector Write Protection Installed for User 1"]
    #[inline(always)]
    pub fn wproin1(&self) -> WPROIN1_R {
        WPROIN1_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Sector OTP Protection Installed for User 2"]
    #[inline(always)]
    pub fn wproin2(&self) -> WPROIN2_R {
        WPROIN2_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Sector Write Protection Disabled for User 0"]
    #[inline(always)]
    pub fn wprodis0(&self) -> WPRODIS0_R {
        WPRODIS0_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Sector Write Protection Disabled for User 1"]
    #[inline(always)]
    pub fn wprodis1(&self) -> WPRODIS1_R {
        WPRODIS1_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Flash Sleep Mode"]
    #[inline(always)]
    pub fn slm(&self) -> SLM_R {
        SLM_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Verify Error"]
    #[inline(always)]
    pub fn ver(&self) -> VER_R {
        VER_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
#[doc = "Flash Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsr](index.html) module"]
pub struct FSR_SPEC;
impl crate::RegisterSpec for FSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fsr::R](R) reader structure"]
impl crate::Readable for FSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FSR to value 0"]
impl crate::Resettable for FSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

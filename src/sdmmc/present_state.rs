#[doc = "Register `PRESENT_STATE` reader"]
pub type R = crate::R<PresentStateSpec>;
#[doc = "Field `COMMAND_INHIBIT_CMD` reader - Command Inhibit (CMD)"]
pub type CommandInhibitCmdR = crate::BitReader;
#[doc = "Command Inhibit (DAT)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandInhibitDat {
    #[doc = "0: Can issue command which uses the DAT line"]
    Value1 = 0,
    #[doc = "1: Cannot issue command which uses the DAT line"]
    Value2 = 1,
}
impl From<CommandInhibitDat> for bool {
    #[inline(always)]
    fn from(variant: CommandInhibitDat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMMAND_INHIBIT_DAT` reader - Command Inhibit (DAT)"]
pub type CommandInhibitDatR = crate::BitReader<CommandInhibitDat>;
impl CommandInhibitDatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CommandInhibitDat {
        match self.bits {
            false => CommandInhibitDat::Value1,
            true => CommandInhibitDat::Value2,
        }
    }
    #[doc = "Can issue command which uses the DAT line"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CommandInhibitDat::Value1
    }
    #[doc = "Cannot issue command which uses the DAT line"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CommandInhibitDat::Value2
    }
}
#[doc = "DAT Line Active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DatLineActive {
    #[doc = "0: DAT line inactive"]
    Value1 = 0,
    #[doc = "1: DAT line active"]
    Value2 = 1,
}
impl From<DatLineActive> for bool {
    #[inline(always)]
    fn from(variant: DatLineActive) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAT_LINE_ACTIVE` reader - DAT Line Active"]
pub type DatLineActiveR = crate::BitReader<DatLineActive>;
impl DatLineActiveR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DatLineActive {
        match self.bits {
            false => DatLineActive::Value1,
            true => DatLineActive::Value2,
        }
    }
    #[doc = "DAT line inactive"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DatLineActive::Value1
    }
    #[doc = "DAT line active"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DatLineActive::Value2
    }
}
#[doc = "Write Transfer Active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WriteTransferActive {
    #[doc = "0: No valid data"]
    Value1 = 0,
    #[doc = "1: Transferring data"]
    Value2 = 1,
}
impl From<WriteTransferActive> for bool {
    #[inline(always)]
    fn from(variant: WriteTransferActive) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRITE_TRANSFER_ACTIVE` reader - Write Transfer Active"]
pub type WriteTransferActiveR = crate::BitReader<WriteTransferActive>;
impl WriteTransferActiveR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WriteTransferActive {
        match self.bits {
            false => WriteTransferActive::Value1,
            true => WriteTransferActive::Value2,
        }
    }
    #[doc = "No valid data"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WriteTransferActive::Value1
    }
    #[doc = "Transferring data"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WriteTransferActive::Value2
    }
}
#[doc = "Read Transfer Active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReadTransferActive {
    #[doc = "0: No valid data"]
    Value1 = 0,
    #[doc = "1: Transferring data"]
    Value2 = 1,
}
impl From<ReadTransferActive> for bool {
    #[inline(always)]
    fn from(variant: ReadTransferActive) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READ_TRANSFER_ACTIVE` reader - Read Transfer Active"]
pub type ReadTransferActiveR = crate::BitReader<ReadTransferActive>;
impl ReadTransferActiveR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ReadTransferActive {
        match self.bits {
            false => ReadTransferActive::Value1,
            true => ReadTransferActive::Value2,
        }
    }
    #[doc = "No valid data"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ReadTransferActive::Value1
    }
    #[doc = "Transferring data"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ReadTransferActive::Value2
    }
}
#[doc = "Buffer Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BufferWriteEnable {
    #[doc = "0: Write Disable"]
    Value1 = 0,
    #[doc = "1: Write Enable."]
    Value2 = 1,
}
impl From<BufferWriteEnable> for bool {
    #[inline(always)]
    fn from(variant: BufferWriteEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUFFER_WRITE_ENABLE` reader - Buffer Write Enable"]
pub type BufferWriteEnableR = crate::BitReader<BufferWriteEnable>;
impl BufferWriteEnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BufferWriteEnable {
        match self.bits {
            false => BufferWriteEnable::Value1,
            true => BufferWriteEnable::Value2,
        }
    }
    #[doc = "Write Disable"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BufferWriteEnable::Value1
    }
    #[doc = "Write Enable."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BufferWriteEnable::Value2
    }
}
#[doc = "Buffer Read Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BufferReadEnable {
    #[doc = "0: Read Disable"]
    Value1 = 0,
    #[doc = "1: Read Enable."]
    Value2 = 1,
}
impl From<BufferReadEnable> for bool {
    #[inline(always)]
    fn from(variant: BufferReadEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUFFER_READ_ENABLE` reader - Buffer Read Enable"]
pub type BufferReadEnableR = crate::BitReader<BufferReadEnable>;
impl BufferReadEnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BufferReadEnable {
        match self.bits {
            false => BufferReadEnable::Value1,
            true => BufferReadEnable::Value2,
        }
    }
    #[doc = "Read Disable"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BufferReadEnable::Value1
    }
    #[doc = "Read Enable."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BufferReadEnable::Value2
    }
}
#[doc = "Card Inserted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CardInserted {
    #[doc = "0: Reset or Debouncing or No Card"]
    Value1 = 0,
    #[doc = "1: Card Inserted"]
    Value2 = 1,
}
impl From<CardInserted> for bool {
    #[inline(always)]
    fn from(variant: CardInserted) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CARD_INSERTED` reader - Card Inserted"]
pub type CardInsertedR = crate::BitReader<CardInserted>;
impl CardInsertedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CardInserted {
        match self.bits {
            false => CardInserted::Value1,
            true => CardInserted::Value2,
        }
    }
    #[doc = "Reset or Debouncing or No Card"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CardInserted::Value1
    }
    #[doc = "Card Inserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CardInserted::Value2
    }
}
#[doc = "Card State Stable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CardStateStable {
    #[doc = "0: Reset of Debouncing"]
    Value1 = 0,
    #[doc = "1: No Card or Inserted"]
    Value2 = 1,
}
impl From<CardStateStable> for bool {
    #[inline(always)]
    fn from(variant: CardStateStable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CARD_STATE_STABLE` reader - Card State Stable"]
pub type CardStateStableR = crate::BitReader<CardStateStable>;
impl CardStateStableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CardStateStable {
        match self.bits {
            false => CardStateStable::Value1,
            true => CardStateStable::Value2,
        }
    }
    #[doc = "Reset of Debouncing"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CardStateStable::Value1
    }
    #[doc = "No Card or Inserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CardStateStable::Value2
    }
}
#[doc = "Card Detect Pin Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CardDetectPinLevel {
    #[doc = "0: No Card present (SDCD = 1)"]
    Value1 = 0,
    #[doc = "1: Card present (SDCD = 0)"]
    Value2 = 1,
}
impl From<CardDetectPinLevel> for bool {
    #[inline(always)]
    fn from(variant: CardDetectPinLevel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CARD_DETECT_PIN_LEVEL` reader - Card Detect Pin Level"]
pub type CardDetectPinLevelR = crate::BitReader<CardDetectPinLevel>;
impl CardDetectPinLevelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CardDetectPinLevel {
        match self.bits {
            false => CardDetectPinLevel::Value1,
            true => CardDetectPinLevel::Value2,
        }
    }
    #[doc = "No Card present (SDCD = 1)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CardDetectPinLevel::Value1
    }
    #[doc = "Card present (SDCD = 0)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CardDetectPinLevel::Value2
    }
}
#[doc = "Write Protect Switch Pin Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WriteProtectPinLevel {
    #[doc = "0: Write protected (SDWP = 1)"]
    Value1 = 0,
    #[doc = "1: Write enabled (SDWP = 0)"]
    Value2 = 1,
}
impl From<WriteProtectPinLevel> for bool {
    #[inline(always)]
    fn from(variant: WriteProtectPinLevel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRITE_PROTECT_PIN_LEVEL` reader - Write Protect Switch Pin Level"]
pub type WriteProtectPinLevelR = crate::BitReader<WriteProtectPinLevel>;
impl WriteProtectPinLevelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WriteProtectPinLevel {
        match self.bits {
            false => WriteProtectPinLevel::Value1,
            true => WriteProtectPinLevel::Value2,
        }
    }
    #[doc = "Write protected (SDWP = 1)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WriteProtectPinLevel::Value1
    }
    #[doc = "Write enabled (SDWP = 0)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WriteProtectPinLevel::Value2
    }
}
#[doc = "Field `DAT_3_0_PIN_LEVEL` reader - Line Signal Level"]
pub type Dat3_0PinLevelR = crate::FieldReader;
#[doc = "Field `CMD_LINE_LEVEL` reader - CMD Line Signal Level"]
pub type CmdLineLevelR = crate::BitReader;
#[doc = "Field `DAT_7_4_PIN_LEVEL` reader - Line Signal Level"]
pub type Dat7_4PinLevelR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Command Inhibit (CMD)"]
    #[inline(always)]
    pub fn command_inhibit_cmd(&self) -> CommandInhibitCmdR {
        CommandInhibitCmdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Command Inhibit (DAT)"]
    #[inline(always)]
    pub fn command_inhibit_dat(&self) -> CommandInhibitDatR {
        CommandInhibitDatR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DAT Line Active"]
    #[inline(always)]
    pub fn dat_line_active(&self) -> DatLineActiveR {
        DatLineActiveR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Write Transfer Active"]
    #[inline(always)]
    pub fn write_transfer_active(&self) -> WriteTransferActiveR {
        WriteTransferActiveR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Read Transfer Active"]
    #[inline(always)]
    pub fn read_transfer_active(&self) -> ReadTransferActiveR {
        ReadTransferActiveR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Buffer Write Enable"]
    #[inline(always)]
    pub fn buffer_write_enable(&self) -> BufferWriteEnableR {
        BufferWriteEnableR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Buffer Read Enable"]
    #[inline(always)]
    pub fn buffer_read_enable(&self) -> BufferReadEnableR {
        BufferReadEnableR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Card Inserted"]
    #[inline(always)]
    pub fn card_inserted(&self) -> CardInsertedR {
        CardInsertedR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Card State Stable"]
    #[inline(always)]
    pub fn card_state_stable(&self) -> CardStateStableR {
        CardStateStableR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Card Detect Pin Level"]
    #[inline(always)]
    pub fn card_detect_pin_level(&self) -> CardDetectPinLevelR {
        CardDetectPinLevelR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Write Protect Switch Pin Level"]
    #[inline(always)]
    pub fn write_protect_pin_level(&self) -> WriteProtectPinLevelR {
        WriteProtectPinLevelR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - Line Signal Level"]
    #[inline(always)]
    pub fn dat_3_0_pin_level(&self) -> Dat3_0PinLevelR {
        Dat3_0PinLevelR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - CMD Line Signal Level"]
    #[inline(always)]
    pub fn cmd_line_level(&self) -> CmdLineLevelR {
        CmdLineLevelR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:28 - Line Signal Level"]
    #[inline(always)]
    pub fn dat_7_4_pin_level(&self) -> Dat7_4PinLevelR {
        Dat7_4PinLevelR::new(((self.bits >> 25) & 0x0f) as u8)
    }
}
#[doc = "Present State Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`present_state::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PresentStateSpec;
impl crate::RegisterSpec for PresentStateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`present_state::R`](R) reader structure"]
impl crate::Readable for PresentStateSpec {}
#[doc = "`reset()` method sets PRESENT_STATE to value 0"]
impl crate::Resettable for PresentStateSpec {
    const RESET_VALUE: u32 = 0;
}

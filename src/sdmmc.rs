#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    block_size: BlockSize,
    block_count: BlockCount,
    argument1: Argument1,
    transfer_mode: TransferMode,
    command: Command,
    response0: Response0,
    response2: Response2,
    response4: Response4,
    response6: Response6,
    data_buffer: DataBuffer,
    present_state: PresentState,
    host_ctrl: HostCtrl,
    power_ctrl: PowerCtrl,
    block_gap_ctrl: BlockGapCtrl,
    wakeup_ctrl: WakeupCtrl,
    clock_ctrl: ClockCtrl,
    timeout_ctrl: TimeoutCtrl,
    sw_reset: SwReset,
    int_status_norm: IntStatusNorm,
    int_status_err: IntStatusErr,
    en_int_status_norm: EnIntStatusNorm,
    en_int_status_err: EnIntStatusErr,
    en_int_signal_norm: EnIntSignalNorm,
    en_int_signal_err: EnIntSignalErr,
    acmd_err_status: AcmdErrStatus,
    _reserved25: [u8; 0x02],
    capabilities: Capabilities,
    capabilities_hi: CapabilitiesHi,
    max_current_cap: MaxCurrentCap,
    _reserved28: [u8; 0x04],
    force_event_acmd_err_status: ForceEventAcmdErrStatus,
    force_event_err_status: ForceEventErrStatus,
    _reserved30: [u8; 0x20],
    debug_sel: DebugSel,
    _reserved31: [u8; 0x84],
    slot_int_status: SlotIntStatus,
}
impl RegisterBlock {
    #[doc = "0x04 - Block Size Register"]
    #[inline(always)]
    pub const fn block_size(&self) -> &BlockSize {
        &self.block_size
    }
    #[doc = "0x06 - Block Count Register"]
    #[inline(always)]
    pub const fn block_count(&self) -> &BlockCount {
        &self.block_count
    }
    #[doc = "0x08 - Argument1 Register"]
    #[inline(always)]
    pub const fn argument1(&self) -> &Argument1 {
        &self.argument1
    }
    #[doc = "0x0c - Transfer Mode Register"]
    #[inline(always)]
    pub const fn transfer_mode(&self) -> &TransferMode {
        &self.transfer_mode
    }
    #[doc = "0x0e - Command Register"]
    #[inline(always)]
    pub const fn command(&self) -> &Command {
        &self.command
    }
    #[doc = "0x10 - Response 0 Register"]
    #[inline(always)]
    pub const fn response0(&self) -> &Response0 {
        &self.response0
    }
    #[doc = "0x14 - Response 2 Register"]
    #[inline(always)]
    pub const fn response2(&self) -> &Response2 {
        &self.response2
    }
    #[doc = "0x18 - Response 4 Register"]
    #[inline(always)]
    pub const fn response4(&self) -> &Response4 {
        &self.response4
    }
    #[doc = "0x1c - Response 6 Register"]
    #[inline(always)]
    pub const fn response6(&self) -> &Response6 {
        &self.response6
    }
    #[doc = "0x20 - Data Buffer Register"]
    #[inline(always)]
    pub const fn data_buffer(&self) -> &DataBuffer {
        &self.data_buffer
    }
    #[doc = "0x24 - Present State Register"]
    #[inline(always)]
    pub const fn present_state(&self) -> &PresentState {
        &self.present_state
    }
    #[doc = "0x28 - Host Control Register"]
    #[inline(always)]
    pub const fn host_ctrl(&self) -> &HostCtrl {
        &self.host_ctrl
    }
    #[doc = "0x29 - Power Control Register"]
    #[inline(always)]
    pub const fn power_ctrl(&self) -> &PowerCtrl {
        &self.power_ctrl
    }
    #[doc = "0x2a - Block Gap Control Register"]
    #[inline(always)]
    pub const fn block_gap_ctrl(&self) -> &BlockGapCtrl {
        &self.block_gap_ctrl
    }
    #[doc = "0x2b - Wake-up Control Register"]
    #[inline(always)]
    pub const fn wakeup_ctrl(&self) -> &WakeupCtrl {
        &self.wakeup_ctrl
    }
    #[doc = "0x2c - Clock Control Register"]
    #[inline(always)]
    pub const fn clock_ctrl(&self) -> &ClockCtrl {
        &self.clock_ctrl
    }
    #[doc = "0x2e - Timeout Control Register"]
    #[inline(always)]
    pub const fn timeout_ctrl(&self) -> &TimeoutCtrl {
        &self.timeout_ctrl
    }
    #[doc = "0x2f - Software Reset Register"]
    #[inline(always)]
    pub const fn sw_reset(&self) -> &SwReset {
        &self.sw_reset
    }
    #[doc = "0x30 - Normal Interrupt Status Register"]
    #[inline(always)]
    pub const fn int_status_norm(&self) -> &IntStatusNorm {
        &self.int_status_norm
    }
    #[doc = "0x32 - Error Interrupt Status Register"]
    #[inline(always)]
    pub const fn int_status_err(&self) -> &IntStatusErr {
        &self.int_status_err
    }
    #[doc = "0x34 - Normal Interrupt Status Enable Register"]
    #[inline(always)]
    pub const fn en_int_status_norm(&self) -> &EnIntStatusNorm {
        &self.en_int_status_norm
    }
    #[doc = "0x36 - Error Interrupt Status Enable Register"]
    #[inline(always)]
    pub const fn en_int_status_err(&self) -> &EnIntStatusErr {
        &self.en_int_status_err
    }
    #[doc = "0x38 - Normal Interrupt Signal Enable Register"]
    #[inline(always)]
    pub const fn en_int_signal_norm(&self) -> &EnIntSignalNorm {
        &self.en_int_signal_norm
    }
    #[doc = "0x3a - Error Interrupt Signal Enable Register"]
    #[inline(always)]
    pub const fn en_int_signal_err(&self) -> &EnIntSignalErr {
        &self.en_int_signal_err
    }
    #[doc = "0x3c - Auto CMD Error Status Register"]
    #[inline(always)]
    pub const fn acmd_err_status(&self) -> &AcmdErrStatus {
        &self.acmd_err_status
    }
    #[doc = "0x40 - Capabilities Register"]
    #[inline(always)]
    pub const fn capabilities(&self) -> &Capabilities {
        &self.capabilities
    }
    #[doc = "0x44 - Capabilities Register High"]
    #[inline(always)]
    pub const fn capabilities_hi(&self) -> &CapabilitiesHi {
        &self.capabilities_hi
    }
    #[doc = "0x48 - Maximum Current Capabilities Register"]
    #[inline(always)]
    pub const fn max_current_cap(&self) -> &MaxCurrentCap {
        &self.max_current_cap
    }
    #[doc = "0x50 - Force Event Register for Auto CMD Error Status"]
    #[inline(always)]
    pub const fn force_event_acmd_err_status(&self) -> &ForceEventAcmdErrStatus {
        &self.force_event_acmd_err_status
    }
    #[doc = "0x52 - Force Event Register for Error Interrupt Status"]
    #[inline(always)]
    pub const fn force_event_err_status(&self) -> &ForceEventErrStatus {
        &self.force_event_err_status
    }
    #[doc = "0x74 - Debug Selection Register"]
    #[inline(always)]
    pub const fn debug_sel(&self) -> &DebugSel {
        &self.debug_sel
    }
    #[doc = "0xfc - Slot Interrupt Status Register"]
    #[inline(always)]
    pub const fn slot_int_status(&self) -> &SlotIntStatus {
        &self.slot_int_status
    }
}
#[doc = "BLOCK_SIZE (rw) register accessor: Block Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`block_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`block_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@block_size`]
module"]
#[doc(alias = "BLOCK_SIZE")]
pub type BlockSize = crate::Reg<block_size::BlockSizeSpec>;
#[doc = "Block Size Register"]
pub mod block_size;
#[doc = "BLOCK_COUNT (rw) register accessor: Block Count Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`block_count::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`block_count::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@block_count`]
module"]
#[doc(alias = "BLOCK_COUNT")]
pub type BlockCount = crate::Reg<block_count::BlockCountSpec>;
#[doc = "Block Count Register"]
pub mod block_count;
#[doc = "ARGUMENT1 (rw) register accessor: Argument1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`argument1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`argument1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@argument1`]
module"]
#[doc(alias = "ARGUMENT1")]
pub type Argument1 = crate::Reg<argument1::Argument1Spec>;
#[doc = "Argument1 Register"]
pub mod argument1;
#[doc = "TRANSFER_MODE (rw) register accessor: Transfer Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`transfer_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`transfer_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@transfer_mode`]
module"]
#[doc(alias = "TRANSFER_MODE")]
pub type TransferMode = crate::Reg<transfer_mode::TransferModeSpec>;
#[doc = "Transfer Mode Register"]
pub mod transfer_mode;
#[doc = "COMMAND (rw) register accessor: Command Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`command::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`command::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@command`]
module"]
#[doc(alias = "COMMAND")]
pub type Command = crate::Reg<command::CommandSpec>;
#[doc = "Command Register"]
pub mod command;
#[doc = "RESPONSE0 (r) register accessor: Response 0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`response0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@response0`]
module"]
#[doc(alias = "RESPONSE0")]
pub type Response0 = crate::Reg<response0::Response0Spec>;
#[doc = "Response 0 Register"]
pub mod response0;
#[doc = "RESPONSE2 (r) register accessor: Response 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`response2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@response2`]
module"]
#[doc(alias = "RESPONSE2")]
pub type Response2 = crate::Reg<response2::Response2Spec>;
#[doc = "Response 2 Register"]
pub mod response2;
#[doc = "RESPONSE4 (r) register accessor: Response 4 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`response4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@response4`]
module"]
#[doc(alias = "RESPONSE4")]
pub type Response4 = crate::Reg<response4::Response4Spec>;
#[doc = "Response 4 Register"]
pub mod response4;
#[doc = "RESPONSE6 (r) register accessor: Response 6 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`response6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@response6`]
module"]
#[doc(alias = "RESPONSE6")]
pub type Response6 = crate::Reg<response6::Response6Spec>;
#[doc = "Response 6 Register"]
pub mod response6;
#[doc = "DATA_BUFFER (rw) register accessor: Data Buffer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data_buffer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data_buffer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_buffer`]
module"]
#[doc(alias = "DATA_BUFFER")]
pub type DataBuffer = crate::Reg<data_buffer::DataBufferSpec>;
#[doc = "Data Buffer Register"]
pub mod data_buffer;
#[doc = "PRESENT_STATE (r) register accessor: Present State Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`present_state::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@present_state`]
module"]
#[doc(alias = "PRESENT_STATE")]
pub type PresentState = crate::Reg<present_state::PresentStateSpec>;
#[doc = "Present State Register"]
pub mod present_state;
#[doc = "HOST_CTRL (rw) register accessor: Host Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_ctrl`]
module"]
#[doc(alias = "HOST_CTRL")]
pub type HostCtrl = crate::Reg<host_ctrl::HostCtrlSpec>;
#[doc = "Host Control Register"]
pub mod host_ctrl;
#[doc = "POWER_CTRL (rw) register accessor: Power Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`power_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`power_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@power_ctrl`]
module"]
#[doc(alias = "POWER_CTRL")]
pub type PowerCtrl = crate::Reg<power_ctrl::PowerCtrlSpec>;
#[doc = "Power Control Register"]
pub mod power_ctrl;
#[doc = "BLOCK_GAP_CTRL (rw) register accessor: Block Gap Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`block_gap_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`block_gap_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@block_gap_ctrl`]
module"]
#[doc(alias = "BLOCK_GAP_CTRL")]
pub type BlockGapCtrl = crate::Reg<block_gap_ctrl::BlockGapCtrlSpec>;
#[doc = "Block Gap Control Register"]
pub mod block_gap_ctrl;
#[doc = "WAKEUP_CTRL (rw) register accessor: Wake-up Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wakeup_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wakeup_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wakeup_ctrl`]
module"]
#[doc(alias = "WAKEUP_CTRL")]
pub type WakeupCtrl = crate::Reg<wakeup_ctrl::WakeupCtrlSpec>;
#[doc = "Wake-up Control Register"]
pub mod wakeup_ctrl;
#[doc = "CLOCK_CTRL (rw) register accessor: Clock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clock_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_ctrl`]
module"]
#[doc(alias = "CLOCK_CTRL")]
pub type ClockCtrl = crate::Reg<clock_ctrl::ClockCtrlSpec>;
#[doc = "Clock Control Register"]
pub mod clock_ctrl;
#[doc = "TIMEOUT_CTRL (rw) register accessor: Timeout Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timeout_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timeout_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timeout_ctrl`]
module"]
#[doc(alias = "TIMEOUT_CTRL")]
pub type TimeoutCtrl = crate::Reg<timeout_ctrl::TimeoutCtrlSpec>;
#[doc = "Timeout Control Register"]
pub mod timeout_ctrl;
#[doc = "SW_RESET (rw) register accessor: Software Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sw_reset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sw_reset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sw_reset`]
module"]
#[doc(alias = "SW_RESET")]
pub type SwReset = crate::Reg<sw_reset::SwResetSpec>;
#[doc = "Software Reset Register"]
pub mod sw_reset;
#[doc = "INT_STATUS_NORM (rw) register accessor: Normal Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_status_norm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_status_norm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_status_norm`]
module"]
#[doc(alias = "INT_STATUS_NORM")]
pub type IntStatusNorm = crate::Reg<int_status_norm::IntStatusNormSpec>;
#[doc = "Normal Interrupt Status Register"]
pub mod int_status_norm;
#[doc = "INT_STATUS_ERR (rw) register accessor: Error Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_status_err::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_status_err::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_status_err`]
module"]
#[doc(alias = "INT_STATUS_ERR")]
pub type IntStatusErr = crate::Reg<int_status_err::IntStatusErrSpec>;
#[doc = "Error Interrupt Status Register"]
pub mod int_status_err;
#[doc = "EN_INT_STATUS_NORM (rw) register accessor: Normal Interrupt Status Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`en_int_status_norm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`en_int_status_norm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@en_int_status_norm`]
module"]
#[doc(alias = "EN_INT_STATUS_NORM")]
pub type EnIntStatusNorm = crate::Reg<en_int_status_norm::EnIntStatusNormSpec>;
#[doc = "Normal Interrupt Status Enable Register"]
pub mod en_int_status_norm;
#[doc = "EN_INT_STATUS_ERR (rw) register accessor: Error Interrupt Status Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`en_int_status_err::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`en_int_status_err::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@en_int_status_err`]
module"]
#[doc(alias = "EN_INT_STATUS_ERR")]
pub type EnIntStatusErr = crate::Reg<en_int_status_err::EnIntStatusErrSpec>;
#[doc = "Error Interrupt Status Enable Register"]
pub mod en_int_status_err;
#[doc = "EN_INT_SIGNAL_NORM (rw) register accessor: Normal Interrupt Signal Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`en_int_signal_norm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`en_int_signal_norm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@en_int_signal_norm`]
module"]
#[doc(alias = "EN_INT_SIGNAL_NORM")]
pub type EnIntSignalNorm = crate::Reg<en_int_signal_norm::EnIntSignalNormSpec>;
#[doc = "Normal Interrupt Signal Enable Register"]
pub mod en_int_signal_norm;
#[doc = "EN_INT_SIGNAL_ERR (rw) register accessor: Error Interrupt Signal Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`en_int_signal_err::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`en_int_signal_err::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@en_int_signal_err`]
module"]
#[doc(alias = "EN_INT_SIGNAL_ERR")]
pub type EnIntSignalErr = crate::Reg<en_int_signal_err::EnIntSignalErrSpec>;
#[doc = "Error Interrupt Signal Enable Register"]
pub mod en_int_signal_err;
#[doc = "ACMD_ERR_STATUS (r) register accessor: Auto CMD Error Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acmd_err_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acmd_err_status`]
module"]
#[doc(alias = "ACMD_ERR_STATUS")]
pub type AcmdErrStatus = crate::Reg<acmd_err_status::AcmdErrStatusSpec>;
#[doc = "Auto CMD Error Status Register"]
pub mod acmd_err_status;
#[doc = "CAPABILITIES (r) register accessor: Capabilities Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`capabilities::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capabilities`]
module"]
#[doc(alias = "CAPABILITIES")]
pub type Capabilities = crate::Reg<capabilities::CapabilitiesSpec>;
#[doc = "Capabilities Register"]
pub mod capabilities;
#[doc = "CAPABILITIES_HI (r) register accessor: Capabilities Register High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`capabilities_hi::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capabilities_hi`]
module"]
#[doc(alias = "CAPABILITIES_HI")]
pub type CapabilitiesHi = crate::Reg<capabilities_hi::CapabilitiesHiSpec>;
#[doc = "Capabilities Register High"]
pub mod capabilities_hi;
#[doc = "MAX_CURRENT_CAP (r) register accessor: Maximum Current Capabilities Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`max_current_cap::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@max_current_cap`]
module"]
#[doc(alias = "MAX_CURRENT_CAP")]
pub type MaxCurrentCap = crate::Reg<max_current_cap::MaxCurrentCapSpec>;
#[doc = "Maximum Current Capabilities Register"]
pub mod max_current_cap;
#[doc = "FORCE_EVENT_ACMD_ERR_STATUS (w) register accessor: Force Event Register for Auto CMD Error Status\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`force_event_acmd_err_status::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@force_event_acmd_err_status`]
module"]
#[doc(alias = "FORCE_EVENT_ACMD_ERR_STATUS")]
pub type ForceEventAcmdErrStatus = crate::Reg<force_event_acmd_err_status::ForceEventAcmdErrStatusSpec>;
#[doc = "Force Event Register for Auto CMD Error Status"]
pub mod force_event_acmd_err_status;
#[doc = "FORCE_EVENT_ERR_STATUS (w) register accessor: Force Event Register for Error Interrupt Status\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`force_event_err_status::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@force_event_err_status`]
module"]
#[doc(alias = "FORCE_EVENT_ERR_STATUS")]
pub type ForceEventErrStatus = crate::Reg<force_event_err_status::ForceEventErrStatusSpec>;
#[doc = "Force Event Register for Error Interrupt Status"]
pub mod force_event_err_status;
#[doc = "DEBUG_SEL (w) register accessor: Debug Selection Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`debug_sel::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug_sel`]
module"]
#[doc(alias = "DEBUG_SEL")]
pub type DebugSel = crate::Reg<debug_sel::DebugSelSpec>;
#[doc = "Debug Selection Register"]
pub mod debug_sel;
#[doc = "SLOT_INT_STATUS (r) register accessor: Slot Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slot_int_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slot_int_status`]
module"]
#[doc(alias = "SLOT_INT_STATUS")]
pub type SlotIntStatus = crate::Reg<slot_int_status::SlotIntStatusSpec>;
#[doc = "Slot Interrupt Status Register"]
pub mod slot_int_status;

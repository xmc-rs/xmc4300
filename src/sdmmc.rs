#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    #[doc = "0x04 - Block Size Register"]
    pub block_size: BLOCK_SIZE,
    #[doc = "0x06 - Block Count Register"]
    pub block_count: BLOCK_COUNT,
    #[doc = "0x08 - Argument1 Register"]
    pub argument1: ARGUMENT1,
    #[doc = "0x0c - Transfer Mode Register"]
    pub transfer_mode: TRANSFER_MODE,
    #[doc = "0x0e - Command Register"]
    pub command: COMMAND,
    #[doc = "0x10 - Response 0 Register"]
    pub response0: RESPONSE0,
    #[doc = "0x14 - Response 2 Register"]
    pub response2: RESPONSE2,
    #[doc = "0x18 - Response 4 Register"]
    pub response4: RESPONSE4,
    #[doc = "0x1c - Response 6 Register"]
    pub response6: RESPONSE6,
    #[doc = "0x20 - Data Buffer Register"]
    pub data_buffer: DATA_BUFFER,
    #[doc = "0x24 - Present State Register"]
    pub present_state: PRESENT_STATE,
    #[doc = "0x28 - Host Control Register"]
    pub host_ctrl: HOST_CTRL,
    #[doc = "0x29 - Power Control Register"]
    pub power_ctrl: POWER_CTRL,
    #[doc = "0x2a - Block Gap Control Register"]
    pub block_gap_ctrl: BLOCK_GAP_CTRL,
    #[doc = "0x2b - Wake-up Control Register"]
    pub wakeup_ctrl: WAKEUP_CTRL,
    #[doc = "0x2c - Clock Control Register"]
    pub clock_ctrl: CLOCK_CTRL,
    #[doc = "0x2e - Timeout Control Register"]
    pub timeout_ctrl: TIMEOUT_CTRL,
    #[doc = "0x2f - Software Reset Register"]
    pub sw_reset: SW_RESET,
    #[doc = "0x30 - Normal Interrupt Status Register"]
    pub int_status_norm: INT_STATUS_NORM,
    #[doc = "0x32 - Error Interrupt Status Register"]
    pub int_status_err: INT_STATUS_ERR,
    #[doc = "0x34 - Normal Interrupt Status Enable Register"]
    pub en_int_status_norm: EN_INT_STATUS_NORM,
    #[doc = "0x36 - Error Interrupt Status Enable Register"]
    pub en_int_status_err: EN_INT_STATUS_ERR,
    #[doc = "0x38 - Normal Interrupt Signal Enable Register"]
    pub en_int_signal_norm: EN_INT_SIGNAL_NORM,
    #[doc = "0x3a - Error Interrupt Signal Enable Register"]
    pub en_int_signal_err: EN_INT_SIGNAL_ERR,
    #[doc = "0x3c - Auto CMD Error Status Register"]
    pub acmd_err_status: ACMD_ERR_STATUS,
    _reserved25: [u8; 0x02],
    #[doc = "0x40 - Capabilities Register"]
    pub capabilities: CAPABILITIES,
    #[doc = "0x44 - Capabilities Register High"]
    pub capabilities_hi: CAPABILITIES_HI,
    #[doc = "0x48 - Maximum Current Capabilities Register"]
    pub max_current_cap: MAX_CURRENT_CAP,
    _reserved28: [u8; 0x04],
    #[doc = "0x50 - Force Event Register for Auto CMD Error Status"]
    pub force_event_acmd_err_status: FORCE_EVENT_ACMD_ERR_STATUS,
    #[doc = "0x52 - Force Event Register for Error Interrupt Status"]
    pub force_event_err_status: FORCE_EVENT_ERR_STATUS,
    _reserved30: [u8; 0x20],
    #[doc = "0x74 - Debug Selection Register"]
    pub debug_sel: DEBUG_SEL,
    _reserved31: [u8; 0x84],
    #[doc = "0xfc - Slot Interrupt Status Register"]
    pub slot_int_status: SLOT_INT_STATUS,
}
#[doc = "BLOCK_SIZE (rw) register accessor: Block Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`block_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`block_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@block_size`]
module"]
pub type BLOCK_SIZE = crate::Reg<block_size::BLOCK_SIZE_SPEC>;
#[doc = "Block Size Register"]
pub mod block_size;
#[doc = "BLOCK_COUNT (rw) register accessor: Block Count Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`block_count::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`block_count::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@block_count`]
module"]
pub type BLOCK_COUNT = crate::Reg<block_count::BLOCK_COUNT_SPEC>;
#[doc = "Block Count Register"]
pub mod block_count;
#[doc = "ARGUMENT1 (rw) register accessor: Argument1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`argument1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`argument1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@argument1`]
module"]
pub type ARGUMENT1 = crate::Reg<argument1::ARGUMENT1_SPEC>;
#[doc = "Argument1 Register"]
pub mod argument1;
#[doc = "TRANSFER_MODE (rw) register accessor: Transfer Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`transfer_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`transfer_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@transfer_mode`]
module"]
pub type TRANSFER_MODE = crate::Reg<transfer_mode::TRANSFER_MODE_SPEC>;
#[doc = "Transfer Mode Register"]
pub mod transfer_mode;
#[doc = "COMMAND (rw) register accessor: Command Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`command::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`command::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@command`]
module"]
pub type COMMAND = crate::Reg<command::COMMAND_SPEC>;
#[doc = "Command Register"]
pub mod command;
#[doc = "RESPONSE0 (r) register accessor: Response 0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`response0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@response0`]
module"]
pub type RESPONSE0 = crate::Reg<response0::RESPONSE0_SPEC>;
#[doc = "Response 0 Register"]
pub mod response0;
#[doc = "RESPONSE2 (r) register accessor: Response 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`response2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@response2`]
module"]
pub type RESPONSE2 = crate::Reg<response2::RESPONSE2_SPEC>;
#[doc = "Response 2 Register"]
pub mod response2;
#[doc = "RESPONSE4 (r) register accessor: Response 4 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`response4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@response4`]
module"]
pub type RESPONSE4 = crate::Reg<response4::RESPONSE4_SPEC>;
#[doc = "Response 4 Register"]
pub mod response4;
#[doc = "RESPONSE6 (r) register accessor: Response 6 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`response6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@response6`]
module"]
pub type RESPONSE6 = crate::Reg<response6::RESPONSE6_SPEC>;
#[doc = "Response 6 Register"]
pub mod response6;
#[doc = "DATA_BUFFER (rw) register accessor: Data Buffer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data_buffer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data_buffer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_buffer`]
module"]
pub type DATA_BUFFER = crate::Reg<data_buffer::DATA_BUFFER_SPEC>;
#[doc = "Data Buffer Register"]
pub mod data_buffer;
#[doc = "PRESENT_STATE (r) register accessor: Present State Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`present_state::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@present_state`]
module"]
pub type PRESENT_STATE = crate::Reg<present_state::PRESENT_STATE_SPEC>;
#[doc = "Present State Register"]
pub mod present_state;
#[doc = "HOST_CTRL (rw) register accessor: Host Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_ctrl`]
module"]
pub type HOST_CTRL = crate::Reg<host_ctrl::HOST_CTRL_SPEC>;
#[doc = "Host Control Register"]
pub mod host_ctrl;
#[doc = "POWER_CTRL (rw) register accessor: Power Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`power_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`power_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@power_ctrl`]
module"]
pub type POWER_CTRL = crate::Reg<power_ctrl::POWER_CTRL_SPEC>;
#[doc = "Power Control Register"]
pub mod power_ctrl;
#[doc = "BLOCK_GAP_CTRL (rw) register accessor: Block Gap Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`block_gap_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`block_gap_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@block_gap_ctrl`]
module"]
pub type BLOCK_GAP_CTRL = crate::Reg<block_gap_ctrl::BLOCK_GAP_CTRL_SPEC>;
#[doc = "Block Gap Control Register"]
pub mod block_gap_ctrl;
#[doc = "WAKEUP_CTRL (rw) register accessor: Wake-up Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wakeup_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wakeup_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wakeup_ctrl`]
module"]
pub type WAKEUP_CTRL = crate::Reg<wakeup_ctrl::WAKEUP_CTRL_SPEC>;
#[doc = "Wake-up Control Register"]
pub mod wakeup_ctrl;
#[doc = "CLOCK_CTRL (rw) register accessor: Clock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clock_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_ctrl`]
module"]
pub type CLOCK_CTRL = crate::Reg<clock_ctrl::CLOCK_CTRL_SPEC>;
#[doc = "Clock Control Register"]
pub mod clock_ctrl;
#[doc = "TIMEOUT_CTRL (rw) register accessor: Timeout Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timeout_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timeout_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timeout_ctrl`]
module"]
pub type TIMEOUT_CTRL = crate::Reg<timeout_ctrl::TIMEOUT_CTRL_SPEC>;
#[doc = "Timeout Control Register"]
pub mod timeout_ctrl;
#[doc = "SW_RESET (rw) register accessor: Software Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sw_reset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sw_reset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sw_reset`]
module"]
pub type SW_RESET = crate::Reg<sw_reset::SW_RESET_SPEC>;
#[doc = "Software Reset Register"]
pub mod sw_reset;
#[doc = "INT_STATUS_NORM (rw) register accessor: Normal Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_status_norm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_status_norm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_status_norm`]
module"]
pub type INT_STATUS_NORM = crate::Reg<int_status_norm::INT_STATUS_NORM_SPEC>;
#[doc = "Normal Interrupt Status Register"]
pub mod int_status_norm;
#[doc = "INT_STATUS_ERR (rw) register accessor: Error Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_status_err::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_status_err::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_status_err`]
module"]
pub type INT_STATUS_ERR = crate::Reg<int_status_err::INT_STATUS_ERR_SPEC>;
#[doc = "Error Interrupt Status Register"]
pub mod int_status_err;
#[doc = "EN_INT_STATUS_NORM (rw) register accessor: Normal Interrupt Status Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`en_int_status_norm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`en_int_status_norm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@en_int_status_norm`]
module"]
pub type EN_INT_STATUS_NORM = crate::Reg<en_int_status_norm::EN_INT_STATUS_NORM_SPEC>;
#[doc = "Normal Interrupt Status Enable Register"]
pub mod en_int_status_norm;
#[doc = "EN_INT_STATUS_ERR (rw) register accessor: Error Interrupt Status Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`en_int_status_err::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`en_int_status_err::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@en_int_status_err`]
module"]
pub type EN_INT_STATUS_ERR = crate::Reg<en_int_status_err::EN_INT_STATUS_ERR_SPEC>;
#[doc = "Error Interrupt Status Enable Register"]
pub mod en_int_status_err;
#[doc = "EN_INT_SIGNAL_NORM (rw) register accessor: Normal Interrupt Signal Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`en_int_signal_norm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`en_int_signal_norm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@en_int_signal_norm`]
module"]
pub type EN_INT_SIGNAL_NORM = crate::Reg<en_int_signal_norm::EN_INT_SIGNAL_NORM_SPEC>;
#[doc = "Normal Interrupt Signal Enable Register"]
pub mod en_int_signal_norm;
#[doc = "EN_INT_SIGNAL_ERR (rw) register accessor: Error Interrupt Signal Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`en_int_signal_err::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`en_int_signal_err::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@en_int_signal_err`]
module"]
pub type EN_INT_SIGNAL_ERR = crate::Reg<en_int_signal_err::EN_INT_SIGNAL_ERR_SPEC>;
#[doc = "Error Interrupt Signal Enable Register"]
pub mod en_int_signal_err;
#[doc = "ACMD_ERR_STATUS (r) register accessor: Auto CMD Error Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acmd_err_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acmd_err_status`]
module"]
pub type ACMD_ERR_STATUS = crate::Reg<acmd_err_status::ACMD_ERR_STATUS_SPEC>;
#[doc = "Auto CMD Error Status Register"]
pub mod acmd_err_status;
#[doc = "CAPABILITIES (r) register accessor: Capabilities Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`capabilities::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capabilities`]
module"]
pub type CAPABILITIES = crate::Reg<capabilities::CAPABILITIES_SPEC>;
#[doc = "Capabilities Register"]
pub mod capabilities;
#[doc = "CAPABILITIES_HI (r) register accessor: Capabilities Register High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`capabilities_hi::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capabilities_hi`]
module"]
pub type CAPABILITIES_HI = crate::Reg<capabilities_hi::CAPABILITIES_HI_SPEC>;
#[doc = "Capabilities Register High"]
pub mod capabilities_hi;
#[doc = "MAX_CURRENT_CAP (r) register accessor: Maximum Current Capabilities Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`max_current_cap::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@max_current_cap`]
module"]
pub type MAX_CURRENT_CAP = crate::Reg<max_current_cap::MAX_CURRENT_CAP_SPEC>;
#[doc = "Maximum Current Capabilities Register"]
pub mod max_current_cap;
#[doc = "FORCE_EVENT_ACMD_ERR_STATUS (w) register accessor: Force Event Register for Auto CMD Error Status\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`force_event_acmd_err_status::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@force_event_acmd_err_status`]
module"]
pub type FORCE_EVENT_ACMD_ERR_STATUS = crate::Reg<force_event_acmd_err_status::FORCE_EVENT_ACMD_ERR_STATUS_SPEC>;
#[doc = "Force Event Register for Auto CMD Error Status"]
pub mod force_event_acmd_err_status;
#[doc = "FORCE_EVENT_ERR_STATUS (w) register accessor: Force Event Register for Error Interrupt Status\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`force_event_err_status::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@force_event_err_status`]
module"]
pub type FORCE_EVENT_ERR_STATUS = crate::Reg<force_event_err_status::FORCE_EVENT_ERR_STATUS_SPEC>;
#[doc = "Force Event Register for Error Interrupt Status"]
pub mod force_event_err_status;
#[doc = "DEBUG_SEL (w) register accessor: Debug Selection Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`debug_sel::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug_sel`]
module"]
pub type DEBUG_SEL = crate::Reg<debug_sel::DEBUG_SEL_SPEC>;
#[doc = "Debug Selection Register"]
pub mod debug_sel;
#[doc = "SLOT_INT_STATUS (r) register accessor: Slot Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slot_int_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slot_int_status`]
module"]
pub type SLOT_INT_STATUS = crate::Reg<slot_int_status::SLOT_INT_STATUS_SPEC>;
#[doc = "Slot Interrupt Status Register"]
pub mod slot_int_status;

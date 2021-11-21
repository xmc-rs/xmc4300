#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    #[doc = "0x04 - Block Size Register"]
    pub block_size: crate::Reg<block_size::BLOCK_SIZE_SPEC>,
    #[doc = "0x06 - Block Count Register"]
    pub block_count: crate::Reg<block_count::BLOCK_COUNT_SPEC>,
    #[doc = "0x08 - Argument1 Register"]
    pub argument1: crate::Reg<argument1::ARGUMENT1_SPEC>,
    #[doc = "0x0c - Transfer Mode Register"]
    pub transfer_mode: crate::Reg<transfer_mode::TRANSFER_MODE_SPEC>,
    #[doc = "0x0e - Command Register"]
    pub command: crate::Reg<command::COMMAND_SPEC>,
    #[doc = "0x10 - Response 0 Register"]
    pub response0: crate::Reg<response0::RESPONSE0_SPEC>,
    #[doc = "0x14 - Response 2 Register"]
    pub response2: crate::Reg<response2::RESPONSE2_SPEC>,
    #[doc = "0x18 - Response 4 Register"]
    pub response4: crate::Reg<response4::RESPONSE4_SPEC>,
    #[doc = "0x1c - Response 6 Register"]
    pub response6: crate::Reg<response6::RESPONSE6_SPEC>,
    #[doc = "0x20 - Data Buffer Register"]
    pub data_buffer: crate::Reg<data_buffer::DATA_BUFFER_SPEC>,
    #[doc = "0x24 - Present State Register"]
    pub present_state: crate::Reg<present_state::PRESENT_STATE_SPEC>,
    #[doc = "0x28 - Host Control Register"]
    pub host_ctrl: crate::Reg<host_ctrl::HOST_CTRL_SPEC>,
    #[doc = "0x29 - Power Control Register"]
    pub power_ctrl: crate::Reg<power_ctrl::POWER_CTRL_SPEC>,
    #[doc = "0x2a - Block Gap Control Register"]
    pub block_gap_ctrl: crate::Reg<block_gap_ctrl::BLOCK_GAP_CTRL_SPEC>,
    #[doc = "0x2b - Wake-up Control Register"]
    pub wakeup_ctrl: crate::Reg<wakeup_ctrl::WAKEUP_CTRL_SPEC>,
    #[doc = "0x2c - Clock Control Register"]
    pub clock_ctrl: crate::Reg<clock_ctrl::CLOCK_CTRL_SPEC>,
    #[doc = "0x2e - Timeout Control Register"]
    pub timeout_ctrl: crate::Reg<timeout_ctrl::TIMEOUT_CTRL_SPEC>,
    #[doc = "0x2f - Software Reset Register"]
    pub sw_reset: crate::Reg<sw_reset::SW_RESET_SPEC>,
    #[doc = "0x30 - Normal Interrupt Status Register"]
    pub int_status_norm: crate::Reg<int_status_norm::INT_STATUS_NORM_SPEC>,
    #[doc = "0x32 - Error Interrupt Status Register"]
    pub int_status_err: crate::Reg<int_status_err::INT_STATUS_ERR_SPEC>,
    #[doc = "0x34 - Normal Interrupt Status Enable Register"]
    pub en_int_status_norm: crate::Reg<en_int_status_norm::EN_INT_STATUS_NORM_SPEC>,
    #[doc = "0x36 - Error Interrupt Status Enable Register"]
    pub en_int_status_err: crate::Reg<en_int_status_err::EN_INT_STATUS_ERR_SPEC>,
    #[doc = "0x38 - Normal Interrupt Signal Enable Register"]
    pub en_int_signal_norm: crate::Reg<en_int_signal_norm::EN_INT_SIGNAL_NORM_SPEC>,
    #[doc = "0x3a - Error Interrupt Signal Enable Register"]
    pub en_int_signal_err: crate::Reg<en_int_signal_err::EN_INT_SIGNAL_ERR_SPEC>,
    #[doc = "0x3c - Auto CMD Error Status Register"]
    pub acmd_err_status: crate::Reg<acmd_err_status::ACMD_ERR_STATUS_SPEC>,
    _reserved25: [u8; 0x02],
    #[doc = "0x40 - Capabilities Register"]
    pub capabilities: crate::Reg<capabilities::CAPABILITIES_SPEC>,
    #[doc = "0x44 - Capabilities Register High"]
    pub capabilities_hi: crate::Reg<capabilities_hi::CAPABILITIES_HI_SPEC>,
    #[doc = "0x48 - Maximum Current Capabilities Register"]
    pub max_current_cap: crate::Reg<max_current_cap::MAX_CURRENT_CAP_SPEC>,
    _reserved28: [u8; 0x04],
    #[doc = "0x50 - Force Event Register for Auto CMD Error Status"]
    pub force_event_acmd_err_status: crate::Reg<force_event_acmd_err_status::FORCE_EVENT_ACMD_ERR_STATUS_SPEC>,
    #[doc = "0x52 - Force Event Register for Error Interrupt Status"]
    pub force_event_err_status: crate::Reg<force_event_err_status::FORCE_EVENT_ERR_STATUS_SPEC>,
    _reserved30: [u8; 0x20],
    #[doc = "0x74 - Debug Selection Register"]
    pub debug_sel: crate::Reg<debug_sel::DEBUG_SEL_SPEC>,
    _reserved31: [u8; 0x84],
    #[doc = "0xfc - Slot Interrupt Status Register"]
    pub slot_int_status: crate::Reg<slot_int_status::SLOT_INT_STATUS_SPEC>,
}
#[doc = "BLOCK_SIZE register accessor: an alias for `Reg<BLOCK_SIZE_SPEC>`"]
pub type BLOCK_SIZE = crate::Reg<block_size::BLOCK_SIZE_SPEC>;
#[doc = "Block Size Register"]
pub mod block_size;
#[doc = "BLOCK_COUNT register accessor: an alias for `Reg<BLOCK_COUNT_SPEC>`"]
pub type BLOCK_COUNT = crate::Reg<block_count::BLOCK_COUNT_SPEC>;
#[doc = "Block Count Register"]
pub mod block_count;
#[doc = "ARGUMENT1 register accessor: an alias for `Reg<ARGUMENT1_SPEC>`"]
pub type ARGUMENT1 = crate::Reg<argument1::ARGUMENT1_SPEC>;
#[doc = "Argument1 Register"]
pub mod argument1;
#[doc = "TRANSFER_MODE register accessor: an alias for `Reg<TRANSFER_MODE_SPEC>`"]
pub type TRANSFER_MODE = crate::Reg<transfer_mode::TRANSFER_MODE_SPEC>;
#[doc = "Transfer Mode Register"]
pub mod transfer_mode;
#[doc = "COMMAND register accessor: an alias for `Reg<COMMAND_SPEC>`"]
pub type COMMAND = crate::Reg<command::COMMAND_SPEC>;
#[doc = "Command Register"]
pub mod command;
#[doc = "RESPONSE0 register accessor: an alias for `Reg<RESPONSE0_SPEC>`"]
pub type RESPONSE0 = crate::Reg<response0::RESPONSE0_SPEC>;
#[doc = "Response 0 Register"]
pub mod response0;
#[doc = "RESPONSE2 register accessor: an alias for `Reg<RESPONSE2_SPEC>`"]
pub type RESPONSE2 = crate::Reg<response2::RESPONSE2_SPEC>;
#[doc = "Response 2 Register"]
pub mod response2;
#[doc = "RESPONSE4 register accessor: an alias for `Reg<RESPONSE4_SPEC>`"]
pub type RESPONSE4 = crate::Reg<response4::RESPONSE4_SPEC>;
#[doc = "Response 4 Register"]
pub mod response4;
#[doc = "RESPONSE6 register accessor: an alias for `Reg<RESPONSE6_SPEC>`"]
pub type RESPONSE6 = crate::Reg<response6::RESPONSE6_SPEC>;
#[doc = "Response 6 Register"]
pub mod response6;
#[doc = "DATA_BUFFER register accessor: an alias for `Reg<DATA_BUFFER_SPEC>`"]
pub type DATA_BUFFER = crate::Reg<data_buffer::DATA_BUFFER_SPEC>;
#[doc = "Data Buffer Register"]
pub mod data_buffer;
#[doc = "PRESENT_STATE register accessor: an alias for `Reg<PRESENT_STATE_SPEC>`"]
pub type PRESENT_STATE = crate::Reg<present_state::PRESENT_STATE_SPEC>;
#[doc = "Present State Register"]
pub mod present_state;
#[doc = "HOST_CTRL register accessor: an alias for `Reg<HOST_CTRL_SPEC>`"]
pub type HOST_CTRL = crate::Reg<host_ctrl::HOST_CTRL_SPEC>;
#[doc = "Host Control Register"]
pub mod host_ctrl;
#[doc = "POWER_CTRL register accessor: an alias for `Reg<POWER_CTRL_SPEC>`"]
pub type POWER_CTRL = crate::Reg<power_ctrl::POWER_CTRL_SPEC>;
#[doc = "Power Control Register"]
pub mod power_ctrl;
#[doc = "BLOCK_GAP_CTRL register accessor: an alias for `Reg<BLOCK_GAP_CTRL_SPEC>`"]
pub type BLOCK_GAP_CTRL = crate::Reg<block_gap_ctrl::BLOCK_GAP_CTRL_SPEC>;
#[doc = "Block Gap Control Register"]
pub mod block_gap_ctrl;
#[doc = "WAKEUP_CTRL register accessor: an alias for `Reg<WAKEUP_CTRL_SPEC>`"]
pub type WAKEUP_CTRL = crate::Reg<wakeup_ctrl::WAKEUP_CTRL_SPEC>;
#[doc = "Wake-up Control Register"]
pub mod wakeup_ctrl;
#[doc = "CLOCK_CTRL register accessor: an alias for `Reg<CLOCK_CTRL_SPEC>`"]
pub type CLOCK_CTRL = crate::Reg<clock_ctrl::CLOCK_CTRL_SPEC>;
#[doc = "Clock Control Register"]
pub mod clock_ctrl;
#[doc = "TIMEOUT_CTRL register accessor: an alias for `Reg<TIMEOUT_CTRL_SPEC>`"]
pub type TIMEOUT_CTRL = crate::Reg<timeout_ctrl::TIMEOUT_CTRL_SPEC>;
#[doc = "Timeout Control Register"]
pub mod timeout_ctrl;
#[doc = "SW_RESET register accessor: an alias for `Reg<SW_RESET_SPEC>`"]
pub type SW_RESET = crate::Reg<sw_reset::SW_RESET_SPEC>;
#[doc = "Software Reset Register"]
pub mod sw_reset;
#[doc = "INT_STATUS_NORM register accessor: an alias for `Reg<INT_STATUS_NORM_SPEC>`"]
pub type INT_STATUS_NORM = crate::Reg<int_status_norm::INT_STATUS_NORM_SPEC>;
#[doc = "Normal Interrupt Status Register"]
pub mod int_status_norm;
#[doc = "INT_STATUS_ERR register accessor: an alias for `Reg<INT_STATUS_ERR_SPEC>`"]
pub type INT_STATUS_ERR = crate::Reg<int_status_err::INT_STATUS_ERR_SPEC>;
#[doc = "Error Interrupt Status Register"]
pub mod int_status_err;
#[doc = "EN_INT_STATUS_NORM register accessor: an alias for `Reg<EN_INT_STATUS_NORM_SPEC>`"]
pub type EN_INT_STATUS_NORM = crate::Reg<en_int_status_norm::EN_INT_STATUS_NORM_SPEC>;
#[doc = "Normal Interrupt Status Enable Register"]
pub mod en_int_status_norm;
#[doc = "EN_INT_STATUS_ERR register accessor: an alias for `Reg<EN_INT_STATUS_ERR_SPEC>`"]
pub type EN_INT_STATUS_ERR = crate::Reg<en_int_status_err::EN_INT_STATUS_ERR_SPEC>;
#[doc = "Error Interrupt Status Enable Register"]
pub mod en_int_status_err;
#[doc = "EN_INT_SIGNAL_NORM register accessor: an alias for `Reg<EN_INT_SIGNAL_NORM_SPEC>`"]
pub type EN_INT_SIGNAL_NORM = crate::Reg<en_int_signal_norm::EN_INT_SIGNAL_NORM_SPEC>;
#[doc = "Normal Interrupt Signal Enable Register"]
pub mod en_int_signal_norm;
#[doc = "EN_INT_SIGNAL_ERR register accessor: an alias for `Reg<EN_INT_SIGNAL_ERR_SPEC>`"]
pub type EN_INT_SIGNAL_ERR = crate::Reg<en_int_signal_err::EN_INT_SIGNAL_ERR_SPEC>;
#[doc = "Error Interrupt Signal Enable Register"]
pub mod en_int_signal_err;
#[doc = "ACMD_ERR_STATUS register accessor: an alias for `Reg<ACMD_ERR_STATUS_SPEC>`"]
pub type ACMD_ERR_STATUS = crate::Reg<acmd_err_status::ACMD_ERR_STATUS_SPEC>;
#[doc = "Auto CMD Error Status Register"]
pub mod acmd_err_status;
#[doc = "CAPABILITIES register accessor: an alias for `Reg<CAPABILITIES_SPEC>`"]
pub type CAPABILITIES = crate::Reg<capabilities::CAPABILITIES_SPEC>;
#[doc = "Capabilities Register"]
pub mod capabilities;
#[doc = "CAPABILITIES_HI register accessor: an alias for `Reg<CAPABILITIES_HI_SPEC>`"]
pub type CAPABILITIES_HI = crate::Reg<capabilities_hi::CAPABILITIES_HI_SPEC>;
#[doc = "Capabilities Register High"]
pub mod capabilities_hi;
#[doc = "MAX_CURRENT_CAP register accessor: an alias for `Reg<MAX_CURRENT_CAP_SPEC>`"]
pub type MAX_CURRENT_CAP = crate::Reg<max_current_cap::MAX_CURRENT_CAP_SPEC>;
#[doc = "Maximum Current Capabilities Register"]
pub mod max_current_cap;
#[doc = "FORCE_EVENT_ACMD_ERR_STATUS register accessor: an alias for `Reg<FORCE_EVENT_ACMD_ERR_STATUS_SPEC>`"]
pub type FORCE_EVENT_ACMD_ERR_STATUS = crate::Reg<force_event_acmd_err_status::FORCE_EVENT_ACMD_ERR_STATUS_SPEC>;
#[doc = "Force Event Register for Auto CMD Error Status"]
pub mod force_event_acmd_err_status;
#[doc = "FORCE_EVENT_ERR_STATUS register accessor: an alias for `Reg<FORCE_EVENT_ERR_STATUS_SPEC>`"]
pub type FORCE_EVENT_ERR_STATUS = crate::Reg<force_event_err_status::FORCE_EVENT_ERR_STATUS_SPEC>;
#[doc = "Force Event Register for Error Interrupt Status"]
pub mod force_event_err_status;
#[doc = "DEBUG_SEL register accessor: an alias for `Reg<DEBUG_SEL_SPEC>`"]
pub type DEBUG_SEL = crate::Reg<debug_sel::DEBUG_SEL_SPEC>;
#[doc = "Debug Selection Register"]
pub mod debug_sel;
#[doc = "SLOT_INT_STATUS register accessor: an alias for `Reg<SLOT_INT_STATUS_SPEC>`"]
pub type SLOT_INT_STATUS = crate::Reg<slot_int_status::SLOT_INT_STATUS_SPEC>;
#[doc = "Slot Interrupt Status Register"]
pub mod slot_int_status;

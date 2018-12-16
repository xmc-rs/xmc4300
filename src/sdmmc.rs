#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 4usize],
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
    _reserved1: [u8; 2usize],
    #[doc = "0x40 - Capabilities Register"]
    pub capabilities: CAPABILITIES,
    #[doc = "0x44 - Capabilities Register High"]
    pub capabilities_hi: CAPABILITIES_HI,
    #[doc = "0x48 - Maximum Current Capabilities Register"]
    pub max_current_cap: MAX_CURRENT_CAP,
    _reserved2: [u8; 4usize],
    #[doc = "0x50 - Force Event Register for Auto CMD Error Status"]
    pub force_event_acmd_err_status: FORCE_EVENT_ACMD_ERR_STATUS,
    #[doc = "0x52 - Force Event Register for Error Interrupt Status"]
    pub force_event_err_status: FORCE_EVENT_ERR_STATUS,
    _reserved3: [u8; 32usize],
    #[doc = "0x74 - Debug Selection Register"]
    pub debug_sel: DEBUG_SEL,
    _reserved4: [u8; 132usize],
    #[doc = "0xfc - Slot Interrupt Status Register"]
    pub slot_int_status: SLOT_INT_STATUS,
}
#[doc = "Block Size Register"]
pub struct BLOCK_SIZE {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Block Size Register"]
pub mod block_size;
#[doc = "Block Count Register"]
pub struct BLOCK_COUNT {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Block Count Register"]
pub mod block_count;
#[doc = "Argument1 Register"]
pub struct ARGUMENT1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Argument1 Register"]
pub mod argument1;
#[doc = "Transfer Mode Register"]
pub struct TRANSFER_MODE {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Transfer Mode Register"]
pub mod transfer_mode;
#[doc = "Command Register"]
pub struct COMMAND {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Command Register"]
pub mod command;
#[doc = "Response 0 Register"]
pub struct RESPONSE0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Response 0 Register"]
pub mod response0;
#[doc = "Response 2 Register"]
pub struct RESPONSE2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Response 2 Register"]
pub mod response2;
#[doc = "Response 4 Register"]
pub struct RESPONSE4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Response 4 Register"]
pub mod response4;
#[doc = "Response 6 Register"]
pub struct RESPONSE6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Response 6 Register"]
pub mod response6;
#[doc = "Data Buffer Register"]
pub struct DATA_BUFFER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Buffer Register"]
pub mod data_buffer;
#[doc = "Present State Register"]
pub struct PRESENT_STATE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Present State Register"]
pub mod present_state;
#[doc = "Host Control Register"]
pub struct HOST_CTRL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Host Control Register"]
pub mod host_ctrl;
#[doc = "Power Control Register"]
pub struct POWER_CTRL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Power Control Register"]
pub mod power_ctrl;
#[doc = "Block Gap Control Register"]
pub struct BLOCK_GAP_CTRL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Block Gap Control Register"]
pub mod block_gap_ctrl;
#[doc = "Wake-up Control Register"]
pub struct WAKEUP_CTRL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Wake-up Control Register"]
pub mod wakeup_ctrl;
#[doc = "Clock Control Register"]
pub struct CLOCK_CTRL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Clock Control Register"]
pub mod clock_ctrl;
#[doc = "Timeout Control Register"]
pub struct TIMEOUT_CTRL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Timeout Control Register"]
pub mod timeout_ctrl;
#[doc = "Software Reset Register"]
pub struct SW_RESET {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Software Reset Register"]
pub mod sw_reset;
#[doc = "Normal Interrupt Status Register"]
pub struct INT_STATUS_NORM {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Normal Interrupt Status Register"]
pub mod int_status_norm;
#[doc = "Error Interrupt Status Register"]
pub struct INT_STATUS_ERR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Error Interrupt Status Register"]
pub mod int_status_err;
#[doc = "Normal Interrupt Status Enable Register"]
pub struct EN_INT_STATUS_NORM {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Normal Interrupt Status Enable Register"]
pub mod en_int_status_norm;
#[doc = "Error Interrupt Status Enable Register"]
pub struct EN_INT_STATUS_ERR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Error Interrupt Status Enable Register"]
pub mod en_int_status_err;
#[doc = "Normal Interrupt Signal Enable Register"]
pub struct EN_INT_SIGNAL_NORM {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Normal Interrupt Signal Enable Register"]
pub mod en_int_signal_norm;
#[doc = "Error Interrupt Signal Enable Register"]
pub struct EN_INT_SIGNAL_ERR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Error Interrupt Signal Enable Register"]
pub mod en_int_signal_err;
#[doc = "Auto CMD Error Status Register"]
pub struct ACMD_ERR_STATUS {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Auto CMD Error Status Register"]
pub mod acmd_err_status;
#[doc = "Capabilities Register"]
pub struct CAPABILITIES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Capabilities Register"]
pub mod capabilities;
#[doc = "Capabilities Register High"]
pub struct CAPABILITIES_HI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Capabilities Register High"]
pub mod capabilities_hi;
#[doc = "Maximum Current Capabilities Register"]
pub struct MAX_CURRENT_CAP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Maximum Current Capabilities Register"]
pub mod max_current_cap;
#[doc = "Force Event Register for Auto CMD Error Status"]
pub struct FORCE_EVENT_ACMD_ERR_STATUS {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Force Event Register for Auto CMD Error Status"]
pub mod force_event_acmd_err_status;
#[doc = "Force Event Register for Error Interrupt Status"]
pub struct FORCE_EVENT_ERR_STATUS {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Force Event Register for Error Interrupt Status"]
pub mod force_event_err_status;
#[doc = "Debug Selection Register"]
pub struct DEBUG_SEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Debug Selection Register"]
pub mod debug_sel;
#[doc = "Slot Interrupt Status Register"]
pub struct SLOT_INT_STATUS {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Slot Interrupt Status Register"]
pub mod slot_int_status;

#[doc = r"Register block"]
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
    _reserved25: [u8; 2usize],
    #[doc = "0x40 - Capabilities Register"]
    pub capabilities: CAPABILITIES,
    #[doc = "0x44 - Capabilities Register High"]
    pub capabilities_hi: CAPABILITIES_HI,
    #[doc = "0x48 - Maximum Current Capabilities Register"]
    pub max_current_cap: MAX_CURRENT_CAP,
    _reserved28: [u8; 4usize],
    #[doc = "0x50 - Force Event Register for Auto CMD Error Status"]
    pub force_event_acmd_err_status: FORCE_EVENT_ACMD_ERR_STATUS,
    #[doc = "0x52 - Force Event Register for Error Interrupt Status"]
    pub force_event_err_status: FORCE_EVENT_ERR_STATUS,
    _reserved30: [u8; 32usize],
    #[doc = "0x74 - Debug Selection Register"]
    pub debug_sel: DEBUG_SEL,
    _reserved31: [u8; 132usize],
    #[doc = "0xfc - Slot Interrupt Status Register"]
    pub slot_int_status: SLOT_INT_STATUS,
}
#[doc = "Block Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [block_size](block_size) module"]
pub type BLOCK_SIZE = crate::Reg<u16, _BLOCK_SIZE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLOCK_SIZE;
#[doc = "`read()` method returns [block_size::R](block_size::R) reader structure"]
impl crate::Readable for BLOCK_SIZE {}
#[doc = "`write(|w| ..)` method takes [block_size::W](block_size::W) writer structure"]
impl crate::Writable for BLOCK_SIZE {}
#[doc = "Block Size Register"]
pub mod block_size;
#[doc = "Block Count Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [block_count](block_count) module"]
pub type BLOCK_COUNT = crate::Reg<u16, _BLOCK_COUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLOCK_COUNT;
#[doc = "`read()` method returns [block_count::R](block_count::R) reader structure"]
impl crate::Readable for BLOCK_COUNT {}
#[doc = "`write(|w| ..)` method takes [block_count::W](block_count::W) writer structure"]
impl crate::Writable for BLOCK_COUNT {}
#[doc = "Block Count Register"]
pub mod block_count;
#[doc = "Argument1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [argument1](argument1) module"]
pub type ARGUMENT1 = crate::Reg<u32, _ARGUMENT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARGUMENT1;
#[doc = "`read()` method returns [argument1::R](argument1::R) reader structure"]
impl crate::Readable for ARGUMENT1 {}
#[doc = "`write(|w| ..)` method takes [argument1::W](argument1::W) writer structure"]
impl crate::Writable for ARGUMENT1 {}
#[doc = "Argument1 Register"]
pub mod argument1;
#[doc = "Transfer Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [transfer_mode](transfer_mode) module"]
pub type TRANSFER_MODE = crate::Reg<u16, _TRANSFER_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRANSFER_MODE;
#[doc = "`read()` method returns [transfer_mode::R](transfer_mode::R) reader structure"]
impl crate::Readable for TRANSFER_MODE {}
#[doc = "`write(|w| ..)` method takes [transfer_mode::W](transfer_mode::W) writer structure"]
impl crate::Writable for TRANSFER_MODE {}
#[doc = "Transfer Mode Register"]
pub mod transfer_mode;
#[doc = "Command Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [command](command) module"]
pub type COMMAND = crate::Reg<u16, _COMMAND>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMMAND;
#[doc = "`read()` method returns [command::R](command::R) reader structure"]
impl crate::Readable for COMMAND {}
#[doc = "`write(|w| ..)` method takes [command::W](command::W) writer structure"]
impl crate::Writable for COMMAND {}
#[doc = "Command Register"]
pub mod command;
#[doc = "Response 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [response0](response0) module"]
pub type RESPONSE0 = crate::Reg<u32, _RESPONSE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESPONSE0;
#[doc = "`read()` method returns [response0::R](response0::R) reader structure"]
impl crate::Readable for RESPONSE0 {}
#[doc = "Response 0 Register"]
pub mod response0;
#[doc = "Response 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [response2](response2) module"]
pub type RESPONSE2 = crate::Reg<u32, _RESPONSE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESPONSE2;
#[doc = "`read()` method returns [response2::R](response2::R) reader structure"]
impl crate::Readable for RESPONSE2 {}
#[doc = "Response 2 Register"]
pub mod response2;
#[doc = "Response 4 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [response4](response4) module"]
pub type RESPONSE4 = crate::Reg<u32, _RESPONSE4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESPONSE4;
#[doc = "`read()` method returns [response4::R](response4::R) reader structure"]
impl crate::Readable for RESPONSE4 {}
#[doc = "Response 4 Register"]
pub mod response4;
#[doc = "Response 6 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [response6](response6) module"]
pub type RESPONSE6 = crate::Reg<u32, _RESPONSE6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESPONSE6;
#[doc = "`read()` method returns [response6::R](response6::R) reader structure"]
impl crate::Readable for RESPONSE6 {}
#[doc = "Response 6 Register"]
pub mod response6;
#[doc = "Data Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [data_buffer](data_buffer) module"]
pub type DATA_BUFFER = crate::Reg<u32, _DATA_BUFFER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA_BUFFER;
#[doc = "`read()` method returns [data_buffer::R](data_buffer::R) reader structure"]
impl crate::Readable for DATA_BUFFER {}
#[doc = "`write(|w| ..)` method takes [data_buffer::W](data_buffer::W) writer structure"]
impl crate::Writable for DATA_BUFFER {}
#[doc = "Data Buffer Register"]
pub mod data_buffer;
#[doc = "Present State Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [present_state](present_state) module"]
pub type PRESENT_STATE = crate::Reg<u32, _PRESENT_STATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRESENT_STATE;
#[doc = "`read()` method returns [present_state::R](present_state::R) reader structure"]
impl crate::Readable for PRESENT_STATE {}
#[doc = "Present State Register"]
pub mod present_state;
#[doc = "Host Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_ctrl](host_ctrl) module"]
pub type HOST_CTRL = crate::Reg<u8, _HOST_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_CTRL;
#[doc = "`read()` method returns [host_ctrl::R](host_ctrl::R) reader structure"]
impl crate::Readable for HOST_CTRL {}
#[doc = "`write(|w| ..)` method takes [host_ctrl::W](host_ctrl::W) writer structure"]
impl crate::Writable for HOST_CTRL {}
#[doc = "Host Control Register"]
pub mod host_ctrl;
#[doc = "Power Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [power_ctrl](power_ctrl) module"]
pub type POWER_CTRL = crate::Reg<u8, _POWER_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _POWER_CTRL;
#[doc = "`read()` method returns [power_ctrl::R](power_ctrl::R) reader structure"]
impl crate::Readable for POWER_CTRL {}
#[doc = "`write(|w| ..)` method takes [power_ctrl::W](power_ctrl::W) writer structure"]
impl crate::Writable for POWER_CTRL {}
#[doc = "Power Control Register"]
pub mod power_ctrl;
#[doc = "Block Gap Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [block_gap_ctrl](block_gap_ctrl) module"]
pub type BLOCK_GAP_CTRL = crate::Reg<u8, _BLOCK_GAP_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLOCK_GAP_CTRL;
#[doc = "`read()` method returns [block_gap_ctrl::R](block_gap_ctrl::R) reader structure"]
impl crate::Readable for BLOCK_GAP_CTRL {}
#[doc = "`write(|w| ..)` method takes [block_gap_ctrl::W](block_gap_ctrl::W) writer structure"]
impl crate::Writable for BLOCK_GAP_CTRL {}
#[doc = "Block Gap Control Register"]
pub mod block_gap_ctrl;
#[doc = "Wake-up Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wakeup_ctrl](wakeup_ctrl) module"]
pub type WAKEUP_CTRL = crate::Reg<u8, _WAKEUP_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WAKEUP_CTRL;
#[doc = "`read()` method returns [wakeup_ctrl::R](wakeup_ctrl::R) reader structure"]
impl crate::Readable for WAKEUP_CTRL {}
#[doc = "`write(|w| ..)` method takes [wakeup_ctrl::W](wakeup_ctrl::W) writer structure"]
impl crate::Writable for WAKEUP_CTRL {}
#[doc = "Wake-up Control Register"]
pub mod wakeup_ctrl;
#[doc = "Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clock_ctrl](clock_ctrl) module"]
pub type CLOCK_CTRL = crate::Reg<u16, _CLOCK_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLOCK_CTRL;
#[doc = "`read()` method returns [clock_ctrl::R](clock_ctrl::R) reader structure"]
impl crate::Readable for CLOCK_CTRL {}
#[doc = "`write(|w| ..)` method takes [clock_ctrl::W](clock_ctrl::W) writer structure"]
impl crate::Writable for CLOCK_CTRL {}
#[doc = "Clock Control Register"]
pub mod clock_ctrl;
#[doc = "Timeout Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timeout_ctrl](timeout_ctrl) module"]
pub type TIMEOUT_CTRL = crate::Reg<u8, _TIMEOUT_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMEOUT_CTRL;
#[doc = "`read()` method returns [timeout_ctrl::R](timeout_ctrl::R) reader structure"]
impl crate::Readable for TIMEOUT_CTRL {}
#[doc = "`write(|w| ..)` method takes [timeout_ctrl::W](timeout_ctrl::W) writer structure"]
impl crate::Writable for TIMEOUT_CTRL {}
#[doc = "Timeout Control Register"]
pub mod timeout_ctrl;
#[doc = "Software Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_reset](sw_reset) module"]
pub type SW_RESET = crate::Reg<u8, _SW_RESET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_RESET;
#[doc = "`read()` method returns [sw_reset::R](sw_reset::R) reader structure"]
impl crate::Readable for SW_RESET {}
#[doc = "`write(|w| ..)` method takes [sw_reset::W](sw_reset::W) writer structure"]
impl crate::Writable for SW_RESET {}
#[doc = "Software Reset Register"]
pub mod sw_reset;
#[doc = "Normal Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [int_status_norm](int_status_norm) module"]
pub type INT_STATUS_NORM = crate::Reg<u16, _INT_STATUS_NORM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_STATUS_NORM;
#[doc = "`read()` method returns [int_status_norm::R](int_status_norm::R) reader structure"]
impl crate::Readable for INT_STATUS_NORM {}
#[doc = "`write(|w| ..)` method takes [int_status_norm::W](int_status_norm::W) writer structure"]
impl crate::Writable for INT_STATUS_NORM {}
#[doc = "Normal Interrupt Status Register"]
pub mod int_status_norm;
#[doc = "Error Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [int_status_err](int_status_err) module"]
pub type INT_STATUS_ERR = crate::Reg<u16, _INT_STATUS_ERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_STATUS_ERR;
#[doc = "`read()` method returns [int_status_err::R](int_status_err::R) reader structure"]
impl crate::Readable for INT_STATUS_ERR {}
#[doc = "`write(|w| ..)` method takes [int_status_err::W](int_status_err::W) writer structure"]
impl crate::Writable for INT_STATUS_ERR {}
#[doc = "Error Interrupt Status Register"]
pub mod int_status_err;
#[doc = "Normal Interrupt Status Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [en_int_status_norm](en_int_status_norm) module"]
pub type EN_INT_STATUS_NORM = crate::Reg<u16, _EN_INT_STATUS_NORM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EN_INT_STATUS_NORM;
#[doc = "`read()` method returns [en_int_status_norm::R](en_int_status_norm::R) reader structure"]
impl crate::Readable for EN_INT_STATUS_NORM {}
#[doc = "`write(|w| ..)` method takes [en_int_status_norm::W](en_int_status_norm::W) writer structure"]
impl crate::Writable for EN_INT_STATUS_NORM {}
#[doc = "Normal Interrupt Status Enable Register"]
pub mod en_int_status_norm;
#[doc = "Error Interrupt Status Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [en_int_status_err](en_int_status_err) module"]
pub type EN_INT_STATUS_ERR = crate::Reg<u16, _EN_INT_STATUS_ERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EN_INT_STATUS_ERR;
#[doc = "`read()` method returns [en_int_status_err::R](en_int_status_err::R) reader structure"]
impl crate::Readable for EN_INT_STATUS_ERR {}
#[doc = "`write(|w| ..)` method takes [en_int_status_err::W](en_int_status_err::W) writer structure"]
impl crate::Writable for EN_INT_STATUS_ERR {}
#[doc = "Error Interrupt Status Enable Register"]
pub mod en_int_status_err;
#[doc = "Normal Interrupt Signal Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [en_int_signal_norm](en_int_signal_norm) module"]
pub type EN_INT_SIGNAL_NORM = crate::Reg<u16, _EN_INT_SIGNAL_NORM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EN_INT_SIGNAL_NORM;
#[doc = "`read()` method returns [en_int_signal_norm::R](en_int_signal_norm::R) reader structure"]
impl crate::Readable for EN_INT_SIGNAL_NORM {}
#[doc = "`write(|w| ..)` method takes [en_int_signal_norm::W](en_int_signal_norm::W) writer structure"]
impl crate::Writable for EN_INT_SIGNAL_NORM {}
#[doc = "Normal Interrupt Signal Enable Register"]
pub mod en_int_signal_norm;
#[doc = "Error Interrupt Signal Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [en_int_signal_err](en_int_signal_err) module"]
pub type EN_INT_SIGNAL_ERR = crate::Reg<u16, _EN_INT_SIGNAL_ERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EN_INT_SIGNAL_ERR;
#[doc = "`read()` method returns [en_int_signal_err::R](en_int_signal_err::R) reader structure"]
impl crate::Readable for EN_INT_SIGNAL_ERR {}
#[doc = "`write(|w| ..)` method takes [en_int_signal_err::W](en_int_signal_err::W) writer structure"]
impl crate::Writable for EN_INT_SIGNAL_ERR {}
#[doc = "Error Interrupt Signal Enable Register"]
pub mod en_int_signal_err;
#[doc = "Auto CMD Error Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [acmd_err_status](acmd_err_status) module"]
pub type ACMD_ERR_STATUS = crate::Reg<u16, _ACMD_ERR_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACMD_ERR_STATUS;
#[doc = "`read()` method returns [acmd_err_status::R](acmd_err_status::R) reader structure"]
impl crate::Readable for ACMD_ERR_STATUS {}
#[doc = "Auto CMD Error Status Register"]
pub mod acmd_err_status;
#[doc = "Capabilities Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [capabilities](capabilities) module"]
pub type CAPABILITIES = crate::Reg<u32, _CAPABILITIES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAPABILITIES;
#[doc = "`read()` method returns [capabilities::R](capabilities::R) reader structure"]
impl crate::Readable for CAPABILITIES {}
#[doc = "Capabilities Register"]
pub mod capabilities;
#[doc = "Capabilities Register High\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [capabilities_hi](capabilities_hi) module"]
pub type CAPABILITIES_HI = crate::Reg<u32, _CAPABILITIES_HI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAPABILITIES_HI;
#[doc = "`read()` method returns [capabilities_hi::R](capabilities_hi::R) reader structure"]
impl crate::Readable for CAPABILITIES_HI {}
#[doc = "Capabilities Register High"]
pub mod capabilities_hi;
#[doc = "Maximum Current Capabilities Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [max_current_cap](max_current_cap) module"]
pub type MAX_CURRENT_CAP = crate::Reg<u32, _MAX_CURRENT_CAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAX_CURRENT_CAP;
#[doc = "`read()` method returns [max_current_cap::R](max_current_cap::R) reader structure"]
impl crate::Readable for MAX_CURRENT_CAP {}
#[doc = "Maximum Current Capabilities Register"]
pub mod max_current_cap;
#[doc = "Force Event Register for Auto CMD Error Status\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [force_event_acmd_err_status](force_event_acmd_err_status) module"]
pub type FORCE_EVENT_ACMD_ERR_STATUS = crate::Reg<u16, _FORCE_EVENT_ACMD_ERR_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FORCE_EVENT_ACMD_ERR_STATUS;
#[doc = "`write(|w| ..)` method takes [force_event_acmd_err_status::W](force_event_acmd_err_status::W) writer structure"]
impl crate::Writable for FORCE_EVENT_ACMD_ERR_STATUS {}
#[doc = "Force Event Register for Auto CMD Error Status"]
pub mod force_event_acmd_err_status;
#[doc = "Force Event Register for Error Interrupt Status\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [force_event_err_status](force_event_err_status) module"]
pub type FORCE_EVENT_ERR_STATUS = crate::Reg<u16, _FORCE_EVENT_ERR_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FORCE_EVENT_ERR_STATUS;
#[doc = "`write(|w| ..)` method takes [force_event_err_status::W](force_event_err_status::W) writer structure"]
impl crate::Writable for FORCE_EVENT_ERR_STATUS {}
#[doc = "Force Event Register for Error Interrupt Status"]
pub mod force_event_err_status;
#[doc = "Debug Selection Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [debug_sel](debug_sel) module"]
pub type DEBUG_SEL = crate::Reg<u32, _DEBUG_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEBUG_SEL;
#[doc = "`write(|w| ..)` method takes [debug_sel::W](debug_sel::W) writer structure"]
impl crate::Writable for DEBUG_SEL {}
#[doc = "Debug Selection Register"]
pub mod debug_sel;
#[doc = "Slot Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slot_int_status](slot_int_status) module"]
pub type SLOT_INT_STATUS = crate::Reg<u16, _SLOT_INT_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLOT_INT_STATUS;
#[doc = "`read()` method returns [slot_int_status::R](slot_int_status::R) reader structure"]
impl crate::Readable for SLOT_INT_STATUS {}
#[doc = "Slot Interrupt Status Register"]
pub mod slot_int_status;

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MAC Configuration Register"]
    pub mac_configuration: MAC_CONFIGURATION,
    #[doc = "0x04 - MAC Frame Filter"]
    pub mac_frame_filter: MAC_FRAME_FILTER,
    #[doc = "0x08 - Hash Table High Register"]
    pub hash_table_high: HASH_TABLE_HIGH,
    #[doc = "0x0c - Hash Table Low Register"]
    pub hash_table_low: HASH_TABLE_LOW,
    #[doc = "0x10 - MII Address Register"]
    pub gmii_address: GMII_ADDRESS,
    #[doc = "0x14 - MII Data Register"]
    pub gmii_data: GMII_DATA,
    #[doc = "0x18 - Flow Control Register"]
    pub flow_control: FLOW_CONTROL,
    #[doc = "0x1c - VLAN Tag Register"]
    pub vlan_tag: VLAN_TAG,
    #[doc = "0x20 - Version Register"]
    pub version: VERSION,
    #[doc = "0x24 - Debug Register"]
    pub debug: DEBUG,
    #[doc = "0x28 - Remote Wake Up Frame Filter Register"]
    pub remote_wake_up_frame_filter: REMOTE_WAKE_UP_FRAME_FILTER,
    #[doc = "0x2c - PMT Control and Status Register"]
    pub pmt_control_status: PMT_CONTROL_STATUS,
    _reserved12: [u8; 0x08],
    #[doc = "0x38 - Interrupt Register"]
    pub interrupt_status: INTERRUPT_STATUS,
    #[doc = "0x3c - Interrupt Mask Register"]
    pub interrupt_mask: INTERRUPT_MASK,
    #[doc = "0x40 - MAC Address0 High Register"]
    pub mac_address0_high: MAC_ADDRESS0_HIGH,
    #[doc = "0x44 - MAC Address0 Low Register"]
    pub mac_address0_low: MAC_ADDRESS0_LOW,
    #[doc = "0x48 - MAC Address1 High Register"]
    pub mac_address1_high: MAC_ADDRESS1_HIGH,
    #[doc = "0x4c - MAC Address1 Low Register"]
    pub mac_address1_low: MAC_ADDRESS1_LOW,
    #[doc = "0x50 - MAC Address2 High Register"]
    pub mac_address2_high: MAC_ADDRESS2_HIGH,
    #[doc = "0x54 - MAC Address2 Low Register"]
    pub mac_address2_low: MAC_ADDRESS2_LOW,
    #[doc = "0x58 - MAC Address3 High Register"]
    pub mac_address3_high: MAC_ADDRESS3_HIGH,
    #[doc = "0x5c - MAC Address3 Low Register"]
    pub mac_address3_low: MAC_ADDRESS3_LOW,
    _reserved22: [u8; 0xa0],
    #[doc = "0x100 - MMC Control Register"]
    pub mmc_control: MMC_CONTROL,
    #[doc = "0x104 - MMC Receive Interrupt Register"]
    pub mmc_receive_interrupt: MMC_RECEIVE_INTERRUPT,
    #[doc = "0x108 - MMC Transmit Interrupt Register"]
    pub mmc_transmit_interrupt: MMC_TRANSMIT_INTERRUPT,
    #[doc = "0x10c - MMC Reveive Interrupt Mask Register"]
    pub mmc_receive_interrupt_mask: MMC_RECEIVE_INTERRUPT_MASK,
    #[doc = "0x110 - MMC Transmit Interrupt Mask Register"]
    pub mmc_transmit_interrupt_mask: MMC_TRANSMIT_INTERRUPT_MASK,
    #[doc = "0x114 - Transmit Octet Count for Good and Bad Frames Register"]
    pub tx_octet_count_good_bad: TX_OCTET_COUNT_GOOD_BAD,
    #[doc = "0x118 - Transmit Frame Count for Goodand Bad Frames Register"]
    pub tx_frame_count_good_bad: TX_FRAME_COUNT_GOOD_BAD,
    #[doc = "0x11c - Transmit Frame Count for Good Broadcast Frames"]
    pub tx_broadcast_frames_good: TX_BROADCAST_FRAMES_GOOD,
    #[doc = "0x120 - Transmit Frame Count for Good Multicast Frames"]
    pub tx_multicast_frames_good: TX_MULTICAST_FRAMES_GOOD,
    #[doc = "0x124 - Transmit Octet Count for Good and Bad 64 Byte Frames"]
    pub tx_64octets_frames_good_bad: TX_64OCTETS_FRAMES_GOOD_BAD,
    #[doc = "0x128 - Transmit Octet Count for Good and Bad 65 to 127 Bytes Frames"]
    pub tx_65to127octets_frames_good_bad: TX_65TO127OCTETS_FRAMES_GOOD_BAD,
    #[doc = "0x12c - Transmit Octet Count for Good and Bad 128 to 255 Bytes Frames"]
    pub tx_128to255octets_frames_good_bad: TX_128TO255OCTETS_FRAMES_GOOD_BAD,
    #[doc = "0x130 - Transmit Octet Count for Good and Bad 256 to 511 Bytes Frames"]
    pub tx_256to511octets_frames_good_bad: TX_256TO511OCTETS_FRAMES_GOOD_BAD,
    #[doc = "0x134 - Transmit Octet Count for Good and Bad 512 to 1023 Bytes Frames"]
    pub tx_512to1023octets_frames_good_bad: TX_512TO1023OCTETS_FRAMES_GOOD_BAD,
    #[doc = "0x138 - Transmit Octet Count for Good and Bad 1024 to Maxsize Bytes Frames"]
    pub tx_1024tomaxoctets_frames_good_bad: TX_1024TOMAXOCTETS_FRAMES_GOOD_BAD,
    #[doc = "0x13c - Transmit Frame Count for Good and Bad Unicast Frames"]
    pub tx_unicast_frames_good_bad: TX_UNICAST_FRAMES_GOOD_BAD,
    #[doc = "0x140 - Transmit Frame Count for Good and Bad Multicast Frames"]
    pub tx_multicast_frames_good_bad: TX_MULTICAST_FRAMES_GOOD_BAD,
    #[doc = "0x144 - Transmit Frame Count for Good and Bad Broadcast Frames"]
    pub tx_broadcast_frames_good_bad: TX_BROADCAST_FRAMES_GOOD_BAD,
    #[doc = "0x148 - Transmit Frame Count for Underflow Error Frames"]
    pub tx_underflow_error_frames: TX_UNDERFLOW_ERROR_FRAMES,
    #[doc = "0x14c - Transmit Frame Count for Frames Transmitted after Single Collision"]
    pub tx_single_collision_good_frames: TX_SINGLE_COLLISION_GOOD_FRAMES,
    #[doc = "0x150 - Transmit Frame Count for Frames Transmitted after Multiple Collision"]
    pub tx_multiple_collision_good_frames: TX_MULTIPLE_COLLISION_GOOD_FRAMES,
    #[doc = "0x154 - Tx Deferred Frames Register"]
    pub tx_deferred_frames: TX_DEFERRED_FRAMES,
    #[doc = "0x158 - Transmit Frame Count for Late Collision Error Frames"]
    pub tx_late_collision_frames: TX_LATE_COLLISION_FRAMES,
    #[doc = "0x15c - Transmit Frame Count for Excessive Collision Error Frames"]
    pub tx_excessive_collision_frames: TX_EXCESSIVE_COLLISION_FRAMES,
    #[doc = "0x160 - Transmit Frame Count for Carrier Sense Error Frames"]
    pub tx_carrier_error_frames: TX_CARRIER_ERROR_FRAMES,
    #[doc = "0x164 - Tx Octet Count Good Register"]
    pub tx_octet_count_good: TX_OCTET_COUNT_GOOD,
    #[doc = "0x168 - Tx Frame Count Good Register"]
    pub tx_frame_count_good: TX_FRAME_COUNT_GOOD,
    #[doc = "0x16c - Transmit Frame Count for Excessive Deferral Error Frames"]
    pub tx_excessive_deferral_error: TX_EXCESSIVE_DEFERRAL_ERROR,
    #[doc = "0x170 - Transmit Frame Count for Good PAUSE Frames"]
    pub tx_pause_frames: TX_PAUSE_FRAMES,
    #[doc = "0x174 - Transmit Frame Count for Good VLAN Frames"]
    pub tx_vlan_frames_good: TX_VLAN_FRAMES_GOOD,
    #[doc = "0x178 - Transmit Frame Count for Good Oversize Frames"]
    pub tx_osize_frames_good: TX_OSIZE_FRAMES_GOOD,
    _reserved53: [u8; 0x04],
    #[doc = "0x180 - Receive Frame Count for Good and Bad Frames"]
    pub rx_frames_count_good_bad: RX_FRAMES_COUNT_GOOD_BAD,
    #[doc = "0x184 - Receive Octet Count for Good and Bad Frames"]
    pub rx_octet_count_good_bad: RX_OCTET_COUNT_GOOD_BAD,
    #[doc = "0x188 - Rx Octet Count Good Register"]
    pub rx_octet_count_good: RX_OCTET_COUNT_GOOD,
    #[doc = "0x18c - Receive Frame Count for Good Broadcast Frames"]
    pub rx_broadcast_frames_good: RX_BROADCAST_FRAMES_GOOD,
    #[doc = "0x190 - Receive Frame Count for Good Multicast Frames"]
    pub rx_multicast_frames_good: RX_MULTICAST_FRAMES_GOOD,
    #[doc = "0x194 - Receive Frame Count for CRC Error Frames"]
    pub rx_crc_error_frames: RX_CRC_ERROR_FRAMES,
    #[doc = "0x198 - Receive Frame Count for Alignment Error Frames"]
    pub rx_alignment_error_frames: RX_ALIGNMENT_ERROR_FRAMES,
    #[doc = "0x19c - Receive Frame Count for Runt Error Frames"]
    pub rx_runt_error_frames: RX_RUNT_ERROR_FRAMES,
    #[doc = "0x1a0 - Receive Frame Count for Jabber Error Frames"]
    pub rx_jabber_error_frames: RX_JABBER_ERROR_FRAMES,
    #[doc = "0x1a4 - Receive Frame Count for Undersize Frames"]
    pub rx_undersize_frames_good: RX_UNDERSIZE_FRAMES_GOOD,
    #[doc = "0x1a8 - Rx Oversize Frames Good Register"]
    pub rx_oversize_frames_good: RX_OVERSIZE_FRAMES_GOOD,
    #[doc = "0x1ac - Receive Frame Count for Good and Bad 64 Byte Frames"]
    pub rx_64octets_frames_good_bad: RX_64OCTETS_FRAMES_GOOD_BAD,
    #[doc = "0x1b0 - Receive Frame Count for Good and Bad 65 to 127 Bytes Frames"]
    pub rx_65to127octets_frames_good_bad: RX_65TO127OCTETS_FRAMES_GOOD_BAD,
    #[doc = "0x1b4 - Receive Frame Count for Good and Bad 128 to 255 Bytes Frames"]
    pub rx_128to255octets_frames_good_bad: RX_128TO255OCTETS_FRAMES_GOOD_BAD,
    #[doc = "0x1b8 - Receive Frame Count for Good and Bad 256 to 511 Bytes Frames"]
    pub rx_256to511octets_frames_good_bad: RX_256TO511OCTETS_FRAMES_GOOD_BAD,
    #[doc = "0x1bc - Receive Frame Count for Good and Bad 512 to 1,023 Bytes Frames"]
    pub rx_512to1023octets_frames_good_bad: RX_512TO1023OCTETS_FRAMES_GOOD_BAD,
    #[doc = "0x1c0 - Receive Frame Count for Good and Bad 1,024 to Maxsize Bytes Frames"]
    pub rx_1024tomaxoctets_frames_good_bad: RX_1024TOMAXOCTETS_FRAMES_GOOD_BAD,
    #[doc = "0x1c4 - Receive Frame Count for Good Unicast Frames"]
    pub rx_unicast_frames_good: RX_UNICAST_FRAMES_GOOD,
    #[doc = "0x1c8 - Receive Frame Count for Length Error Frames"]
    pub rx_length_error_frames: RX_LENGTH_ERROR_FRAMES,
    #[doc = "0x1cc - Receive Frame Count for Out of Range Frames"]
    pub rx_out_of_range_type_frames: RX_OUT_OF_RANGE_TYPE_FRAMES,
    #[doc = "0x1d0 - Receive Frame Count for PAUSE Frames"]
    pub rx_pause_frames: RX_PAUSE_FRAMES,
    #[doc = "0x1d4 - Receive Frame Count for FIFO Overflow Frames"]
    pub rx_fifo_overflow_frames: RX_FIFO_OVERFLOW_FRAMES,
    #[doc = "0x1d8 - Receive Frame Count for Good and Bad VLAN Frames"]
    pub rx_vlan_frames_good_bad: RX_VLAN_FRAMES_GOOD_BAD,
    #[doc = "0x1dc - Receive Frame Count for Watchdog Error Frames"]
    pub rx_watchdog_error_frames: RX_WATCHDOG_ERROR_FRAMES,
    #[doc = "0x1e0 - Receive Frame Count for Receive Error Frames"]
    pub rx_receive_error_frames: RX_RECEIVE_ERROR_FRAMES,
    #[doc = "0x1e4 - Receive Frame Count for Good Control Frames Frames"]
    pub rx_control_frames_good: RX_CONTROL_FRAMES_GOOD,
    _reserved79: [u8; 0x18],
    #[doc = "0x200 - MMC Receive Checksum Offload Interrupt Mask Register"]
    pub mmc_ipc_receive_interrupt_mask: MMC_IPC_RECEIVE_INTERRUPT_MASK,
    _reserved80: [u8; 0x04],
    #[doc = "0x208 - MMC Receive Checksum Offload Interrupt Register"]
    pub mmc_ipc_receive_interrupt: MMC_IPC_RECEIVE_INTERRUPT,
    _reserved81: [u8; 0x04],
    #[doc = "0x210 - RxIPv4 Good Frames Register"]
    pub rxipv4_good_frames: RXIPV4_GOOD_FRAMES,
    #[doc = "0x214 - Receive IPV4 Header Error Frame Counter Register"]
    pub rxipv4_header_error_frames: RXIPV4_HEADER_ERROR_FRAMES,
    #[doc = "0x218 - Receive IPV4 No Payload Frame Counter Register"]
    pub rxipv4_no_payload_frames: RXIPV4_NO_PAYLOAD_FRAMES,
    #[doc = "0x21c - Receive IPV4 Fragmented Frame Counter Register"]
    pub rxipv4_fragmented_frames: RXIPV4_FRAGMENTED_FRAMES,
    #[doc = "0x220 - Receive IPV4 UDP Checksum Disabled Frame Counter Register"]
    pub rxipv4_udp_checksum_disabled_frames: RXIPV4_UDP_CHECKSUM_DISABLED_FRAMES,
    #[doc = "0x224 - RxIPv6 Good Frames Register"]
    pub rxipv6_good_frames: RXIPV6_GOOD_FRAMES,
    #[doc = "0x228 - Receive IPV6 Header Error Frame Counter Register"]
    pub rxipv6_header_error_frames: RXIPV6_HEADER_ERROR_FRAMES,
    #[doc = "0x22c - Receive IPV6 No Payload Frame Counter Register"]
    pub rxipv6_no_payload_frames: RXIPV6_NO_PAYLOAD_FRAMES,
    #[doc = "0x230 - RxUDP Good Frames Register"]
    pub rxudp_good_frames: RXUDP_GOOD_FRAMES,
    #[doc = "0x234 - RxUDP Error Frames Register"]
    pub rxudp_error_frames: RXUDP_ERROR_FRAMES,
    #[doc = "0x238 - RxTCP Good Frames Register"]
    pub rxtcp_good_frames: RXTCP_GOOD_FRAMES,
    #[doc = "0x23c - RxTCP Error Frames Register"]
    pub rxtcp_error_frames: RXTCP_ERROR_FRAMES,
    #[doc = "0x240 - RxICMP Good Frames Register"]
    pub rxicmp_good_frames: RXICMP_GOOD_FRAMES,
    #[doc = "0x244 - RxICMP Error Frames Register"]
    pub rxicmp_error_frames: RXICMP_ERROR_FRAMES,
    _reserved95: [u8; 0x08],
    #[doc = "0x250 - RxIPv4 Good Octets Register"]
    pub rxipv4_good_octets: RXIPV4_GOOD_OCTETS,
    #[doc = "0x254 - Receive IPV4 Header Error Octet Counter Register"]
    pub rxipv4_header_error_octets: RXIPV4_HEADER_ERROR_OCTETS,
    #[doc = "0x258 - Receive IPV4 No Payload Octet Counter Register"]
    pub rxipv4_no_payload_octets: RXIPV4_NO_PAYLOAD_OCTETS,
    #[doc = "0x25c - Receive IPV4 Fragmented Octet Counter Register"]
    pub rxipv4_fragmented_octets: RXIPV4_FRAGMENTED_OCTETS,
    #[doc = "0x260 - Receive IPV4 Fragmented Octet Counter Register"]
    pub rxipv4_udp_checksum_disable_octets: RXIPV4_UDP_CHECKSUM_DISABLE_OCTETS,
    #[doc = "0x264 - RxIPv6 Good Octets Register"]
    pub rxipv6_good_octets: RXIPV6_GOOD_OCTETS,
    #[doc = "0x268 - Receive IPV6 Header Error Octet Counter Register"]
    pub rxipv6_header_error_octets: RXIPV6_HEADER_ERROR_OCTETS,
    #[doc = "0x26c - Receive IPV6 No Payload Octet Counter Register"]
    pub rxipv6_no_payload_octets: RXIPV6_NO_PAYLOAD_OCTETS,
    #[doc = "0x270 - Receive UDP Good Octets Register"]
    pub rxudp_good_octets: RXUDP_GOOD_OCTETS,
    #[doc = "0x274 - Receive UDP Error Octets Register"]
    pub rxudp_error_octets: RXUDP_ERROR_OCTETS,
    #[doc = "0x278 - Receive TCP Good Octets Register"]
    pub rxtcp_good_octets: RXTCP_GOOD_OCTETS,
    #[doc = "0x27c - Receive TCP Error Octets Register"]
    pub rxtcp_error_octets: RXTCP_ERROR_OCTETS,
    #[doc = "0x280 - Receive ICMP Good Octets Register"]
    pub rxicmp_good_octets: RXICMP_GOOD_OCTETS,
    #[doc = "0x284 - Receive ICMP Error Octets Register"]
    pub rxicmp_error_octets: RXICMP_ERROR_OCTETS,
    _reserved109: [u8; 0x0478],
    #[doc = "0x700 - Timestamp Control Register"]
    pub timestamp_control: TIMESTAMP_CONTROL,
    #[doc = "0x704 - Sub-Second Increment Register"]
    pub sub_second_increment: SUB_SECOND_INCREMENT,
    #[doc = "0x708 - System Time - Seconds Register"]
    pub system_time_seconds: SYSTEM_TIME_SECONDS,
    #[doc = "0x70c - System Time Nanoseconds Register"]
    pub system_time_nanoseconds: SYSTEM_TIME_NANOSECONDS,
    #[doc = "0x710 - System Time - Seconds Update Register"]
    pub system_time_seconds_update: SYSTEM_TIME_SECONDS_UPDATE,
    #[doc = "0x714 - System Time Nanoseconds Update Register"]
    pub system_time_nanoseconds_update: SYSTEM_TIME_NANOSECONDS_UPDATE,
    #[doc = "0x718 - Timestamp Addend Register"]
    pub timestamp_addend: TIMESTAMP_ADDEND,
    #[doc = "0x71c - Target Time Seconds Register"]
    pub target_time_seconds: TARGET_TIME_SECONDS,
    #[doc = "0x720 - Target Time Nanoseconds Register"]
    pub target_time_nanoseconds: TARGET_TIME_NANOSECONDS,
    #[doc = "0x724 - System Time - Higher Word Seconds Register"]
    pub system_time_higher_word_seconds: SYSTEM_TIME_HIGHER_WORD_SECONDS,
    #[doc = "0x728 - Timestamp Status Register"]
    pub timestamp_status: TIMESTAMP_STATUS,
    _reserved120: [u8; 0x08d4],
    #[doc = "0x1000 - Bus Mode Register"]
    pub bus_mode: BUS_MODE,
    #[doc = "0x1004 - Transmit Poll Demand Register"]
    pub transmit_poll_demand: TRANSMIT_POLL_DEMAND,
    #[doc = "0x1008 - Receive Poll Demand Register"]
    pub receive_poll_demand: RECEIVE_POLL_DEMAND,
    #[doc = "0x100c - Receive Descriptor Address Register"]
    pub receive_descriptor_list_address: RECEIVE_DESCRIPTOR_LIST_ADDRESS,
    #[doc = "0x1010 - Transmit descripter Address Register"]
    pub transmit_descriptor_list_address: TRANSMIT_DESCRIPTOR_LIST_ADDRESS,
    #[doc = "0x1014 - Status Register"]
    pub status: STATUS,
    #[doc = "0x1018 - Operation Mode Register"]
    pub operation_mode: OPERATION_MODE,
    #[doc = "0x101c - Interrupt Enable Register"]
    pub interrupt_enable: INTERRUPT_ENABLE,
    #[doc = "0x1020 - Missed Frame and Buffer Overflow Counter Register"]
    pub missed_frame_and_buffer_overflow_counter: MISSED_FRAME_AND_BUFFER_OVERFLOW_COUNTER,
    #[doc = "0x1024 - Receive Interrupt Watchdog Timer Register"]
    pub receive_interrupt_watchdog_timer: RECEIVE_INTERRUPT_WATCHDOG_TIMER,
    _reserved130: [u8; 0x04],
    #[doc = "0x102c - AHB Status Register"]
    pub ahb_status: AHB_STATUS,
    _reserved131: [u8; 0x18],
    #[doc = "0x1048 - Current Host Transmit Descriptor Register"]
    pub current_host_transmit_descriptor: CURRENT_HOST_TRANSMIT_DESCRIPTOR,
    #[doc = "0x104c - Current Host Receive Descriptor Register"]
    pub current_host_receive_descriptor: CURRENT_HOST_RECEIVE_DESCRIPTOR,
    #[doc = "0x1050 - Current Host Transmit Buffer Address Register"]
    pub current_host_transmit_buffer_address: CURRENT_HOST_TRANSMIT_BUFFER_ADDRESS,
    #[doc = "0x1054 - Current Host Receive Buffer Address Register"]
    pub current_host_receive_buffer_address: CURRENT_HOST_RECEIVE_BUFFER_ADDRESS,
    #[doc = "0x1058 - HW Feature Register"]
    pub hw_feature: HW_FEATURE,
}
#[doc = "MAC_CONFIGURATION (rw) register accessor: an alias for `Reg<MAC_CONFIGURATION_SPEC>`"]
pub type MAC_CONFIGURATION = crate::Reg<mac_configuration::MAC_CONFIGURATION_SPEC>;
#[doc = "MAC Configuration Register"]
pub mod mac_configuration;
#[doc = "MAC_FRAME_FILTER (rw) register accessor: an alias for `Reg<MAC_FRAME_FILTER_SPEC>`"]
pub type MAC_FRAME_FILTER = crate::Reg<mac_frame_filter::MAC_FRAME_FILTER_SPEC>;
#[doc = "MAC Frame Filter"]
pub mod mac_frame_filter;
#[doc = "HASH_TABLE_HIGH (rw) register accessor: an alias for `Reg<HASH_TABLE_HIGH_SPEC>`"]
pub type HASH_TABLE_HIGH = crate::Reg<hash_table_high::HASH_TABLE_HIGH_SPEC>;
#[doc = "Hash Table High Register"]
pub mod hash_table_high;
#[doc = "HASH_TABLE_LOW (rw) register accessor: an alias for `Reg<HASH_TABLE_LOW_SPEC>`"]
pub type HASH_TABLE_LOW = crate::Reg<hash_table_low::HASH_TABLE_LOW_SPEC>;
#[doc = "Hash Table Low Register"]
pub mod hash_table_low;
#[doc = "GMII_ADDRESS (rw) register accessor: an alias for `Reg<GMII_ADDRESS_SPEC>`"]
pub type GMII_ADDRESS = crate::Reg<gmii_address::GMII_ADDRESS_SPEC>;
#[doc = "MII Address Register"]
pub mod gmii_address;
#[doc = "GMII_DATA (rw) register accessor: an alias for `Reg<GMII_DATA_SPEC>`"]
pub type GMII_DATA = crate::Reg<gmii_data::GMII_DATA_SPEC>;
#[doc = "MII Data Register"]
pub mod gmii_data;
#[doc = "FLOW_CONTROL (rw) register accessor: an alias for `Reg<FLOW_CONTROL_SPEC>`"]
pub type FLOW_CONTROL = crate::Reg<flow_control::FLOW_CONTROL_SPEC>;
#[doc = "Flow Control Register"]
pub mod flow_control;
#[doc = "VLAN_TAG (rw) register accessor: an alias for `Reg<VLAN_TAG_SPEC>`"]
pub type VLAN_TAG = crate::Reg<vlan_tag::VLAN_TAG_SPEC>;
#[doc = "VLAN Tag Register"]
pub mod vlan_tag;
#[doc = "VERSION (r) register accessor: an alias for `Reg<VERSION_SPEC>`"]
pub type VERSION = crate::Reg<version::VERSION_SPEC>;
#[doc = "Version Register"]
pub mod version;
#[doc = "DEBUG (r) register accessor: an alias for `Reg<DEBUG_SPEC>`"]
pub type DEBUG = crate::Reg<debug::DEBUG_SPEC>;
#[doc = "Debug Register"]
pub mod debug;
#[doc = "REMOTE_WAKE_UP_FRAME_FILTER (rw) register accessor: an alias for `Reg<REMOTE_WAKE_UP_FRAME_FILTER_SPEC>`"]
pub type REMOTE_WAKE_UP_FRAME_FILTER = crate::Reg<remote_wake_up_frame_filter::REMOTE_WAKE_UP_FRAME_FILTER_SPEC>;
#[doc = "Remote Wake Up Frame Filter Register"]
pub mod remote_wake_up_frame_filter;
#[doc = "PMT_CONTROL_STATUS (rw) register accessor: an alias for `Reg<PMT_CONTROL_STATUS_SPEC>`"]
pub type PMT_CONTROL_STATUS = crate::Reg<pmt_control_status::PMT_CONTROL_STATUS_SPEC>;
#[doc = "PMT Control and Status Register"]
pub mod pmt_control_status;
#[doc = "INTERRUPT_STATUS (r) register accessor: an alias for `Reg<INTERRUPT_STATUS_SPEC>`"]
pub type INTERRUPT_STATUS = crate::Reg<interrupt_status::INTERRUPT_STATUS_SPEC>;
#[doc = "Interrupt Register"]
pub mod interrupt_status;
#[doc = "INTERRUPT_MASK (rw) register accessor: an alias for `Reg<INTERRUPT_MASK_SPEC>`"]
pub type INTERRUPT_MASK = crate::Reg<interrupt_mask::INTERRUPT_MASK_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod interrupt_mask;
#[doc = "MAC_ADDRESS0_HIGH (rw) register accessor: an alias for `Reg<MAC_ADDRESS0_HIGH_SPEC>`"]
pub type MAC_ADDRESS0_HIGH = crate::Reg<mac_address0_high::MAC_ADDRESS0_HIGH_SPEC>;
#[doc = "MAC Address0 High Register"]
pub mod mac_address0_high;
#[doc = "MAC_ADDRESS0_LOW (rw) register accessor: an alias for `Reg<MAC_ADDRESS0_LOW_SPEC>`"]
pub type MAC_ADDRESS0_LOW = crate::Reg<mac_address0_low::MAC_ADDRESS0_LOW_SPEC>;
#[doc = "MAC Address0 Low Register"]
pub mod mac_address0_low;
#[doc = "MAC_ADDRESS1_HIGH (rw) register accessor: an alias for `Reg<MAC_ADDRESS1_HIGH_SPEC>`"]
pub type MAC_ADDRESS1_HIGH = crate::Reg<mac_address1_high::MAC_ADDRESS1_HIGH_SPEC>;
#[doc = "MAC Address1 High Register"]
pub mod mac_address1_high;
#[doc = "MAC_ADDRESS1_LOW (rw) register accessor: an alias for `Reg<MAC_ADDRESS1_LOW_SPEC>`"]
pub type MAC_ADDRESS1_LOW = crate::Reg<mac_address1_low::MAC_ADDRESS1_LOW_SPEC>;
#[doc = "MAC Address1 Low Register"]
pub mod mac_address1_low;
#[doc = "MAC_ADDRESS2_HIGH (rw) register accessor: an alias for `Reg<MAC_ADDRESS2_HIGH_SPEC>`"]
pub type MAC_ADDRESS2_HIGH = crate::Reg<mac_address2_high::MAC_ADDRESS2_HIGH_SPEC>;
#[doc = "MAC Address2 High Register"]
pub mod mac_address2_high;
#[doc = "MAC_ADDRESS2_LOW (rw) register accessor: an alias for `Reg<MAC_ADDRESS2_LOW_SPEC>`"]
pub type MAC_ADDRESS2_LOW = crate::Reg<mac_address2_low::MAC_ADDRESS2_LOW_SPEC>;
#[doc = "MAC Address2 Low Register"]
pub mod mac_address2_low;
#[doc = "MAC_ADDRESS3_HIGH (rw) register accessor: an alias for `Reg<MAC_ADDRESS3_HIGH_SPEC>`"]
pub type MAC_ADDRESS3_HIGH = crate::Reg<mac_address3_high::MAC_ADDRESS3_HIGH_SPEC>;
#[doc = "MAC Address3 High Register"]
pub mod mac_address3_high;
#[doc = "MAC_ADDRESS3_LOW (rw) register accessor: an alias for `Reg<MAC_ADDRESS3_LOW_SPEC>`"]
pub type MAC_ADDRESS3_LOW = crate::Reg<mac_address3_low::MAC_ADDRESS3_LOW_SPEC>;
#[doc = "MAC Address3 Low Register"]
pub mod mac_address3_low;
#[doc = "MMC_CONTROL (rw) register accessor: an alias for `Reg<MMC_CONTROL_SPEC>`"]
pub type MMC_CONTROL = crate::Reg<mmc_control::MMC_CONTROL_SPEC>;
#[doc = "MMC Control Register"]
pub mod mmc_control;
#[doc = "MMC_RECEIVE_INTERRUPT (r) register accessor: an alias for `Reg<MMC_RECEIVE_INTERRUPT_SPEC>`"]
pub type MMC_RECEIVE_INTERRUPT = crate::Reg<mmc_receive_interrupt::MMC_RECEIVE_INTERRUPT_SPEC>;
#[doc = "MMC Receive Interrupt Register"]
pub mod mmc_receive_interrupt;
#[doc = "MMC_TRANSMIT_INTERRUPT (r) register accessor: an alias for `Reg<MMC_TRANSMIT_INTERRUPT_SPEC>`"]
pub type MMC_TRANSMIT_INTERRUPT = crate::Reg<mmc_transmit_interrupt::MMC_TRANSMIT_INTERRUPT_SPEC>;
#[doc = "MMC Transmit Interrupt Register"]
pub mod mmc_transmit_interrupt;
#[doc = "MMC_RECEIVE_INTERRUPT_MASK (rw) register accessor: an alias for `Reg<MMC_RECEIVE_INTERRUPT_MASK_SPEC>`"]
pub type MMC_RECEIVE_INTERRUPT_MASK = crate::Reg<mmc_receive_interrupt_mask::MMC_RECEIVE_INTERRUPT_MASK_SPEC>;
#[doc = "MMC Reveive Interrupt Mask Register"]
pub mod mmc_receive_interrupt_mask;
#[doc = "MMC_TRANSMIT_INTERRUPT_MASK (rw) register accessor: an alias for `Reg<MMC_TRANSMIT_INTERRUPT_MASK_SPEC>`"]
pub type MMC_TRANSMIT_INTERRUPT_MASK = crate::Reg<mmc_transmit_interrupt_mask::MMC_TRANSMIT_INTERRUPT_MASK_SPEC>;
#[doc = "MMC Transmit Interrupt Mask Register"]
pub mod mmc_transmit_interrupt_mask;
#[doc = "TX_OCTET_COUNT_GOOD_BAD (r) register accessor: an alias for `Reg<TX_OCTET_COUNT_GOOD_BAD_SPEC>`"]
pub type TX_OCTET_COUNT_GOOD_BAD = crate::Reg<tx_octet_count_good_bad::TX_OCTET_COUNT_GOOD_BAD_SPEC>;
#[doc = "Transmit Octet Count for Good and Bad Frames Register"]
pub mod tx_octet_count_good_bad;
#[doc = "TX_FRAME_COUNT_GOOD_BAD (r) register accessor: an alias for `Reg<TX_FRAME_COUNT_GOOD_BAD_SPEC>`"]
pub type TX_FRAME_COUNT_GOOD_BAD = crate::Reg<tx_frame_count_good_bad::TX_FRAME_COUNT_GOOD_BAD_SPEC>;
#[doc = "Transmit Frame Count for Goodand Bad Frames Register"]
pub mod tx_frame_count_good_bad;
#[doc = "TX_BROADCAST_FRAMES_GOOD (r) register accessor: an alias for `Reg<TX_BROADCAST_FRAMES_GOOD_SPEC>`"]
pub type TX_BROADCAST_FRAMES_GOOD = crate::Reg<tx_broadcast_frames_good::TX_BROADCAST_FRAMES_GOOD_SPEC>;
#[doc = "Transmit Frame Count for Good Broadcast Frames"]
pub mod tx_broadcast_frames_good;
#[doc = "TX_MULTICAST_FRAMES_GOOD (r) register accessor: an alias for `Reg<TX_MULTICAST_FRAMES_GOOD_SPEC>`"]
pub type TX_MULTICAST_FRAMES_GOOD = crate::Reg<tx_multicast_frames_good::TX_MULTICAST_FRAMES_GOOD_SPEC>;
#[doc = "Transmit Frame Count for Good Multicast Frames"]
pub mod tx_multicast_frames_good;
#[doc = "TX_64OCTETS_FRAMES_GOOD_BAD (r) register accessor: an alias for `Reg<TX_64OCTETS_FRAMES_GOOD_BAD_SPEC>`"]
pub type TX_64OCTETS_FRAMES_GOOD_BAD = crate::Reg<tx_64octets_frames_good_bad::TX_64OCTETS_FRAMES_GOOD_BAD_SPEC>;
#[doc = "Transmit Octet Count for Good and Bad 64 Byte Frames"]
pub mod tx_64octets_frames_good_bad;
#[doc = "TX_65TO127OCTETS_FRAMES_GOOD_BAD (r) register accessor: an alias for `Reg<TX_65TO127OCTETS_FRAMES_GOOD_BAD_SPEC>`"]
pub type TX_65TO127OCTETS_FRAMES_GOOD_BAD = crate::Reg<tx_65to127octets_frames_good_bad::TX_65TO127OCTETS_FRAMES_GOOD_BAD_SPEC>;
#[doc = "Transmit Octet Count for Good and Bad 65 to 127 Bytes Frames"]
pub mod tx_65to127octets_frames_good_bad;
#[doc = "TX_128TO255OCTETS_FRAMES_GOOD_BAD (r) register accessor: an alias for `Reg<TX_128TO255OCTETS_FRAMES_GOOD_BAD_SPEC>`"]
pub type TX_128TO255OCTETS_FRAMES_GOOD_BAD = crate::Reg<tx_128to255octets_frames_good_bad::TX_128TO255OCTETS_FRAMES_GOOD_BAD_SPEC>;
#[doc = "Transmit Octet Count for Good and Bad 128 to 255 Bytes Frames"]
pub mod tx_128to255octets_frames_good_bad;
#[doc = "TX_256TO511OCTETS_FRAMES_GOOD_BAD (r) register accessor: an alias for `Reg<TX_256TO511OCTETS_FRAMES_GOOD_BAD_SPEC>`"]
pub type TX_256TO511OCTETS_FRAMES_GOOD_BAD = crate::Reg<tx_256to511octets_frames_good_bad::TX_256TO511OCTETS_FRAMES_GOOD_BAD_SPEC>;
#[doc = "Transmit Octet Count for Good and Bad 256 to 511 Bytes Frames"]
pub mod tx_256to511octets_frames_good_bad;
#[doc = "TX_512TO1023OCTETS_FRAMES_GOOD_BAD (r) register accessor: an alias for `Reg<TX_512TO1023OCTETS_FRAMES_GOOD_BAD_SPEC>`"]
pub type TX_512TO1023OCTETS_FRAMES_GOOD_BAD = crate::Reg<tx_512to1023octets_frames_good_bad::TX_512TO1023OCTETS_FRAMES_GOOD_BAD_SPEC>;
#[doc = "Transmit Octet Count for Good and Bad 512 to 1023 Bytes Frames"]
pub mod tx_512to1023octets_frames_good_bad;
#[doc = "TX_1024TOMAXOCTETS_FRAMES_GOOD_BAD (r) register accessor: an alias for `Reg<TX_1024TOMAXOCTETS_FRAMES_GOOD_BAD_SPEC>`"]
pub type TX_1024TOMAXOCTETS_FRAMES_GOOD_BAD = crate::Reg<tx_1024tomaxoctets_frames_good_bad::TX_1024TOMAXOCTETS_FRAMES_GOOD_BAD_SPEC>;
#[doc = "Transmit Octet Count for Good and Bad 1024 to Maxsize Bytes Frames"]
pub mod tx_1024tomaxoctets_frames_good_bad;
#[doc = "TX_UNICAST_FRAMES_GOOD_BAD (r) register accessor: an alias for `Reg<TX_UNICAST_FRAMES_GOOD_BAD_SPEC>`"]
pub type TX_UNICAST_FRAMES_GOOD_BAD = crate::Reg<tx_unicast_frames_good_bad::TX_UNICAST_FRAMES_GOOD_BAD_SPEC>;
#[doc = "Transmit Frame Count for Good and Bad Unicast Frames"]
pub mod tx_unicast_frames_good_bad;
#[doc = "TX_MULTICAST_FRAMES_GOOD_BAD (r) register accessor: an alias for `Reg<TX_MULTICAST_FRAMES_GOOD_BAD_SPEC>`"]
pub type TX_MULTICAST_FRAMES_GOOD_BAD = crate::Reg<tx_multicast_frames_good_bad::TX_MULTICAST_FRAMES_GOOD_BAD_SPEC>;
#[doc = "Transmit Frame Count for Good and Bad Multicast Frames"]
pub mod tx_multicast_frames_good_bad;
#[doc = "TX_BROADCAST_FRAMES_GOOD_BAD (r) register accessor: an alias for `Reg<TX_BROADCAST_FRAMES_GOOD_BAD_SPEC>`"]
pub type TX_BROADCAST_FRAMES_GOOD_BAD = crate::Reg<tx_broadcast_frames_good_bad::TX_BROADCAST_FRAMES_GOOD_BAD_SPEC>;
#[doc = "Transmit Frame Count for Good and Bad Broadcast Frames"]
pub mod tx_broadcast_frames_good_bad;
#[doc = "TX_UNDERFLOW_ERROR_FRAMES (r) register accessor: an alias for `Reg<TX_UNDERFLOW_ERROR_FRAMES_SPEC>`"]
pub type TX_UNDERFLOW_ERROR_FRAMES = crate::Reg<tx_underflow_error_frames::TX_UNDERFLOW_ERROR_FRAMES_SPEC>;
#[doc = "Transmit Frame Count for Underflow Error Frames"]
pub mod tx_underflow_error_frames;
#[doc = "TX_SINGLE_COLLISION_GOOD_FRAMES (r) register accessor: an alias for `Reg<TX_SINGLE_COLLISION_GOOD_FRAMES_SPEC>`"]
pub type TX_SINGLE_COLLISION_GOOD_FRAMES = crate::Reg<tx_single_collision_good_frames::TX_SINGLE_COLLISION_GOOD_FRAMES_SPEC>;
#[doc = "Transmit Frame Count for Frames Transmitted after Single Collision"]
pub mod tx_single_collision_good_frames;
#[doc = "TX_MULTIPLE_COLLISION_GOOD_FRAMES (r) register accessor: an alias for `Reg<TX_MULTIPLE_COLLISION_GOOD_FRAMES_SPEC>`"]
pub type TX_MULTIPLE_COLLISION_GOOD_FRAMES = crate::Reg<tx_multiple_collision_good_frames::TX_MULTIPLE_COLLISION_GOOD_FRAMES_SPEC>;
#[doc = "Transmit Frame Count for Frames Transmitted after Multiple Collision"]
pub mod tx_multiple_collision_good_frames;
#[doc = "TX_DEFERRED_FRAMES (r) register accessor: an alias for `Reg<TX_DEFERRED_FRAMES_SPEC>`"]
pub type TX_DEFERRED_FRAMES = crate::Reg<tx_deferred_frames::TX_DEFERRED_FRAMES_SPEC>;
#[doc = "Tx Deferred Frames Register"]
pub mod tx_deferred_frames;
#[doc = "TX_LATE_COLLISION_FRAMES (r) register accessor: an alias for `Reg<TX_LATE_COLLISION_FRAMES_SPEC>`"]
pub type TX_LATE_COLLISION_FRAMES = crate::Reg<tx_late_collision_frames::TX_LATE_COLLISION_FRAMES_SPEC>;
#[doc = "Transmit Frame Count for Late Collision Error Frames"]
pub mod tx_late_collision_frames;
#[doc = "TX_EXCESSIVE_COLLISION_FRAMES (r) register accessor: an alias for `Reg<TX_EXCESSIVE_COLLISION_FRAMES_SPEC>`"]
pub type TX_EXCESSIVE_COLLISION_FRAMES = crate::Reg<tx_excessive_collision_frames::TX_EXCESSIVE_COLLISION_FRAMES_SPEC>;
#[doc = "Transmit Frame Count for Excessive Collision Error Frames"]
pub mod tx_excessive_collision_frames;
#[doc = "TX_CARRIER_ERROR_FRAMES (r) register accessor: an alias for `Reg<TX_CARRIER_ERROR_FRAMES_SPEC>`"]
pub type TX_CARRIER_ERROR_FRAMES = crate::Reg<tx_carrier_error_frames::TX_CARRIER_ERROR_FRAMES_SPEC>;
#[doc = "Transmit Frame Count for Carrier Sense Error Frames"]
pub mod tx_carrier_error_frames;
#[doc = "TX_OCTET_COUNT_GOOD (r) register accessor: an alias for `Reg<TX_OCTET_COUNT_GOOD_SPEC>`"]
pub type TX_OCTET_COUNT_GOOD = crate::Reg<tx_octet_count_good::TX_OCTET_COUNT_GOOD_SPEC>;
#[doc = "Tx Octet Count Good Register"]
pub mod tx_octet_count_good;
#[doc = "TX_FRAME_COUNT_GOOD (r) register accessor: an alias for `Reg<TX_FRAME_COUNT_GOOD_SPEC>`"]
pub type TX_FRAME_COUNT_GOOD = crate::Reg<tx_frame_count_good::TX_FRAME_COUNT_GOOD_SPEC>;
#[doc = "Tx Frame Count Good Register"]
pub mod tx_frame_count_good;
#[doc = "TX_EXCESSIVE_DEFERRAL_ERROR (r) register accessor: an alias for `Reg<TX_EXCESSIVE_DEFERRAL_ERROR_SPEC>`"]
pub type TX_EXCESSIVE_DEFERRAL_ERROR = crate::Reg<tx_excessive_deferral_error::TX_EXCESSIVE_DEFERRAL_ERROR_SPEC>;
#[doc = "Transmit Frame Count for Excessive Deferral Error Frames"]
pub mod tx_excessive_deferral_error;
#[doc = "TX_PAUSE_FRAMES (r) register accessor: an alias for `Reg<TX_PAUSE_FRAMES_SPEC>`"]
pub type TX_PAUSE_FRAMES = crate::Reg<tx_pause_frames::TX_PAUSE_FRAMES_SPEC>;
#[doc = "Transmit Frame Count for Good PAUSE Frames"]
pub mod tx_pause_frames;
#[doc = "TX_VLAN_FRAMES_GOOD (r) register accessor: an alias for `Reg<TX_VLAN_FRAMES_GOOD_SPEC>`"]
pub type TX_VLAN_FRAMES_GOOD = crate::Reg<tx_vlan_frames_good::TX_VLAN_FRAMES_GOOD_SPEC>;
#[doc = "Transmit Frame Count for Good VLAN Frames"]
pub mod tx_vlan_frames_good;
#[doc = "TX_OSIZE_FRAMES_GOOD (r) register accessor: an alias for `Reg<TX_OSIZE_FRAMES_GOOD_SPEC>`"]
pub type TX_OSIZE_FRAMES_GOOD = crate::Reg<tx_osize_frames_good::TX_OSIZE_FRAMES_GOOD_SPEC>;
#[doc = "Transmit Frame Count for Good Oversize Frames"]
pub mod tx_osize_frames_good;
#[doc = "RX_FRAMES_COUNT_GOOD_BAD (r) register accessor: an alias for `Reg<RX_FRAMES_COUNT_GOOD_BAD_SPEC>`"]
pub type RX_FRAMES_COUNT_GOOD_BAD = crate::Reg<rx_frames_count_good_bad::RX_FRAMES_COUNT_GOOD_BAD_SPEC>;
#[doc = "Receive Frame Count for Good and Bad Frames"]
pub mod rx_frames_count_good_bad;
#[doc = "RX_OCTET_COUNT_GOOD_BAD (r) register accessor: an alias for `Reg<RX_OCTET_COUNT_GOOD_BAD_SPEC>`"]
pub type RX_OCTET_COUNT_GOOD_BAD = crate::Reg<rx_octet_count_good_bad::RX_OCTET_COUNT_GOOD_BAD_SPEC>;
#[doc = "Receive Octet Count for Good and Bad Frames"]
pub mod rx_octet_count_good_bad;
#[doc = "RX_OCTET_COUNT_GOOD (r) register accessor: an alias for `Reg<RX_OCTET_COUNT_GOOD_SPEC>`"]
pub type RX_OCTET_COUNT_GOOD = crate::Reg<rx_octet_count_good::RX_OCTET_COUNT_GOOD_SPEC>;
#[doc = "Rx Octet Count Good Register"]
pub mod rx_octet_count_good;
#[doc = "RX_BROADCAST_FRAMES_GOOD (r) register accessor: an alias for `Reg<RX_BROADCAST_FRAMES_GOOD_SPEC>`"]
pub type RX_BROADCAST_FRAMES_GOOD = crate::Reg<rx_broadcast_frames_good::RX_BROADCAST_FRAMES_GOOD_SPEC>;
#[doc = "Receive Frame Count for Good Broadcast Frames"]
pub mod rx_broadcast_frames_good;
#[doc = "RX_MULTICAST_FRAMES_GOOD (r) register accessor: an alias for `Reg<RX_MULTICAST_FRAMES_GOOD_SPEC>`"]
pub type RX_MULTICAST_FRAMES_GOOD = crate::Reg<rx_multicast_frames_good::RX_MULTICAST_FRAMES_GOOD_SPEC>;
#[doc = "Receive Frame Count for Good Multicast Frames"]
pub mod rx_multicast_frames_good;
#[doc = "RX_CRC_ERROR_FRAMES (r) register accessor: an alias for `Reg<RX_CRC_ERROR_FRAMES_SPEC>`"]
pub type RX_CRC_ERROR_FRAMES = crate::Reg<rx_crc_error_frames::RX_CRC_ERROR_FRAMES_SPEC>;
#[doc = "Receive Frame Count for CRC Error Frames"]
pub mod rx_crc_error_frames;
#[doc = "RX_ALIGNMENT_ERROR_FRAMES (r) register accessor: an alias for `Reg<RX_ALIGNMENT_ERROR_FRAMES_SPEC>`"]
pub type RX_ALIGNMENT_ERROR_FRAMES = crate::Reg<rx_alignment_error_frames::RX_ALIGNMENT_ERROR_FRAMES_SPEC>;
#[doc = "Receive Frame Count for Alignment Error Frames"]
pub mod rx_alignment_error_frames;
#[doc = "RX_RUNT_ERROR_FRAMES (r) register accessor: an alias for `Reg<RX_RUNT_ERROR_FRAMES_SPEC>`"]
pub type RX_RUNT_ERROR_FRAMES = crate::Reg<rx_runt_error_frames::RX_RUNT_ERROR_FRAMES_SPEC>;
#[doc = "Receive Frame Count for Runt Error Frames"]
pub mod rx_runt_error_frames;
#[doc = "RX_JABBER_ERROR_FRAMES (r) register accessor: an alias for `Reg<RX_JABBER_ERROR_FRAMES_SPEC>`"]
pub type RX_JABBER_ERROR_FRAMES = crate::Reg<rx_jabber_error_frames::RX_JABBER_ERROR_FRAMES_SPEC>;
#[doc = "Receive Frame Count for Jabber Error Frames"]
pub mod rx_jabber_error_frames;
#[doc = "RX_UNDERSIZE_FRAMES_GOOD (r) register accessor: an alias for `Reg<RX_UNDERSIZE_FRAMES_GOOD_SPEC>`"]
pub type RX_UNDERSIZE_FRAMES_GOOD = crate::Reg<rx_undersize_frames_good::RX_UNDERSIZE_FRAMES_GOOD_SPEC>;
#[doc = "Receive Frame Count for Undersize Frames"]
pub mod rx_undersize_frames_good;
#[doc = "RX_OVERSIZE_FRAMES_GOOD (r) register accessor: an alias for `Reg<RX_OVERSIZE_FRAMES_GOOD_SPEC>`"]
pub type RX_OVERSIZE_FRAMES_GOOD = crate::Reg<rx_oversize_frames_good::RX_OVERSIZE_FRAMES_GOOD_SPEC>;
#[doc = "Rx Oversize Frames Good Register"]
pub mod rx_oversize_frames_good;
#[doc = "RX_64OCTETS_FRAMES_GOOD_BAD (r) register accessor: an alias for `Reg<RX_64OCTETS_FRAMES_GOOD_BAD_SPEC>`"]
pub type RX_64OCTETS_FRAMES_GOOD_BAD = crate::Reg<rx_64octets_frames_good_bad::RX_64OCTETS_FRAMES_GOOD_BAD_SPEC>;
#[doc = "Receive Frame Count for Good and Bad 64 Byte Frames"]
pub mod rx_64octets_frames_good_bad;
#[doc = "RX_65TO127OCTETS_FRAMES_GOOD_BAD (r) register accessor: an alias for `Reg<RX_65TO127OCTETS_FRAMES_GOOD_BAD_SPEC>`"]
pub type RX_65TO127OCTETS_FRAMES_GOOD_BAD = crate::Reg<rx_65to127octets_frames_good_bad::RX_65TO127OCTETS_FRAMES_GOOD_BAD_SPEC>;
#[doc = "Receive Frame Count for Good and Bad 65 to 127 Bytes Frames"]
pub mod rx_65to127octets_frames_good_bad;
#[doc = "RX_128TO255OCTETS_FRAMES_GOOD_BAD (r) register accessor: an alias for `Reg<RX_128TO255OCTETS_FRAMES_GOOD_BAD_SPEC>`"]
pub type RX_128TO255OCTETS_FRAMES_GOOD_BAD = crate::Reg<rx_128to255octets_frames_good_bad::RX_128TO255OCTETS_FRAMES_GOOD_BAD_SPEC>;
#[doc = "Receive Frame Count for Good and Bad 128 to 255 Bytes Frames"]
pub mod rx_128to255octets_frames_good_bad;
#[doc = "RX_256TO511OCTETS_FRAMES_GOOD_BAD (r) register accessor: an alias for `Reg<RX_256TO511OCTETS_FRAMES_GOOD_BAD_SPEC>`"]
pub type RX_256TO511OCTETS_FRAMES_GOOD_BAD = crate::Reg<rx_256to511octets_frames_good_bad::RX_256TO511OCTETS_FRAMES_GOOD_BAD_SPEC>;
#[doc = "Receive Frame Count for Good and Bad 256 to 511 Bytes Frames"]
pub mod rx_256to511octets_frames_good_bad;
#[doc = "RX_512TO1023OCTETS_FRAMES_GOOD_BAD (r) register accessor: an alias for `Reg<RX_512TO1023OCTETS_FRAMES_GOOD_BAD_SPEC>`"]
pub type RX_512TO1023OCTETS_FRAMES_GOOD_BAD = crate::Reg<rx_512to1023octets_frames_good_bad::RX_512TO1023OCTETS_FRAMES_GOOD_BAD_SPEC>;
#[doc = "Receive Frame Count for Good and Bad 512 to 1,023 Bytes Frames"]
pub mod rx_512to1023octets_frames_good_bad;
#[doc = "RX_1024TOMAXOCTETS_FRAMES_GOOD_BAD (r) register accessor: an alias for `Reg<RX_1024TOMAXOCTETS_FRAMES_GOOD_BAD_SPEC>`"]
pub type RX_1024TOMAXOCTETS_FRAMES_GOOD_BAD = crate::Reg<rx_1024tomaxoctets_frames_good_bad::RX_1024TOMAXOCTETS_FRAMES_GOOD_BAD_SPEC>;
#[doc = "Receive Frame Count for Good and Bad 1,024 to Maxsize Bytes Frames"]
pub mod rx_1024tomaxoctets_frames_good_bad;
#[doc = "RX_UNICAST_FRAMES_GOOD (r) register accessor: an alias for `Reg<RX_UNICAST_FRAMES_GOOD_SPEC>`"]
pub type RX_UNICAST_FRAMES_GOOD = crate::Reg<rx_unicast_frames_good::RX_UNICAST_FRAMES_GOOD_SPEC>;
#[doc = "Receive Frame Count for Good Unicast Frames"]
pub mod rx_unicast_frames_good;
#[doc = "RX_LENGTH_ERROR_FRAMES (r) register accessor: an alias for `Reg<RX_LENGTH_ERROR_FRAMES_SPEC>`"]
pub type RX_LENGTH_ERROR_FRAMES = crate::Reg<rx_length_error_frames::RX_LENGTH_ERROR_FRAMES_SPEC>;
#[doc = "Receive Frame Count for Length Error Frames"]
pub mod rx_length_error_frames;
#[doc = "RX_OUT_OF_RANGE_TYPE_FRAMES (r) register accessor: an alias for `Reg<RX_OUT_OF_RANGE_TYPE_FRAMES_SPEC>`"]
pub type RX_OUT_OF_RANGE_TYPE_FRAMES = crate::Reg<rx_out_of_range_type_frames::RX_OUT_OF_RANGE_TYPE_FRAMES_SPEC>;
#[doc = "Receive Frame Count for Out of Range Frames"]
pub mod rx_out_of_range_type_frames;
#[doc = "RX_PAUSE_FRAMES (r) register accessor: an alias for `Reg<RX_PAUSE_FRAMES_SPEC>`"]
pub type RX_PAUSE_FRAMES = crate::Reg<rx_pause_frames::RX_PAUSE_FRAMES_SPEC>;
#[doc = "Receive Frame Count for PAUSE Frames"]
pub mod rx_pause_frames;
#[doc = "RX_FIFO_OVERFLOW_FRAMES (r) register accessor: an alias for `Reg<RX_FIFO_OVERFLOW_FRAMES_SPEC>`"]
pub type RX_FIFO_OVERFLOW_FRAMES = crate::Reg<rx_fifo_overflow_frames::RX_FIFO_OVERFLOW_FRAMES_SPEC>;
#[doc = "Receive Frame Count for FIFO Overflow Frames"]
pub mod rx_fifo_overflow_frames;
#[doc = "RX_VLAN_FRAMES_GOOD_BAD (r) register accessor: an alias for `Reg<RX_VLAN_FRAMES_GOOD_BAD_SPEC>`"]
pub type RX_VLAN_FRAMES_GOOD_BAD = crate::Reg<rx_vlan_frames_good_bad::RX_VLAN_FRAMES_GOOD_BAD_SPEC>;
#[doc = "Receive Frame Count for Good and Bad VLAN Frames"]
pub mod rx_vlan_frames_good_bad;
#[doc = "RX_WATCHDOG_ERROR_FRAMES (r) register accessor: an alias for `Reg<RX_WATCHDOG_ERROR_FRAMES_SPEC>`"]
pub type RX_WATCHDOG_ERROR_FRAMES = crate::Reg<rx_watchdog_error_frames::RX_WATCHDOG_ERROR_FRAMES_SPEC>;
#[doc = "Receive Frame Count for Watchdog Error Frames"]
pub mod rx_watchdog_error_frames;
#[doc = "RX_RECEIVE_ERROR_FRAMES (r) register accessor: an alias for `Reg<RX_RECEIVE_ERROR_FRAMES_SPEC>`"]
pub type RX_RECEIVE_ERROR_FRAMES = crate::Reg<rx_receive_error_frames::RX_RECEIVE_ERROR_FRAMES_SPEC>;
#[doc = "Receive Frame Count for Receive Error Frames"]
pub mod rx_receive_error_frames;
#[doc = "RX_CONTROL_FRAMES_GOOD (r) register accessor: an alias for `Reg<RX_CONTROL_FRAMES_GOOD_SPEC>`"]
pub type RX_CONTROL_FRAMES_GOOD = crate::Reg<rx_control_frames_good::RX_CONTROL_FRAMES_GOOD_SPEC>;
#[doc = "Receive Frame Count for Good Control Frames Frames"]
pub mod rx_control_frames_good;
#[doc = "MMC_IPC_RECEIVE_INTERRUPT_MASK (rw) register accessor: an alias for `Reg<MMC_IPC_RECEIVE_INTERRUPT_MASK_SPEC>`"]
pub type MMC_IPC_RECEIVE_INTERRUPT_MASK = crate::Reg<mmc_ipc_receive_interrupt_mask::MMC_IPC_RECEIVE_INTERRUPT_MASK_SPEC>;
#[doc = "MMC Receive Checksum Offload Interrupt Mask Register"]
pub mod mmc_ipc_receive_interrupt_mask;
#[doc = "MMC_IPC_RECEIVE_INTERRUPT (r) register accessor: an alias for `Reg<MMC_IPC_RECEIVE_INTERRUPT_SPEC>`"]
pub type MMC_IPC_RECEIVE_INTERRUPT = crate::Reg<mmc_ipc_receive_interrupt::MMC_IPC_RECEIVE_INTERRUPT_SPEC>;
#[doc = "MMC Receive Checksum Offload Interrupt Register"]
pub mod mmc_ipc_receive_interrupt;
#[doc = "RXIPV4_GOOD_FRAMES (r) register accessor: an alias for `Reg<RXIPV4_GOOD_FRAMES_SPEC>`"]
pub type RXIPV4_GOOD_FRAMES = crate::Reg<rxipv4_good_frames::RXIPV4_GOOD_FRAMES_SPEC>;
#[doc = "RxIPv4 Good Frames Register"]
pub mod rxipv4_good_frames;
#[doc = "RXIPV4_HEADER_ERROR_FRAMES (r) register accessor: an alias for `Reg<RXIPV4_HEADER_ERROR_FRAMES_SPEC>`"]
pub type RXIPV4_HEADER_ERROR_FRAMES = crate::Reg<rxipv4_header_error_frames::RXIPV4_HEADER_ERROR_FRAMES_SPEC>;
#[doc = "Receive IPV4 Header Error Frame Counter Register"]
pub mod rxipv4_header_error_frames;
#[doc = "RXIPV4_NO_PAYLOAD_FRAMES (r) register accessor: an alias for `Reg<RXIPV4_NO_PAYLOAD_FRAMES_SPEC>`"]
pub type RXIPV4_NO_PAYLOAD_FRAMES = crate::Reg<rxipv4_no_payload_frames::RXIPV4_NO_PAYLOAD_FRAMES_SPEC>;
#[doc = "Receive IPV4 No Payload Frame Counter Register"]
pub mod rxipv4_no_payload_frames;
#[doc = "RXIPV4_FRAGMENTED_FRAMES (r) register accessor: an alias for `Reg<RXIPV4_FRAGMENTED_FRAMES_SPEC>`"]
pub type RXIPV4_FRAGMENTED_FRAMES = crate::Reg<rxipv4_fragmented_frames::RXIPV4_FRAGMENTED_FRAMES_SPEC>;
#[doc = "Receive IPV4 Fragmented Frame Counter Register"]
pub mod rxipv4_fragmented_frames;
#[doc = "RXIPV4_UDP_CHECKSUM_DISABLED_FRAMES (r) register accessor: an alias for `Reg<RXIPV4_UDP_CHECKSUM_DISABLED_FRAMES_SPEC>`"]
pub type RXIPV4_UDP_CHECKSUM_DISABLED_FRAMES = crate::Reg<rxipv4_udp_checksum_disabled_frames::RXIPV4_UDP_CHECKSUM_DISABLED_FRAMES_SPEC>;
#[doc = "Receive IPV4 UDP Checksum Disabled Frame Counter Register"]
pub mod rxipv4_udp_checksum_disabled_frames;
#[doc = "RXIPV6_GOOD_FRAMES (r) register accessor: an alias for `Reg<RXIPV6_GOOD_FRAMES_SPEC>`"]
pub type RXIPV6_GOOD_FRAMES = crate::Reg<rxipv6_good_frames::RXIPV6_GOOD_FRAMES_SPEC>;
#[doc = "RxIPv6 Good Frames Register"]
pub mod rxipv6_good_frames;
#[doc = "RXIPV6_HEADER_ERROR_FRAMES (r) register accessor: an alias for `Reg<RXIPV6_HEADER_ERROR_FRAMES_SPEC>`"]
pub type RXIPV6_HEADER_ERROR_FRAMES = crate::Reg<rxipv6_header_error_frames::RXIPV6_HEADER_ERROR_FRAMES_SPEC>;
#[doc = "Receive IPV6 Header Error Frame Counter Register"]
pub mod rxipv6_header_error_frames;
#[doc = "RXIPV6_NO_PAYLOAD_FRAMES (r) register accessor: an alias for `Reg<RXIPV6_NO_PAYLOAD_FRAMES_SPEC>`"]
pub type RXIPV6_NO_PAYLOAD_FRAMES = crate::Reg<rxipv6_no_payload_frames::RXIPV6_NO_PAYLOAD_FRAMES_SPEC>;
#[doc = "Receive IPV6 No Payload Frame Counter Register"]
pub mod rxipv6_no_payload_frames;
#[doc = "RXUDP_GOOD_FRAMES (r) register accessor: an alias for `Reg<RXUDP_GOOD_FRAMES_SPEC>`"]
pub type RXUDP_GOOD_FRAMES = crate::Reg<rxudp_good_frames::RXUDP_GOOD_FRAMES_SPEC>;
#[doc = "RxUDP Good Frames Register"]
pub mod rxudp_good_frames;
#[doc = "RXUDP_ERROR_FRAMES (r) register accessor: an alias for `Reg<RXUDP_ERROR_FRAMES_SPEC>`"]
pub type RXUDP_ERROR_FRAMES = crate::Reg<rxudp_error_frames::RXUDP_ERROR_FRAMES_SPEC>;
#[doc = "RxUDP Error Frames Register"]
pub mod rxudp_error_frames;
#[doc = "RXTCP_GOOD_FRAMES (r) register accessor: an alias for `Reg<RXTCP_GOOD_FRAMES_SPEC>`"]
pub type RXTCP_GOOD_FRAMES = crate::Reg<rxtcp_good_frames::RXTCP_GOOD_FRAMES_SPEC>;
#[doc = "RxTCP Good Frames Register"]
pub mod rxtcp_good_frames;
#[doc = "RXTCP_ERROR_FRAMES (r) register accessor: an alias for `Reg<RXTCP_ERROR_FRAMES_SPEC>`"]
pub type RXTCP_ERROR_FRAMES = crate::Reg<rxtcp_error_frames::RXTCP_ERROR_FRAMES_SPEC>;
#[doc = "RxTCP Error Frames Register"]
pub mod rxtcp_error_frames;
#[doc = "RXICMP_GOOD_FRAMES (r) register accessor: an alias for `Reg<RXICMP_GOOD_FRAMES_SPEC>`"]
pub type RXICMP_GOOD_FRAMES = crate::Reg<rxicmp_good_frames::RXICMP_GOOD_FRAMES_SPEC>;
#[doc = "RxICMP Good Frames Register"]
pub mod rxicmp_good_frames;
#[doc = "RXICMP_ERROR_FRAMES (r) register accessor: an alias for `Reg<RXICMP_ERROR_FRAMES_SPEC>`"]
pub type RXICMP_ERROR_FRAMES = crate::Reg<rxicmp_error_frames::RXICMP_ERROR_FRAMES_SPEC>;
#[doc = "RxICMP Error Frames Register"]
pub mod rxicmp_error_frames;
#[doc = "RXIPV4_GOOD_OCTETS (r) register accessor: an alias for `Reg<RXIPV4_GOOD_OCTETS_SPEC>`"]
pub type RXIPV4_GOOD_OCTETS = crate::Reg<rxipv4_good_octets::RXIPV4_GOOD_OCTETS_SPEC>;
#[doc = "RxIPv4 Good Octets Register"]
pub mod rxipv4_good_octets;
#[doc = "RXIPV4_HEADER_ERROR_OCTETS (r) register accessor: an alias for `Reg<RXIPV4_HEADER_ERROR_OCTETS_SPEC>`"]
pub type RXIPV4_HEADER_ERROR_OCTETS = crate::Reg<rxipv4_header_error_octets::RXIPV4_HEADER_ERROR_OCTETS_SPEC>;
#[doc = "Receive IPV4 Header Error Octet Counter Register"]
pub mod rxipv4_header_error_octets;
#[doc = "RXIPV4_NO_PAYLOAD_OCTETS (r) register accessor: an alias for `Reg<RXIPV4_NO_PAYLOAD_OCTETS_SPEC>`"]
pub type RXIPV4_NO_PAYLOAD_OCTETS = crate::Reg<rxipv4_no_payload_octets::RXIPV4_NO_PAYLOAD_OCTETS_SPEC>;
#[doc = "Receive IPV4 No Payload Octet Counter Register"]
pub mod rxipv4_no_payload_octets;
#[doc = "RXIPV4_FRAGMENTED_OCTETS (r) register accessor: an alias for `Reg<RXIPV4_FRAGMENTED_OCTETS_SPEC>`"]
pub type RXIPV4_FRAGMENTED_OCTETS = crate::Reg<rxipv4_fragmented_octets::RXIPV4_FRAGMENTED_OCTETS_SPEC>;
#[doc = "Receive IPV4 Fragmented Octet Counter Register"]
pub mod rxipv4_fragmented_octets;
#[doc = "RXIPV4_UDP_CHECKSUM_DISABLE_OCTETS (r) register accessor: an alias for `Reg<RXIPV4_UDP_CHECKSUM_DISABLE_OCTETS_SPEC>`"]
pub type RXIPV4_UDP_CHECKSUM_DISABLE_OCTETS = crate::Reg<rxipv4_udp_checksum_disable_octets::RXIPV4_UDP_CHECKSUM_DISABLE_OCTETS_SPEC>;
#[doc = "Receive IPV4 Fragmented Octet Counter Register"]
pub mod rxipv4_udp_checksum_disable_octets;
#[doc = "RXIPV6_GOOD_OCTETS (r) register accessor: an alias for `Reg<RXIPV6_GOOD_OCTETS_SPEC>`"]
pub type RXIPV6_GOOD_OCTETS = crate::Reg<rxipv6_good_octets::RXIPV6_GOOD_OCTETS_SPEC>;
#[doc = "RxIPv6 Good Octets Register"]
pub mod rxipv6_good_octets;
#[doc = "RXIPV6_HEADER_ERROR_OCTETS (r) register accessor: an alias for `Reg<RXIPV6_HEADER_ERROR_OCTETS_SPEC>`"]
pub type RXIPV6_HEADER_ERROR_OCTETS = crate::Reg<rxipv6_header_error_octets::RXIPV6_HEADER_ERROR_OCTETS_SPEC>;
#[doc = "Receive IPV6 Header Error Octet Counter Register"]
pub mod rxipv6_header_error_octets;
#[doc = "RXIPV6_NO_PAYLOAD_OCTETS (r) register accessor: an alias for `Reg<RXIPV6_NO_PAYLOAD_OCTETS_SPEC>`"]
pub type RXIPV6_NO_PAYLOAD_OCTETS = crate::Reg<rxipv6_no_payload_octets::RXIPV6_NO_PAYLOAD_OCTETS_SPEC>;
#[doc = "Receive IPV6 No Payload Octet Counter Register"]
pub mod rxipv6_no_payload_octets;
#[doc = "RXUDP_GOOD_OCTETS (r) register accessor: an alias for `Reg<RXUDP_GOOD_OCTETS_SPEC>`"]
pub type RXUDP_GOOD_OCTETS = crate::Reg<rxudp_good_octets::RXUDP_GOOD_OCTETS_SPEC>;
#[doc = "Receive UDP Good Octets Register"]
pub mod rxudp_good_octets;
#[doc = "RXUDP_ERROR_OCTETS (r) register accessor: an alias for `Reg<RXUDP_ERROR_OCTETS_SPEC>`"]
pub type RXUDP_ERROR_OCTETS = crate::Reg<rxudp_error_octets::RXUDP_ERROR_OCTETS_SPEC>;
#[doc = "Receive UDP Error Octets Register"]
pub mod rxudp_error_octets;
#[doc = "RXTCP_GOOD_OCTETS (r) register accessor: an alias for `Reg<RXTCP_GOOD_OCTETS_SPEC>`"]
pub type RXTCP_GOOD_OCTETS = crate::Reg<rxtcp_good_octets::RXTCP_GOOD_OCTETS_SPEC>;
#[doc = "Receive TCP Good Octets Register"]
pub mod rxtcp_good_octets;
#[doc = "RXTCP_ERROR_OCTETS (r) register accessor: an alias for `Reg<RXTCP_ERROR_OCTETS_SPEC>`"]
pub type RXTCP_ERROR_OCTETS = crate::Reg<rxtcp_error_octets::RXTCP_ERROR_OCTETS_SPEC>;
#[doc = "Receive TCP Error Octets Register"]
pub mod rxtcp_error_octets;
#[doc = "RXICMP_GOOD_OCTETS (r) register accessor: an alias for `Reg<RXICMP_GOOD_OCTETS_SPEC>`"]
pub type RXICMP_GOOD_OCTETS = crate::Reg<rxicmp_good_octets::RXICMP_GOOD_OCTETS_SPEC>;
#[doc = "Receive ICMP Good Octets Register"]
pub mod rxicmp_good_octets;
#[doc = "RXICMP_ERROR_OCTETS (r) register accessor: an alias for `Reg<RXICMP_ERROR_OCTETS_SPEC>`"]
pub type RXICMP_ERROR_OCTETS = crate::Reg<rxicmp_error_octets::RXICMP_ERROR_OCTETS_SPEC>;
#[doc = "Receive ICMP Error Octets Register"]
pub mod rxicmp_error_octets;
#[doc = "TIMESTAMP_CONTROL (rw) register accessor: an alias for `Reg<TIMESTAMP_CONTROL_SPEC>`"]
pub type TIMESTAMP_CONTROL = crate::Reg<timestamp_control::TIMESTAMP_CONTROL_SPEC>;
#[doc = "Timestamp Control Register"]
pub mod timestamp_control;
#[doc = "SUB_SECOND_INCREMENT (rw) register accessor: an alias for `Reg<SUB_SECOND_INCREMENT_SPEC>`"]
pub type SUB_SECOND_INCREMENT = crate::Reg<sub_second_increment::SUB_SECOND_INCREMENT_SPEC>;
#[doc = "Sub-Second Increment Register"]
pub mod sub_second_increment;
#[doc = "SYSTEM_TIME_SECONDS (r) register accessor: an alias for `Reg<SYSTEM_TIME_SECONDS_SPEC>`"]
pub type SYSTEM_TIME_SECONDS = crate::Reg<system_time_seconds::SYSTEM_TIME_SECONDS_SPEC>;
#[doc = "System Time - Seconds Register"]
pub mod system_time_seconds;
#[doc = "SYSTEM_TIME_NANOSECONDS (r) register accessor: an alias for `Reg<SYSTEM_TIME_NANOSECONDS_SPEC>`"]
pub type SYSTEM_TIME_NANOSECONDS = crate::Reg<system_time_nanoseconds::SYSTEM_TIME_NANOSECONDS_SPEC>;
#[doc = "System Time Nanoseconds Register"]
pub mod system_time_nanoseconds;
#[doc = "SYSTEM_TIME_SECONDS_UPDATE (rw) register accessor: an alias for `Reg<SYSTEM_TIME_SECONDS_UPDATE_SPEC>`"]
pub type SYSTEM_TIME_SECONDS_UPDATE = crate::Reg<system_time_seconds_update::SYSTEM_TIME_SECONDS_UPDATE_SPEC>;
#[doc = "System Time - Seconds Update Register"]
pub mod system_time_seconds_update;
#[doc = "SYSTEM_TIME_NANOSECONDS_UPDATE (rw) register accessor: an alias for `Reg<SYSTEM_TIME_NANOSECONDS_UPDATE_SPEC>`"]
pub type SYSTEM_TIME_NANOSECONDS_UPDATE = crate::Reg<system_time_nanoseconds_update::SYSTEM_TIME_NANOSECONDS_UPDATE_SPEC>;
#[doc = "System Time Nanoseconds Update Register"]
pub mod system_time_nanoseconds_update;
#[doc = "TIMESTAMP_ADDEND (rw) register accessor: an alias for `Reg<TIMESTAMP_ADDEND_SPEC>`"]
pub type TIMESTAMP_ADDEND = crate::Reg<timestamp_addend::TIMESTAMP_ADDEND_SPEC>;
#[doc = "Timestamp Addend Register"]
pub mod timestamp_addend;
#[doc = "TARGET_TIME_SECONDS (rw) register accessor: an alias for `Reg<TARGET_TIME_SECONDS_SPEC>`"]
pub type TARGET_TIME_SECONDS = crate::Reg<target_time_seconds::TARGET_TIME_SECONDS_SPEC>;
#[doc = "Target Time Seconds Register"]
pub mod target_time_seconds;
#[doc = "TARGET_TIME_NANOSECONDS (rw) register accessor: an alias for `Reg<TARGET_TIME_NANOSECONDS_SPEC>`"]
pub type TARGET_TIME_NANOSECONDS = crate::Reg<target_time_nanoseconds::TARGET_TIME_NANOSECONDS_SPEC>;
#[doc = "Target Time Nanoseconds Register"]
pub mod target_time_nanoseconds;
#[doc = "SYSTEM_TIME_HIGHER_WORD_SECONDS (rw) register accessor: an alias for `Reg<SYSTEM_TIME_HIGHER_WORD_SECONDS_SPEC>`"]
pub type SYSTEM_TIME_HIGHER_WORD_SECONDS = crate::Reg<system_time_higher_word_seconds::SYSTEM_TIME_HIGHER_WORD_SECONDS_SPEC>;
#[doc = "System Time - Higher Word Seconds Register"]
pub mod system_time_higher_word_seconds;
#[doc = "TIMESTAMP_STATUS (r) register accessor: an alias for `Reg<TIMESTAMP_STATUS_SPEC>`"]
pub type TIMESTAMP_STATUS = crate::Reg<timestamp_status::TIMESTAMP_STATUS_SPEC>;
#[doc = "Timestamp Status Register"]
pub mod timestamp_status;
#[doc = "BUS_MODE (rw) register accessor: an alias for `Reg<BUS_MODE_SPEC>`"]
pub type BUS_MODE = crate::Reg<bus_mode::BUS_MODE_SPEC>;
#[doc = "Bus Mode Register"]
pub mod bus_mode;
#[doc = "TRANSMIT_POLL_DEMAND (rw) register accessor: an alias for `Reg<TRANSMIT_POLL_DEMAND_SPEC>`"]
pub type TRANSMIT_POLL_DEMAND = crate::Reg<transmit_poll_demand::TRANSMIT_POLL_DEMAND_SPEC>;
#[doc = "Transmit Poll Demand Register"]
pub mod transmit_poll_demand;
#[doc = "RECEIVE_POLL_DEMAND (rw) register accessor: an alias for `Reg<RECEIVE_POLL_DEMAND_SPEC>`"]
pub type RECEIVE_POLL_DEMAND = crate::Reg<receive_poll_demand::RECEIVE_POLL_DEMAND_SPEC>;
#[doc = "Receive Poll Demand Register"]
pub mod receive_poll_demand;
#[doc = "RECEIVE_DESCRIPTOR_LIST_ADDRESS (rw) register accessor: an alias for `Reg<RECEIVE_DESCRIPTOR_LIST_ADDRESS_SPEC>`"]
pub type RECEIVE_DESCRIPTOR_LIST_ADDRESS = crate::Reg<receive_descriptor_list_address::RECEIVE_DESCRIPTOR_LIST_ADDRESS_SPEC>;
#[doc = "Receive Descriptor Address Register"]
pub mod receive_descriptor_list_address;
#[doc = "TRANSMIT_DESCRIPTOR_LIST_ADDRESS (rw) register accessor: an alias for `Reg<TRANSMIT_DESCRIPTOR_LIST_ADDRESS_SPEC>`"]
pub type TRANSMIT_DESCRIPTOR_LIST_ADDRESS = crate::Reg<transmit_descriptor_list_address::TRANSMIT_DESCRIPTOR_LIST_ADDRESS_SPEC>;
#[doc = "Transmit descripter Address Register"]
pub mod transmit_descriptor_list_address;
#[doc = "STATUS (rw) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status Register"]
pub mod status;
#[doc = "OPERATION_MODE (rw) register accessor: an alias for `Reg<OPERATION_MODE_SPEC>`"]
pub type OPERATION_MODE = crate::Reg<operation_mode::OPERATION_MODE_SPEC>;
#[doc = "Operation Mode Register"]
pub mod operation_mode;
#[doc = "INTERRUPT_ENABLE (rw) register accessor: an alias for `Reg<INTERRUPT_ENABLE_SPEC>`"]
pub type INTERRUPT_ENABLE = crate::Reg<interrupt_enable::INTERRUPT_ENABLE_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod interrupt_enable;
#[doc = "MISSED_FRAME_AND_BUFFER_OVERFLOW_COUNTER (r) register accessor: an alias for `Reg<MISSED_FRAME_AND_BUFFER_OVERFLOW_COUNTER_SPEC>`"]
pub type MISSED_FRAME_AND_BUFFER_OVERFLOW_COUNTER = crate::Reg<missed_frame_and_buffer_overflow_counter::MISSED_FRAME_AND_BUFFER_OVERFLOW_COUNTER_SPEC>;
#[doc = "Missed Frame and Buffer Overflow Counter Register"]
pub mod missed_frame_and_buffer_overflow_counter;
#[doc = "RECEIVE_INTERRUPT_WATCHDOG_TIMER (rw) register accessor: an alias for `Reg<RECEIVE_INTERRUPT_WATCHDOG_TIMER_SPEC>`"]
pub type RECEIVE_INTERRUPT_WATCHDOG_TIMER = crate::Reg<receive_interrupt_watchdog_timer::RECEIVE_INTERRUPT_WATCHDOG_TIMER_SPEC>;
#[doc = "Receive Interrupt Watchdog Timer Register"]
pub mod receive_interrupt_watchdog_timer;
#[doc = "AHB_STATUS (r) register accessor: an alias for `Reg<AHB_STATUS_SPEC>`"]
pub type AHB_STATUS = crate::Reg<ahb_status::AHB_STATUS_SPEC>;
#[doc = "AHB Status Register"]
pub mod ahb_status;
#[doc = "CURRENT_HOST_TRANSMIT_DESCRIPTOR (r) register accessor: an alias for `Reg<CURRENT_HOST_TRANSMIT_DESCRIPTOR_SPEC>`"]
pub type CURRENT_HOST_TRANSMIT_DESCRIPTOR = crate::Reg<current_host_transmit_descriptor::CURRENT_HOST_TRANSMIT_DESCRIPTOR_SPEC>;
#[doc = "Current Host Transmit Descriptor Register"]
pub mod current_host_transmit_descriptor;
#[doc = "CURRENT_HOST_RECEIVE_DESCRIPTOR (r) register accessor: an alias for `Reg<CURRENT_HOST_RECEIVE_DESCRIPTOR_SPEC>`"]
pub type CURRENT_HOST_RECEIVE_DESCRIPTOR = crate::Reg<current_host_receive_descriptor::CURRENT_HOST_RECEIVE_DESCRIPTOR_SPEC>;
#[doc = "Current Host Receive Descriptor Register"]
pub mod current_host_receive_descriptor;
#[doc = "CURRENT_HOST_TRANSMIT_BUFFER_ADDRESS (r) register accessor: an alias for `Reg<CURRENT_HOST_TRANSMIT_BUFFER_ADDRESS_SPEC>`"]
pub type CURRENT_HOST_TRANSMIT_BUFFER_ADDRESS = crate::Reg<current_host_transmit_buffer_address::CURRENT_HOST_TRANSMIT_BUFFER_ADDRESS_SPEC>;
#[doc = "Current Host Transmit Buffer Address Register"]
pub mod current_host_transmit_buffer_address;
#[doc = "CURRENT_HOST_RECEIVE_BUFFER_ADDRESS (r) register accessor: an alias for `Reg<CURRENT_HOST_RECEIVE_BUFFER_ADDRESS_SPEC>`"]
pub type CURRENT_HOST_RECEIVE_BUFFER_ADDRESS = crate::Reg<current_host_receive_buffer_address::CURRENT_HOST_RECEIVE_BUFFER_ADDRESS_SPEC>;
#[doc = "Current Host Receive Buffer Address Register"]
pub mod current_host_receive_buffer_address;
#[doc = "HW_FEATURE (rw) register accessor: an alias for `Reg<HW_FEATURE_SPEC>`"]
pub type HW_FEATURE = crate::Reg<hw_feature::HW_FEATURE_SPEC>;
#[doc = "HW Feature Register"]
pub mod hw_feature;

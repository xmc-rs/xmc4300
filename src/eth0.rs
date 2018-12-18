#[doc = r" Register block"]
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
    _reserved0: [u8; 8usize],
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
    _reserved1: [u8; 160usize],
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
    _reserved2: [u8; 4usize],
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
    _reserved3: [u8; 24usize],
    #[doc = "0x200 - MMC Receive Checksum Offload Interrupt Mask Register"]
    pub mmc_ipc_receive_interrupt_mask: MMC_IPC_RECEIVE_INTERRUPT_MASK,
    _reserved4: [u8; 4usize],
    #[doc = "0x208 - MMC Receive Checksum Offload Interrupt Register"]
    pub mmc_ipc_receive_interrupt: MMC_IPC_RECEIVE_INTERRUPT,
    _reserved5: [u8; 4usize],
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
    _reserved6: [u8; 8usize],
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
    _reserved7: [u8; 1144usize],
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
    _reserved8: [u8; 2260usize],
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
    _reserved9: [u8; 4usize],
    #[doc = "0x102c - AHB Status Register"]
    pub ahb_status: AHB_STATUS,
    _reserved10: [u8; 24usize],
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
#[doc = "MAC Configuration Register"]
pub struct MAC_CONFIGURATION {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MAC Configuration Register"]
pub mod mac_configuration;
#[doc = "MAC Frame Filter"]
pub struct MAC_FRAME_FILTER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MAC Frame Filter"]
pub mod mac_frame_filter;
#[doc = "Hash Table High Register"]
pub struct HASH_TABLE_HIGH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hash Table High Register"]
pub mod hash_table_high;
#[doc = "Hash Table Low Register"]
pub struct HASH_TABLE_LOW {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hash Table Low Register"]
pub mod hash_table_low;
#[doc = "MII Address Register"]
pub struct GMII_ADDRESS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MII Address Register"]
pub mod gmii_address;
#[doc = "MII Data Register"]
pub struct GMII_DATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MII Data Register"]
pub mod gmii_data;
#[doc = "Flow Control Register"]
pub struct FLOW_CONTROL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flow Control Register"]
pub mod flow_control;
#[doc = "VLAN Tag Register"]
pub struct VLAN_TAG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "VLAN Tag Register"]
pub mod vlan_tag;
#[doc = "Version Register"]
pub struct VERSION {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Version Register"]
pub mod version;
#[doc = "Debug Register"]
pub struct DEBUG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Debug Register"]
pub mod debug;
#[doc = "Remote Wake Up Frame Filter Register"]
pub struct REMOTE_WAKE_UP_FRAME_FILTER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Remote Wake Up Frame Filter Register"]
pub mod remote_wake_up_frame_filter;
#[doc = "PMT Control and Status Register"]
pub struct PMT_CONTROL_STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PMT Control and Status Register"]
pub mod pmt_control_status;
#[doc = "Interrupt Register"]
pub struct INTERRUPT_STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Register"]
pub mod interrupt_status;
#[doc = "Interrupt Mask Register"]
pub struct INTERRUPT_MASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Mask Register"]
pub mod interrupt_mask;
#[doc = "MAC Address0 High Register"]
pub struct MAC_ADDRESS0_HIGH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MAC Address0 High Register"]
pub mod mac_address0_high;
#[doc = "MAC Address0 Low Register"]
pub struct MAC_ADDRESS0_LOW {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MAC Address0 Low Register"]
pub mod mac_address0_low;
#[doc = "MAC Address1 High Register"]
pub struct MAC_ADDRESS1_HIGH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MAC Address1 High Register"]
pub mod mac_address1_high;
#[doc = "MAC Address1 Low Register"]
pub struct MAC_ADDRESS1_LOW {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MAC Address1 Low Register"]
pub mod mac_address1_low;
#[doc = "MAC Address2 High Register"]
pub struct MAC_ADDRESS2_HIGH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MAC Address2 High Register"]
pub mod mac_address2_high;
#[doc = "MAC Address2 Low Register"]
pub struct MAC_ADDRESS2_LOW {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MAC Address2 Low Register"]
pub mod mac_address2_low;
#[doc = "MAC Address3 High Register"]
pub struct MAC_ADDRESS3_HIGH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MAC Address3 High Register"]
pub mod mac_address3_high;
#[doc = "MAC Address3 Low Register"]
pub struct MAC_ADDRESS3_LOW {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MAC Address3 Low Register"]
pub mod mac_address3_low;
#[doc = "MMC Control Register"]
pub struct MMC_CONTROL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MMC Control Register"]
pub mod mmc_control;
#[doc = "MMC Receive Interrupt Register"]
pub struct MMC_RECEIVE_INTERRUPT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MMC Receive Interrupt Register"]
pub mod mmc_receive_interrupt;
#[doc = "MMC Transmit Interrupt Register"]
pub struct MMC_TRANSMIT_INTERRUPT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MMC Transmit Interrupt Register"]
pub mod mmc_transmit_interrupt;
#[doc = "MMC Reveive Interrupt Mask Register"]
pub struct MMC_RECEIVE_INTERRUPT_MASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MMC Reveive Interrupt Mask Register"]
pub mod mmc_receive_interrupt_mask;
#[doc = "MMC Transmit Interrupt Mask Register"]
pub struct MMC_TRANSMIT_INTERRUPT_MASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MMC Transmit Interrupt Mask Register"]
pub mod mmc_transmit_interrupt_mask;
#[doc = "Transmit Octet Count for Good and Bad Frames Register"]
pub struct TX_OCTET_COUNT_GOOD_BAD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Octet Count for Good and Bad Frames Register"]
pub mod tx_octet_count_good_bad;
#[doc = "Transmit Frame Count for Goodand Bad Frames Register"]
pub struct TX_FRAME_COUNT_GOOD_BAD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Frame Count for Goodand Bad Frames Register"]
pub mod tx_frame_count_good_bad;
#[doc = "Transmit Frame Count for Good Broadcast Frames"]
pub struct TX_BROADCAST_FRAMES_GOOD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Frame Count for Good Broadcast Frames"]
pub mod tx_broadcast_frames_good;
#[doc = "Transmit Frame Count for Good Multicast Frames"]
pub struct TX_MULTICAST_FRAMES_GOOD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Frame Count for Good Multicast Frames"]
pub mod tx_multicast_frames_good;
#[doc = "Transmit Octet Count for Good and Bad 64 Byte Frames"]
pub struct TX_64OCTETS_FRAMES_GOOD_BAD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Octet Count for Good and Bad 64 Byte Frames"]
pub mod tx_64octets_frames_good_bad;
#[doc = "Transmit Octet Count for Good and Bad 65 to 127 Bytes Frames"]
pub struct TX_65TO127OCTETS_FRAMES_GOOD_BAD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Octet Count for Good and Bad 65 to 127 Bytes Frames"]
pub mod tx_65to127octets_frames_good_bad;
#[doc = "Transmit Octet Count for Good and Bad 128 to 255 Bytes Frames"]
pub struct TX_128TO255OCTETS_FRAMES_GOOD_BAD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Octet Count for Good and Bad 128 to 255 Bytes Frames"]
pub mod tx_128to255octets_frames_good_bad;
#[doc = "Transmit Octet Count for Good and Bad 256 to 511 Bytes Frames"]
pub struct TX_256TO511OCTETS_FRAMES_GOOD_BAD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Octet Count for Good and Bad 256 to 511 Bytes Frames"]
pub mod tx_256to511octets_frames_good_bad;
#[doc = "Transmit Octet Count for Good and Bad 512 to 1023 Bytes Frames"]
pub struct TX_512TO1023OCTETS_FRAMES_GOOD_BAD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Octet Count for Good and Bad 512 to 1023 Bytes Frames"]
pub mod tx_512to1023octets_frames_good_bad;
#[doc = "Transmit Octet Count for Good and Bad 1024 to Maxsize Bytes Frames"]
pub struct TX_1024TOMAXOCTETS_FRAMES_GOOD_BAD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Octet Count for Good and Bad 1024 to Maxsize Bytes Frames"]
pub mod tx_1024tomaxoctets_frames_good_bad;
#[doc = "Transmit Frame Count for Good and Bad Unicast Frames"]
pub struct TX_UNICAST_FRAMES_GOOD_BAD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Frame Count for Good and Bad Unicast Frames"]
pub mod tx_unicast_frames_good_bad;
#[doc = "Transmit Frame Count for Good and Bad Multicast Frames"]
pub struct TX_MULTICAST_FRAMES_GOOD_BAD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Frame Count for Good and Bad Multicast Frames"]
pub mod tx_multicast_frames_good_bad;
#[doc = "Transmit Frame Count for Good and Bad Broadcast Frames"]
pub struct TX_BROADCAST_FRAMES_GOOD_BAD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Frame Count for Good and Bad Broadcast Frames"]
pub mod tx_broadcast_frames_good_bad;
#[doc = "Transmit Frame Count for Underflow Error Frames"]
pub struct TX_UNDERFLOW_ERROR_FRAMES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Frame Count for Underflow Error Frames"]
pub mod tx_underflow_error_frames;
#[doc = "Transmit Frame Count for Frames Transmitted after Single Collision"]
pub struct TX_SINGLE_COLLISION_GOOD_FRAMES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Frame Count for Frames Transmitted after Single Collision"]
pub mod tx_single_collision_good_frames;
#[doc = "Transmit Frame Count for Frames Transmitted after Multiple Collision"]
pub struct TX_MULTIPLE_COLLISION_GOOD_FRAMES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Frame Count for Frames Transmitted after Multiple Collision"]
pub mod tx_multiple_collision_good_frames;
#[doc = "Tx Deferred Frames Register"]
pub struct TX_DEFERRED_FRAMES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Tx Deferred Frames Register"]
pub mod tx_deferred_frames;
#[doc = "Transmit Frame Count for Late Collision Error Frames"]
pub struct TX_LATE_COLLISION_FRAMES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Frame Count for Late Collision Error Frames"]
pub mod tx_late_collision_frames;
#[doc = "Transmit Frame Count for Excessive Collision Error Frames"]
pub struct TX_EXCESSIVE_COLLISION_FRAMES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Frame Count for Excessive Collision Error Frames"]
pub mod tx_excessive_collision_frames;
#[doc = "Transmit Frame Count for Carrier Sense Error Frames"]
pub struct TX_CARRIER_ERROR_FRAMES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Frame Count for Carrier Sense Error Frames"]
pub mod tx_carrier_error_frames;
#[doc = "Tx Octet Count Good Register"]
pub struct TX_OCTET_COUNT_GOOD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Tx Octet Count Good Register"]
pub mod tx_octet_count_good;
#[doc = "Tx Frame Count Good Register"]
pub struct TX_FRAME_COUNT_GOOD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Tx Frame Count Good Register"]
pub mod tx_frame_count_good;
#[doc = "Transmit Frame Count for Excessive Deferral Error Frames"]
pub struct TX_EXCESSIVE_DEFERRAL_ERROR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Frame Count for Excessive Deferral Error Frames"]
pub mod tx_excessive_deferral_error;
#[doc = "Transmit Frame Count for Good PAUSE Frames"]
pub struct TX_PAUSE_FRAMES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Frame Count for Good PAUSE Frames"]
pub mod tx_pause_frames;
#[doc = "Transmit Frame Count for Good VLAN Frames"]
pub struct TX_VLAN_FRAMES_GOOD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Frame Count for Good VLAN Frames"]
pub mod tx_vlan_frames_good;
#[doc = "Transmit Frame Count for Good Oversize Frames"]
pub struct TX_OSIZE_FRAMES_GOOD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Frame Count for Good Oversize Frames"]
pub mod tx_osize_frames_good;
#[doc = "Receive Frame Count for Good and Bad Frames"]
pub struct RX_FRAMES_COUNT_GOOD_BAD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Frame Count for Good and Bad Frames"]
pub mod rx_frames_count_good_bad;
#[doc = "Receive Octet Count for Good and Bad Frames"]
pub struct RX_OCTET_COUNT_GOOD_BAD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Octet Count for Good and Bad Frames"]
pub mod rx_octet_count_good_bad;
#[doc = "Rx Octet Count Good Register"]
pub struct RX_OCTET_COUNT_GOOD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Rx Octet Count Good Register"]
pub mod rx_octet_count_good;
#[doc = "Receive Frame Count for Good Broadcast Frames"]
pub struct RX_BROADCAST_FRAMES_GOOD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Frame Count for Good Broadcast Frames"]
pub mod rx_broadcast_frames_good;
#[doc = "Receive Frame Count for Good Multicast Frames"]
pub struct RX_MULTICAST_FRAMES_GOOD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Frame Count for Good Multicast Frames"]
pub mod rx_multicast_frames_good;
#[doc = "Receive Frame Count for CRC Error Frames"]
pub struct RX_CRC_ERROR_FRAMES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Frame Count for CRC Error Frames"]
pub mod rx_crc_error_frames;
#[doc = "Receive Frame Count for Alignment Error Frames"]
pub struct RX_ALIGNMENT_ERROR_FRAMES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Frame Count for Alignment Error Frames"]
pub mod rx_alignment_error_frames;
#[doc = "Receive Frame Count for Runt Error Frames"]
pub struct RX_RUNT_ERROR_FRAMES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Frame Count for Runt Error Frames"]
pub mod rx_runt_error_frames;
#[doc = "Receive Frame Count for Jabber Error Frames"]
pub struct RX_JABBER_ERROR_FRAMES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Frame Count for Jabber Error Frames"]
pub mod rx_jabber_error_frames;
#[doc = "Receive Frame Count for Undersize Frames"]
pub struct RX_UNDERSIZE_FRAMES_GOOD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Frame Count for Undersize Frames"]
pub mod rx_undersize_frames_good;
#[doc = "Rx Oversize Frames Good Register"]
pub struct RX_OVERSIZE_FRAMES_GOOD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Rx Oversize Frames Good Register"]
pub mod rx_oversize_frames_good;
#[doc = "Receive Frame Count for Good and Bad 64 Byte Frames"]
pub struct RX_64OCTETS_FRAMES_GOOD_BAD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Frame Count for Good and Bad 64 Byte Frames"]
pub mod rx_64octets_frames_good_bad;
#[doc = "Receive Frame Count for Good and Bad 65 to 127 Bytes Frames"]
pub struct RX_65TO127OCTETS_FRAMES_GOOD_BAD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Frame Count for Good and Bad 65 to 127 Bytes Frames"]
pub mod rx_65to127octets_frames_good_bad;
#[doc = "Receive Frame Count for Good and Bad 128 to 255 Bytes Frames"]
pub struct RX_128TO255OCTETS_FRAMES_GOOD_BAD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Frame Count for Good and Bad 128 to 255 Bytes Frames"]
pub mod rx_128to255octets_frames_good_bad;
#[doc = "Receive Frame Count for Good and Bad 256 to 511 Bytes Frames"]
pub struct RX_256TO511OCTETS_FRAMES_GOOD_BAD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Frame Count for Good and Bad 256 to 511 Bytes Frames"]
pub mod rx_256to511octets_frames_good_bad;
#[doc = "Receive Frame Count for Good and Bad 512 to 1,023 Bytes Frames"]
pub struct RX_512TO1023OCTETS_FRAMES_GOOD_BAD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Frame Count for Good and Bad 512 to 1,023 Bytes Frames"]
pub mod rx_512to1023octets_frames_good_bad;
#[doc = "Receive Frame Count for Good and Bad 1,024 to Maxsize Bytes Frames"]
pub struct RX_1024TOMAXOCTETS_FRAMES_GOOD_BAD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Frame Count for Good and Bad 1,024 to Maxsize Bytes Frames"]
pub mod rx_1024tomaxoctets_frames_good_bad;
#[doc = "Receive Frame Count for Good Unicast Frames"]
pub struct RX_UNICAST_FRAMES_GOOD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Frame Count for Good Unicast Frames"]
pub mod rx_unicast_frames_good;
#[doc = "Receive Frame Count for Length Error Frames"]
pub struct RX_LENGTH_ERROR_FRAMES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Frame Count for Length Error Frames"]
pub mod rx_length_error_frames;
#[doc = "Receive Frame Count for Out of Range Frames"]
pub struct RX_OUT_OF_RANGE_TYPE_FRAMES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Frame Count for Out of Range Frames"]
pub mod rx_out_of_range_type_frames;
#[doc = "Receive Frame Count for PAUSE Frames"]
pub struct RX_PAUSE_FRAMES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Frame Count for PAUSE Frames"]
pub mod rx_pause_frames;
#[doc = "Receive Frame Count for FIFO Overflow Frames"]
pub struct RX_FIFO_OVERFLOW_FRAMES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Frame Count for FIFO Overflow Frames"]
pub mod rx_fifo_overflow_frames;
#[doc = "Receive Frame Count for Good and Bad VLAN Frames"]
pub struct RX_VLAN_FRAMES_GOOD_BAD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Frame Count for Good and Bad VLAN Frames"]
pub mod rx_vlan_frames_good_bad;
#[doc = "Receive Frame Count for Watchdog Error Frames"]
pub struct RX_WATCHDOG_ERROR_FRAMES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Frame Count for Watchdog Error Frames"]
pub mod rx_watchdog_error_frames;
#[doc = "Receive Frame Count for Receive Error Frames"]
pub struct RX_RECEIVE_ERROR_FRAMES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Frame Count for Receive Error Frames"]
pub mod rx_receive_error_frames;
#[doc = "Receive Frame Count for Good Control Frames Frames"]
pub struct RX_CONTROL_FRAMES_GOOD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Frame Count for Good Control Frames Frames"]
pub mod rx_control_frames_good;
#[doc = "MMC Receive Checksum Offload Interrupt Mask Register"]
pub struct MMC_IPC_RECEIVE_INTERRUPT_MASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MMC Receive Checksum Offload Interrupt Mask Register"]
pub mod mmc_ipc_receive_interrupt_mask;
#[doc = "MMC Receive Checksum Offload Interrupt Register"]
pub struct MMC_IPC_RECEIVE_INTERRUPT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MMC Receive Checksum Offload Interrupt Register"]
pub mod mmc_ipc_receive_interrupt;
#[doc = "RxIPv4 Good Frames Register"]
pub struct RXIPV4_GOOD_FRAMES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RxIPv4 Good Frames Register"]
pub mod rxipv4_good_frames;
#[doc = "Receive IPV4 Header Error Frame Counter Register"]
pub struct RXIPV4_HEADER_ERROR_FRAMES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive IPV4 Header Error Frame Counter Register"]
pub mod rxipv4_header_error_frames;
#[doc = "Receive IPV4 No Payload Frame Counter Register"]
pub struct RXIPV4_NO_PAYLOAD_FRAMES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive IPV4 No Payload Frame Counter Register"]
pub mod rxipv4_no_payload_frames;
#[doc = "Receive IPV4 Fragmented Frame Counter Register"]
pub struct RXIPV4_FRAGMENTED_FRAMES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive IPV4 Fragmented Frame Counter Register"]
pub mod rxipv4_fragmented_frames;
#[doc = "Receive IPV4 UDP Checksum Disabled Frame Counter Register"]
pub struct RXIPV4_UDP_CHECKSUM_DISABLED_FRAMES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive IPV4 UDP Checksum Disabled Frame Counter Register"]
pub mod rxipv4_udp_checksum_disabled_frames;
#[doc = "RxIPv6 Good Frames Register"]
pub struct RXIPV6_GOOD_FRAMES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RxIPv6 Good Frames Register"]
pub mod rxipv6_good_frames;
#[doc = "Receive IPV6 Header Error Frame Counter Register"]
pub struct RXIPV6_HEADER_ERROR_FRAMES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive IPV6 Header Error Frame Counter Register"]
pub mod rxipv6_header_error_frames;
#[doc = "Receive IPV6 No Payload Frame Counter Register"]
pub struct RXIPV6_NO_PAYLOAD_FRAMES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive IPV6 No Payload Frame Counter Register"]
pub mod rxipv6_no_payload_frames;
#[doc = "RxUDP Good Frames Register"]
pub struct RXUDP_GOOD_FRAMES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RxUDP Good Frames Register"]
pub mod rxudp_good_frames;
#[doc = "RxUDP Error Frames Register"]
pub struct RXUDP_ERROR_FRAMES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RxUDP Error Frames Register"]
pub mod rxudp_error_frames;
#[doc = "RxTCP Good Frames Register"]
pub struct RXTCP_GOOD_FRAMES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RxTCP Good Frames Register"]
pub mod rxtcp_good_frames;
#[doc = "RxTCP Error Frames Register"]
pub struct RXTCP_ERROR_FRAMES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RxTCP Error Frames Register"]
pub mod rxtcp_error_frames;
#[doc = "RxICMP Good Frames Register"]
pub struct RXICMP_GOOD_FRAMES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RxICMP Good Frames Register"]
pub mod rxicmp_good_frames;
#[doc = "RxICMP Error Frames Register"]
pub struct RXICMP_ERROR_FRAMES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RxICMP Error Frames Register"]
pub mod rxicmp_error_frames;
#[doc = "RxIPv4 Good Octets Register"]
pub struct RXIPV4_GOOD_OCTETS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RxIPv4 Good Octets Register"]
pub mod rxipv4_good_octets;
#[doc = "Receive IPV4 Header Error Octet Counter Register"]
pub struct RXIPV4_HEADER_ERROR_OCTETS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive IPV4 Header Error Octet Counter Register"]
pub mod rxipv4_header_error_octets;
#[doc = "Receive IPV4 No Payload Octet Counter Register"]
pub struct RXIPV4_NO_PAYLOAD_OCTETS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive IPV4 No Payload Octet Counter Register"]
pub mod rxipv4_no_payload_octets;
#[doc = "Receive IPV4 Fragmented Octet Counter Register"]
pub struct RXIPV4_FRAGMENTED_OCTETS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive IPV4 Fragmented Octet Counter Register"]
pub mod rxipv4_fragmented_octets;
#[doc = "Receive IPV4 Fragmented Octet Counter Register"]
pub struct RXIPV4_UDP_CHECKSUM_DISABLE_OCTETS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive IPV4 Fragmented Octet Counter Register"]
pub mod rxipv4_udp_checksum_disable_octets;
#[doc = "RxIPv6 Good Octets Register"]
pub struct RXIPV6_GOOD_OCTETS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RxIPv6 Good Octets Register"]
pub mod rxipv6_good_octets;
#[doc = "Receive IPV6 Header Error Octet Counter Register"]
pub struct RXIPV6_HEADER_ERROR_OCTETS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive IPV6 Header Error Octet Counter Register"]
pub mod rxipv6_header_error_octets;
#[doc = "Receive IPV6 No Payload Octet Counter Register"]
pub struct RXIPV6_NO_PAYLOAD_OCTETS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive IPV6 No Payload Octet Counter Register"]
pub mod rxipv6_no_payload_octets;
#[doc = "Receive UDP Good Octets Register"]
pub struct RXUDP_GOOD_OCTETS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive UDP Good Octets Register"]
pub mod rxudp_good_octets;
#[doc = "Receive UDP Error Octets Register"]
pub struct RXUDP_ERROR_OCTETS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive UDP Error Octets Register"]
pub mod rxudp_error_octets;
#[doc = "Receive TCP Good Octets Register"]
pub struct RXTCP_GOOD_OCTETS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive TCP Good Octets Register"]
pub mod rxtcp_good_octets;
#[doc = "Receive TCP Error Octets Register"]
pub struct RXTCP_ERROR_OCTETS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive TCP Error Octets Register"]
pub mod rxtcp_error_octets;
#[doc = "Receive ICMP Good Octets Register"]
pub struct RXICMP_GOOD_OCTETS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive ICMP Good Octets Register"]
pub mod rxicmp_good_octets;
#[doc = "Receive ICMP Error Octets Register"]
pub struct RXICMP_ERROR_OCTETS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive ICMP Error Octets Register"]
pub mod rxicmp_error_octets;
#[doc = "Timestamp Control Register"]
pub struct TIMESTAMP_CONTROL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timestamp Control Register"]
pub mod timestamp_control;
#[doc = "Sub-Second Increment Register"]
pub struct SUB_SECOND_INCREMENT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Sub-Second Increment Register"]
pub mod sub_second_increment;
#[doc = "System Time - Seconds Register"]
pub struct SYSTEM_TIME_SECONDS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Time - Seconds Register"]
pub mod system_time_seconds;
#[doc = "System Time Nanoseconds Register"]
pub struct SYSTEM_TIME_NANOSECONDS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Time Nanoseconds Register"]
pub mod system_time_nanoseconds;
#[doc = "System Time - Seconds Update Register"]
pub struct SYSTEM_TIME_SECONDS_UPDATE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Time - Seconds Update Register"]
pub mod system_time_seconds_update;
#[doc = "System Time Nanoseconds Update Register"]
pub struct SYSTEM_TIME_NANOSECONDS_UPDATE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Time Nanoseconds Update Register"]
pub mod system_time_nanoseconds_update;
#[doc = "Timestamp Addend Register"]
pub struct TIMESTAMP_ADDEND {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timestamp Addend Register"]
pub mod timestamp_addend;
#[doc = "Target Time Seconds Register"]
pub struct TARGET_TIME_SECONDS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Time Seconds Register"]
pub mod target_time_seconds;
#[doc = "Target Time Nanoseconds Register"]
pub struct TARGET_TIME_NANOSECONDS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Time Nanoseconds Register"]
pub mod target_time_nanoseconds;
#[doc = "System Time - Higher Word Seconds Register"]
pub struct SYSTEM_TIME_HIGHER_WORD_SECONDS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Time - Higher Word Seconds Register"]
pub mod system_time_higher_word_seconds;
#[doc = "Timestamp Status Register"]
pub struct TIMESTAMP_STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timestamp Status Register"]
pub mod timestamp_status;
#[doc = "Bus Mode Register"]
pub struct BUS_MODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Bus Mode Register"]
pub mod bus_mode;
#[doc = "Transmit Poll Demand Register"]
pub struct TRANSMIT_POLL_DEMAND {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Poll Demand Register"]
pub mod transmit_poll_demand;
#[doc = "Receive Poll Demand Register"]
pub struct RECEIVE_POLL_DEMAND {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Poll Demand Register"]
pub mod receive_poll_demand;
#[doc = "Receive Descriptor Address Register"]
pub struct RECEIVE_DESCRIPTOR_LIST_ADDRESS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Descriptor Address Register"]
pub mod receive_descriptor_list_address;
#[doc = "Transmit descripter Address Register"]
pub struct TRANSMIT_DESCRIPTOR_LIST_ADDRESS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit descripter Address Register"]
pub mod transmit_descriptor_list_address;
#[doc = "Status Register"]
pub struct STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Register"]
pub mod status;
#[doc = "Operation Mode Register"]
pub struct OPERATION_MODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Operation Mode Register"]
pub mod operation_mode;
#[doc = "Interrupt Enable Register"]
pub struct INTERRUPT_ENABLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Register"]
pub mod interrupt_enable;
#[doc = "Missed Frame and Buffer Overflow Counter Register"]
pub struct MISSED_FRAME_AND_BUFFER_OVERFLOW_COUNTER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Missed Frame and Buffer Overflow Counter Register"]
pub mod missed_frame_and_buffer_overflow_counter;
#[doc = "Receive Interrupt Watchdog Timer Register"]
pub struct RECEIVE_INTERRUPT_WATCHDOG_TIMER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Interrupt Watchdog Timer Register"]
pub mod receive_interrupt_watchdog_timer;
#[doc = "AHB Status Register"]
pub struct AHB_STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AHB Status Register"]
pub mod ahb_status;
#[doc = "Current Host Transmit Descriptor Register"]
pub struct CURRENT_HOST_TRANSMIT_DESCRIPTOR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Current Host Transmit Descriptor Register"]
pub mod current_host_transmit_descriptor;
#[doc = "Current Host Receive Descriptor Register"]
pub struct CURRENT_HOST_RECEIVE_DESCRIPTOR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Current Host Receive Descriptor Register"]
pub mod current_host_receive_descriptor;
#[doc = "Current Host Transmit Buffer Address Register"]
pub struct CURRENT_HOST_TRANSMIT_BUFFER_ADDRESS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Current Host Transmit Buffer Address Register"]
pub mod current_host_transmit_buffer_address;
#[doc = "Current Host Receive Buffer Address Register"]
pub struct CURRENT_HOST_RECEIVE_BUFFER_ADDRESS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Current Host Receive Buffer Address Register"]
pub mod current_host_receive_buffer_address;
#[doc = "HW Feature Register"]
pub struct HW_FEATURE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HW Feature Register"]
pub mod hw_feature;

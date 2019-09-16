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
    _reserved12: [u8; 8usize],
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
    _reserved22: [u8; 160usize],
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
    _reserved53: [u8; 4usize],
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
    _reserved79: [u8; 24usize],
    #[doc = "0x200 - MMC Receive Checksum Offload Interrupt Mask Register"]
    pub mmc_ipc_receive_interrupt_mask: MMC_IPC_RECEIVE_INTERRUPT_MASK,
    _reserved80: [u8; 4usize],
    #[doc = "0x208 - MMC Receive Checksum Offload Interrupt Register"]
    pub mmc_ipc_receive_interrupt: MMC_IPC_RECEIVE_INTERRUPT,
    _reserved81: [u8; 4usize],
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
    _reserved95: [u8; 8usize],
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
    _reserved109: [u8; 1144usize],
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
    _reserved120: [u8; 2260usize],
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
    _reserved130: [u8; 4usize],
    #[doc = "0x102c - AHB Status Register"]
    pub ahb_status: AHB_STATUS,
    _reserved131: [u8; 24usize],
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
#[doc = "MAC Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mac_configuration](mac_configuration) module"]
pub type MAC_CONFIGURATION = crate::Reg<u32, _MAC_CONFIGURATION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAC_CONFIGURATION;
#[doc = "`read()` method returns [mac_configuration::R](mac_configuration::R) reader structure"]
impl crate::Readable for MAC_CONFIGURATION {}
#[doc = "`write(|w| ..)` method takes [mac_configuration::W](mac_configuration::W) writer structure"]
impl crate::Writable for MAC_CONFIGURATION {}
#[doc = "MAC Configuration Register"]
pub mod mac_configuration;
#[doc = "MAC Frame Filter\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mac_frame_filter](mac_frame_filter) module"]
pub type MAC_FRAME_FILTER = crate::Reg<u32, _MAC_FRAME_FILTER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAC_FRAME_FILTER;
#[doc = "`read()` method returns [mac_frame_filter::R](mac_frame_filter::R) reader structure"]
impl crate::Readable for MAC_FRAME_FILTER {}
#[doc = "`write(|w| ..)` method takes [mac_frame_filter::W](mac_frame_filter::W) writer structure"]
impl crate::Writable for MAC_FRAME_FILTER {}
#[doc = "MAC Frame Filter"]
pub mod mac_frame_filter;
#[doc = "Hash Table High Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hash_table_high](hash_table_high) module"]
pub type HASH_TABLE_HIGH = crate::Reg<u32, _HASH_TABLE_HIGH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_TABLE_HIGH;
#[doc = "`read()` method returns [hash_table_high::R](hash_table_high::R) reader structure"]
impl crate::Readable for HASH_TABLE_HIGH {}
#[doc = "`write(|w| ..)` method takes [hash_table_high::W](hash_table_high::W) writer structure"]
impl crate::Writable for HASH_TABLE_HIGH {}
#[doc = "Hash Table High Register"]
pub mod hash_table_high;
#[doc = "Hash Table Low Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hash_table_low](hash_table_low) module"]
pub type HASH_TABLE_LOW = crate::Reg<u32, _HASH_TABLE_LOW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_TABLE_LOW;
#[doc = "`read()` method returns [hash_table_low::R](hash_table_low::R) reader structure"]
impl crate::Readable for HASH_TABLE_LOW {}
#[doc = "`write(|w| ..)` method takes [hash_table_low::W](hash_table_low::W) writer structure"]
impl crate::Writable for HASH_TABLE_LOW {}
#[doc = "Hash Table Low Register"]
pub mod hash_table_low;
#[doc = "MII Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gmii_address](gmii_address) module"]
pub type GMII_ADDRESS = crate::Reg<u32, _GMII_ADDRESS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMII_ADDRESS;
#[doc = "`read()` method returns [gmii_address::R](gmii_address::R) reader structure"]
impl crate::Readable for GMII_ADDRESS {}
#[doc = "`write(|w| ..)` method takes [gmii_address::W](gmii_address::W) writer structure"]
impl crate::Writable for GMII_ADDRESS {}
#[doc = "MII Address Register"]
pub mod gmii_address;
#[doc = "MII Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gmii_data](gmii_data) module"]
pub type GMII_DATA = crate::Reg<u32, _GMII_DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMII_DATA;
#[doc = "`read()` method returns [gmii_data::R](gmii_data::R) reader structure"]
impl crate::Readable for GMII_DATA {}
#[doc = "`write(|w| ..)` method takes [gmii_data::W](gmii_data::W) writer structure"]
impl crate::Writable for GMII_DATA {}
#[doc = "MII Data Register"]
pub mod gmii_data;
#[doc = "Flow Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flow_control](flow_control) module"]
pub type FLOW_CONTROL = crate::Reg<u32, _FLOW_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLOW_CONTROL;
#[doc = "`read()` method returns [flow_control::R](flow_control::R) reader structure"]
impl crate::Readable for FLOW_CONTROL {}
#[doc = "`write(|w| ..)` method takes [flow_control::W](flow_control::W) writer structure"]
impl crate::Writable for FLOW_CONTROL {}
#[doc = "Flow Control Register"]
pub mod flow_control;
#[doc = "VLAN Tag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [vlan_tag](vlan_tag) module"]
pub type VLAN_TAG = crate::Reg<u32, _VLAN_TAG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VLAN_TAG;
#[doc = "`read()` method returns [vlan_tag::R](vlan_tag::R) reader structure"]
impl crate::Readable for VLAN_TAG {}
#[doc = "`write(|w| ..)` method takes [vlan_tag::W](vlan_tag::W) writer structure"]
impl crate::Writable for VLAN_TAG {}
#[doc = "VLAN Tag Register"]
pub mod vlan_tag;
#[doc = "Version Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [version](version) module"]
pub type VERSION = crate::Reg<u32, _VERSION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VERSION;
#[doc = "`read()` method returns [version::R](version::R) reader structure"]
impl crate::Readable for VERSION {}
#[doc = "Version Register"]
pub mod version;
#[doc = "Debug Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [debug](debug) module"]
pub type DEBUG = crate::Reg<u32, _DEBUG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEBUG;
#[doc = "`read()` method returns [debug::R](debug::R) reader structure"]
impl crate::Readable for DEBUG {}
#[doc = "Debug Register"]
pub mod debug;
#[doc = "Remote Wake Up Frame Filter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [remote_wake_up_frame_filter](remote_wake_up_frame_filter) module"]
pub type REMOTE_WAKE_UP_FRAME_FILTER = crate::Reg<u32, _REMOTE_WAKE_UP_FRAME_FILTER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REMOTE_WAKE_UP_FRAME_FILTER;
#[doc = "`read()` method returns [remote_wake_up_frame_filter::R](remote_wake_up_frame_filter::R) reader structure"]
impl crate::Readable for REMOTE_WAKE_UP_FRAME_FILTER {}
#[doc = "`write(|w| ..)` method takes [remote_wake_up_frame_filter::W](remote_wake_up_frame_filter::W) writer structure"]
impl crate::Writable for REMOTE_WAKE_UP_FRAME_FILTER {}
#[doc = "Remote Wake Up Frame Filter Register"]
pub mod remote_wake_up_frame_filter;
#[doc = "PMT Control and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pmt_control_status](pmt_control_status) module"]
pub type PMT_CONTROL_STATUS = crate::Reg<u32, _PMT_CONTROL_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMT_CONTROL_STATUS;
#[doc = "`read()` method returns [pmt_control_status::R](pmt_control_status::R) reader structure"]
impl crate::Readable for PMT_CONTROL_STATUS {}
#[doc = "`write(|w| ..)` method takes [pmt_control_status::W](pmt_control_status::W) writer structure"]
impl crate::Writable for PMT_CONTROL_STATUS {}
#[doc = "PMT Control and Status Register"]
pub mod pmt_control_status;
#[doc = "Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [interrupt_status](interrupt_status) module"]
pub type INTERRUPT_STATUS = crate::Reg<u32, _INTERRUPT_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_STATUS;
#[doc = "`read()` method returns [interrupt_status::R](interrupt_status::R) reader structure"]
impl crate::Readable for INTERRUPT_STATUS {}
#[doc = "Interrupt Register"]
pub mod interrupt_status;
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [interrupt_mask](interrupt_mask) module"]
pub type INTERRUPT_MASK = crate::Reg<u32, _INTERRUPT_MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_MASK;
#[doc = "`read()` method returns [interrupt_mask::R](interrupt_mask::R) reader structure"]
impl crate::Readable for INTERRUPT_MASK {}
#[doc = "`write(|w| ..)` method takes [interrupt_mask::W](interrupt_mask::W) writer structure"]
impl crate::Writable for INTERRUPT_MASK {}
#[doc = "Interrupt Mask Register"]
pub mod interrupt_mask;
#[doc = "MAC Address0 High Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mac_address0_high](mac_address0_high) module"]
pub type MAC_ADDRESS0_HIGH = crate::Reg<u32, _MAC_ADDRESS0_HIGH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAC_ADDRESS0_HIGH;
#[doc = "`read()` method returns [mac_address0_high::R](mac_address0_high::R) reader structure"]
impl crate::Readable for MAC_ADDRESS0_HIGH {}
#[doc = "`write(|w| ..)` method takes [mac_address0_high::W](mac_address0_high::W) writer structure"]
impl crate::Writable for MAC_ADDRESS0_HIGH {}
#[doc = "MAC Address0 High Register"]
pub mod mac_address0_high;
#[doc = "MAC Address0 Low Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mac_address0_low](mac_address0_low) module"]
pub type MAC_ADDRESS0_LOW = crate::Reg<u32, _MAC_ADDRESS0_LOW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAC_ADDRESS0_LOW;
#[doc = "`read()` method returns [mac_address0_low::R](mac_address0_low::R) reader structure"]
impl crate::Readable for MAC_ADDRESS0_LOW {}
#[doc = "`write(|w| ..)` method takes [mac_address0_low::W](mac_address0_low::W) writer structure"]
impl crate::Writable for MAC_ADDRESS0_LOW {}
#[doc = "MAC Address0 Low Register"]
pub mod mac_address0_low;
#[doc = "MAC Address1 High Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mac_address1_high](mac_address1_high) module"]
pub type MAC_ADDRESS1_HIGH = crate::Reg<u32, _MAC_ADDRESS1_HIGH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAC_ADDRESS1_HIGH;
#[doc = "`read()` method returns [mac_address1_high::R](mac_address1_high::R) reader structure"]
impl crate::Readable for MAC_ADDRESS1_HIGH {}
#[doc = "`write(|w| ..)` method takes [mac_address1_high::W](mac_address1_high::W) writer structure"]
impl crate::Writable for MAC_ADDRESS1_HIGH {}
#[doc = "MAC Address1 High Register"]
pub mod mac_address1_high;
#[doc = "MAC Address1 Low Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mac_address1_low](mac_address1_low) module"]
pub type MAC_ADDRESS1_LOW = crate::Reg<u32, _MAC_ADDRESS1_LOW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAC_ADDRESS1_LOW;
#[doc = "`read()` method returns [mac_address1_low::R](mac_address1_low::R) reader structure"]
impl crate::Readable for MAC_ADDRESS1_LOW {}
#[doc = "`write(|w| ..)` method takes [mac_address1_low::W](mac_address1_low::W) writer structure"]
impl crate::Writable for MAC_ADDRESS1_LOW {}
#[doc = "MAC Address1 Low Register"]
pub mod mac_address1_low;
#[doc = "MAC Address2 High Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mac_address2_high](mac_address2_high) module"]
pub type MAC_ADDRESS2_HIGH = crate::Reg<u32, _MAC_ADDRESS2_HIGH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAC_ADDRESS2_HIGH;
#[doc = "`read()` method returns [mac_address2_high::R](mac_address2_high::R) reader structure"]
impl crate::Readable for MAC_ADDRESS2_HIGH {}
#[doc = "`write(|w| ..)` method takes [mac_address2_high::W](mac_address2_high::W) writer structure"]
impl crate::Writable for MAC_ADDRESS2_HIGH {}
#[doc = "MAC Address2 High Register"]
pub mod mac_address2_high;
#[doc = "MAC Address2 Low Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mac_address2_low](mac_address2_low) module"]
pub type MAC_ADDRESS2_LOW = crate::Reg<u32, _MAC_ADDRESS2_LOW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAC_ADDRESS2_LOW;
#[doc = "`read()` method returns [mac_address2_low::R](mac_address2_low::R) reader structure"]
impl crate::Readable for MAC_ADDRESS2_LOW {}
#[doc = "`write(|w| ..)` method takes [mac_address2_low::W](mac_address2_low::W) writer structure"]
impl crate::Writable for MAC_ADDRESS2_LOW {}
#[doc = "MAC Address2 Low Register"]
pub mod mac_address2_low;
#[doc = "MAC Address3 High Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mac_address3_high](mac_address3_high) module"]
pub type MAC_ADDRESS3_HIGH = crate::Reg<u32, _MAC_ADDRESS3_HIGH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAC_ADDRESS3_HIGH;
#[doc = "`read()` method returns [mac_address3_high::R](mac_address3_high::R) reader structure"]
impl crate::Readable for MAC_ADDRESS3_HIGH {}
#[doc = "`write(|w| ..)` method takes [mac_address3_high::W](mac_address3_high::W) writer structure"]
impl crate::Writable for MAC_ADDRESS3_HIGH {}
#[doc = "MAC Address3 High Register"]
pub mod mac_address3_high;
#[doc = "MAC Address3 Low Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mac_address3_low](mac_address3_low) module"]
pub type MAC_ADDRESS3_LOW = crate::Reg<u32, _MAC_ADDRESS3_LOW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAC_ADDRESS3_LOW;
#[doc = "`read()` method returns [mac_address3_low::R](mac_address3_low::R) reader structure"]
impl crate::Readable for MAC_ADDRESS3_LOW {}
#[doc = "`write(|w| ..)` method takes [mac_address3_low::W](mac_address3_low::W) writer structure"]
impl crate::Writable for MAC_ADDRESS3_LOW {}
#[doc = "MAC Address3 Low Register"]
pub mod mac_address3_low;
#[doc = "MMC Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mmc_control](mmc_control) module"]
pub type MMC_CONTROL = crate::Reg<u32, _MMC_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MMC_CONTROL;
#[doc = "`read()` method returns [mmc_control::R](mmc_control::R) reader structure"]
impl crate::Readable for MMC_CONTROL {}
#[doc = "`write(|w| ..)` method takes [mmc_control::W](mmc_control::W) writer structure"]
impl crate::Writable for MMC_CONTROL {}
#[doc = "MMC Control Register"]
pub mod mmc_control;
#[doc = "MMC Receive Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mmc_receive_interrupt](mmc_receive_interrupt) module"]
pub type MMC_RECEIVE_INTERRUPT = crate::Reg<u32, _MMC_RECEIVE_INTERRUPT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MMC_RECEIVE_INTERRUPT;
#[doc = "`read()` method returns [mmc_receive_interrupt::R](mmc_receive_interrupt::R) reader structure"]
impl crate::Readable for MMC_RECEIVE_INTERRUPT {}
#[doc = "MMC Receive Interrupt Register"]
pub mod mmc_receive_interrupt;
#[doc = "MMC Transmit Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mmc_transmit_interrupt](mmc_transmit_interrupt) module"]
pub type MMC_TRANSMIT_INTERRUPT = crate::Reg<u32, _MMC_TRANSMIT_INTERRUPT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MMC_TRANSMIT_INTERRUPT;
#[doc = "`read()` method returns [mmc_transmit_interrupt::R](mmc_transmit_interrupt::R) reader structure"]
impl crate::Readable for MMC_TRANSMIT_INTERRUPT {}
#[doc = "MMC Transmit Interrupt Register"]
pub mod mmc_transmit_interrupt;
#[doc = "MMC Reveive Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mmc_receive_interrupt_mask](mmc_receive_interrupt_mask) module"]
pub type MMC_RECEIVE_INTERRUPT_MASK = crate::Reg<u32, _MMC_RECEIVE_INTERRUPT_MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MMC_RECEIVE_INTERRUPT_MASK;
#[doc = "`read()` method returns [mmc_receive_interrupt_mask::R](mmc_receive_interrupt_mask::R) reader structure"]
impl crate::Readable for MMC_RECEIVE_INTERRUPT_MASK {}
#[doc = "`write(|w| ..)` method takes [mmc_receive_interrupt_mask::W](mmc_receive_interrupt_mask::W) writer structure"]
impl crate::Writable for MMC_RECEIVE_INTERRUPT_MASK {}
#[doc = "MMC Reveive Interrupt Mask Register"]
pub mod mmc_receive_interrupt_mask;
#[doc = "MMC Transmit Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mmc_transmit_interrupt_mask](mmc_transmit_interrupt_mask) module"]
pub type MMC_TRANSMIT_INTERRUPT_MASK = crate::Reg<u32, _MMC_TRANSMIT_INTERRUPT_MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MMC_TRANSMIT_INTERRUPT_MASK;
#[doc = "`read()` method returns [mmc_transmit_interrupt_mask::R](mmc_transmit_interrupt_mask::R) reader structure"]
impl crate::Readable for MMC_TRANSMIT_INTERRUPT_MASK {}
#[doc = "`write(|w| ..)` method takes [mmc_transmit_interrupt_mask::W](mmc_transmit_interrupt_mask::W) writer structure"]
impl crate::Writable for MMC_TRANSMIT_INTERRUPT_MASK {}
#[doc = "MMC Transmit Interrupt Mask Register"]
pub mod mmc_transmit_interrupt_mask;
#[doc = "Transmit Octet Count for Good and Bad Frames Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tx_octet_count_good_bad](tx_octet_count_good_bad) module"]
pub type TX_OCTET_COUNT_GOOD_BAD = crate::Reg<u32, _TX_OCTET_COUNT_GOOD_BAD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_OCTET_COUNT_GOOD_BAD;
#[doc = "`read()` method returns [tx_octet_count_good_bad::R](tx_octet_count_good_bad::R) reader structure"]
impl crate::Readable for TX_OCTET_COUNT_GOOD_BAD {}
#[doc = "Transmit Octet Count for Good and Bad Frames Register"]
pub mod tx_octet_count_good_bad;
#[doc = "Transmit Frame Count for Goodand Bad Frames Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tx_frame_count_good_bad](tx_frame_count_good_bad) module"]
pub type TX_FRAME_COUNT_GOOD_BAD = crate::Reg<u32, _TX_FRAME_COUNT_GOOD_BAD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_FRAME_COUNT_GOOD_BAD;
#[doc = "`read()` method returns [tx_frame_count_good_bad::R](tx_frame_count_good_bad::R) reader structure"]
impl crate::Readable for TX_FRAME_COUNT_GOOD_BAD {}
#[doc = "Transmit Frame Count for Goodand Bad Frames Register"]
pub mod tx_frame_count_good_bad;
#[doc = "Transmit Frame Count for Good Broadcast Frames\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tx_broadcast_frames_good](tx_broadcast_frames_good) module"]
pub type TX_BROADCAST_FRAMES_GOOD = crate::Reg<u32, _TX_BROADCAST_FRAMES_GOOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_BROADCAST_FRAMES_GOOD;
#[doc = "`read()` method returns [tx_broadcast_frames_good::R](tx_broadcast_frames_good::R) reader structure"]
impl crate::Readable for TX_BROADCAST_FRAMES_GOOD {}
#[doc = "Transmit Frame Count for Good Broadcast Frames"]
pub mod tx_broadcast_frames_good;
#[doc = "Transmit Frame Count for Good Multicast Frames\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tx_multicast_frames_good](tx_multicast_frames_good) module"]
pub type TX_MULTICAST_FRAMES_GOOD = crate::Reg<u32, _TX_MULTICAST_FRAMES_GOOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_MULTICAST_FRAMES_GOOD;
#[doc = "`read()` method returns [tx_multicast_frames_good::R](tx_multicast_frames_good::R) reader structure"]
impl crate::Readable for TX_MULTICAST_FRAMES_GOOD {}
#[doc = "Transmit Frame Count for Good Multicast Frames"]
pub mod tx_multicast_frames_good;
#[doc = "Transmit Octet Count for Good and Bad 64 Byte Frames\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tx_64octets_frames_good_bad](tx_64octets_frames_good_bad) module"]
pub type TX_64OCTETS_FRAMES_GOOD_BAD = crate::Reg<u32, _TX_64OCTETS_FRAMES_GOOD_BAD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_64OCTETS_FRAMES_GOOD_BAD;
#[doc = "`read()` method returns [tx_64octets_frames_good_bad::R](tx_64octets_frames_good_bad::R) reader structure"]
impl crate::Readable for TX_64OCTETS_FRAMES_GOOD_BAD {}
#[doc = "Transmit Octet Count for Good and Bad 64 Byte Frames"]
pub mod tx_64octets_frames_good_bad;
#[doc = "Transmit Octet Count for Good and Bad 65 to 127 Bytes Frames\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tx_65to127octets_frames_good_bad](tx_65to127octets_frames_good_bad) module"]
pub type TX_65TO127OCTETS_FRAMES_GOOD_BAD = crate::Reg<u32, _TX_65TO127OCTETS_FRAMES_GOOD_BAD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_65TO127OCTETS_FRAMES_GOOD_BAD;
#[doc = "`read()` method returns [tx_65to127octets_frames_good_bad::R](tx_65to127octets_frames_good_bad::R) reader structure"]
impl crate::Readable for TX_65TO127OCTETS_FRAMES_GOOD_BAD {}
#[doc = "Transmit Octet Count for Good and Bad 65 to 127 Bytes Frames"]
pub mod tx_65to127octets_frames_good_bad;
#[doc = "Transmit Octet Count for Good and Bad 128 to 255 Bytes Frames\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tx_128to255octets_frames_good_bad](tx_128to255octets_frames_good_bad) module"]
pub type TX_128TO255OCTETS_FRAMES_GOOD_BAD = crate::Reg<u32, _TX_128TO255OCTETS_FRAMES_GOOD_BAD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_128TO255OCTETS_FRAMES_GOOD_BAD;
#[doc = "`read()` method returns [tx_128to255octets_frames_good_bad::R](tx_128to255octets_frames_good_bad::R) reader structure"]
impl crate::Readable for TX_128TO255OCTETS_FRAMES_GOOD_BAD {}
#[doc = "Transmit Octet Count for Good and Bad 128 to 255 Bytes Frames"]
pub mod tx_128to255octets_frames_good_bad;
#[doc = "Transmit Octet Count for Good and Bad 256 to 511 Bytes Frames\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tx_256to511octets_frames_good_bad](tx_256to511octets_frames_good_bad) module"]
pub type TX_256TO511OCTETS_FRAMES_GOOD_BAD = crate::Reg<u32, _TX_256TO511OCTETS_FRAMES_GOOD_BAD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_256TO511OCTETS_FRAMES_GOOD_BAD;
#[doc = "`read()` method returns [tx_256to511octets_frames_good_bad::R](tx_256to511octets_frames_good_bad::R) reader structure"]
impl crate::Readable for TX_256TO511OCTETS_FRAMES_GOOD_BAD {}
#[doc = "Transmit Octet Count for Good and Bad 256 to 511 Bytes Frames"]
pub mod tx_256to511octets_frames_good_bad;
#[doc = "Transmit Octet Count for Good and Bad 512 to 1023 Bytes Frames\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tx_512to1023octets_frames_good_bad](tx_512to1023octets_frames_good_bad) module"]
pub type TX_512TO1023OCTETS_FRAMES_GOOD_BAD = crate::Reg<u32, _TX_512TO1023OCTETS_FRAMES_GOOD_BAD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_512TO1023OCTETS_FRAMES_GOOD_BAD;
#[doc = "`read()` method returns [tx_512to1023octets_frames_good_bad::R](tx_512to1023octets_frames_good_bad::R) reader structure"]
impl crate::Readable for TX_512TO1023OCTETS_FRAMES_GOOD_BAD {}
#[doc = "Transmit Octet Count for Good and Bad 512 to 1023 Bytes Frames"]
pub mod tx_512to1023octets_frames_good_bad;
#[doc = "Transmit Octet Count for Good and Bad 1024 to Maxsize Bytes Frames\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tx_1024tomaxoctets_frames_good_bad](tx_1024tomaxoctets_frames_good_bad) module"]
pub type TX_1024TOMAXOCTETS_FRAMES_GOOD_BAD = crate::Reg<u32, _TX_1024TOMAXOCTETS_FRAMES_GOOD_BAD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_1024TOMAXOCTETS_FRAMES_GOOD_BAD;
#[doc = "`read()` method returns [tx_1024tomaxoctets_frames_good_bad::R](tx_1024tomaxoctets_frames_good_bad::R) reader structure"]
impl crate::Readable for TX_1024TOMAXOCTETS_FRAMES_GOOD_BAD {}
#[doc = "Transmit Octet Count for Good and Bad 1024 to Maxsize Bytes Frames"]
pub mod tx_1024tomaxoctets_frames_good_bad;
#[doc = "Transmit Frame Count for Good and Bad Unicast Frames\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tx_unicast_frames_good_bad](tx_unicast_frames_good_bad) module"]
pub type TX_UNICAST_FRAMES_GOOD_BAD = crate::Reg<u32, _TX_UNICAST_FRAMES_GOOD_BAD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_UNICAST_FRAMES_GOOD_BAD;
#[doc = "`read()` method returns [tx_unicast_frames_good_bad::R](tx_unicast_frames_good_bad::R) reader structure"]
impl crate::Readable for TX_UNICAST_FRAMES_GOOD_BAD {}
#[doc = "Transmit Frame Count for Good and Bad Unicast Frames"]
pub mod tx_unicast_frames_good_bad;
#[doc = "Transmit Frame Count for Good and Bad Multicast Frames\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tx_multicast_frames_good_bad](tx_multicast_frames_good_bad) module"]
pub type TX_MULTICAST_FRAMES_GOOD_BAD = crate::Reg<u32, _TX_MULTICAST_FRAMES_GOOD_BAD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_MULTICAST_FRAMES_GOOD_BAD;
#[doc = "`read()` method returns [tx_multicast_frames_good_bad::R](tx_multicast_frames_good_bad::R) reader structure"]
impl crate::Readable for TX_MULTICAST_FRAMES_GOOD_BAD {}
#[doc = "Transmit Frame Count for Good and Bad Multicast Frames"]
pub mod tx_multicast_frames_good_bad;
#[doc = "Transmit Frame Count for Good and Bad Broadcast Frames\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tx_broadcast_frames_good_bad](tx_broadcast_frames_good_bad) module"]
pub type TX_BROADCAST_FRAMES_GOOD_BAD = crate::Reg<u32, _TX_BROADCAST_FRAMES_GOOD_BAD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_BROADCAST_FRAMES_GOOD_BAD;
#[doc = "`read()` method returns [tx_broadcast_frames_good_bad::R](tx_broadcast_frames_good_bad::R) reader structure"]
impl crate::Readable for TX_BROADCAST_FRAMES_GOOD_BAD {}
#[doc = "Transmit Frame Count for Good and Bad Broadcast Frames"]
pub mod tx_broadcast_frames_good_bad;
#[doc = "Transmit Frame Count for Underflow Error Frames\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tx_underflow_error_frames](tx_underflow_error_frames) module"]
pub type TX_UNDERFLOW_ERROR_FRAMES = crate::Reg<u32, _TX_UNDERFLOW_ERROR_FRAMES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_UNDERFLOW_ERROR_FRAMES;
#[doc = "`read()` method returns [tx_underflow_error_frames::R](tx_underflow_error_frames::R) reader structure"]
impl crate::Readable for TX_UNDERFLOW_ERROR_FRAMES {}
#[doc = "Transmit Frame Count for Underflow Error Frames"]
pub mod tx_underflow_error_frames;
#[doc = "Transmit Frame Count for Frames Transmitted after Single Collision\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tx_single_collision_good_frames](tx_single_collision_good_frames) module"]
pub type TX_SINGLE_COLLISION_GOOD_FRAMES = crate::Reg<u32, _TX_SINGLE_COLLISION_GOOD_FRAMES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_SINGLE_COLLISION_GOOD_FRAMES;
#[doc = "`read()` method returns [tx_single_collision_good_frames::R](tx_single_collision_good_frames::R) reader structure"]
impl crate::Readable for TX_SINGLE_COLLISION_GOOD_FRAMES {}
#[doc = "Transmit Frame Count for Frames Transmitted after Single Collision"]
pub mod tx_single_collision_good_frames;
#[doc = "Transmit Frame Count for Frames Transmitted after Multiple Collision\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tx_multiple_collision_good_frames](tx_multiple_collision_good_frames) module"]
pub type TX_MULTIPLE_COLLISION_GOOD_FRAMES = crate::Reg<u32, _TX_MULTIPLE_COLLISION_GOOD_FRAMES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_MULTIPLE_COLLISION_GOOD_FRAMES;
#[doc = "`read()` method returns [tx_multiple_collision_good_frames::R](tx_multiple_collision_good_frames::R) reader structure"]
impl crate::Readable for TX_MULTIPLE_COLLISION_GOOD_FRAMES {}
#[doc = "Transmit Frame Count for Frames Transmitted after Multiple Collision"]
pub mod tx_multiple_collision_good_frames;
#[doc = "Tx Deferred Frames Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tx_deferred_frames](tx_deferred_frames) module"]
pub type TX_DEFERRED_FRAMES = crate::Reg<u32, _TX_DEFERRED_FRAMES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_DEFERRED_FRAMES;
#[doc = "`read()` method returns [tx_deferred_frames::R](tx_deferred_frames::R) reader structure"]
impl crate::Readable for TX_DEFERRED_FRAMES {}
#[doc = "Tx Deferred Frames Register"]
pub mod tx_deferred_frames;
#[doc = "Transmit Frame Count for Late Collision Error Frames\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tx_late_collision_frames](tx_late_collision_frames) module"]
pub type TX_LATE_COLLISION_FRAMES = crate::Reg<u32, _TX_LATE_COLLISION_FRAMES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_LATE_COLLISION_FRAMES;
#[doc = "`read()` method returns [tx_late_collision_frames::R](tx_late_collision_frames::R) reader structure"]
impl crate::Readable for TX_LATE_COLLISION_FRAMES {}
#[doc = "Transmit Frame Count for Late Collision Error Frames"]
pub mod tx_late_collision_frames;
#[doc = "Transmit Frame Count for Excessive Collision Error Frames\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tx_excessive_collision_frames](tx_excessive_collision_frames) module"]
pub type TX_EXCESSIVE_COLLISION_FRAMES = crate::Reg<u32, _TX_EXCESSIVE_COLLISION_FRAMES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_EXCESSIVE_COLLISION_FRAMES;
#[doc = "`read()` method returns [tx_excessive_collision_frames::R](tx_excessive_collision_frames::R) reader structure"]
impl crate::Readable for TX_EXCESSIVE_COLLISION_FRAMES {}
#[doc = "Transmit Frame Count for Excessive Collision Error Frames"]
pub mod tx_excessive_collision_frames;
#[doc = "Transmit Frame Count for Carrier Sense Error Frames\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tx_carrier_error_frames](tx_carrier_error_frames) module"]
pub type TX_CARRIER_ERROR_FRAMES = crate::Reg<u32, _TX_CARRIER_ERROR_FRAMES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_CARRIER_ERROR_FRAMES;
#[doc = "`read()` method returns [tx_carrier_error_frames::R](tx_carrier_error_frames::R) reader structure"]
impl crate::Readable for TX_CARRIER_ERROR_FRAMES {}
#[doc = "Transmit Frame Count for Carrier Sense Error Frames"]
pub mod tx_carrier_error_frames;
#[doc = "Tx Octet Count Good Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tx_octet_count_good](tx_octet_count_good) module"]
pub type TX_OCTET_COUNT_GOOD = crate::Reg<u32, _TX_OCTET_COUNT_GOOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_OCTET_COUNT_GOOD;
#[doc = "`read()` method returns [tx_octet_count_good::R](tx_octet_count_good::R) reader structure"]
impl crate::Readable for TX_OCTET_COUNT_GOOD {}
#[doc = "Tx Octet Count Good Register"]
pub mod tx_octet_count_good;
#[doc = "Tx Frame Count Good Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tx_frame_count_good](tx_frame_count_good) module"]
pub type TX_FRAME_COUNT_GOOD = crate::Reg<u32, _TX_FRAME_COUNT_GOOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_FRAME_COUNT_GOOD;
#[doc = "`read()` method returns [tx_frame_count_good::R](tx_frame_count_good::R) reader structure"]
impl crate::Readable for TX_FRAME_COUNT_GOOD {}
#[doc = "Tx Frame Count Good Register"]
pub mod tx_frame_count_good;
#[doc = "Transmit Frame Count for Excessive Deferral Error Frames\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tx_excessive_deferral_error](tx_excessive_deferral_error) module"]
pub type TX_EXCESSIVE_DEFERRAL_ERROR = crate::Reg<u32, _TX_EXCESSIVE_DEFERRAL_ERROR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_EXCESSIVE_DEFERRAL_ERROR;
#[doc = "`read()` method returns [tx_excessive_deferral_error::R](tx_excessive_deferral_error::R) reader structure"]
impl crate::Readable for TX_EXCESSIVE_DEFERRAL_ERROR {}
#[doc = "Transmit Frame Count for Excessive Deferral Error Frames"]
pub mod tx_excessive_deferral_error;
#[doc = "Transmit Frame Count for Good PAUSE Frames\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tx_pause_frames](tx_pause_frames) module"]
pub type TX_PAUSE_FRAMES = crate::Reg<u32, _TX_PAUSE_FRAMES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_PAUSE_FRAMES;
#[doc = "`read()` method returns [tx_pause_frames::R](tx_pause_frames::R) reader structure"]
impl crate::Readable for TX_PAUSE_FRAMES {}
#[doc = "Transmit Frame Count for Good PAUSE Frames"]
pub mod tx_pause_frames;
#[doc = "Transmit Frame Count for Good VLAN Frames\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tx_vlan_frames_good](tx_vlan_frames_good) module"]
pub type TX_VLAN_FRAMES_GOOD = crate::Reg<u32, _TX_VLAN_FRAMES_GOOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_VLAN_FRAMES_GOOD;
#[doc = "`read()` method returns [tx_vlan_frames_good::R](tx_vlan_frames_good::R) reader structure"]
impl crate::Readable for TX_VLAN_FRAMES_GOOD {}
#[doc = "Transmit Frame Count for Good VLAN Frames"]
pub mod tx_vlan_frames_good;
#[doc = "Transmit Frame Count for Good Oversize Frames\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tx_osize_frames_good](tx_osize_frames_good) module"]
pub type TX_OSIZE_FRAMES_GOOD = crate::Reg<u32, _TX_OSIZE_FRAMES_GOOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_OSIZE_FRAMES_GOOD;
#[doc = "`read()` method returns [tx_osize_frames_good::R](tx_osize_frames_good::R) reader structure"]
impl crate::Readable for TX_OSIZE_FRAMES_GOOD {}
#[doc = "Transmit Frame Count for Good Oversize Frames"]
pub mod tx_osize_frames_good;
#[doc = "Receive Frame Count for Good and Bad Frames\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rx_frames_count_good_bad](rx_frames_count_good_bad) module"]
pub type RX_FRAMES_COUNT_GOOD_BAD = crate::Reg<u32, _RX_FRAMES_COUNT_GOOD_BAD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_FRAMES_COUNT_GOOD_BAD;
#[doc = "`read()` method returns [rx_frames_count_good_bad::R](rx_frames_count_good_bad::R) reader structure"]
impl crate::Readable for RX_FRAMES_COUNT_GOOD_BAD {}
#[doc = "Receive Frame Count for Good and Bad Frames"]
pub mod rx_frames_count_good_bad;
#[doc = "Receive Octet Count for Good and Bad Frames\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rx_octet_count_good_bad](rx_octet_count_good_bad) module"]
pub type RX_OCTET_COUNT_GOOD_BAD = crate::Reg<u32, _RX_OCTET_COUNT_GOOD_BAD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_OCTET_COUNT_GOOD_BAD;
#[doc = "`read()` method returns [rx_octet_count_good_bad::R](rx_octet_count_good_bad::R) reader structure"]
impl crate::Readable for RX_OCTET_COUNT_GOOD_BAD {}
#[doc = "Receive Octet Count for Good and Bad Frames"]
pub mod rx_octet_count_good_bad;
#[doc = "Rx Octet Count Good Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rx_octet_count_good](rx_octet_count_good) module"]
pub type RX_OCTET_COUNT_GOOD = crate::Reg<u32, _RX_OCTET_COUNT_GOOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_OCTET_COUNT_GOOD;
#[doc = "`read()` method returns [rx_octet_count_good::R](rx_octet_count_good::R) reader structure"]
impl crate::Readable for RX_OCTET_COUNT_GOOD {}
#[doc = "Rx Octet Count Good Register"]
pub mod rx_octet_count_good;
#[doc = "Receive Frame Count for Good Broadcast Frames\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rx_broadcast_frames_good](rx_broadcast_frames_good) module"]
pub type RX_BROADCAST_FRAMES_GOOD = crate::Reg<u32, _RX_BROADCAST_FRAMES_GOOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_BROADCAST_FRAMES_GOOD;
#[doc = "`read()` method returns [rx_broadcast_frames_good::R](rx_broadcast_frames_good::R) reader structure"]
impl crate::Readable for RX_BROADCAST_FRAMES_GOOD {}
#[doc = "Receive Frame Count for Good Broadcast Frames"]
pub mod rx_broadcast_frames_good;
#[doc = "Receive Frame Count for Good Multicast Frames\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rx_multicast_frames_good](rx_multicast_frames_good) module"]
pub type RX_MULTICAST_FRAMES_GOOD = crate::Reg<u32, _RX_MULTICAST_FRAMES_GOOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_MULTICAST_FRAMES_GOOD;
#[doc = "`read()` method returns [rx_multicast_frames_good::R](rx_multicast_frames_good::R) reader structure"]
impl crate::Readable for RX_MULTICAST_FRAMES_GOOD {}
#[doc = "Receive Frame Count for Good Multicast Frames"]
pub mod rx_multicast_frames_good;
#[doc = "Receive Frame Count for CRC Error Frames\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rx_crc_error_frames](rx_crc_error_frames) module"]
pub type RX_CRC_ERROR_FRAMES = crate::Reg<u32, _RX_CRC_ERROR_FRAMES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_CRC_ERROR_FRAMES;
#[doc = "`read()` method returns [rx_crc_error_frames::R](rx_crc_error_frames::R) reader structure"]
impl crate::Readable for RX_CRC_ERROR_FRAMES {}
#[doc = "Receive Frame Count for CRC Error Frames"]
pub mod rx_crc_error_frames;
#[doc = "Receive Frame Count for Alignment Error Frames\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rx_alignment_error_frames](rx_alignment_error_frames) module"]
pub type RX_ALIGNMENT_ERROR_FRAMES = crate::Reg<u32, _RX_ALIGNMENT_ERROR_FRAMES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_ALIGNMENT_ERROR_FRAMES;
#[doc = "`read()` method returns [rx_alignment_error_frames::R](rx_alignment_error_frames::R) reader structure"]
impl crate::Readable for RX_ALIGNMENT_ERROR_FRAMES {}
#[doc = "Receive Frame Count for Alignment Error Frames"]
pub mod rx_alignment_error_frames;
#[doc = "Receive Frame Count for Runt Error Frames\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rx_runt_error_frames](rx_runt_error_frames) module"]
pub type RX_RUNT_ERROR_FRAMES = crate::Reg<u32, _RX_RUNT_ERROR_FRAMES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_RUNT_ERROR_FRAMES;
#[doc = "`read()` method returns [rx_runt_error_frames::R](rx_runt_error_frames::R) reader structure"]
impl crate::Readable for RX_RUNT_ERROR_FRAMES {}
#[doc = "Receive Frame Count for Runt Error Frames"]
pub mod rx_runt_error_frames;
#[doc = "Receive Frame Count for Jabber Error Frames\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rx_jabber_error_frames](rx_jabber_error_frames) module"]
pub type RX_JABBER_ERROR_FRAMES = crate::Reg<u32, _RX_JABBER_ERROR_FRAMES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_JABBER_ERROR_FRAMES;
#[doc = "`read()` method returns [rx_jabber_error_frames::R](rx_jabber_error_frames::R) reader structure"]
impl crate::Readable for RX_JABBER_ERROR_FRAMES {}
#[doc = "Receive Frame Count for Jabber Error Frames"]
pub mod rx_jabber_error_frames;
#[doc = "Receive Frame Count for Undersize Frames\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rx_undersize_frames_good](rx_undersize_frames_good) module"]
pub type RX_UNDERSIZE_FRAMES_GOOD = crate::Reg<u32, _RX_UNDERSIZE_FRAMES_GOOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_UNDERSIZE_FRAMES_GOOD;
#[doc = "`read()` method returns [rx_undersize_frames_good::R](rx_undersize_frames_good::R) reader structure"]
impl crate::Readable for RX_UNDERSIZE_FRAMES_GOOD {}
#[doc = "Receive Frame Count for Undersize Frames"]
pub mod rx_undersize_frames_good;
#[doc = "Rx Oversize Frames Good Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rx_oversize_frames_good](rx_oversize_frames_good) module"]
pub type RX_OVERSIZE_FRAMES_GOOD = crate::Reg<u32, _RX_OVERSIZE_FRAMES_GOOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_OVERSIZE_FRAMES_GOOD;
#[doc = "`read()` method returns [rx_oversize_frames_good::R](rx_oversize_frames_good::R) reader structure"]
impl crate::Readable for RX_OVERSIZE_FRAMES_GOOD {}
#[doc = "Rx Oversize Frames Good Register"]
pub mod rx_oversize_frames_good;
#[doc = "Receive Frame Count for Good and Bad 64 Byte Frames\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rx_64octets_frames_good_bad](rx_64octets_frames_good_bad) module"]
pub type RX_64OCTETS_FRAMES_GOOD_BAD = crate::Reg<u32, _RX_64OCTETS_FRAMES_GOOD_BAD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_64OCTETS_FRAMES_GOOD_BAD;
#[doc = "`read()` method returns [rx_64octets_frames_good_bad::R](rx_64octets_frames_good_bad::R) reader structure"]
impl crate::Readable for RX_64OCTETS_FRAMES_GOOD_BAD {}
#[doc = "Receive Frame Count for Good and Bad 64 Byte Frames"]
pub mod rx_64octets_frames_good_bad;
#[doc = "Receive Frame Count for Good and Bad 65 to 127 Bytes Frames\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rx_65to127octets_frames_good_bad](rx_65to127octets_frames_good_bad) module"]
pub type RX_65TO127OCTETS_FRAMES_GOOD_BAD = crate::Reg<u32, _RX_65TO127OCTETS_FRAMES_GOOD_BAD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_65TO127OCTETS_FRAMES_GOOD_BAD;
#[doc = "`read()` method returns [rx_65to127octets_frames_good_bad::R](rx_65to127octets_frames_good_bad::R) reader structure"]
impl crate::Readable for RX_65TO127OCTETS_FRAMES_GOOD_BAD {}
#[doc = "Receive Frame Count for Good and Bad 65 to 127 Bytes Frames"]
pub mod rx_65to127octets_frames_good_bad;
#[doc = "Receive Frame Count for Good and Bad 128 to 255 Bytes Frames\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rx_128to255octets_frames_good_bad](rx_128to255octets_frames_good_bad) module"]
pub type RX_128TO255OCTETS_FRAMES_GOOD_BAD = crate::Reg<u32, _RX_128TO255OCTETS_FRAMES_GOOD_BAD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_128TO255OCTETS_FRAMES_GOOD_BAD;
#[doc = "`read()` method returns [rx_128to255octets_frames_good_bad::R](rx_128to255octets_frames_good_bad::R) reader structure"]
impl crate::Readable for RX_128TO255OCTETS_FRAMES_GOOD_BAD {}
#[doc = "Receive Frame Count for Good and Bad 128 to 255 Bytes Frames"]
pub mod rx_128to255octets_frames_good_bad;
#[doc = "Receive Frame Count for Good and Bad 256 to 511 Bytes Frames\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rx_256to511octets_frames_good_bad](rx_256to511octets_frames_good_bad) module"]
pub type RX_256TO511OCTETS_FRAMES_GOOD_BAD = crate::Reg<u32, _RX_256TO511OCTETS_FRAMES_GOOD_BAD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_256TO511OCTETS_FRAMES_GOOD_BAD;
#[doc = "`read()` method returns [rx_256to511octets_frames_good_bad::R](rx_256to511octets_frames_good_bad::R) reader structure"]
impl crate::Readable for RX_256TO511OCTETS_FRAMES_GOOD_BAD {}
#[doc = "Receive Frame Count for Good and Bad 256 to 511 Bytes Frames"]
pub mod rx_256to511octets_frames_good_bad;
#[doc = "Receive Frame Count for Good and Bad 512 to 1,023 Bytes Frames\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rx_512to1023octets_frames_good_bad](rx_512to1023octets_frames_good_bad) module"]
pub type RX_512TO1023OCTETS_FRAMES_GOOD_BAD = crate::Reg<u32, _RX_512TO1023OCTETS_FRAMES_GOOD_BAD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_512TO1023OCTETS_FRAMES_GOOD_BAD;
#[doc = "`read()` method returns [rx_512to1023octets_frames_good_bad::R](rx_512to1023octets_frames_good_bad::R) reader structure"]
impl crate::Readable for RX_512TO1023OCTETS_FRAMES_GOOD_BAD {}
#[doc = "Receive Frame Count for Good and Bad 512 to 1,023 Bytes Frames"]
pub mod rx_512to1023octets_frames_good_bad;
#[doc = "Receive Frame Count for Good and Bad 1,024 to Maxsize Bytes Frames\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rx_1024tomaxoctets_frames_good_bad](rx_1024tomaxoctets_frames_good_bad) module"]
pub type RX_1024TOMAXOCTETS_FRAMES_GOOD_BAD = crate::Reg<u32, _RX_1024TOMAXOCTETS_FRAMES_GOOD_BAD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_1024TOMAXOCTETS_FRAMES_GOOD_BAD;
#[doc = "`read()` method returns [rx_1024tomaxoctets_frames_good_bad::R](rx_1024tomaxoctets_frames_good_bad::R) reader structure"]
impl crate::Readable for RX_1024TOMAXOCTETS_FRAMES_GOOD_BAD {}
#[doc = "Receive Frame Count for Good and Bad 1,024 to Maxsize Bytes Frames"]
pub mod rx_1024tomaxoctets_frames_good_bad;
#[doc = "Receive Frame Count for Good Unicast Frames\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rx_unicast_frames_good](rx_unicast_frames_good) module"]
pub type RX_UNICAST_FRAMES_GOOD = crate::Reg<u32, _RX_UNICAST_FRAMES_GOOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_UNICAST_FRAMES_GOOD;
#[doc = "`read()` method returns [rx_unicast_frames_good::R](rx_unicast_frames_good::R) reader structure"]
impl crate::Readable for RX_UNICAST_FRAMES_GOOD {}
#[doc = "Receive Frame Count for Good Unicast Frames"]
pub mod rx_unicast_frames_good;
#[doc = "Receive Frame Count for Length Error Frames\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rx_length_error_frames](rx_length_error_frames) module"]
pub type RX_LENGTH_ERROR_FRAMES = crate::Reg<u32, _RX_LENGTH_ERROR_FRAMES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_LENGTH_ERROR_FRAMES;
#[doc = "`read()` method returns [rx_length_error_frames::R](rx_length_error_frames::R) reader structure"]
impl crate::Readable for RX_LENGTH_ERROR_FRAMES {}
#[doc = "Receive Frame Count for Length Error Frames"]
pub mod rx_length_error_frames;
#[doc = "Receive Frame Count for Out of Range Frames\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rx_out_of_range_type_frames](rx_out_of_range_type_frames) module"]
pub type RX_OUT_OF_RANGE_TYPE_FRAMES = crate::Reg<u32, _RX_OUT_OF_RANGE_TYPE_FRAMES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_OUT_OF_RANGE_TYPE_FRAMES;
#[doc = "`read()` method returns [rx_out_of_range_type_frames::R](rx_out_of_range_type_frames::R) reader structure"]
impl crate::Readable for RX_OUT_OF_RANGE_TYPE_FRAMES {}
#[doc = "Receive Frame Count for Out of Range Frames"]
pub mod rx_out_of_range_type_frames;
#[doc = "Receive Frame Count for PAUSE Frames\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rx_pause_frames](rx_pause_frames) module"]
pub type RX_PAUSE_FRAMES = crate::Reg<u32, _RX_PAUSE_FRAMES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_PAUSE_FRAMES;
#[doc = "`read()` method returns [rx_pause_frames::R](rx_pause_frames::R) reader structure"]
impl crate::Readable for RX_PAUSE_FRAMES {}
#[doc = "Receive Frame Count for PAUSE Frames"]
pub mod rx_pause_frames;
#[doc = "Receive Frame Count for FIFO Overflow Frames\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rx_fifo_overflow_frames](rx_fifo_overflow_frames) module"]
pub type RX_FIFO_OVERFLOW_FRAMES = crate::Reg<u32, _RX_FIFO_OVERFLOW_FRAMES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_FIFO_OVERFLOW_FRAMES;
#[doc = "`read()` method returns [rx_fifo_overflow_frames::R](rx_fifo_overflow_frames::R) reader structure"]
impl crate::Readable for RX_FIFO_OVERFLOW_FRAMES {}
#[doc = "Receive Frame Count for FIFO Overflow Frames"]
pub mod rx_fifo_overflow_frames;
#[doc = "Receive Frame Count for Good and Bad VLAN Frames\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rx_vlan_frames_good_bad](rx_vlan_frames_good_bad) module"]
pub type RX_VLAN_FRAMES_GOOD_BAD = crate::Reg<u32, _RX_VLAN_FRAMES_GOOD_BAD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_VLAN_FRAMES_GOOD_BAD;
#[doc = "`read()` method returns [rx_vlan_frames_good_bad::R](rx_vlan_frames_good_bad::R) reader structure"]
impl crate::Readable for RX_VLAN_FRAMES_GOOD_BAD {}
#[doc = "Receive Frame Count for Good and Bad VLAN Frames"]
pub mod rx_vlan_frames_good_bad;
#[doc = "Receive Frame Count for Watchdog Error Frames\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rx_watchdog_error_frames](rx_watchdog_error_frames) module"]
pub type RX_WATCHDOG_ERROR_FRAMES = crate::Reg<u32, _RX_WATCHDOG_ERROR_FRAMES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_WATCHDOG_ERROR_FRAMES;
#[doc = "`read()` method returns [rx_watchdog_error_frames::R](rx_watchdog_error_frames::R) reader structure"]
impl crate::Readable for RX_WATCHDOG_ERROR_FRAMES {}
#[doc = "Receive Frame Count for Watchdog Error Frames"]
pub mod rx_watchdog_error_frames;
#[doc = "Receive Frame Count for Receive Error Frames\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rx_receive_error_frames](rx_receive_error_frames) module"]
pub type RX_RECEIVE_ERROR_FRAMES = crate::Reg<u32, _RX_RECEIVE_ERROR_FRAMES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_RECEIVE_ERROR_FRAMES;
#[doc = "`read()` method returns [rx_receive_error_frames::R](rx_receive_error_frames::R) reader structure"]
impl crate::Readable for RX_RECEIVE_ERROR_FRAMES {}
#[doc = "Receive Frame Count for Receive Error Frames"]
pub mod rx_receive_error_frames;
#[doc = "Receive Frame Count for Good Control Frames Frames\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rx_control_frames_good](rx_control_frames_good) module"]
pub type RX_CONTROL_FRAMES_GOOD = crate::Reg<u32, _RX_CONTROL_FRAMES_GOOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_CONTROL_FRAMES_GOOD;
#[doc = "`read()` method returns [rx_control_frames_good::R](rx_control_frames_good::R) reader structure"]
impl crate::Readable for RX_CONTROL_FRAMES_GOOD {}
#[doc = "Receive Frame Count for Good Control Frames Frames"]
pub mod rx_control_frames_good;
#[doc = "MMC Receive Checksum Offload Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mmc_ipc_receive_interrupt_mask](mmc_ipc_receive_interrupt_mask) module"]
pub type MMC_IPC_RECEIVE_INTERRUPT_MASK = crate::Reg<u32, _MMC_IPC_RECEIVE_INTERRUPT_MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MMC_IPC_RECEIVE_INTERRUPT_MASK;
#[doc = "`read()` method returns [mmc_ipc_receive_interrupt_mask::R](mmc_ipc_receive_interrupt_mask::R) reader structure"]
impl crate::Readable for MMC_IPC_RECEIVE_INTERRUPT_MASK {}
#[doc = "`write(|w| ..)` method takes [mmc_ipc_receive_interrupt_mask::W](mmc_ipc_receive_interrupt_mask::W) writer structure"]
impl crate::Writable for MMC_IPC_RECEIVE_INTERRUPT_MASK {}
#[doc = "MMC Receive Checksum Offload Interrupt Mask Register"]
pub mod mmc_ipc_receive_interrupt_mask;
#[doc = "MMC Receive Checksum Offload Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mmc_ipc_receive_interrupt](mmc_ipc_receive_interrupt) module"]
pub type MMC_IPC_RECEIVE_INTERRUPT = crate::Reg<u32, _MMC_IPC_RECEIVE_INTERRUPT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MMC_IPC_RECEIVE_INTERRUPT;
#[doc = "`read()` method returns [mmc_ipc_receive_interrupt::R](mmc_ipc_receive_interrupt::R) reader structure"]
impl crate::Readable for MMC_IPC_RECEIVE_INTERRUPT {}
#[doc = "MMC Receive Checksum Offload Interrupt Register"]
pub mod mmc_ipc_receive_interrupt;
#[doc = "RxIPv4 Good Frames Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxipv4_good_frames](rxipv4_good_frames) module"]
pub type RXIPV4_GOOD_FRAMES = crate::Reg<u32, _RXIPV4_GOOD_FRAMES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXIPV4_GOOD_FRAMES;
#[doc = "`read()` method returns [rxipv4_good_frames::R](rxipv4_good_frames::R) reader structure"]
impl crate::Readable for RXIPV4_GOOD_FRAMES {}
#[doc = "RxIPv4 Good Frames Register"]
pub mod rxipv4_good_frames;
#[doc = "Receive IPV4 Header Error Frame Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxipv4_header_error_frames](rxipv4_header_error_frames) module"]
pub type RXIPV4_HEADER_ERROR_FRAMES = crate::Reg<u32, _RXIPV4_HEADER_ERROR_FRAMES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXIPV4_HEADER_ERROR_FRAMES;
#[doc = "`read()` method returns [rxipv4_header_error_frames::R](rxipv4_header_error_frames::R) reader structure"]
impl crate::Readable for RXIPV4_HEADER_ERROR_FRAMES {}
#[doc = "Receive IPV4 Header Error Frame Counter Register"]
pub mod rxipv4_header_error_frames;
#[doc = "Receive IPV4 No Payload Frame Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxipv4_no_payload_frames](rxipv4_no_payload_frames) module"]
pub type RXIPV4_NO_PAYLOAD_FRAMES = crate::Reg<u32, _RXIPV4_NO_PAYLOAD_FRAMES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXIPV4_NO_PAYLOAD_FRAMES;
#[doc = "`read()` method returns [rxipv4_no_payload_frames::R](rxipv4_no_payload_frames::R) reader structure"]
impl crate::Readable for RXIPV4_NO_PAYLOAD_FRAMES {}
#[doc = "Receive IPV4 No Payload Frame Counter Register"]
pub mod rxipv4_no_payload_frames;
#[doc = "Receive IPV4 Fragmented Frame Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxipv4_fragmented_frames](rxipv4_fragmented_frames) module"]
pub type RXIPV4_FRAGMENTED_FRAMES = crate::Reg<u32, _RXIPV4_FRAGMENTED_FRAMES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXIPV4_FRAGMENTED_FRAMES;
#[doc = "`read()` method returns [rxipv4_fragmented_frames::R](rxipv4_fragmented_frames::R) reader structure"]
impl crate::Readable for RXIPV4_FRAGMENTED_FRAMES {}
#[doc = "Receive IPV4 Fragmented Frame Counter Register"]
pub mod rxipv4_fragmented_frames;
#[doc = "Receive IPV4 UDP Checksum Disabled Frame Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxipv4_udp_checksum_disabled_frames](rxipv4_udp_checksum_disabled_frames) module"]
pub type RXIPV4_UDP_CHECKSUM_DISABLED_FRAMES = crate::Reg<u32, _RXIPV4_UDP_CHECKSUM_DISABLED_FRAMES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXIPV4_UDP_CHECKSUM_DISABLED_FRAMES;
#[doc = "`read()` method returns [rxipv4_udp_checksum_disabled_frames::R](rxipv4_udp_checksum_disabled_frames::R) reader structure"]
impl crate::Readable for RXIPV4_UDP_CHECKSUM_DISABLED_FRAMES {}
#[doc = "Receive IPV4 UDP Checksum Disabled Frame Counter Register"]
pub mod rxipv4_udp_checksum_disabled_frames;
#[doc = "RxIPv6 Good Frames Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxipv6_good_frames](rxipv6_good_frames) module"]
pub type RXIPV6_GOOD_FRAMES = crate::Reg<u32, _RXIPV6_GOOD_FRAMES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXIPV6_GOOD_FRAMES;
#[doc = "`read()` method returns [rxipv6_good_frames::R](rxipv6_good_frames::R) reader structure"]
impl crate::Readable for RXIPV6_GOOD_FRAMES {}
#[doc = "RxIPv6 Good Frames Register"]
pub mod rxipv6_good_frames;
#[doc = "Receive IPV6 Header Error Frame Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxipv6_header_error_frames](rxipv6_header_error_frames) module"]
pub type RXIPV6_HEADER_ERROR_FRAMES = crate::Reg<u32, _RXIPV6_HEADER_ERROR_FRAMES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXIPV6_HEADER_ERROR_FRAMES;
#[doc = "`read()` method returns [rxipv6_header_error_frames::R](rxipv6_header_error_frames::R) reader structure"]
impl crate::Readable for RXIPV6_HEADER_ERROR_FRAMES {}
#[doc = "Receive IPV6 Header Error Frame Counter Register"]
pub mod rxipv6_header_error_frames;
#[doc = "Receive IPV6 No Payload Frame Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxipv6_no_payload_frames](rxipv6_no_payload_frames) module"]
pub type RXIPV6_NO_PAYLOAD_FRAMES = crate::Reg<u32, _RXIPV6_NO_PAYLOAD_FRAMES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXIPV6_NO_PAYLOAD_FRAMES;
#[doc = "`read()` method returns [rxipv6_no_payload_frames::R](rxipv6_no_payload_frames::R) reader structure"]
impl crate::Readable for RXIPV6_NO_PAYLOAD_FRAMES {}
#[doc = "Receive IPV6 No Payload Frame Counter Register"]
pub mod rxipv6_no_payload_frames;
#[doc = "RxUDP Good Frames Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxudp_good_frames](rxudp_good_frames) module"]
pub type RXUDP_GOOD_FRAMES = crate::Reg<u32, _RXUDP_GOOD_FRAMES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXUDP_GOOD_FRAMES;
#[doc = "`read()` method returns [rxudp_good_frames::R](rxudp_good_frames::R) reader structure"]
impl crate::Readable for RXUDP_GOOD_FRAMES {}
#[doc = "RxUDP Good Frames Register"]
pub mod rxudp_good_frames;
#[doc = "RxUDP Error Frames Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxudp_error_frames](rxudp_error_frames) module"]
pub type RXUDP_ERROR_FRAMES = crate::Reg<u32, _RXUDP_ERROR_FRAMES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXUDP_ERROR_FRAMES;
#[doc = "`read()` method returns [rxudp_error_frames::R](rxudp_error_frames::R) reader structure"]
impl crate::Readable for RXUDP_ERROR_FRAMES {}
#[doc = "RxUDP Error Frames Register"]
pub mod rxudp_error_frames;
#[doc = "RxTCP Good Frames Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxtcp_good_frames](rxtcp_good_frames) module"]
pub type RXTCP_GOOD_FRAMES = crate::Reg<u32, _RXTCP_GOOD_FRAMES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXTCP_GOOD_FRAMES;
#[doc = "`read()` method returns [rxtcp_good_frames::R](rxtcp_good_frames::R) reader structure"]
impl crate::Readable for RXTCP_GOOD_FRAMES {}
#[doc = "RxTCP Good Frames Register"]
pub mod rxtcp_good_frames;
#[doc = "RxTCP Error Frames Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxtcp_error_frames](rxtcp_error_frames) module"]
pub type RXTCP_ERROR_FRAMES = crate::Reg<u32, _RXTCP_ERROR_FRAMES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXTCP_ERROR_FRAMES;
#[doc = "`read()` method returns [rxtcp_error_frames::R](rxtcp_error_frames::R) reader structure"]
impl crate::Readable for RXTCP_ERROR_FRAMES {}
#[doc = "RxTCP Error Frames Register"]
pub mod rxtcp_error_frames;
#[doc = "RxICMP Good Frames Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxicmp_good_frames](rxicmp_good_frames) module"]
pub type RXICMP_GOOD_FRAMES = crate::Reg<u32, _RXICMP_GOOD_FRAMES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXICMP_GOOD_FRAMES;
#[doc = "`read()` method returns [rxicmp_good_frames::R](rxicmp_good_frames::R) reader structure"]
impl crate::Readable for RXICMP_GOOD_FRAMES {}
#[doc = "RxICMP Good Frames Register"]
pub mod rxicmp_good_frames;
#[doc = "RxICMP Error Frames Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxicmp_error_frames](rxicmp_error_frames) module"]
pub type RXICMP_ERROR_FRAMES = crate::Reg<u32, _RXICMP_ERROR_FRAMES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXICMP_ERROR_FRAMES;
#[doc = "`read()` method returns [rxicmp_error_frames::R](rxicmp_error_frames::R) reader structure"]
impl crate::Readable for RXICMP_ERROR_FRAMES {}
#[doc = "RxICMP Error Frames Register"]
pub mod rxicmp_error_frames;
#[doc = "RxIPv4 Good Octets Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxipv4_good_octets](rxipv4_good_octets) module"]
pub type RXIPV4_GOOD_OCTETS = crate::Reg<u32, _RXIPV4_GOOD_OCTETS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXIPV4_GOOD_OCTETS;
#[doc = "`read()` method returns [rxipv4_good_octets::R](rxipv4_good_octets::R) reader structure"]
impl crate::Readable for RXIPV4_GOOD_OCTETS {}
#[doc = "RxIPv4 Good Octets Register"]
pub mod rxipv4_good_octets;
#[doc = "Receive IPV4 Header Error Octet Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxipv4_header_error_octets](rxipv4_header_error_octets) module"]
pub type RXIPV4_HEADER_ERROR_OCTETS = crate::Reg<u32, _RXIPV4_HEADER_ERROR_OCTETS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXIPV4_HEADER_ERROR_OCTETS;
#[doc = "`read()` method returns [rxipv4_header_error_octets::R](rxipv4_header_error_octets::R) reader structure"]
impl crate::Readable for RXIPV4_HEADER_ERROR_OCTETS {}
#[doc = "Receive IPV4 Header Error Octet Counter Register"]
pub mod rxipv4_header_error_octets;
#[doc = "Receive IPV4 No Payload Octet Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxipv4_no_payload_octets](rxipv4_no_payload_octets) module"]
pub type RXIPV4_NO_PAYLOAD_OCTETS = crate::Reg<u32, _RXIPV4_NO_PAYLOAD_OCTETS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXIPV4_NO_PAYLOAD_OCTETS;
#[doc = "`read()` method returns [rxipv4_no_payload_octets::R](rxipv4_no_payload_octets::R) reader structure"]
impl crate::Readable for RXIPV4_NO_PAYLOAD_OCTETS {}
#[doc = "Receive IPV4 No Payload Octet Counter Register"]
pub mod rxipv4_no_payload_octets;
#[doc = "Receive IPV4 Fragmented Octet Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxipv4_fragmented_octets](rxipv4_fragmented_octets) module"]
pub type RXIPV4_FRAGMENTED_OCTETS = crate::Reg<u32, _RXIPV4_FRAGMENTED_OCTETS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXIPV4_FRAGMENTED_OCTETS;
#[doc = "`read()` method returns [rxipv4_fragmented_octets::R](rxipv4_fragmented_octets::R) reader structure"]
impl crate::Readable for RXIPV4_FRAGMENTED_OCTETS {}
#[doc = "Receive IPV4 Fragmented Octet Counter Register"]
pub mod rxipv4_fragmented_octets;
#[doc = "Receive IPV4 Fragmented Octet Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxipv4_udp_checksum_disable_octets](rxipv4_udp_checksum_disable_octets) module"]
pub type RXIPV4_UDP_CHECKSUM_DISABLE_OCTETS = crate::Reg<u32, _RXIPV4_UDP_CHECKSUM_DISABLE_OCTETS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXIPV4_UDP_CHECKSUM_DISABLE_OCTETS;
#[doc = "`read()` method returns [rxipv4_udp_checksum_disable_octets::R](rxipv4_udp_checksum_disable_octets::R) reader structure"]
impl crate::Readable for RXIPV4_UDP_CHECKSUM_DISABLE_OCTETS {}
#[doc = "Receive IPV4 Fragmented Octet Counter Register"]
pub mod rxipv4_udp_checksum_disable_octets;
#[doc = "RxIPv6 Good Octets Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxipv6_good_octets](rxipv6_good_octets) module"]
pub type RXIPV6_GOOD_OCTETS = crate::Reg<u32, _RXIPV6_GOOD_OCTETS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXIPV6_GOOD_OCTETS;
#[doc = "`read()` method returns [rxipv6_good_octets::R](rxipv6_good_octets::R) reader structure"]
impl crate::Readable for RXIPV6_GOOD_OCTETS {}
#[doc = "RxIPv6 Good Octets Register"]
pub mod rxipv6_good_octets;
#[doc = "Receive IPV6 Header Error Octet Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxipv6_header_error_octets](rxipv6_header_error_octets) module"]
pub type RXIPV6_HEADER_ERROR_OCTETS = crate::Reg<u32, _RXIPV6_HEADER_ERROR_OCTETS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXIPV6_HEADER_ERROR_OCTETS;
#[doc = "`read()` method returns [rxipv6_header_error_octets::R](rxipv6_header_error_octets::R) reader structure"]
impl crate::Readable for RXIPV6_HEADER_ERROR_OCTETS {}
#[doc = "Receive IPV6 Header Error Octet Counter Register"]
pub mod rxipv6_header_error_octets;
#[doc = "Receive IPV6 No Payload Octet Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxipv6_no_payload_octets](rxipv6_no_payload_octets) module"]
pub type RXIPV6_NO_PAYLOAD_OCTETS = crate::Reg<u32, _RXIPV6_NO_PAYLOAD_OCTETS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXIPV6_NO_PAYLOAD_OCTETS;
#[doc = "`read()` method returns [rxipv6_no_payload_octets::R](rxipv6_no_payload_octets::R) reader structure"]
impl crate::Readable for RXIPV6_NO_PAYLOAD_OCTETS {}
#[doc = "Receive IPV6 No Payload Octet Counter Register"]
pub mod rxipv6_no_payload_octets;
#[doc = "Receive UDP Good Octets Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxudp_good_octets](rxudp_good_octets) module"]
pub type RXUDP_GOOD_OCTETS = crate::Reg<u32, _RXUDP_GOOD_OCTETS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXUDP_GOOD_OCTETS;
#[doc = "`read()` method returns [rxudp_good_octets::R](rxudp_good_octets::R) reader structure"]
impl crate::Readable for RXUDP_GOOD_OCTETS {}
#[doc = "Receive UDP Good Octets Register"]
pub mod rxudp_good_octets;
#[doc = "Receive UDP Error Octets Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxudp_error_octets](rxudp_error_octets) module"]
pub type RXUDP_ERROR_OCTETS = crate::Reg<u32, _RXUDP_ERROR_OCTETS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXUDP_ERROR_OCTETS;
#[doc = "`read()` method returns [rxudp_error_octets::R](rxudp_error_octets::R) reader structure"]
impl crate::Readable for RXUDP_ERROR_OCTETS {}
#[doc = "Receive UDP Error Octets Register"]
pub mod rxudp_error_octets;
#[doc = "Receive TCP Good Octets Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxtcp_good_octets](rxtcp_good_octets) module"]
pub type RXTCP_GOOD_OCTETS = crate::Reg<u32, _RXTCP_GOOD_OCTETS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXTCP_GOOD_OCTETS;
#[doc = "`read()` method returns [rxtcp_good_octets::R](rxtcp_good_octets::R) reader structure"]
impl crate::Readable for RXTCP_GOOD_OCTETS {}
#[doc = "Receive TCP Good Octets Register"]
pub mod rxtcp_good_octets;
#[doc = "Receive TCP Error Octets Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxtcp_error_octets](rxtcp_error_octets) module"]
pub type RXTCP_ERROR_OCTETS = crate::Reg<u32, _RXTCP_ERROR_OCTETS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXTCP_ERROR_OCTETS;
#[doc = "`read()` method returns [rxtcp_error_octets::R](rxtcp_error_octets::R) reader structure"]
impl crate::Readable for RXTCP_ERROR_OCTETS {}
#[doc = "Receive TCP Error Octets Register"]
pub mod rxtcp_error_octets;
#[doc = "Receive ICMP Good Octets Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxicmp_good_octets](rxicmp_good_octets) module"]
pub type RXICMP_GOOD_OCTETS = crate::Reg<u32, _RXICMP_GOOD_OCTETS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXICMP_GOOD_OCTETS;
#[doc = "`read()` method returns [rxicmp_good_octets::R](rxicmp_good_octets::R) reader structure"]
impl crate::Readable for RXICMP_GOOD_OCTETS {}
#[doc = "Receive ICMP Good Octets Register"]
pub mod rxicmp_good_octets;
#[doc = "Receive ICMP Error Octets Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxicmp_error_octets](rxicmp_error_octets) module"]
pub type RXICMP_ERROR_OCTETS = crate::Reg<u32, _RXICMP_ERROR_OCTETS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXICMP_ERROR_OCTETS;
#[doc = "`read()` method returns [rxicmp_error_octets::R](rxicmp_error_octets::R) reader structure"]
impl crate::Readable for RXICMP_ERROR_OCTETS {}
#[doc = "Receive ICMP Error Octets Register"]
pub mod rxicmp_error_octets;
#[doc = "Timestamp Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timestamp_control](timestamp_control) module"]
pub type TIMESTAMP_CONTROL = crate::Reg<u32, _TIMESTAMP_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMESTAMP_CONTROL;
#[doc = "`read()` method returns [timestamp_control::R](timestamp_control::R) reader structure"]
impl crate::Readable for TIMESTAMP_CONTROL {}
#[doc = "`write(|w| ..)` method takes [timestamp_control::W](timestamp_control::W) writer structure"]
impl crate::Writable for TIMESTAMP_CONTROL {}
#[doc = "Timestamp Control Register"]
pub mod timestamp_control;
#[doc = "Sub-Second Increment Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sub_second_increment](sub_second_increment) module"]
pub type SUB_SECOND_INCREMENT = crate::Reg<u32, _SUB_SECOND_INCREMENT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUB_SECOND_INCREMENT;
#[doc = "`read()` method returns [sub_second_increment::R](sub_second_increment::R) reader structure"]
impl crate::Readable for SUB_SECOND_INCREMENT {}
#[doc = "`write(|w| ..)` method takes [sub_second_increment::W](sub_second_increment::W) writer structure"]
impl crate::Writable for SUB_SECOND_INCREMENT {}
#[doc = "Sub-Second Increment Register"]
pub mod sub_second_increment;
#[doc = "System Time - Seconds Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [system_time_seconds](system_time_seconds) module"]
pub type SYSTEM_TIME_SECONDS = crate::Reg<u32, _SYSTEM_TIME_SECONDS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSTEM_TIME_SECONDS;
#[doc = "`read()` method returns [system_time_seconds::R](system_time_seconds::R) reader structure"]
impl crate::Readable for SYSTEM_TIME_SECONDS {}
#[doc = "System Time - Seconds Register"]
pub mod system_time_seconds;
#[doc = "System Time Nanoseconds Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [system_time_nanoseconds](system_time_nanoseconds) module"]
pub type SYSTEM_TIME_NANOSECONDS = crate::Reg<u32, _SYSTEM_TIME_NANOSECONDS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSTEM_TIME_NANOSECONDS;
#[doc = "`read()` method returns [system_time_nanoseconds::R](system_time_nanoseconds::R) reader structure"]
impl crate::Readable for SYSTEM_TIME_NANOSECONDS {}
#[doc = "System Time Nanoseconds Register"]
pub mod system_time_nanoseconds;
#[doc = "System Time - Seconds Update Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [system_time_seconds_update](system_time_seconds_update) module"]
pub type SYSTEM_TIME_SECONDS_UPDATE = crate::Reg<u32, _SYSTEM_TIME_SECONDS_UPDATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSTEM_TIME_SECONDS_UPDATE;
#[doc = "`read()` method returns [system_time_seconds_update::R](system_time_seconds_update::R) reader structure"]
impl crate::Readable for SYSTEM_TIME_SECONDS_UPDATE {}
#[doc = "`write(|w| ..)` method takes [system_time_seconds_update::W](system_time_seconds_update::W) writer structure"]
impl crate::Writable for SYSTEM_TIME_SECONDS_UPDATE {}
#[doc = "System Time - Seconds Update Register"]
pub mod system_time_seconds_update;
#[doc = "System Time Nanoseconds Update Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [system_time_nanoseconds_update](system_time_nanoseconds_update) module"]
pub type SYSTEM_TIME_NANOSECONDS_UPDATE = crate::Reg<u32, _SYSTEM_TIME_NANOSECONDS_UPDATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSTEM_TIME_NANOSECONDS_UPDATE;
#[doc = "`read()` method returns [system_time_nanoseconds_update::R](system_time_nanoseconds_update::R) reader structure"]
impl crate::Readable for SYSTEM_TIME_NANOSECONDS_UPDATE {}
#[doc = "`write(|w| ..)` method takes [system_time_nanoseconds_update::W](system_time_nanoseconds_update::W) writer structure"]
impl crate::Writable for SYSTEM_TIME_NANOSECONDS_UPDATE {}
#[doc = "System Time Nanoseconds Update Register"]
pub mod system_time_nanoseconds_update;
#[doc = "Timestamp Addend Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timestamp_addend](timestamp_addend) module"]
pub type TIMESTAMP_ADDEND = crate::Reg<u32, _TIMESTAMP_ADDEND>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMESTAMP_ADDEND;
#[doc = "`read()` method returns [timestamp_addend::R](timestamp_addend::R) reader structure"]
impl crate::Readable for TIMESTAMP_ADDEND {}
#[doc = "`write(|w| ..)` method takes [timestamp_addend::W](timestamp_addend::W) writer structure"]
impl crate::Writable for TIMESTAMP_ADDEND {}
#[doc = "Timestamp Addend Register"]
pub mod timestamp_addend;
#[doc = "Target Time Seconds Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [target_time_seconds](target_time_seconds) module"]
pub type TARGET_TIME_SECONDS = crate::Reg<u32, _TARGET_TIME_SECONDS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TARGET_TIME_SECONDS;
#[doc = "`read()` method returns [target_time_seconds::R](target_time_seconds::R) reader structure"]
impl crate::Readable for TARGET_TIME_SECONDS {}
#[doc = "`write(|w| ..)` method takes [target_time_seconds::W](target_time_seconds::W) writer structure"]
impl crate::Writable for TARGET_TIME_SECONDS {}
#[doc = "Target Time Seconds Register"]
pub mod target_time_seconds;
#[doc = "Target Time Nanoseconds Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [target_time_nanoseconds](target_time_nanoseconds) module"]
pub type TARGET_TIME_NANOSECONDS = crate::Reg<u32, _TARGET_TIME_NANOSECONDS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TARGET_TIME_NANOSECONDS;
#[doc = "`read()` method returns [target_time_nanoseconds::R](target_time_nanoseconds::R) reader structure"]
impl crate::Readable for TARGET_TIME_NANOSECONDS {}
#[doc = "`write(|w| ..)` method takes [target_time_nanoseconds::W](target_time_nanoseconds::W) writer structure"]
impl crate::Writable for TARGET_TIME_NANOSECONDS {}
#[doc = "Target Time Nanoseconds Register"]
pub mod target_time_nanoseconds;
#[doc = "System Time - Higher Word Seconds Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [system_time_higher_word_seconds](system_time_higher_word_seconds) module"]
pub type SYSTEM_TIME_HIGHER_WORD_SECONDS = crate::Reg<u32, _SYSTEM_TIME_HIGHER_WORD_SECONDS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSTEM_TIME_HIGHER_WORD_SECONDS;
#[doc = "`read()` method returns [system_time_higher_word_seconds::R](system_time_higher_word_seconds::R) reader structure"]
impl crate::Readable for SYSTEM_TIME_HIGHER_WORD_SECONDS {}
#[doc = "`write(|w| ..)` method takes [system_time_higher_word_seconds::W](system_time_higher_word_seconds::W) writer structure"]
impl crate::Writable for SYSTEM_TIME_HIGHER_WORD_SECONDS {}
#[doc = "System Time - Higher Word Seconds Register"]
pub mod system_time_higher_word_seconds;
#[doc = "Timestamp Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timestamp_status](timestamp_status) module"]
pub type TIMESTAMP_STATUS = crate::Reg<u32, _TIMESTAMP_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMESTAMP_STATUS;
#[doc = "`read()` method returns [timestamp_status::R](timestamp_status::R) reader structure"]
impl crate::Readable for TIMESTAMP_STATUS {}
#[doc = "Timestamp Status Register"]
pub mod timestamp_status;
#[doc = "Bus Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [bus_mode](bus_mode) module"]
pub type BUS_MODE = crate::Reg<u32, _BUS_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUS_MODE;
#[doc = "`read()` method returns [bus_mode::R](bus_mode::R) reader structure"]
impl crate::Readable for BUS_MODE {}
#[doc = "`write(|w| ..)` method takes [bus_mode::W](bus_mode::W) writer structure"]
impl crate::Writable for BUS_MODE {}
#[doc = "Bus Mode Register"]
pub mod bus_mode;
#[doc = "Transmit Poll Demand Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [transmit_poll_demand](transmit_poll_demand) module"]
pub type TRANSMIT_POLL_DEMAND = crate::Reg<u32, _TRANSMIT_POLL_DEMAND>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRANSMIT_POLL_DEMAND;
#[doc = "`read()` method returns [transmit_poll_demand::R](transmit_poll_demand::R) reader structure"]
impl crate::Readable for TRANSMIT_POLL_DEMAND {}
#[doc = "`write(|w| ..)` method takes [transmit_poll_demand::W](transmit_poll_demand::W) writer structure"]
impl crate::Writable for TRANSMIT_POLL_DEMAND {}
#[doc = "Transmit Poll Demand Register"]
pub mod transmit_poll_demand;
#[doc = "Receive Poll Demand Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [receive_poll_demand](receive_poll_demand) module"]
pub type RECEIVE_POLL_DEMAND = crate::Reg<u32, _RECEIVE_POLL_DEMAND>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RECEIVE_POLL_DEMAND;
#[doc = "`read()` method returns [receive_poll_demand::R](receive_poll_demand::R) reader structure"]
impl crate::Readable for RECEIVE_POLL_DEMAND {}
#[doc = "`write(|w| ..)` method takes [receive_poll_demand::W](receive_poll_demand::W) writer structure"]
impl crate::Writable for RECEIVE_POLL_DEMAND {}
#[doc = "Receive Poll Demand Register"]
pub mod receive_poll_demand;
#[doc = "Receive Descriptor Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [receive_descriptor_list_address](receive_descriptor_list_address) module"]
pub type RECEIVE_DESCRIPTOR_LIST_ADDRESS = crate::Reg<u32, _RECEIVE_DESCRIPTOR_LIST_ADDRESS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RECEIVE_DESCRIPTOR_LIST_ADDRESS;
#[doc = "`read()` method returns [receive_descriptor_list_address::R](receive_descriptor_list_address::R) reader structure"]
impl crate::Readable for RECEIVE_DESCRIPTOR_LIST_ADDRESS {}
#[doc = "`write(|w| ..)` method takes [receive_descriptor_list_address::W](receive_descriptor_list_address::W) writer structure"]
impl crate::Writable for RECEIVE_DESCRIPTOR_LIST_ADDRESS {}
#[doc = "Receive Descriptor Address Register"]
pub mod receive_descriptor_list_address;
#[doc = "Transmit descripter Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [transmit_descriptor_list_address](transmit_descriptor_list_address) module"]
pub type TRANSMIT_DESCRIPTOR_LIST_ADDRESS = crate::Reg<u32, _TRANSMIT_DESCRIPTOR_LIST_ADDRESS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRANSMIT_DESCRIPTOR_LIST_ADDRESS;
#[doc = "`read()` method returns [transmit_descriptor_list_address::R](transmit_descriptor_list_address::R) reader structure"]
impl crate::Readable for TRANSMIT_DESCRIPTOR_LIST_ADDRESS {}
#[doc = "`write(|w| ..)` method takes [transmit_descriptor_list_address::W](transmit_descriptor_list_address::W) writer structure"]
impl crate::Writable for TRANSMIT_DESCRIPTOR_LIST_ADDRESS {}
#[doc = "Transmit descripter Address Register"]
pub mod transmit_descriptor_list_address;
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "`write(|w| ..)` method takes [status::W](status::W) writer structure"]
impl crate::Writable for STATUS {}
#[doc = "Status Register"]
pub mod status;
#[doc = "Operation Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [operation_mode](operation_mode) module"]
pub type OPERATION_MODE = crate::Reg<u32, _OPERATION_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPERATION_MODE;
#[doc = "`read()` method returns [operation_mode::R](operation_mode::R) reader structure"]
impl crate::Readable for OPERATION_MODE {}
#[doc = "`write(|w| ..)` method takes [operation_mode::W](operation_mode::W) writer structure"]
impl crate::Writable for OPERATION_MODE {}
#[doc = "Operation Mode Register"]
pub mod operation_mode;
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [interrupt_enable](interrupt_enable) module"]
pub type INTERRUPT_ENABLE = crate::Reg<u32, _INTERRUPT_ENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_ENABLE;
#[doc = "`read()` method returns [interrupt_enable::R](interrupt_enable::R) reader structure"]
impl crate::Readable for INTERRUPT_ENABLE {}
#[doc = "`write(|w| ..)` method takes [interrupt_enable::W](interrupt_enable::W) writer structure"]
impl crate::Writable for INTERRUPT_ENABLE {}
#[doc = "Interrupt Enable Register"]
pub mod interrupt_enable;
#[doc = "Missed Frame and Buffer Overflow Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [missed_frame_and_buffer_overflow_counter](missed_frame_and_buffer_overflow_counter) module"]
pub type MISSED_FRAME_AND_BUFFER_OVERFLOW_COUNTER = crate::Reg<u32, _MISSED_FRAME_AND_BUFFER_OVERFLOW_COUNTER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MISSED_FRAME_AND_BUFFER_OVERFLOW_COUNTER;
#[doc = "`read()` method returns [missed_frame_and_buffer_overflow_counter::R](missed_frame_and_buffer_overflow_counter::R) reader structure"]
impl crate::Readable for MISSED_FRAME_AND_BUFFER_OVERFLOW_COUNTER {}
#[doc = "Missed Frame and Buffer Overflow Counter Register"]
pub mod missed_frame_and_buffer_overflow_counter;
#[doc = "Receive Interrupt Watchdog Timer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [receive_interrupt_watchdog_timer](receive_interrupt_watchdog_timer) module"]
pub type RECEIVE_INTERRUPT_WATCHDOG_TIMER = crate::Reg<u32, _RECEIVE_INTERRUPT_WATCHDOG_TIMER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RECEIVE_INTERRUPT_WATCHDOG_TIMER;
#[doc = "`read()` method returns [receive_interrupt_watchdog_timer::R](receive_interrupt_watchdog_timer::R) reader structure"]
impl crate::Readable for RECEIVE_INTERRUPT_WATCHDOG_TIMER {}
#[doc = "`write(|w| ..)` method takes [receive_interrupt_watchdog_timer::W](receive_interrupt_watchdog_timer::W) writer structure"]
impl crate::Writable for RECEIVE_INTERRUPT_WATCHDOG_TIMER {}
#[doc = "Receive Interrupt Watchdog Timer Register"]
pub mod receive_interrupt_watchdog_timer;
#[doc = "AHB Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ahb_status](ahb_status) module"]
pub type AHB_STATUS = crate::Reg<u32, _AHB_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHB_STATUS;
#[doc = "`read()` method returns [ahb_status::R](ahb_status::R) reader structure"]
impl crate::Readable for AHB_STATUS {}
#[doc = "AHB Status Register"]
pub mod ahb_status;
#[doc = "Current Host Transmit Descriptor Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [current_host_transmit_descriptor](current_host_transmit_descriptor) module"]
pub type CURRENT_HOST_TRANSMIT_DESCRIPTOR = crate::Reg<u32, _CURRENT_HOST_TRANSMIT_DESCRIPTOR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CURRENT_HOST_TRANSMIT_DESCRIPTOR;
#[doc = "`read()` method returns [current_host_transmit_descriptor::R](current_host_transmit_descriptor::R) reader structure"]
impl crate::Readable for CURRENT_HOST_TRANSMIT_DESCRIPTOR {}
#[doc = "Current Host Transmit Descriptor Register"]
pub mod current_host_transmit_descriptor;
#[doc = "Current Host Receive Descriptor Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [current_host_receive_descriptor](current_host_receive_descriptor) module"]
pub type CURRENT_HOST_RECEIVE_DESCRIPTOR = crate::Reg<u32, _CURRENT_HOST_RECEIVE_DESCRIPTOR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CURRENT_HOST_RECEIVE_DESCRIPTOR;
#[doc = "`read()` method returns [current_host_receive_descriptor::R](current_host_receive_descriptor::R) reader structure"]
impl crate::Readable for CURRENT_HOST_RECEIVE_DESCRIPTOR {}
#[doc = "Current Host Receive Descriptor Register"]
pub mod current_host_receive_descriptor;
#[doc = "Current Host Transmit Buffer Address Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [current_host_transmit_buffer_address](current_host_transmit_buffer_address) module"]
pub type CURRENT_HOST_TRANSMIT_BUFFER_ADDRESS = crate::Reg<u32, _CURRENT_HOST_TRANSMIT_BUFFER_ADDRESS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CURRENT_HOST_TRANSMIT_BUFFER_ADDRESS;
#[doc = "`read()` method returns [current_host_transmit_buffer_address::R](current_host_transmit_buffer_address::R) reader structure"]
impl crate::Readable for CURRENT_HOST_TRANSMIT_BUFFER_ADDRESS {}
#[doc = "Current Host Transmit Buffer Address Register"]
pub mod current_host_transmit_buffer_address;
#[doc = "Current Host Receive Buffer Address Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [current_host_receive_buffer_address](current_host_receive_buffer_address) module"]
pub type CURRENT_HOST_RECEIVE_BUFFER_ADDRESS = crate::Reg<u32, _CURRENT_HOST_RECEIVE_BUFFER_ADDRESS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CURRENT_HOST_RECEIVE_BUFFER_ADDRESS;
#[doc = "`read()` method returns [current_host_receive_buffer_address::R](current_host_receive_buffer_address::R) reader structure"]
impl crate::Readable for CURRENT_HOST_RECEIVE_BUFFER_ADDRESS {}
#[doc = "Current Host Receive Buffer Address Register"]
pub mod current_host_receive_buffer_address;
#[doc = "HW Feature Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hw_feature](hw_feature) module"]
pub type HW_FEATURE = crate::Reg<u32, _HW_FEATURE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HW_FEATURE;
#[doc = "`read()` method returns [hw_feature::R](hw_feature::R) reader structure"]
impl crate::Readable for HW_FEATURE {}
#[doc = "`write(|w| ..)` method takes [hw_feature::W](hw_feature::W) writer structure"]
impl crate::Writable for HW_FEATURE {}
#[doc = "HW Feature Register"]
pub mod hw_feature;

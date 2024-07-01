#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    mac_configuration: MAC_CONFIGURATION,
    mac_frame_filter: MAC_FRAME_FILTER,
    hash_table_high: HASH_TABLE_HIGH,
    hash_table_low: HASH_TABLE_LOW,
    gmii_address: GMII_ADDRESS,
    gmii_data: GMII_DATA,
    flow_control: FLOW_CONTROL,
    vlan_tag: VLAN_TAG,
    version: VERSION,
    debug: DEBUG,
    remote_wake_up_frame_filter: REMOTE_WAKE_UP_FRAME_FILTER,
    pmt_control_status: PMT_CONTROL_STATUS,
    _reserved12: [u8; 0x08],
    interrupt_status: INTERRUPT_STATUS,
    interrupt_mask: INTERRUPT_MASK,
    mac_address0_high: MAC_ADDRESS0_HIGH,
    mac_address0_low: MAC_ADDRESS0_LOW,
    mac_address1_high: MAC_ADDRESS1_HIGH,
    mac_address1_low: MAC_ADDRESS1_LOW,
    mac_address2_high: MAC_ADDRESS2_HIGH,
    mac_address2_low: MAC_ADDRESS2_LOW,
    mac_address3_high: MAC_ADDRESS3_HIGH,
    mac_address3_low: MAC_ADDRESS3_LOW,
    _reserved22: [u8; 0xa0],
    mmc_control: MMC_CONTROL,
    mmc_receive_interrupt: MMC_RECEIVE_INTERRUPT,
    mmc_transmit_interrupt: MMC_TRANSMIT_INTERRUPT,
    mmc_receive_interrupt_mask: MMC_RECEIVE_INTERRUPT_MASK,
    mmc_transmit_interrupt_mask: MMC_TRANSMIT_INTERRUPT_MASK,
    tx_octet_count_good_bad: TX_OCTET_COUNT_GOOD_BAD,
    tx_frame_count_good_bad: TX_FRAME_COUNT_GOOD_BAD,
    tx_broadcast_frames_good: TX_BROADCAST_FRAMES_GOOD,
    tx_multicast_frames_good: TX_MULTICAST_FRAMES_GOOD,
    tx_64octets_frames_good_bad: TX_64OCTETS_FRAMES_GOOD_BAD,
    tx_65to127octets_frames_good_bad: TX_65TO127OCTETS_FRAMES_GOOD_BAD,
    tx_128to255octets_frames_good_bad: TX_128TO255OCTETS_FRAMES_GOOD_BAD,
    tx_256to511octets_frames_good_bad: TX_256TO511OCTETS_FRAMES_GOOD_BAD,
    tx_512to1023octets_frames_good_bad: TX_512TO1023OCTETS_FRAMES_GOOD_BAD,
    tx_1024tomaxoctets_frames_good_bad: TX_1024TOMAXOCTETS_FRAMES_GOOD_BAD,
    tx_unicast_frames_good_bad: TX_UNICAST_FRAMES_GOOD_BAD,
    tx_multicast_frames_good_bad: TX_MULTICAST_FRAMES_GOOD_BAD,
    tx_broadcast_frames_good_bad: TX_BROADCAST_FRAMES_GOOD_BAD,
    tx_underflow_error_frames: TX_UNDERFLOW_ERROR_FRAMES,
    tx_single_collision_good_frames: TX_SINGLE_COLLISION_GOOD_FRAMES,
    tx_multiple_collision_good_frames: TX_MULTIPLE_COLLISION_GOOD_FRAMES,
    tx_deferred_frames: TX_DEFERRED_FRAMES,
    tx_late_collision_frames: TX_LATE_COLLISION_FRAMES,
    tx_excessive_collision_frames: TX_EXCESSIVE_COLLISION_FRAMES,
    tx_carrier_error_frames: TX_CARRIER_ERROR_FRAMES,
    tx_octet_count_good: TX_OCTET_COUNT_GOOD,
    tx_frame_count_good: TX_FRAME_COUNT_GOOD,
    tx_excessive_deferral_error: TX_EXCESSIVE_DEFERRAL_ERROR,
    tx_pause_frames: TX_PAUSE_FRAMES,
    tx_vlan_frames_good: TX_VLAN_FRAMES_GOOD,
    tx_osize_frames_good: TX_OSIZE_FRAMES_GOOD,
    _reserved53: [u8; 0x04],
    rx_frames_count_good_bad: RX_FRAMES_COUNT_GOOD_BAD,
    rx_octet_count_good_bad: RX_OCTET_COUNT_GOOD_BAD,
    rx_octet_count_good: RX_OCTET_COUNT_GOOD,
    rx_broadcast_frames_good: RX_BROADCAST_FRAMES_GOOD,
    rx_multicast_frames_good: RX_MULTICAST_FRAMES_GOOD,
    rx_crc_error_frames: RX_CRC_ERROR_FRAMES,
    rx_alignment_error_frames: RX_ALIGNMENT_ERROR_FRAMES,
    rx_runt_error_frames: RX_RUNT_ERROR_FRAMES,
    rx_jabber_error_frames: RX_JABBER_ERROR_FRAMES,
    rx_undersize_frames_good: RX_UNDERSIZE_FRAMES_GOOD,
    rx_oversize_frames_good: RX_OVERSIZE_FRAMES_GOOD,
    rx_64octets_frames_good_bad: RX_64OCTETS_FRAMES_GOOD_BAD,
    rx_65to127octets_frames_good_bad: RX_65TO127OCTETS_FRAMES_GOOD_BAD,
    rx_128to255octets_frames_good_bad: RX_128TO255OCTETS_FRAMES_GOOD_BAD,
    rx_256to511octets_frames_good_bad: RX_256TO511OCTETS_FRAMES_GOOD_BAD,
    rx_512to1023octets_frames_good_bad: RX_512TO1023OCTETS_FRAMES_GOOD_BAD,
    rx_1024tomaxoctets_frames_good_bad: RX_1024TOMAXOCTETS_FRAMES_GOOD_BAD,
    rx_unicast_frames_good: RX_UNICAST_FRAMES_GOOD,
    rx_length_error_frames: RX_LENGTH_ERROR_FRAMES,
    rx_out_of_range_type_frames: RX_OUT_OF_RANGE_TYPE_FRAMES,
    rx_pause_frames: RX_PAUSE_FRAMES,
    rx_fifo_overflow_frames: RX_FIFO_OVERFLOW_FRAMES,
    rx_vlan_frames_good_bad: RX_VLAN_FRAMES_GOOD_BAD,
    rx_watchdog_error_frames: RX_WATCHDOG_ERROR_FRAMES,
    rx_receive_error_frames: RX_RECEIVE_ERROR_FRAMES,
    rx_control_frames_good: RX_CONTROL_FRAMES_GOOD,
    _reserved79: [u8; 0x18],
    mmc_ipc_receive_interrupt_mask: MMC_IPC_RECEIVE_INTERRUPT_MASK,
    _reserved80: [u8; 0x04],
    mmc_ipc_receive_interrupt: MMC_IPC_RECEIVE_INTERRUPT,
    _reserved81: [u8; 0x04],
    rxipv4_good_frames: RXIPV4_GOOD_FRAMES,
    rxipv4_header_error_frames: RXIPV4_HEADER_ERROR_FRAMES,
    rxipv4_no_payload_frames: RXIPV4_NO_PAYLOAD_FRAMES,
    rxipv4_fragmented_frames: RXIPV4_FRAGMENTED_FRAMES,
    rxipv4_udp_checksum_disabled_frames: RXIPV4_UDP_CHECKSUM_DISABLED_FRAMES,
    rxipv6_good_frames: RXIPV6_GOOD_FRAMES,
    rxipv6_header_error_frames: RXIPV6_HEADER_ERROR_FRAMES,
    rxipv6_no_payload_frames: RXIPV6_NO_PAYLOAD_FRAMES,
    rxudp_good_frames: RXUDP_GOOD_FRAMES,
    rxudp_error_frames: RXUDP_ERROR_FRAMES,
    rxtcp_good_frames: RXTCP_GOOD_FRAMES,
    rxtcp_error_frames: RXTCP_ERROR_FRAMES,
    rxicmp_good_frames: RXICMP_GOOD_FRAMES,
    rxicmp_error_frames: RXICMP_ERROR_FRAMES,
    _reserved95: [u8; 0x08],
    rxipv4_good_octets: RXIPV4_GOOD_OCTETS,
    rxipv4_header_error_octets: RXIPV4_HEADER_ERROR_OCTETS,
    rxipv4_no_payload_octets: RXIPV4_NO_PAYLOAD_OCTETS,
    rxipv4_fragmented_octets: RXIPV4_FRAGMENTED_OCTETS,
    rxipv4_udp_checksum_disable_octets: RXIPV4_UDP_CHECKSUM_DISABLE_OCTETS,
    rxipv6_good_octets: RXIPV6_GOOD_OCTETS,
    rxipv6_header_error_octets: RXIPV6_HEADER_ERROR_OCTETS,
    rxipv6_no_payload_octets: RXIPV6_NO_PAYLOAD_OCTETS,
    rxudp_good_octets: RXUDP_GOOD_OCTETS,
    rxudp_error_octets: RXUDP_ERROR_OCTETS,
    rxtcp_good_octets: RXTCP_GOOD_OCTETS,
    rxtcp_error_octets: RXTCP_ERROR_OCTETS,
    rxicmp_good_octets: RXICMP_GOOD_OCTETS,
    rxicmp_error_octets: RXICMP_ERROR_OCTETS,
    _reserved109: [u8; 0x0478],
    timestamp_control: TIMESTAMP_CONTROL,
    sub_second_increment: SUB_SECOND_INCREMENT,
    system_time_seconds: SYSTEM_TIME_SECONDS,
    system_time_nanoseconds: SYSTEM_TIME_NANOSECONDS,
    system_time_seconds_update: SYSTEM_TIME_SECONDS_UPDATE,
    system_time_nanoseconds_update: SYSTEM_TIME_NANOSECONDS_UPDATE,
    timestamp_addend: TIMESTAMP_ADDEND,
    target_time_seconds: TARGET_TIME_SECONDS,
    target_time_nanoseconds: TARGET_TIME_NANOSECONDS,
    system_time_higher_word_seconds: SYSTEM_TIME_HIGHER_WORD_SECONDS,
    timestamp_status: TIMESTAMP_STATUS,
    _reserved120: [u8; 0x08d4],
    bus_mode: BUS_MODE,
    transmit_poll_demand: TRANSMIT_POLL_DEMAND,
    receive_poll_demand: RECEIVE_POLL_DEMAND,
    receive_descriptor_list_address: RECEIVE_DESCRIPTOR_LIST_ADDRESS,
    transmit_descriptor_list_address: TRANSMIT_DESCRIPTOR_LIST_ADDRESS,
    status: STATUS,
    operation_mode: OPERATION_MODE,
    interrupt_enable: INTERRUPT_ENABLE,
    missed_frame_and_buffer_overflow_counter: MISSED_FRAME_AND_BUFFER_OVERFLOW_COUNTER,
    receive_interrupt_watchdog_timer: RECEIVE_INTERRUPT_WATCHDOG_TIMER,
    _reserved130: [u8; 0x04],
    ahb_status: AHB_STATUS,
    _reserved131: [u8; 0x18],
    current_host_transmit_descriptor: CURRENT_HOST_TRANSMIT_DESCRIPTOR,
    current_host_receive_descriptor: CURRENT_HOST_RECEIVE_DESCRIPTOR,
    current_host_transmit_buffer_address: CURRENT_HOST_TRANSMIT_BUFFER_ADDRESS,
    current_host_receive_buffer_address: CURRENT_HOST_RECEIVE_BUFFER_ADDRESS,
    hw_feature: HW_FEATURE,
}
impl RegisterBlock {
    #[doc = "0x00 - MAC Configuration Register"]
    #[inline(always)]
    pub const fn mac_configuration(&self) -> &MAC_CONFIGURATION {
        &self.mac_configuration
    }
    #[doc = "0x04 - MAC Frame Filter"]
    #[inline(always)]
    pub const fn mac_frame_filter(&self) -> &MAC_FRAME_FILTER {
        &self.mac_frame_filter
    }
    #[doc = "0x08 - Hash Table High Register"]
    #[inline(always)]
    pub const fn hash_table_high(&self) -> &HASH_TABLE_HIGH {
        &self.hash_table_high
    }
    #[doc = "0x0c - Hash Table Low Register"]
    #[inline(always)]
    pub const fn hash_table_low(&self) -> &HASH_TABLE_LOW {
        &self.hash_table_low
    }
    #[doc = "0x10 - MII Address Register"]
    #[inline(always)]
    pub const fn gmii_address(&self) -> &GMII_ADDRESS {
        &self.gmii_address
    }
    #[doc = "0x14 - MII Data Register"]
    #[inline(always)]
    pub const fn gmii_data(&self) -> &GMII_DATA {
        &self.gmii_data
    }
    #[doc = "0x18 - Flow Control Register"]
    #[inline(always)]
    pub const fn flow_control(&self) -> &FLOW_CONTROL {
        &self.flow_control
    }
    #[doc = "0x1c - VLAN Tag Register"]
    #[inline(always)]
    pub const fn vlan_tag(&self) -> &VLAN_TAG {
        &self.vlan_tag
    }
    #[doc = "0x20 - Version Register"]
    #[inline(always)]
    pub const fn version(&self) -> &VERSION {
        &self.version
    }
    #[doc = "0x24 - Debug Register"]
    #[inline(always)]
    pub const fn debug(&self) -> &DEBUG {
        &self.debug
    }
    #[doc = "0x28 - Remote Wake Up Frame Filter Register"]
    #[inline(always)]
    pub const fn remote_wake_up_frame_filter(&self) -> &REMOTE_WAKE_UP_FRAME_FILTER {
        &self.remote_wake_up_frame_filter
    }
    #[doc = "0x2c - PMT Control and Status Register"]
    #[inline(always)]
    pub const fn pmt_control_status(&self) -> &PMT_CONTROL_STATUS {
        &self.pmt_control_status
    }
    #[doc = "0x38 - Interrupt Register"]
    #[inline(always)]
    pub const fn interrupt_status(&self) -> &INTERRUPT_STATUS {
        &self.interrupt_status
    }
    #[doc = "0x3c - Interrupt Mask Register"]
    #[inline(always)]
    pub const fn interrupt_mask(&self) -> &INTERRUPT_MASK {
        &self.interrupt_mask
    }
    #[doc = "0x40 - MAC Address0 High Register"]
    #[inline(always)]
    pub const fn mac_address0_high(&self) -> &MAC_ADDRESS0_HIGH {
        &self.mac_address0_high
    }
    #[doc = "0x44 - MAC Address0 Low Register"]
    #[inline(always)]
    pub const fn mac_address0_low(&self) -> &MAC_ADDRESS0_LOW {
        &self.mac_address0_low
    }
    #[doc = "0x48 - MAC Address1 High Register"]
    #[inline(always)]
    pub const fn mac_address1_high(&self) -> &MAC_ADDRESS1_HIGH {
        &self.mac_address1_high
    }
    #[doc = "0x4c - MAC Address1 Low Register"]
    #[inline(always)]
    pub const fn mac_address1_low(&self) -> &MAC_ADDRESS1_LOW {
        &self.mac_address1_low
    }
    #[doc = "0x50 - MAC Address2 High Register"]
    #[inline(always)]
    pub const fn mac_address2_high(&self) -> &MAC_ADDRESS2_HIGH {
        &self.mac_address2_high
    }
    #[doc = "0x54 - MAC Address2 Low Register"]
    #[inline(always)]
    pub const fn mac_address2_low(&self) -> &MAC_ADDRESS2_LOW {
        &self.mac_address2_low
    }
    #[doc = "0x58 - MAC Address3 High Register"]
    #[inline(always)]
    pub const fn mac_address3_high(&self) -> &MAC_ADDRESS3_HIGH {
        &self.mac_address3_high
    }
    #[doc = "0x5c - MAC Address3 Low Register"]
    #[inline(always)]
    pub const fn mac_address3_low(&self) -> &MAC_ADDRESS3_LOW {
        &self.mac_address3_low
    }
    #[doc = "0x100 - MMC Control Register"]
    #[inline(always)]
    pub const fn mmc_control(&self) -> &MMC_CONTROL {
        &self.mmc_control
    }
    #[doc = "0x104 - MMC Receive Interrupt Register"]
    #[inline(always)]
    pub const fn mmc_receive_interrupt(&self) -> &MMC_RECEIVE_INTERRUPT {
        &self.mmc_receive_interrupt
    }
    #[doc = "0x108 - MMC Transmit Interrupt Register"]
    #[inline(always)]
    pub const fn mmc_transmit_interrupt(&self) -> &MMC_TRANSMIT_INTERRUPT {
        &self.mmc_transmit_interrupt
    }
    #[doc = "0x10c - MMC Reveive Interrupt Mask Register"]
    #[inline(always)]
    pub const fn mmc_receive_interrupt_mask(&self) -> &MMC_RECEIVE_INTERRUPT_MASK {
        &self.mmc_receive_interrupt_mask
    }
    #[doc = "0x110 - MMC Transmit Interrupt Mask Register"]
    #[inline(always)]
    pub const fn mmc_transmit_interrupt_mask(&self) -> &MMC_TRANSMIT_INTERRUPT_MASK {
        &self.mmc_transmit_interrupt_mask
    }
    #[doc = "0x114 - Transmit Octet Count for Good and Bad Frames Register"]
    #[inline(always)]
    pub const fn tx_octet_count_good_bad(&self) -> &TX_OCTET_COUNT_GOOD_BAD {
        &self.tx_octet_count_good_bad
    }
    #[doc = "0x118 - Transmit Frame Count for Goodand Bad Frames Register"]
    #[inline(always)]
    pub const fn tx_frame_count_good_bad(&self) -> &TX_FRAME_COUNT_GOOD_BAD {
        &self.tx_frame_count_good_bad
    }
    #[doc = "0x11c - Transmit Frame Count for Good Broadcast Frames"]
    #[inline(always)]
    pub const fn tx_broadcast_frames_good(&self) -> &TX_BROADCAST_FRAMES_GOOD {
        &self.tx_broadcast_frames_good
    }
    #[doc = "0x120 - Transmit Frame Count for Good Multicast Frames"]
    #[inline(always)]
    pub const fn tx_multicast_frames_good(&self) -> &TX_MULTICAST_FRAMES_GOOD {
        &self.tx_multicast_frames_good
    }
    #[doc = "0x124 - Transmit Octet Count for Good and Bad 64 Byte Frames"]
    #[inline(always)]
    pub const fn tx_64octets_frames_good_bad(&self) -> &TX_64OCTETS_FRAMES_GOOD_BAD {
        &self.tx_64octets_frames_good_bad
    }
    #[doc = "0x128 - Transmit Octet Count for Good and Bad 65 to 127 Bytes Frames"]
    #[inline(always)]
    pub const fn tx_65to127octets_frames_good_bad(&self) -> &TX_65TO127OCTETS_FRAMES_GOOD_BAD {
        &self.tx_65to127octets_frames_good_bad
    }
    #[doc = "0x12c - Transmit Octet Count for Good and Bad 128 to 255 Bytes Frames"]
    #[inline(always)]
    pub const fn tx_128to255octets_frames_good_bad(&self) -> &TX_128TO255OCTETS_FRAMES_GOOD_BAD {
        &self.tx_128to255octets_frames_good_bad
    }
    #[doc = "0x130 - Transmit Octet Count for Good and Bad 256 to 511 Bytes Frames"]
    #[inline(always)]
    pub const fn tx_256to511octets_frames_good_bad(&self) -> &TX_256TO511OCTETS_FRAMES_GOOD_BAD {
        &self.tx_256to511octets_frames_good_bad
    }
    #[doc = "0x134 - Transmit Octet Count for Good and Bad 512 to 1023 Bytes Frames"]
    #[inline(always)]
    pub const fn tx_512to1023octets_frames_good_bad(&self) -> &TX_512TO1023OCTETS_FRAMES_GOOD_BAD {
        &self.tx_512to1023octets_frames_good_bad
    }
    #[doc = "0x138 - Transmit Octet Count for Good and Bad 1024 to Maxsize Bytes Frames"]
    #[inline(always)]
    pub const fn tx_1024tomaxoctets_frames_good_bad(&self) -> &TX_1024TOMAXOCTETS_FRAMES_GOOD_BAD {
        &self.tx_1024tomaxoctets_frames_good_bad
    }
    #[doc = "0x13c - Transmit Frame Count for Good and Bad Unicast Frames"]
    #[inline(always)]
    pub const fn tx_unicast_frames_good_bad(&self) -> &TX_UNICAST_FRAMES_GOOD_BAD {
        &self.tx_unicast_frames_good_bad
    }
    #[doc = "0x140 - Transmit Frame Count for Good and Bad Multicast Frames"]
    #[inline(always)]
    pub const fn tx_multicast_frames_good_bad(&self) -> &TX_MULTICAST_FRAMES_GOOD_BAD {
        &self.tx_multicast_frames_good_bad
    }
    #[doc = "0x144 - Transmit Frame Count for Good and Bad Broadcast Frames"]
    #[inline(always)]
    pub const fn tx_broadcast_frames_good_bad(&self) -> &TX_BROADCAST_FRAMES_GOOD_BAD {
        &self.tx_broadcast_frames_good_bad
    }
    #[doc = "0x148 - Transmit Frame Count for Underflow Error Frames"]
    #[inline(always)]
    pub const fn tx_underflow_error_frames(&self) -> &TX_UNDERFLOW_ERROR_FRAMES {
        &self.tx_underflow_error_frames
    }
    #[doc = "0x14c - Transmit Frame Count for Frames Transmitted after Single Collision"]
    #[inline(always)]
    pub const fn tx_single_collision_good_frames(&self) -> &TX_SINGLE_COLLISION_GOOD_FRAMES {
        &self.tx_single_collision_good_frames
    }
    #[doc = "0x150 - Transmit Frame Count for Frames Transmitted after Multiple Collision"]
    #[inline(always)]
    pub const fn tx_multiple_collision_good_frames(&self) -> &TX_MULTIPLE_COLLISION_GOOD_FRAMES {
        &self.tx_multiple_collision_good_frames
    }
    #[doc = "0x154 - Tx Deferred Frames Register"]
    #[inline(always)]
    pub const fn tx_deferred_frames(&self) -> &TX_DEFERRED_FRAMES {
        &self.tx_deferred_frames
    }
    #[doc = "0x158 - Transmit Frame Count for Late Collision Error Frames"]
    #[inline(always)]
    pub const fn tx_late_collision_frames(&self) -> &TX_LATE_COLLISION_FRAMES {
        &self.tx_late_collision_frames
    }
    #[doc = "0x15c - Transmit Frame Count for Excessive Collision Error Frames"]
    #[inline(always)]
    pub const fn tx_excessive_collision_frames(&self) -> &TX_EXCESSIVE_COLLISION_FRAMES {
        &self.tx_excessive_collision_frames
    }
    #[doc = "0x160 - Transmit Frame Count for Carrier Sense Error Frames"]
    #[inline(always)]
    pub const fn tx_carrier_error_frames(&self) -> &TX_CARRIER_ERROR_FRAMES {
        &self.tx_carrier_error_frames
    }
    #[doc = "0x164 - Tx Octet Count Good Register"]
    #[inline(always)]
    pub const fn tx_octet_count_good(&self) -> &TX_OCTET_COUNT_GOOD {
        &self.tx_octet_count_good
    }
    #[doc = "0x168 - Tx Frame Count Good Register"]
    #[inline(always)]
    pub const fn tx_frame_count_good(&self) -> &TX_FRAME_COUNT_GOOD {
        &self.tx_frame_count_good
    }
    #[doc = "0x16c - Transmit Frame Count for Excessive Deferral Error Frames"]
    #[inline(always)]
    pub const fn tx_excessive_deferral_error(&self) -> &TX_EXCESSIVE_DEFERRAL_ERROR {
        &self.tx_excessive_deferral_error
    }
    #[doc = "0x170 - Transmit Frame Count for Good PAUSE Frames"]
    #[inline(always)]
    pub const fn tx_pause_frames(&self) -> &TX_PAUSE_FRAMES {
        &self.tx_pause_frames
    }
    #[doc = "0x174 - Transmit Frame Count for Good VLAN Frames"]
    #[inline(always)]
    pub const fn tx_vlan_frames_good(&self) -> &TX_VLAN_FRAMES_GOOD {
        &self.tx_vlan_frames_good
    }
    #[doc = "0x178 - Transmit Frame Count for Good Oversize Frames"]
    #[inline(always)]
    pub const fn tx_osize_frames_good(&self) -> &TX_OSIZE_FRAMES_GOOD {
        &self.tx_osize_frames_good
    }
    #[doc = "0x180 - Receive Frame Count for Good and Bad Frames"]
    #[inline(always)]
    pub const fn rx_frames_count_good_bad(&self) -> &RX_FRAMES_COUNT_GOOD_BAD {
        &self.rx_frames_count_good_bad
    }
    #[doc = "0x184 - Receive Octet Count for Good and Bad Frames"]
    #[inline(always)]
    pub const fn rx_octet_count_good_bad(&self) -> &RX_OCTET_COUNT_GOOD_BAD {
        &self.rx_octet_count_good_bad
    }
    #[doc = "0x188 - Rx Octet Count Good Register"]
    #[inline(always)]
    pub const fn rx_octet_count_good(&self) -> &RX_OCTET_COUNT_GOOD {
        &self.rx_octet_count_good
    }
    #[doc = "0x18c - Receive Frame Count for Good Broadcast Frames"]
    #[inline(always)]
    pub const fn rx_broadcast_frames_good(&self) -> &RX_BROADCAST_FRAMES_GOOD {
        &self.rx_broadcast_frames_good
    }
    #[doc = "0x190 - Receive Frame Count for Good Multicast Frames"]
    #[inline(always)]
    pub const fn rx_multicast_frames_good(&self) -> &RX_MULTICAST_FRAMES_GOOD {
        &self.rx_multicast_frames_good
    }
    #[doc = "0x194 - Receive Frame Count for CRC Error Frames"]
    #[inline(always)]
    pub const fn rx_crc_error_frames(&self) -> &RX_CRC_ERROR_FRAMES {
        &self.rx_crc_error_frames
    }
    #[doc = "0x198 - Receive Frame Count for Alignment Error Frames"]
    #[inline(always)]
    pub const fn rx_alignment_error_frames(&self) -> &RX_ALIGNMENT_ERROR_FRAMES {
        &self.rx_alignment_error_frames
    }
    #[doc = "0x19c - Receive Frame Count for Runt Error Frames"]
    #[inline(always)]
    pub const fn rx_runt_error_frames(&self) -> &RX_RUNT_ERROR_FRAMES {
        &self.rx_runt_error_frames
    }
    #[doc = "0x1a0 - Receive Frame Count for Jabber Error Frames"]
    #[inline(always)]
    pub const fn rx_jabber_error_frames(&self) -> &RX_JABBER_ERROR_FRAMES {
        &self.rx_jabber_error_frames
    }
    #[doc = "0x1a4 - Receive Frame Count for Undersize Frames"]
    #[inline(always)]
    pub const fn rx_undersize_frames_good(&self) -> &RX_UNDERSIZE_FRAMES_GOOD {
        &self.rx_undersize_frames_good
    }
    #[doc = "0x1a8 - Rx Oversize Frames Good Register"]
    #[inline(always)]
    pub const fn rx_oversize_frames_good(&self) -> &RX_OVERSIZE_FRAMES_GOOD {
        &self.rx_oversize_frames_good
    }
    #[doc = "0x1ac - Receive Frame Count for Good and Bad 64 Byte Frames"]
    #[inline(always)]
    pub const fn rx_64octets_frames_good_bad(&self) -> &RX_64OCTETS_FRAMES_GOOD_BAD {
        &self.rx_64octets_frames_good_bad
    }
    #[doc = "0x1b0 - Receive Frame Count for Good and Bad 65 to 127 Bytes Frames"]
    #[inline(always)]
    pub const fn rx_65to127octets_frames_good_bad(&self) -> &RX_65TO127OCTETS_FRAMES_GOOD_BAD {
        &self.rx_65to127octets_frames_good_bad
    }
    #[doc = "0x1b4 - Receive Frame Count for Good and Bad 128 to 255 Bytes Frames"]
    #[inline(always)]
    pub const fn rx_128to255octets_frames_good_bad(&self) -> &RX_128TO255OCTETS_FRAMES_GOOD_BAD {
        &self.rx_128to255octets_frames_good_bad
    }
    #[doc = "0x1b8 - Receive Frame Count for Good and Bad 256 to 511 Bytes Frames"]
    #[inline(always)]
    pub const fn rx_256to511octets_frames_good_bad(&self) -> &RX_256TO511OCTETS_FRAMES_GOOD_BAD {
        &self.rx_256to511octets_frames_good_bad
    }
    #[doc = "0x1bc - Receive Frame Count for Good and Bad 512 to 1,023 Bytes Frames"]
    #[inline(always)]
    pub const fn rx_512to1023octets_frames_good_bad(&self) -> &RX_512TO1023OCTETS_FRAMES_GOOD_BAD {
        &self.rx_512to1023octets_frames_good_bad
    }
    #[doc = "0x1c0 - Receive Frame Count for Good and Bad 1,024 to Maxsize Bytes Frames"]
    #[inline(always)]
    pub const fn rx_1024tomaxoctets_frames_good_bad(&self) -> &RX_1024TOMAXOCTETS_FRAMES_GOOD_BAD {
        &self.rx_1024tomaxoctets_frames_good_bad
    }
    #[doc = "0x1c4 - Receive Frame Count for Good Unicast Frames"]
    #[inline(always)]
    pub const fn rx_unicast_frames_good(&self) -> &RX_UNICAST_FRAMES_GOOD {
        &self.rx_unicast_frames_good
    }
    #[doc = "0x1c8 - Receive Frame Count for Length Error Frames"]
    #[inline(always)]
    pub const fn rx_length_error_frames(&self) -> &RX_LENGTH_ERROR_FRAMES {
        &self.rx_length_error_frames
    }
    #[doc = "0x1cc - Receive Frame Count for Out of Range Frames"]
    #[inline(always)]
    pub const fn rx_out_of_range_type_frames(&self) -> &RX_OUT_OF_RANGE_TYPE_FRAMES {
        &self.rx_out_of_range_type_frames
    }
    #[doc = "0x1d0 - Receive Frame Count for PAUSE Frames"]
    #[inline(always)]
    pub const fn rx_pause_frames(&self) -> &RX_PAUSE_FRAMES {
        &self.rx_pause_frames
    }
    #[doc = "0x1d4 - Receive Frame Count for FIFO Overflow Frames"]
    #[inline(always)]
    pub const fn rx_fifo_overflow_frames(&self) -> &RX_FIFO_OVERFLOW_FRAMES {
        &self.rx_fifo_overflow_frames
    }
    #[doc = "0x1d8 - Receive Frame Count for Good and Bad VLAN Frames"]
    #[inline(always)]
    pub const fn rx_vlan_frames_good_bad(&self) -> &RX_VLAN_FRAMES_GOOD_BAD {
        &self.rx_vlan_frames_good_bad
    }
    #[doc = "0x1dc - Receive Frame Count for Watchdog Error Frames"]
    #[inline(always)]
    pub const fn rx_watchdog_error_frames(&self) -> &RX_WATCHDOG_ERROR_FRAMES {
        &self.rx_watchdog_error_frames
    }
    #[doc = "0x1e0 - Receive Frame Count for Receive Error Frames"]
    #[inline(always)]
    pub const fn rx_receive_error_frames(&self) -> &RX_RECEIVE_ERROR_FRAMES {
        &self.rx_receive_error_frames
    }
    #[doc = "0x1e4 - Receive Frame Count for Good Control Frames Frames"]
    #[inline(always)]
    pub const fn rx_control_frames_good(&self) -> &RX_CONTROL_FRAMES_GOOD {
        &self.rx_control_frames_good
    }
    #[doc = "0x200 - MMC Receive Checksum Offload Interrupt Mask Register"]
    #[inline(always)]
    pub const fn mmc_ipc_receive_interrupt_mask(&self) -> &MMC_IPC_RECEIVE_INTERRUPT_MASK {
        &self.mmc_ipc_receive_interrupt_mask
    }
    #[doc = "0x208 - MMC Receive Checksum Offload Interrupt Register"]
    #[inline(always)]
    pub const fn mmc_ipc_receive_interrupt(&self) -> &MMC_IPC_RECEIVE_INTERRUPT {
        &self.mmc_ipc_receive_interrupt
    }
    #[doc = "0x210 - RxIPv4 Good Frames Register"]
    #[inline(always)]
    pub const fn rxipv4_good_frames(&self) -> &RXIPV4_GOOD_FRAMES {
        &self.rxipv4_good_frames
    }
    #[doc = "0x214 - Receive IPV4 Header Error Frame Counter Register"]
    #[inline(always)]
    pub const fn rxipv4_header_error_frames(&self) -> &RXIPV4_HEADER_ERROR_FRAMES {
        &self.rxipv4_header_error_frames
    }
    #[doc = "0x218 - Receive IPV4 No Payload Frame Counter Register"]
    #[inline(always)]
    pub const fn rxipv4_no_payload_frames(&self) -> &RXIPV4_NO_PAYLOAD_FRAMES {
        &self.rxipv4_no_payload_frames
    }
    #[doc = "0x21c - Receive IPV4 Fragmented Frame Counter Register"]
    #[inline(always)]
    pub const fn rxipv4_fragmented_frames(&self) -> &RXIPV4_FRAGMENTED_FRAMES {
        &self.rxipv4_fragmented_frames
    }
    #[doc = "0x220 - Receive IPV4 UDP Checksum Disabled Frame Counter Register"]
    #[inline(always)]
    pub const fn rxipv4_udp_checksum_disabled_frames(&self) -> &RXIPV4_UDP_CHECKSUM_DISABLED_FRAMES {
        &self.rxipv4_udp_checksum_disabled_frames
    }
    #[doc = "0x224 - RxIPv6 Good Frames Register"]
    #[inline(always)]
    pub const fn rxipv6_good_frames(&self) -> &RXIPV6_GOOD_FRAMES {
        &self.rxipv6_good_frames
    }
    #[doc = "0x228 - Receive IPV6 Header Error Frame Counter Register"]
    #[inline(always)]
    pub const fn rxipv6_header_error_frames(&self) -> &RXIPV6_HEADER_ERROR_FRAMES {
        &self.rxipv6_header_error_frames
    }
    #[doc = "0x22c - Receive IPV6 No Payload Frame Counter Register"]
    #[inline(always)]
    pub const fn rxipv6_no_payload_frames(&self) -> &RXIPV6_NO_PAYLOAD_FRAMES {
        &self.rxipv6_no_payload_frames
    }
    #[doc = "0x230 - RxUDP Good Frames Register"]
    #[inline(always)]
    pub const fn rxudp_good_frames(&self) -> &RXUDP_GOOD_FRAMES {
        &self.rxudp_good_frames
    }
    #[doc = "0x234 - RxUDP Error Frames Register"]
    #[inline(always)]
    pub const fn rxudp_error_frames(&self) -> &RXUDP_ERROR_FRAMES {
        &self.rxudp_error_frames
    }
    #[doc = "0x238 - RxTCP Good Frames Register"]
    #[inline(always)]
    pub const fn rxtcp_good_frames(&self) -> &RXTCP_GOOD_FRAMES {
        &self.rxtcp_good_frames
    }
    #[doc = "0x23c - RxTCP Error Frames Register"]
    #[inline(always)]
    pub const fn rxtcp_error_frames(&self) -> &RXTCP_ERROR_FRAMES {
        &self.rxtcp_error_frames
    }
    #[doc = "0x240 - RxICMP Good Frames Register"]
    #[inline(always)]
    pub const fn rxicmp_good_frames(&self) -> &RXICMP_GOOD_FRAMES {
        &self.rxicmp_good_frames
    }
    #[doc = "0x244 - RxICMP Error Frames Register"]
    #[inline(always)]
    pub const fn rxicmp_error_frames(&self) -> &RXICMP_ERROR_FRAMES {
        &self.rxicmp_error_frames
    }
    #[doc = "0x250 - RxIPv4 Good Octets Register"]
    #[inline(always)]
    pub const fn rxipv4_good_octets(&self) -> &RXIPV4_GOOD_OCTETS {
        &self.rxipv4_good_octets
    }
    #[doc = "0x254 - Receive IPV4 Header Error Octet Counter Register"]
    #[inline(always)]
    pub const fn rxipv4_header_error_octets(&self) -> &RXIPV4_HEADER_ERROR_OCTETS {
        &self.rxipv4_header_error_octets
    }
    #[doc = "0x258 - Receive IPV4 No Payload Octet Counter Register"]
    #[inline(always)]
    pub const fn rxipv4_no_payload_octets(&self) -> &RXIPV4_NO_PAYLOAD_OCTETS {
        &self.rxipv4_no_payload_octets
    }
    #[doc = "0x25c - Receive IPV4 Fragmented Octet Counter Register"]
    #[inline(always)]
    pub const fn rxipv4_fragmented_octets(&self) -> &RXIPV4_FRAGMENTED_OCTETS {
        &self.rxipv4_fragmented_octets
    }
    #[doc = "0x260 - Receive IPV4 Fragmented Octet Counter Register"]
    #[inline(always)]
    pub const fn rxipv4_udp_checksum_disable_octets(&self) -> &RXIPV4_UDP_CHECKSUM_DISABLE_OCTETS {
        &self.rxipv4_udp_checksum_disable_octets
    }
    #[doc = "0x264 - RxIPv6 Good Octets Register"]
    #[inline(always)]
    pub const fn rxipv6_good_octets(&self) -> &RXIPV6_GOOD_OCTETS {
        &self.rxipv6_good_octets
    }
    #[doc = "0x268 - Receive IPV6 Header Error Octet Counter Register"]
    #[inline(always)]
    pub const fn rxipv6_header_error_octets(&self) -> &RXIPV6_HEADER_ERROR_OCTETS {
        &self.rxipv6_header_error_octets
    }
    #[doc = "0x26c - Receive IPV6 No Payload Octet Counter Register"]
    #[inline(always)]
    pub const fn rxipv6_no_payload_octets(&self) -> &RXIPV6_NO_PAYLOAD_OCTETS {
        &self.rxipv6_no_payload_octets
    }
    #[doc = "0x270 - Receive UDP Good Octets Register"]
    #[inline(always)]
    pub const fn rxudp_good_octets(&self) -> &RXUDP_GOOD_OCTETS {
        &self.rxudp_good_octets
    }
    #[doc = "0x274 - Receive UDP Error Octets Register"]
    #[inline(always)]
    pub const fn rxudp_error_octets(&self) -> &RXUDP_ERROR_OCTETS {
        &self.rxudp_error_octets
    }
    #[doc = "0x278 - Receive TCP Good Octets Register"]
    #[inline(always)]
    pub const fn rxtcp_good_octets(&self) -> &RXTCP_GOOD_OCTETS {
        &self.rxtcp_good_octets
    }
    #[doc = "0x27c - Receive TCP Error Octets Register"]
    #[inline(always)]
    pub const fn rxtcp_error_octets(&self) -> &RXTCP_ERROR_OCTETS {
        &self.rxtcp_error_octets
    }
    #[doc = "0x280 - Receive ICMP Good Octets Register"]
    #[inline(always)]
    pub const fn rxicmp_good_octets(&self) -> &RXICMP_GOOD_OCTETS {
        &self.rxicmp_good_octets
    }
    #[doc = "0x284 - Receive ICMP Error Octets Register"]
    #[inline(always)]
    pub const fn rxicmp_error_octets(&self) -> &RXICMP_ERROR_OCTETS {
        &self.rxicmp_error_octets
    }
    #[doc = "0x700 - Timestamp Control Register"]
    #[inline(always)]
    pub const fn timestamp_control(&self) -> &TIMESTAMP_CONTROL {
        &self.timestamp_control
    }
    #[doc = "0x704 - Sub-Second Increment Register"]
    #[inline(always)]
    pub const fn sub_second_increment(&self) -> &SUB_SECOND_INCREMENT {
        &self.sub_second_increment
    }
    #[doc = "0x708 - System Time - Seconds Register"]
    #[inline(always)]
    pub const fn system_time_seconds(&self) -> &SYSTEM_TIME_SECONDS {
        &self.system_time_seconds
    }
    #[doc = "0x70c - System Time Nanoseconds Register"]
    #[inline(always)]
    pub const fn system_time_nanoseconds(&self) -> &SYSTEM_TIME_NANOSECONDS {
        &self.system_time_nanoseconds
    }
    #[doc = "0x710 - System Time - Seconds Update Register"]
    #[inline(always)]
    pub const fn system_time_seconds_update(&self) -> &SYSTEM_TIME_SECONDS_UPDATE {
        &self.system_time_seconds_update
    }
    #[doc = "0x714 - System Time Nanoseconds Update Register"]
    #[inline(always)]
    pub const fn system_time_nanoseconds_update(&self) -> &SYSTEM_TIME_NANOSECONDS_UPDATE {
        &self.system_time_nanoseconds_update
    }
    #[doc = "0x718 - Timestamp Addend Register"]
    #[inline(always)]
    pub const fn timestamp_addend(&self) -> &TIMESTAMP_ADDEND {
        &self.timestamp_addend
    }
    #[doc = "0x71c - Target Time Seconds Register"]
    #[inline(always)]
    pub const fn target_time_seconds(&self) -> &TARGET_TIME_SECONDS {
        &self.target_time_seconds
    }
    #[doc = "0x720 - Target Time Nanoseconds Register"]
    #[inline(always)]
    pub const fn target_time_nanoseconds(&self) -> &TARGET_TIME_NANOSECONDS {
        &self.target_time_nanoseconds
    }
    #[doc = "0x724 - System Time - Higher Word Seconds Register"]
    #[inline(always)]
    pub const fn system_time_higher_word_seconds(&self) -> &SYSTEM_TIME_HIGHER_WORD_SECONDS {
        &self.system_time_higher_word_seconds
    }
    #[doc = "0x728 - Timestamp Status Register"]
    #[inline(always)]
    pub const fn timestamp_status(&self) -> &TIMESTAMP_STATUS {
        &self.timestamp_status
    }
    #[doc = "0x1000 - Bus Mode Register"]
    #[inline(always)]
    pub const fn bus_mode(&self) -> &BUS_MODE {
        &self.bus_mode
    }
    #[doc = "0x1004 - Transmit Poll Demand Register"]
    #[inline(always)]
    pub const fn transmit_poll_demand(&self) -> &TRANSMIT_POLL_DEMAND {
        &self.transmit_poll_demand
    }
    #[doc = "0x1008 - Receive Poll Demand Register"]
    #[inline(always)]
    pub const fn receive_poll_demand(&self) -> &RECEIVE_POLL_DEMAND {
        &self.receive_poll_demand
    }
    #[doc = "0x100c - Receive Descriptor Address Register"]
    #[inline(always)]
    pub const fn receive_descriptor_list_address(&self) -> &RECEIVE_DESCRIPTOR_LIST_ADDRESS {
        &self.receive_descriptor_list_address
    }
    #[doc = "0x1010 - Transmit descripter Address Register"]
    #[inline(always)]
    pub const fn transmit_descriptor_list_address(&self) -> &TRANSMIT_DESCRIPTOR_LIST_ADDRESS {
        &self.transmit_descriptor_list_address
    }
    #[doc = "0x1014 - Status Register"]
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    #[doc = "0x1018 - Operation Mode Register"]
    #[inline(always)]
    pub const fn operation_mode(&self) -> &OPERATION_MODE {
        &self.operation_mode
    }
    #[doc = "0x101c - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn interrupt_enable(&self) -> &INTERRUPT_ENABLE {
        &self.interrupt_enable
    }
    #[doc = "0x1020 - Missed Frame and Buffer Overflow Counter Register"]
    #[inline(always)]
    pub const fn missed_frame_and_buffer_overflow_counter(&self) -> &MISSED_FRAME_AND_BUFFER_OVERFLOW_COUNTER {
        &self.missed_frame_and_buffer_overflow_counter
    }
    #[doc = "0x1024 - Receive Interrupt Watchdog Timer Register"]
    #[inline(always)]
    pub const fn receive_interrupt_watchdog_timer(&self) -> &RECEIVE_INTERRUPT_WATCHDOG_TIMER {
        &self.receive_interrupt_watchdog_timer
    }
    #[doc = "0x102c - AHB Status Register"]
    #[inline(always)]
    pub const fn ahb_status(&self) -> &AHB_STATUS {
        &self.ahb_status
    }
    #[doc = "0x1048 - Current Host Transmit Descriptor Register"]
    #[inline(always)]
    pub const fn current_host_transmit_descriptor(&self) -> &CURRENT_HOST_TRANSMIT_DESCRIPTOR {
        &self.current_host_transmit_descriptor
    }
    #[doc = "0x104c - Current Host Receive Descriptor Register"]
    #[inline(always)]
    pub const fn current_host_receive_descriptor(&self) -> &CURRENT_HOST_RECEIVE_DESCRIPTOR {
        &self.current_host_receive_descriptor
    }
    #[doc = "0x1050 - Current Host Transmit Buffer Address Register"]
    #[inline(always)]
    pub const fn current_host_transmit_buffer_address(&self) -> &CURRENT_HOST_TRANSMIT_BUFFER_ADDRESS {
        &self.current_host_transmit_buffer_address
    }
    #[doc = "0x1054 - Current Host Receive Buffer Address Register"]
    #[inline(always)]
    pub const fn current_host_receive_buffer_address(&self) -> &CURRENT_HOST_RECEIVE_BUFFER_ADDRESS {
        &self.current_host_receive_buffer_address
    }
    #[doc = "0x1058 - HW Feature Register"]
    #[inline(always)]
    pub const fn hw_feature(&self) -> &HW_FEATURE {
        &self.hw_feature
    }
}
#[doc = "MAC_CONFIGURATION (rw) register accessor: MAC Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mac_configuration::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mac_configuration::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mac_configuration`]
module"]
pub type MAC_CONFIGURATION = crate::Reg<mac_configuration::MAC_CONFIGURATION_SPEC>;
#[doc = "MAC Configuration Register"]
pub mod mac_configuration;
#[doc = "MAC_FRAME_FILTER (rw) register accessor: MAC Frame Filter\n\nYou can [`read`](crate::Reg::read) this register and get [`mac_frame_filter::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mac_frame_filter::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mac_frame_filter`]
module"]
pub type MAC_FRAME_FILTER = crate::Reg<mac_frame_filter::MAC_FRAME_FILTER_SPEC>;
#[doc = "MAC Frame Filter"]
pub mod mac_frame_filter;
#[doc = "HASH_TABLE_HIGH (rw) register accessor: Hash Table High Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hash_table_high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_table_high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_table_high`]
module"]
pub type HASH_TABLE_HIGH = crate::Reg<hash_table_high::HASH_TABLE_HIGH_SPEC>;
#[doc = "Hash Table High Register"]
pub mod hash_table_high;
#[doc = "HASH_TABLE_LOW (rw) register accessor: Hash Table Low Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hash_table_low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_table_low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_table_low`]
module"]
pub type HASH_TABLE_LOW = crate::Reg<hash_table_low::HASH_TABLE_LOW_SPEC>;
#[doc = "Hash Table Low Register"]
pub mod hash_table_low;
#[doc = "GMII_ADDRESS (rw) register accessor: MII Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gmii_address::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gmii_address::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmii_address`]
module"]
pub type GMII_ADDRESS = crate::Reg<gmii_address::GMII_ADDRESS_SPEC>;
#[doc = "MII Address Register"]
pub mod gmii_address;
#[doc = "GMII_DATA (rw) register accessor: MII Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gmii_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gmii_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmii_data`]
module"]
pub type GMII_DATA = crate::Reg<gmii_data::GMII_DATA_SPEC>;
#[doc = "MII Data Register"]
pub mod gmii_data;
#[doc = "FLOW_CONTROL (rw) register accessor: Flow Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flow_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flow_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flow_control`]
module"]
pub type FLOW_CONTROL = crate::Reg<flow_control::FLOW_CONTROL_SPEC>;
#[doc = "Flow Control Register"]
pub mod flow_control;
#[doc = "VLAN_TAG (rw) register accessor: VLAN Tag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`vlan_tag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vlan_tag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vlan_tag`]
module"]
pub type VLAN_TAG = crate::Reg<vlan_tag::VLAN_TAG_SPEC>;
#[doc = "VLAN Tag Register"]
pub mod vlan_tag;
#[doc = "VERSION (r) register accessor: Version Register\n\nYou can [`read`](crate::Reg::read) this register and get [`version::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@version`]
module"]
pub type VERSION = crate::Reg<version::VERSION_SPEC>;
#[doc = "Version Register"]
pub mod version;
#[doc = "DEBUG (r) register accessor: Debug Register\n\nYou can [`read`](crate::Reg::read) this register and get [`debug::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug`]
module"]
pub type DEBUG = crate::Reg<debug::DEBUG_SPEC>;
#[doc = "Debug Register"]
pub mod debug;
#[doc = "REMOTE_WAKE_UP_FRAME_FILTER (rw) register accessor: Remote Wake Up Frame Filter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`remote_wake_up_frame_filter::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`remote_wake_up_frame_filter::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@remote_wake_up_frame_filter`]
module"]
pub type REMOTE_WAKE_UP_FRAME_FILTER = crate::Reg<remote_wake_up_frame_filter::REMOTE_WAKE_UP_FRAME_FILTER_SPEC>;
#[doc = "Remote Wake Up Frame Filter Register"]
pub mod remote_wake_up_frame_filter;
#[doc = "PMT_CONTROL_STATUS (rw) register accessor: PMT Control and Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pmt_control_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmt_control_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmt_control_status`]
module"]
pub type PMT_CONTROL_STATUS = crate::Reg<pmt_control_status::PMT_CONTROL_STATUS_SPEC>;
#[doc = "PMT Control and Status Register"]
pub mod pmt_control_status;
#[doc = "INTERRUPT_STATUS (r) register accessor: Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrupt_status`]
module"]
pub type INTERRUPT_STATUS = crate::Reg<interrupt_status::INTERRUPT_STATUS_SPEC>;
#[doc = "Interrupt Register"]
pub mod interrupt_status;
#[doc = "INTERRUPT_MASK (rw) register accessor: Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrupt_mask`]
module"]
pub type INTERRUPT_MASK = crate::Reg<interrupt_mask::INTERRUPT_MASK_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod interrupt_mask;
#[doc = "MAC_ADDRESS0_HIGH (rw) register accessor: MAC Address0 High Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mac_address0_high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mac_address0_high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mac_address0_high`]
module"]
pub type MAC_ADDRESS0_HIGH = crate::Reg<mac_address0_high::MAC_ADDRESS0_HIGH_SPEC>;
#[doc = "MAC Address0 High Register"]
pub mod mac_address0_high;
#[doc = "MAC_ADDRESS0_LOW (rw) register accessor: MAC Address0 Low Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mac_address0_low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mac_address0_low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mac_address0_low`]
module"]
pub type MAC_ADDRESS0_LOW = crate::Reg<mac_address0_low::MAC_ADDRESS0_LOW_SPEC>;
#[doc = "MAC Address0 Low Register"]
pub mod mac_address0_low;
#[doc = "MAC_ADDRESS1_HIGH (rw) register accessor: MAC Address1 High Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mac_address1_high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mac_address1_high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mac_address1_high`]
module"]
pub type MAC_ADDRESS1_HIGH = crate::Reg<mac_address1_high::MAC_ADDRESS1_HIGH_SPEC>;
#[doc = "MAC Address1 High Register"]
pub mod mac_address1_high;
#[doc = "MAC_ADDRESS1_LOW (rw) register accessor: MAC Address1 Low Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mac_address1_low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mac_address1_low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mac_address1_low`]
module"]
pub type MAC_ADDRESS1_LOW = crate::Reg<mac_address1_low::MAC_ADDRESS1_LOW_SPEC>;
#[doc = "MAC Address1 Low Register"]
pub mod mac_address1_low;
#[doc = "MAC_ADDRESS2_HIGH (rw) register accessor: MAC Address2 High Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mac_address2_high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mac_address2_high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mac_address2_high`]
module"]
pub type MAC_ADDRESS2_HIGH = crate::Reg<mac_address2_high::MAC_ADDRESS2_HIGH_SPEC>;
#[doc = "MAC Address2 High Register"]
pub mod mac_address2_high;
#[doc = "MAC_ADDRESS2_LOW (rw) register accessor: MAC Address2 Low Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mac_address2_low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mac_address2_low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mac_address2_low`]
module"]
pub type MAC_ADDRESS2_LOW = crate::Reg<mac_address2_low::MAC_ADDRESS2_LOW_SPEC>;
#[doc = "MAC Address2 Low Register"]
pub mod mac_address2_low;
#[doc = "MAC_ADDRESS3_HIGH (rw) register accessor: MAC Address3 High Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mac_address3_high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mac_address3_high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mac_address3_high`]
module"]
pub type MAC_ADDRESS3_HIGH = crate::Reg<mac_address3_high::MAC_ADDRESS3_HIGH_SPEC>;
#[doc = "MAC Address3 High Register"]
pub mod mac_address3_high;
#[doc = "MAC_ADDRESS3_LOW (rw) register accessor: MAC Address3 Low Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mac_address3_low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mac_address3_low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mac_address3_low`]
module"]
pub type MAC_ADDRESS3_LOW = crate::Reg<mac_address3_low::MAC_ADDRESS3_LOW_SPEC>;
#[doc = "MAC Address3 Low Register"]
pub mod mac_address3_low;
#[doc = "MMC_CONTROL (rw) register accessor: MMC Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mmc_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmc_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmc_control`]
module"]
pub type MMC_CONTROL = crate::Reg<mmc_control::MMC_CONTROL_SPEC>;
#[doc = "MMC Control Register"]
pub mod mmc_control;
#[doc = "MMC_RECEIVE_INTERRUPT (r) register accessor: MMC Receive Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mmc_receive_interrupt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmc_receive_interrupt`]
module"]
pub type MMC_RECEIVE_INTERRUPT = crate::Reg<mmc_receive_interrupt::MMC_RECEIVE_INTERRUPT_SPEC>;
#[doc = "MMC Receive Interrupt Register"]
pub mod mmc_receive_interrupt;
#[doc = "MMC_TRANSMIT_INTERRUPT (r) register accessor: MMC Transmit Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mmc_transmit_interrupt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmc_transmit_interrupt`]
module"]
pub type MMC_TRANSMIT_INTERRUPT = crate::Reg<mmc_transmit_interrupt::MMC_TRANSMIT_INTERRUPT_SPEC>;
#[doc = "MMC Transmit Interrupt Register"]
pub mod mmc_transmit_interrupt;
#[doc = "MMC_RECEIVE_INTERRUPT_MASK (rw) register accessor: MMC Reveive Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mmc_receive_interrupt_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmc_receive_interrupt_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmc_receive_interrupt_mask`]
module"]
pub type MMC_RECEIVE_INTERRUPT_MASK = crate::Reg<mmc_receive_interrupt_mask::MMC_RECEIVE_INTERRUPT_MASK_SPEC>;
#[doc = "MMC Reveive Interrupt Mask Register"]
pub mod mmc_receive_interrupt_mask;
#[doc = "MMC_TRANSMIT_INTERRUPT_MASK (rw) register accessor: MMC Transmit Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mmc_transmit_interrupt_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmc_transmit_interrupt_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmc_transmit_interrupt_mask`]
module"]
pub type MMC_TRANSMIT_INTERRUPT_MASK = crate::Reg<mmc_transmit_interrupt_mask::MMC_TRANSMIT_INTERRUPT_MASK_SPEC>;
#[doc = "MMC Transmit Interrupt Mask Register"]
pub mod mmc_transmit_interrupt_mask;
#[doc = "TX_OCTET_COUNT_GOOD_BAD (r) register accessor: Transmit Octet Count for Good and Bad Frames Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_octet_count_good_bad::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_octet_count_good_bad`]
module"]
pub type TX_OCTET_COUNT_GOOD_BAD = crate::Reg<tx_octet_count_good_bad::TX_OCTET_COUNT_GOOD_BAD_SPEC>;
#[doc = "Transmit Octet Count for Good and Bad Frames Register"]
pub mod tx_octet_count_good_bad;
#[doc = "TX_FRAME_COUNT_GOOD_BAD (r) register accessor: Transmit Frame Count for Goodand Bad Frames Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_frame_count_good_bad::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_frame_count_good_bad`]
module"]
pub type TX_FRAME_COUNT_GOOD_BAD = crate::Reg<tx_frame_count_good_bad::TX_FRAME_COUNT_GOOD_BAD_SPEC>;
#[doc = "Transmit Frame Count for Goodand Bad Frames Register"]
pub mod tx_frame_count_good_bad;
#[doc = "TX_BROADCAST_FRAMES_GOOD (r) register accessor: Transmit Frame Count for Good Broadcast Frames\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_broadcast_frames_good::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_broadcast_frames_good`]
module"]
pub type TX_BROADCAST_FRAMES_GOOD = crate::Reg<tx_broadcast_frames_good::TX_BROADCAST_FRAMES_GOOD_SPEC>;
#[doc = "Transmit Frame Count for Good Broadcast Frames"]
pub mod tx_broadcast_frames_good;
#[doc = "TX_MULTICAST_FRAMES_GOOD (r) register accessor: Transmit Frame Count for Good Multicast Frames\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_multicast_frames_good::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_multicast_frames_good`]
module"]
pub type TX_MULTICAST_FRAMES_GOOD = crate::Reg<tx_multicast_frames_good::TX_MULTICAST_FRAMES_GOOD_SPEC>;
#[doc = "Transmit Frame Count for Good Multicast Frames"]
pub mod tx_multicast_frames_good;
#[doc = "TX_64OCTETS_FRAMES_GOOD_BAD (r) register accessor: Transmit Octet Count for Good and Bad 64 Byte Frames\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_64octets_frames_good_bad::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_64octets_frames_good_bad`]
module"]
pub type TX_64OCTETS_FRAMES_GOOD_BAD = crate::Reg<tx_64octets_frames_good_bad::TX_64OCTETS_FRAMES_GOOD_BAD_SPEC>;
#[doc = "Transmit Octet Count for Good and Bad 64 Byte Frames"]
pub mod tx_64octets_frames_good_bad;
#[doc = "TX_65TO127OCTETS_FRAMES_GOOD_BAD (r) register accessor: Transmit Octet Count for Good and Bad 65 to 127 Bytes Frames\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_65to127octets_frames_good_bad::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_65to127octets_frames_good_bad`]
module"]
pub type TX_65TO127OCTETS_FRAMES_GOOD_BAD = crate::Reg<tx_65to127octets_frames_good_bad::TX_65TO127OCTETS_FRAMES_GOOD_BAD_SPEC>;
#[doc = "Transmit Octet Count for Good and Bad 65 to 127 Bytes Frames"]
pub mod tx_65to127octets_frames_good_bad;
#[doc = "TX_128TO255OCTETS_FRAMES_GOOD_BAD (r) register accessor: Transmit Octet Count for Good and Bad 128 to 255 Bytes Frames\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_128to255octets_frames_good_bad::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_128to255octets_frames_good_bad`]
module"]
pub type TX_128TO255OCTETS_FRAMES_GOOD_BAD = crate::Reg<tx_128to255octets_frames_good_bad::TX_128TO255OCTETS_FRAMES_GOOD_BAD_SPEC>;
#[doc = "Transmit Octet Count for Good and Bad 128 to 255 Bytes Frames"]
pub mod tx_128to255octets_frames_good_bad;
#[doc = "TX_256TO511OCTETS_FRAMES_GOOD_BAD (r) register accessor: Transmit Octet Count for Good and Bad 256 to 511 Bytes Frames\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_256to511octets_frames_good_bad::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_256to511octets_frames_good_bad`]
module"]
pub type TX_256TO511OCTETS_FRAMES_GOOD_BAD = crate::Reg<tx_256to511octets_frames_good_bad::TX_256TO511OCTETS_FRAMES_GOOD_BAD_SPEC>;
#[doc = "Transmit Octet Count for Good and Bad 256 to 511 Bytes Frames"]
pub mod tx_256to511octets_frames_good_bad;
#[doc = "TX_512TO1023OCTETS_FRAMES_GOOD_BAD (r) register accessor: Transmit Octet Count for Good and Bad 512 to 1023 Bytes Frames\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_512to1023octets_frames_good_bad::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_512to1023octets_frames_good_bad`]
module"]
pub type TX_512TO1023OCTETS_FRAMES_GOOD_BAD = crate::Reg<tx_512to1023octets_frames_good_bad::TX_512TO1023OCTETS_FRAMES_GOOD_BAD_SPEC>;
#[doc = "Transmit Octet Count for Good and Bad 512 to 1023 Bytes Frames"]
pub mod tx_512to1023octets_frames_good_bad;
#[doc = "TX_1024TOMAXOCTETS_FRAMES_GOOD_BAD (r) register accessor: Transmit Octet Count for Good and Bad 1024 to Maxsize Bytes Frames\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_1024tomaxoctets_frames_good_bad::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_1024tomaxoctets_frames_good_bad`]
module"]
pub type TX_1024TOMAXOCTETS_FRAMES_GOOD_BAD = crate::Reg<tx_1024tomaxoctets_frames_good_bad::TX_1024TOMAXOCTETS_FRAMES_GOOD_BAD_SPEC>;
#[doc = "Transmit Octet Count for Good and Bad 1024 to Maxsize Bytes Frames"]
pub mod tx_1024tomaxoctets_frames_good_bad;
#[doc = "TX_UNICAST_FRAMES_GOOD_BAD (r) register accessor: Transmit Frame Count for Good and Bad Unicast Frames\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_unicast_frames_good_bad::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_unicast_frames_good_bad`]
module"]
pub type TX_UNICAST_FRAMES_GOOD_BAD = crate::Reg<tx_unicast_frames_good_bad::TX_UNICAST_FRAMES_GOOD_BAD_SPEC>;
#[doc = "Transmit Frame Count for Good and Bad Unicast Frames"]
pub mod tx_unicast_frames_good_bad;
#[doc = "TX_MULTICAST_FRAMES_GOOD_BAD (r) register accessor: Transmit Frame Count for Good and Bad Multicast Frames\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_multicast_frames_good_bad::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_multicast_frames_good_bad`]
module"]
pub type TX_MULTICAST_FRAMES_GOOD_BAD = crate::Reg<tx_multicast_frames_good_bad::TX_MULTICAST_FRAMES_GOOD_BAD_SPEC>;
#[doc = "Transmit Frame Count for Good and Bad Multicast Frames"]
pub mod tx_multicast_frames_good_bad;
#[doc = "TX_BROADCAST_FRAMES_GOOD_BAD (r) register accessor: Transmit Frame Count for Good and Bad Broadcast Frames\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_broadcast_frames_good_bad::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_broadcast_frames_good_bad`]
module"]
pub type TX_BROADCAST_FRAMES_GOOD_BAD = crate::Reg<tx_broadcast_frames_good_bad::TX_BROADCAST_FRAMES_GOOD_BAD_SPEC>;
#[doc = "Transmit Frame Count for Good and Bad Broadcast Frames"]
pub mod tx_broadcast_frames_good_bad;
#[doc = "TX_UNDERFLOW_ERROR_FRAMES (r) register accessor: Transmit Frame Count for Underflow Error Frames\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_underflow_error_frames::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_underflow_error_frames`]
module"]
pub type TX_UNDERFLOW_ERROR_FRAMES = crate::Reg<tx_underflow_error_frames::TX_UNDERFLOW_ERROR_FRAMES_SPEC>;
#[doc = "Transmit Frame Count for Underflow Error Frames"]
pub mod tx_underflow_error_frames;
#[doc = "TX_SINGLE_COLLISION_GOOD_FRAMES (r) register accessor: Transmit Frame Count for Frames Transmitted after Single Collision\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_single_collision_good_frames::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_single_collision_good_frames`]
module"]
pub type TX_SINGLE_COLLISION_GOOD_FRAMES = crate::Reg<tx_single_collision_good_frames::TX_SINGLE_COLLISION_GOOD_FRAMES_SPEC>;
#[doc = "Transmit Frame Count for Frames Transmitted after Single Collision"]
pub mod tx_single_collision_good_frames;
#[doc = "TX_MULTIPLE_COLLISION_GOOD_FRAMES (r) register accessor: Transmit Frame Count for Frames Transmitted after Multiple Collision\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_multiple_collision_good_frames::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_multiple_collision_good_frames`]
module"]
pub type TX_MULTIPLE_COLLISION_GOOD_FRAMES = crate::Reg<tx_multiple_collision_good_frames::TX_MULTIPLE_COLLISION_GOOD_FRAMES_SPEC>;
#[doc = "Transmit Frame Count for Frames Transmitted after Multiple Collision"]
pub mod tx_multiple_collision_good_frames;
#[doc = "TX_DEFERRED_FRAMES (r) register accessor: Tx Deferred Frames Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_deferred_frames::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_deferred_frames`]
module"]
pub type TX_DEFERRED_FRAMES = crate::Reg<tx_deferred_frames::TX_DEFERRED_FRAMES_SPEC>;
#[doc = "Tx Deferred Frames Register"]
pub mod tx_deferred_frames;
#[doc = "TX_LATE_COLLISION_FRAMES (r) register accessor: Transmit Frame Count for Late Collision Error Frames\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_late_collision_frames::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_late_collision_frames`]
module"]
pub type TX_LATE_COLLISION_FRAMES = crate::Reg<tx_late_collision_frames::TX_LATE_COLLISION_FRAMES_SPEC>;
#[doc = "Transmit Frame Count for Late Collision Error Frames"]
pub mod tx_late_collision_frames;
#[doc = "TX_EXCESSIVE_COLLISION_FRAMES (r) register accessor: Transmit Frame Count for Excessive Collision Error Frames\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_excessive_collision_frames::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_excessive_collision_frames`]
module"]
pub type TX_EXCESSIVE_COLLISION_FRAMES = crate::Reg<tx_excessive_collision_frames::TX_EXCESSIVE_COLLISION_FRAMES_SPEC>;
#[doc = "Transmit Frame Count for Excessive Collision Error Frames"]
pub mod tx_excessive_collision_frames;
#[doc = "TX_CARRIER_ERROR_FRAMES (r) register accessor: Transmit Frame Count for Carrier Sense Error Frames\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_carrier_error_frames::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_carrier_error_frames`]
module"]
pub type TX_CARRIER_ERROR_FRAMES = crate::Reg<tx_carrier_error_frames::TX_CARRIER_ERROR_FRAMES_SPEC>;
#[doc = "Transmit Frame Count for Carrier Sense Error Frames"]
pub mod tx_carrier_error_frames;
#[doc = "TX_OCTET_COUNT_GOOD (r) register accessor: Tx Octet Count Good Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_octet_count_good::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_octet_count_good`]
module"]
pub type TX_OCTET_COUNT_GOOD = crate::Reg<tx_octet_count_good::TX_OCTET_COUNT_GOOD_SPEC>;
#[doc = "Tx Octet Count Good Register"]
pub mod tx_octet_count_good;
#[doc = "TX_FRAME_COUNT_GOOD (r) register accessor: Tx Frame Count Good Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_frame_count_good::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_frame_count_good`]
module"]
pub type TX_FRAME_COUNT_GOOD = crate::Reg<tx_frame_count_good::TX_FRAME_COUNT_GOOD_SPEC>;
#[doc = "Tx Frame Count Good Register"]
pub mod tx_frame_count_good;
#[doc = "TX_EXCESSIVE_DEFERRAL_ERROR (r) register accessor: Transmit Frame Count for Excessive Deferral Error Frames\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_excessive_deferral_error::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_excessive_deferral_error`]
module"]
pub type TX_EXCESSIVE_DEFERRAL_ERROR = crate::Reg<tx_excessive_deferral_error::TX_EXCESSIVE_DEFERRAL_ERROR_SPEC>;
#[doc = "Transmit Frame Count for Excessive Deferral Error Frames"]
pub mod tx_excessive_deferral_error;
#[doc = "TX_PAUSE_FRAMES (r) register accessor: Transmit Frame Count for Good PAUSE Frames\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_pause_frames::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_pause_frames`]
module"]
pub type TX_PAUSE_FRAMES = crate::Reg<tx_pause_frames::TX_PAUSE_FRAMES_SPEC>;
#[doc = "Transmit Frame Count for Good PAUSE Frames"]
pub mod tx_pause_frames;
#[doc = "TX_VLAN_FRAMES_GOOD (r) register accessor: Transmit Frame Count for Good VLAN Frames\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_vlan_frames_good::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_vlan_frames_good`]
module"]
pub type TX_VLAN_FRAMES_GOOD = crate::Reg<tx_vlan_frames_good::TX_VLAN_FRAMES_GOOD_SPEC>;
#[doc = "Transmit Frame Count for Good VLAN Frames"]
pub mod tx_vlan_frames_good;
#[doc = "TX_OSIZE_FRAMES_GOOD (r) register accessor: Transmit Frame Count for Good Oversize Frames\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_osize_frames_good::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_osize_frames_good`]
module"]
pub type TX_OSIZE_FRAMES_GOOD = crate::Reg<tx_osize_frames_good::TX_OSIZE_FRAMES_GOOD_SPEC>;
#[doc = "Transmit Frame Count for Good Oversize Frames"]
pub mod tx_osize_frames_good;
#[doc = "RX_FRAMES_COUNT_GOOD_BAD (r) register accessor: Receive Frame Count for Good and Bad Frames\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_frames_count_good_bad::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_frames_count_good_bad`]
module"]
pub type RX_FRAMES_COUNT_GOOD_BAD = crate::Reg<rx_frames_count_good_bad::RX_FRAMES_COUNT_GOOD_BAD_SPEC>;
#[doc = "Receive Frame Count for Good and Bad Frames"]
pub mod rx_frames_count_good_bad;
#[doc = "RX_OCTET_COUNT_GOOD_BAD (r) register accessor: Receive Octet Count for Good and Bad Frames\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_octet_count_good_bad::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_octet_count_good_bad`]
module"]
pub type RX_OCTET_COUNT_GOOD_BAD = crate::Reg<rx_octet_count_good_bad::RX_OCTET_COUNT_GOOD_BAD_SPEC>;
#[doc = "Receive Octet Count for Good and Bad Frames"]
pub mod rx_octet_count_good_bad;
#[doc = "RX_OCTET_COUNT_GOOD (r) register accessor: Rx Octet Count Good Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_octet_count_good::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_octet_count_good`]
module"]
pub type RX_OCTET_COUNT_GOOD = crate::Reg<rx_octet_count_good::RX_OCTET_COUNT_GOOD_SPEC>;
#[doc = "Rx Octet Count Good Register"]
pub mod rx_octet_count_good;
#[doc = "RX_BROADCAST_FRAMES_GOOD (r) register accessor: Receive Frame Count for Good Broadcast Frames\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_broadcast_frames_good::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_broadcast_frames_good`]
module"]
pub type RX_BROADCAST_FRAMES_GOOD = crate::Reg<rx_broadcast_frames_good::RX_BROADCAST_FRAMES_GOOD_SPEC>;
#[doc = "Receive Frame Count for Good Broadcast Frames"]
pub mod rx_broadcast_frames_good;
#[doc = "RX_MULTICAST_FRAMES_GOOD (r) register accessor: Receive Frame Count for Good Multicast Frames\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_multicast_frames_good::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_multicast_frames_good`]
module"]
pub type RX_MULTICAST_FRAMES_GOOD = crate::Reg<rx_multicast_frames_good::RX_MULTICAST_FRAMES_GOOD_SPEC>;
#[doc = "Receive Frame Count for Good Multicast Frames"]
pub mod rx_multicast_frames_good;
#[doc = "RX_CRC_ERROR_FRAMES (r) register accessor: Receive Frame Count for CRC Error Frames\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_crc_error_frames::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_crc_error_frames`]
module"]
pub type RX_CRC_ERROR_FRAMES = crate::Reg<rx_crc_error_frames::RX_CRC_ERROR_FRAMES_SPEC>;
#[doc = "Receive Frame Count for CRC Error Frames"]
pub mod rx_crc_error_frames;
#[doc = "RX_ALIGNMENT_ERROR_FRAMES (r) register accessor: Receive Frame Count for Alignment Error Frames\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_alignment_error_frames::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_alignment_error_frames`]
module"]
pub type RX_ALIGNMENT_ERROR_FRAMES = crate::Reg<rx_alignment_error_frames::RX_ALIGNMENT_ERROR_FRAMES_SPEC>;
#[doc = "Receive Frame Count for Alignment Error Frames"]
pub mod rx_alignment_error_frames;
#[doc = "RX_RUNT_ERROR_FRAMES (r) register accessor: Receive Frame Count for Runt Error Frames\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_runt_error_frames::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_runt_error_frames`]
module"]
pub type RX_RUNT_ERROR_FRAMES = crate::Reg<rx_runt_error_frames::RX_RUNT_ERROR_FRAMES_SPEC>;
#[doc = "Receive Frame Count for Runt Error Frames"]
pub mod rx_runt_error_frames;
#[doc = "RX_JABBER_ERROR_FRAMES (r) register accessor: Receive Frame Count for Jabber Error Frames\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_jabber_error_frames::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_jabber_error_frames`]
module"]
pub type RX_JABBER_ERROR_FRAMES = crate::Reg<rx_jabber_error_frames::RX_JABBER_ERROR_FRAMES_SPEC>;
#[doc = "Receive Frame Count for Jabber Error Frames"]
pub mod rx_jabber_error_frames;
#[doc = "RX_UNDERSIZE_FRAMES_GOOD (r) register accessor: Receive Frame Count for Undersize Frames\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_undersize_frames_good::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_undersize_frames_good`]
module"]
pub type RX_UNDERSIZE_FRAMES_GOOD = crate::Reg<rx_undersize_frames_good::RX_UNDERSIZE_FRAMES_GOOD_SPEC>;
#[doc = "Receive Frame Count for Undersize Frames"]
pub mod rx_undersize_frames_good;
#[doc = "RX_OVERSIZE_FRAMES_GOOD (r) register accessor: Rx Oversize Frames Good Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_oversize_frames_good::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_oversize_frames_good`]
module"]
pub type RX_OVERSIZE_FRAMES_GOOD = crate::Reg<rx_oversize_frames_good::RX_OVERSIZE_FRAMES_GOOD_SPEC>;
#[doc = "Rx Oversize Frames Good Register"]
pub mod rx_oversize_frames_good;
#[doc = "RX_64OCTETS_FRAMES_GOOD_BAD (r) register accessor: Receive Frame Count for Good and Bad 64 Byte Frames\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_64octets_frames_good_bad::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_64octets_frames_good_bad`]
module"]
pub type RX_64OCTETS_FRAMES_GOOD_BAD = crate::Reg<rx_64octets_frames_good_bad::RX_64OCTETS_FRAMES_GOOD_BAD_SPEC>;
#[doc = "Receive Frame Count for Good and Bad 64 Byte Frames"]
pub mod rx_64octets_frames_good_bad;
#[doc = "RX_65TO127OCTETS_FRAMES_GOOD_BAD (r) register accessor: Receive Frame Count for Good and Bad 65 to 127 Bytes Frames\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_65to127octets_frames_good_bad::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_65to127octets_frames_good_bad`]
module"]
pub type RX_65TO127OCTETS_FRAMES_GOOD_BAD = crate::Reg<rx_65to127octets_frames_good_bad::RX_65TO127OCTETS_FRAMES_GOOD_BAD_SPEC>;
#[doc = "Receive Frame Count for Good and Bad 65 to 127 Bytes Frames"]
pub mod rx_65to127octets_frames_good_bad;
#[doc = "RX_128TO255OCTETS_FRAMES_GOOD_BAD (r) register accessor: Receive Frame Count for Good and Bad 128 to 255 Bytes Frames\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_128to255octets_frames_good_bad::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_128to255octets_frames_good_bad`]
module"]
pub type RX_128TO255OCTETS_FRAMES_GOOD_BAD = crate::Reg<rx_128to255octets_frames_good_bad::RX_128TO255OCTETS_FRAMES_GOOD_BAD_SPEC>;
#[doc = "Receive Frame Count for Good and Bad 128 to 255 Bytes Frames"]
pub mod rx_128to255octets_frames_good_bad;
#[doc = "RX_256TO511OCTETS_FRAMES_GOOD_BAD (r) register accessor: Receive Frame Count for Good and Bad 256 to 511 Bytes Frames\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_256to511octets_frames_good_bad::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_256to511octets_frames_good_bad`]
module"]
pub type RX_256TO511OCTETS_FRAMES_GOOD_BAD = crate::Reg<rx_256to511octets_frames_good_bad::RX_256TO511OCTETS_FRAMES_GOOD_BAD_SPEC>;
#[doc = "Receive Frame Count for Good and Bad 256 to 511 Bytes Frames"]
pub mod rx_256to511octets_frames_good_bad;
#[doc = "RX_512TO1023OCTETS_FRAMES_GOOD_BAD (r) register accessor: Receive Frame Count for Good and Bad 512 to 1,023 Bytes Frames\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_512to1023octets_frames_good_bad::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_512to1023octets_frames_good_bad`]
module"]
pub type RX_512TO1023OCTETS_FRAMES_GOOD_BAD = crate::Reg<rx_512to1023octets_frames_good_bad::RX_512TO1023OCTETS_FRAMES_GOOD_BAD_SPEC>;
#[doc = "Receive Frame Count for Good and Bad 512 to 1,023 Bytes Frames"]
pub mod rx_512to1023octets_frames_good_bad;
#[doc = "RX_1024TOMAXOCTETS_FRAMES_GOOD_BAD (r) register accessor: Receive Frame Count for Good and Bad 1,024 to Maxsize Bytes Frames\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_1024tomaxoctets_frames_good_bad::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_1024tomaxoctets_frames_good_bad`]
module"]
pub type RX_1024TOMAXOCTETS_FRAMES_GOOD_BAD = crate::Reg<rx_1024tomaxoctets_frames_good_bad::RX_1024TOMAXOCTETS_FRAMES_GOOD_BAD_SPEC>;
#[doc = "Receive Frame Count for Good and Bad 1,024 to Maxsize Bytes Frames"]
pub mod rx_1024tomaxoctets_frames_good_bad;
#[doc = "RX_UNICAST_FRAMES_GOOD (r) register accessor: Receive Frame Count for Good Unicast Frames\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_unicast_frames_good::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_unicast_frames_good`]
module"]
pub type RX_UNICAST_FRAMES_GOOD = crate::Reg<rx_unicast_frames_good::RX_UNICAST_FRAMES_GOOD_SPEC>;
#[doc = "Receive Frame Count for Good Unicast Frames"]
pub mod rx_unicast_frames_good;
#[doc = "RX_LENGTH_ERROR_FRAMES (r) register accessor: Receive Frame Count for Length Error Frames\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_length_error_frames::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_length_error_frames`]
module"]
pub type RX_LENGTH_ERROR_FRAMES = crate::Reg<rx_length_error_frames::RX_LENGTH_ERROR_FRAMES_SPEC>;
#[doc = "Receive Frame Count for Length Error Frames"]
pub mod rx_length_error_frames;
#[doc = "RX_OUT_OF_RANGE_TYPE_FRAMES (r) register accessor: Receive Frame Count for Out of Range Frames\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_out_of_range_type_frames::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_out_of_range_type_frames`]
module"]
pub type RX_OUT_OF_RANGE_TYPE_FRAMES = crate::Reg<rx_out_of_range_type_frames::RX_OUT_OF_RANGE_TYPE_FRAMES_SPEC>;
#[doc = "Receive Frame Count for Out of Range Frames"]
pub mod rx_out_of_range_type_frames;
#[doc = "RX_PAUSE_FRAMES (r) register accessor: Receive Frame Count for PAUSE Frames\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_pause_frames::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_pause_frames`]
module"]
pub type RX_PAUSE_FRAMES = crate::Reg<rx_pause_frames::RX_PAUSE_FRAMES_SPEC>;
#[doc = "Receive Frame Count for PAUSE Frames"]
pub mod rx_pause_frames;
#[doc = "RX_FIFO_OVERFLOW_FRAMES (r) register accessor: Receive Frame Count for FIFO Overflow Frames\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_fifo_overflow_frames::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_fifo_overflow_frames`]
module"]
pub type RX_FIFO_OVERFLOW_FRAMES = crate::Reg<rx_fifo_overflow_frames::RX_FIFO_OVERFLOW_FRAMES_SPEC>;
#[doc = "Receive Frame Count for FIFO Overflow Frames"]
pub mod rx_fifo_overflow_frames;
#[doc = "RX_VLAN_FRAMES_GOOD_BAD (r) register accessor: Receive Frame Count for Good and Bad VLAN Frames\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_vlan_frames_good_bad::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_vlan_frames_good_bad`]
module"]
pub type RX_VLAN_FRAMES_GOOD_BAD = crate::Reg<rx_vlan_frames_good_bad::RX_VLAN_FRAMES_GOOD_BAD_SPEC>;
#[doc = "Receive Frame Count for Good and Bad VLAN Frames"]
pub mod rx_vlan_frames_good_bad;
#[doc = "RX_WATCHDOG_ERROR_FRAMES (r) register accessor: Receive Frame Count for Watchdog Error Frames\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_watchdog_error_frames::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_watchdog_error_frames`]
module"]
pub type RX_WATCHDOG_ERROR_FRAMES = crate::Reg<rx_watchdog_error_frames::RX_WATCHDOG_ERROR_FRAMES_SPEC>;
#[doc = "Receive Frame Count for Watchdog Error Frames"]
pub mod rx_watchdog_error_frames;
#[doc = "RX_RECEIVE_ERROR_FRAMES (r) register accessor: Receive Frame Count for Receive Error Frames\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_receive_error_frames::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_receive_error_frames`]
module"]
pub type RX_RECEIVE_ERROR_FRAMES = crate::Reg<rx_receive_error_frames::RX_RECEIVE_ERROR_FRAMES_SPEC>;
#[doc = "Receive Frame Count for Receive Error Frames"]
pub mod rx_receive_error_frames;
#[doc = "RX_CONTROL_FRAMES_GOOD (r) register accessor: Receive Frame Count for Good Control Frames Frames\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_control_frames_good::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_control_frames_good`]
module"]
pub type RX_CONTROL_FRAMES_GOOD = crate::Reg<rx_control_frames_good::RX_CONTROL_FRAMES_GOOD_SPEC>;
#[doc = "Receive Frame Count for Good Control Frames Frames"]
pub mod rx_control_frames_good;
#[doc = "MMC_IPC_RECEIVE_INTERRUPT_MASK (rw) register accessor: MMC Receive Checksum Offload Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mmc_ipc_receive_interrupt_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmc_ipc_receive_interrupt_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmc_ipc_receive_interrupt_mask`]
module"]
pub type MMC_IPC_RECEIVE_INTERRUPT_MASK = crate::Reg<mmc_ipc_receive_interrupt_mask::MMC_IPC_RECEIVE_INTERRUPT_MASK_SPEC>;
#[doc = "MMC Receive Checksum Offload Interrupt Mask Register"]
pub mod mmc_ipc_receive_interrupt_mask;
#[doc = "MMC_IPC_RECEIVE_INTERRUPT (r) register accessor: MMC Receive Checksum Offload Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mmc_ipc_receive_interrupt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmc_ipc_receive_interrupt`]
module"]
pub type MMC_IPC_RECEIVE_INTERRUPT = crate::Reg<mmc_ipc_receive_interrupt::MMC_IPC_RECEIVE_INTERRUPT_SPEC>;
#[doc = "MMC Receive Checksum Offload Interrupt Register"]
pub mod mmc_ipc_receive_interrupt;
#[doc = "RXIPV4_GOOD_FRAMES (r) register accessor: RxIPv4 Good Frames Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxipv4_good_frames::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxipv4_good_frames`]
module"]
pub type RXIPV4_GOOD_FRAMES = crate::Reg<rxipv4_good_frames::RXIPV4_GOOD_FRAMES_SPEC>;
#[doc = "RxIPv4 Good Frames Register"]
pub mod rxipv4_good_frames;
#[doc = "RXIPV4_HEADER_ERROR_FRAMES (r) register accessor: Receive IPV4 Header Error Frame Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxipv4_header_error_frames::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxipv4_header_error_frames`]
module"]
pub type RXIPV4_HEADER_ERROR_FRAMES = crate::Reg<rxipv4_header_error_frames::RXIPV4_HEADER_ERROR_FRAMES_SPEC>;
#[doc = "Receive IPV4 Header Error Frame Counter Register"]
pub mod rxipv4_header_error_frames;
#[doc = "RXIPV4_NO_PAYLOAD_FRAMES (r) register accessor: Receive IPV4 No Payload Frame Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxipv4_no_payload_frames::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxipv4_no_payload_frames`]
module"]
pub type RXIPV4_NO_PAYLOAD_FRAMES = crate::Reg<rxipv4_no_payload_frames::RXIPV4_NO_PAYLOAD_FRAMES_SPEC>;
#[doc = "Receive IPV4 No Payload Frame Counter Register"]
pub mod rxipv4_no_payload_frames;
#[doc = "RXIPV4_FRAGMENTED_FRAMES (r) register accessor: Receive IPV4 Fragmented Frame Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxipv4_fragmented_frames::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxipv4_fragmented_frames`]
module"]
pub type RXIPV4_FRAGMENTED_FRAMES = crate::Reg<rxipv4_fragmented_frames::RXIPV4_FRAGMENTED_FRAMES_SPEC>;
#[doc = "Receive IPV4 Fragmented Frame Counter Register"]
pub mod rxipv4_fragmented_frames;
#[doc = "RXIPV4_UDP_CHECKSUM_DISABLED_FRAMES (r) register accessor: Receive IPV4 UDP Checksum Disabled Frame Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxipv4_udp_checksum_disabled_frames::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxipv4_udp_checksum_disabled_frames`]
module"]
pub type RXIPV4_UDP_CHECKSUM_DISABLED_FRAMES = crate::Reg<rxipv4_udp_checksum_disabled_frames::RXIPV4_UDP_CHECKSUM_DISABLED_FRAMES_SPEC>;
#[doc = "Receive IPV4 UDP Checksum Disabled Frame Counter Register"]
pub mod rxipv4_udp_checksum_disabled_frames;
#[doc = "RXIPV6_GOOD_FRAMES (r) register accessor: RxIPv6 Good Frames Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxipv6_good_frames::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxipv6_good_frames`]
module"]
pub type RXIPV6_GOOD_FRAMES = crate::Reg<rxipv6_good_frames::RXIPV6_GOOD_FRAMES_SPEC>;
#[doc = "RxIPv6 Good Frames Register"]
pub mod rxipv6_good_frames;
#[doc = "RXIPV6_HEADER_ERROR_FRAMES (r) register accessor: Receive IPV6 Header Error Frame Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxipv6_header_error_frames::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxipv6_header_error_frames`]
module"]
pub type RXIPV6_HEADER_ERROR_FRAMES = crate::Reg<rxipv6_header_error_frames::RXIPV6_HEADER_ERROR_FRAMES_SPEC>;
#[doc = "Receive IPV6 Header Error Frame Counter Register"]
pub mod rxipv6_header_error_frames;
#[doc = "RXIPV6_NO_PAYLOAD_FRAMES (r) register accessor: Receive IPV6 No Payload Frame Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxipv6_no_payload_frames::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxipv6_no_payload_frames`]
module"]
pub type RXIPV6_NO_PAYLOAD_FRAMES = crate::Reg<rxipv6_no_payload_frames::RXIPV6_NO_PAYLOAD_FRAMES_SPEC>;
#[doc = "Receive IPV6 No Payload Frame Counter Register"]
pub mod rxipv6_no_payload_frames;
#[doc = "RXUDP_GOOD_FRAMES (r) register accessor: RxUDP Good Frames Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxudp_good_frames::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxudp_good_frames`]
module"]
pub type RXUDP_GOOD_FRAMES = crate::Reg<rxudp_good_frames::RXUDP_GOOD_FRAMES_SPEC>;
#[doc = "RxUDP Good Frames Register"]
pub mod rxudp_good_frames;
#[doc = "RXUDP_ERROR_FRAMES (r) register accessor: RxUDP Error Frames Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxudp_error_frames::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxudp_error_frames`]
module"]
pub type RXUDP_ERROR_FRAMES = crate::Reg<rxudp_error_frames::RXUDP_ERROR_FRAMES_SPEC>;
#[doc = "RxUDP Error Frames Register"]
pub mod rxudp_error_frames;
#[doc = "RXTCP_GOOD_FRAMES (r) register accessor: RxTCP Good Frames Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxtcp_good_frames::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxtcp_good_frames`]
module"]
pub type RXTCP_GOOD_FRAMES = crate::Reg<rxtcp_good_frames::RXTCP_GOOD_FRAMES_SPEC>;
#[doc = "RxTCP Good Frames Register"]
pub mod rxtcp_good_frames;
#[doc = "RXTCP_ERROR_FRAMES (r) register accessor: RxTCP Error Frames Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxtcp_error_frames::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxtcp_error_frames`]
module"]
pub type RXTCP_ERROR_FRAMES = crate::Reg<rxtcp_error_frames::RXTCP_ERROR_FRAMES_SPEC>;
#[doc = "RxTCP Error Frames Register"]
pub mod rxtcp_error_frames;
#[doc = "RXICMP_GOOD_FRAMES (r) register accessor: RxICMP Good Frames Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxicmp_good_frames::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxicmp_good_frames`]
module"]
pub type RXICMP_GOOD_FRAMES = crate::Reg<rxicmp_good_frames::RXICMP_GOOD_FRAMES_SPEC>;
#[doc = "RxICMP Good Frames Register"]
pub mod rxicmp_good_frames;
#[doc = "RXICMP_ERROR_FRAMES (r) register accessor: RxICMP Error Frames Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxicmp_error_frames::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxicmp_error_frames`]
module"]
pub type RXICMP_ERROR_FRAMES = crate::Reg<rxicmp_error_frames::RXICMP_ERROR_FRAMES_SPEC>;
#[doc = "RxICMP Error Frames Register"]
pub mod rxicmp_error_frames;
#[doc = "RXIPV4_GOOD_OCTETS (r) register accessor: RxIPv4 Good Octets Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxipv4_good_octets::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxipv4_good_octets`]
module"]
pub type RXIPV4_GOOD_OCTETS = crate::Reg<rxipv4_good_octets::RXIPV4_GOOD_OCTETS_SPEC>;
#[doc = "RxIPv4 Good Octets Register"]
pub mod rxipv4_good_octets;
#[doc = "RXIPV4_HEADER_ERROR_OCTETS (r) register accessor: Receive IPV4 Header Error Octet Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxipv4_header_error_octets::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxipv4_header_error_octets`]
module"]
pub type RXIPV4_HEADER_ERROR_OCTETS = crate::Reg<rxipv4_header_error_octets::RXIPV4_HEADER_ERROR_OCTETS_SPEC>;
#[doc = "Receive IPV4 Header Error Octet Counter Register"]
pub mod rxipv4_header_error_octets;
#[doc = "RXIPV4_NO_PAYLOAD_OCTETS (r) register accessor: Receive IPV4 No Payload Octet Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxipv4_no_payload_octets::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxipv4_no_payload_octets`]
module"]
pub type RXIPV4_NO_PAYLOAD_OCTETS = crate::Reg<rxipv4_no_payload_octets::RXIPV4_NO_PAYLOAD_OCTETS_SPEC>;
#[doc = "Receive IPV4 No Payload Octet Counter Register"]
pub mod rxipv4_no_payload_octets;
#[doc = "RXIPV4_FRAGMENTED_OCTETS (r) register accessor: Receive IPV4 Fragmented Octet Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxipv4_fragmented_octets::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxipv4_fragmented_octets`]
module"]
pub type RXIPV4_FRAGMENTED_OCTETS = crate::Reg<rxipv4_fragmented_octets::RXIPV4_FRAGMENTED_OCTETS_SPEC>;
#[doc = "Receive IPV4 Fragmented Octet Counter Register"]
pub mod rxipv4_fragmented_octets;
#[doc = "RXIPV4_UDP_CHECKSUM_DISABLE_OCTETS (r) register accessor: Receive IPV4 Fragmented Octet Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxipv4_udp_checksum_disable_octets::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxipv4_udp_checksum_disable_octets`]
module"]
pub type RXIPV4_UDP_CHECKSUM_DISABLE_OCTETS = crate::Reg<rxipv4_udp_checksum_disable_octets::RXIPV4_UDP_CHECKSUM_DISABLE_OCTETS_SPEC>;
#[doc = "Receive IPV4 Fragmented Octet Counter Register"]
pub mod rxipv4_udp_checksum_disable_octets;
#[doc = "RXIPV6_GOOD_OCTETS (r) register accessor: RxIPv6 Good Octets Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxipv6_good_octets::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxipv6_good_octets`]
module"]
pub type RXIPV6_GOOD_OCTETS = crate::Reg<rxipv6_good_octets::RXIPV6_GOOD_OCTETS_SPEC>;
#[doc = "RxIPv6 Good Octets Register"]
pub mod rxipv6_good_octets;
#[doc = "RXIPV6_HEADER_ERROR_OCTETS (r) register accessor: Receive IPV6 Header Error Octet Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxipv6_header_error_octets::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxipv6_header_error_octets`]
module"]
pub type RXIPV6_HEADER_ERROR_OCTETS = crate::Reg<rxipv6_header_error_octets::RXIPV6_HEADER_ERROR_OCTETS_SPEC>;
#[doc = "Receive IPV6 Header Error Octet Counter Register"]
pub mod rxipv6_header_error_octets;
#[doc = "RXIPV6_NO_PAYLOAD_OCTETS (r) register accessor: Receive IPV6 No Payload Octet Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxipv6_no_payload_octets::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxipv6_no_payload_octets`]
module"]
pub type RXIPV6_NO_PAYLOAD_OCTETS = crate::Reg<rxipv6_no_payload_octets::RXIPV6_NO_PAYLOAD_OCTETS_SPEC>;
#[doc = "Receive IPV6 No Payload Octet Counter Register"]
pub mod rxipv6_no_payload_octets;
#[doc = "RXUDP_GOOD_OCTETS (r) register accessor: Receive UDP Good Octets Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxudp_good_octets::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxudp_good_octets`]
module"]
pub type RXUDP_GOOD_OCTETS = crate::Reg<rxudp_good_octets::RXUDP_GOOD_OCTETS_SPEC>;
#[doc = "Receive UDP Good Octets Register"]
pub mod rxudp_good_octets;
#[doc = "RXUDP_ERROR_OCTETS (r) register accessor: Receive UDP Error Octets Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxudp_error_octets::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxudp_error_octets`]
module"]
pub type RXUDP_ERROR_OCTETS = crate::Reg<rxudp_error_octets::RXUDP_ERROR_OCTETS_SPEC>;
#[doc = "Receive UDP Error Octets Register"]
pub mod rxudp_error_octets;
#[doc = "RXTCP_GOOD_OCTETS (r) register accessor: Receive TCP Good Octets Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxtcp_good_octets::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxtcp_good_octets`]
module"]
pub type RXTCP_GOOD_OCTETS = crate::Reg<rxtcp_good_octets::RXTCP_GOOD_OCTETS_SPEC>;
#[doc = "Receive TCP Good Octets Register"]
pub mod rxtcp_good_octets;
#[doc = "RXTCP_ERROR_OCTETS (r) register accessor: Receive TCP Error Octets Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxtcp_error_octets::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxtcp_error_octets`]
module"]
pub type RXTCP_ERROR_OCTETS = crate::Reg<rxtcp_error_octets::RXTCP_ERROR_OCTETS_SPEC>;
#[doc = "Receive TCP Error Octets Register"]
pub mod rxtcp_error_octets;
#[doc = "RXICMP_GOOD_OCTETS (r) register accessor: Receive ICMP Good Octets Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxicmp_good_octets::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxicmp_good_octets`]
module"]
pub type RXICMP_GOOD_OCTETS = crate::Reg<rxicmp_good_octets::RXICMP_GOOD_OCTETS_SPEC>;
#[doc = "Receive ICMP Good Octets Register"]
pub mod rxicmp_good_octets;
#[doc = "RXICMP_ERROR_OCTETS (r) register accessor: Receive ICMP Error Octets Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxicmp_error_octets::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxicmp_error_octets`]
module"]
pub type RXICMP_ERROR_OCTETS = crate::Reg<rxicmp_error_octets::RXICMP_ERROR_OCTETS_SPEC>;
#[doc = "Receive ICMP Error Octets Register"]
pub mod rxicmp_error_octets;
#[doc = "TIMESTAMP_CONTROL (rw) register accessor: Timestamp Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timestamp_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timestamp_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timestamp_control`]
module"]
pub type TIMESTAMP_CONTROL = crate::Reg<timestamp_control::TIMESTAMP_CONTROL_SPEC>;
#[doc = "Timestamp Control Register"]
pub mod timestamp_control;
#[doc = "SUB_SECOND_INCREMENT (rw) register accessor: Sub-Second Increment Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sub_second_increment::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sub_second_increment::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sub_second_increment`]
module"]
pub type SUB_SECOND_INCREMENT = crate::Reg<sub_second_increment::SUB_SECOND_INCREMENT_SPEC>;
#[doc = "Sub-Second Increment Register"]
pub mod sub_second_increment;
#[doc = "SYSTEM_TIME_SECONDS (r) register accessor: System Time - Seconds Register\n\nYou can [`read`](crate::Reg::read) this register and get [`system_time_seconds::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@system_time_seconds`]
module"]
pub type SYSTEM_TIME_SECONDS = crate::Reg<system_time_seconds::SYSTEM_TIME_SECONDS_SPEC>;
#[doc = "System Time - Seconds Register"]
pub mod system_time_seconds;
#[doc = "SYSTEM_TIME_NANOSECONDS (r) register accessor: System Time Nanoseconds Register\n\nYou can [`read`](crate::Reg::read) this register and get [`system_time_nanoseconds::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@system_time_nanoseconds`]
module"]
pub type SYSTEM_TIME_NANOSECONDS = crate::Reg<system_time_nanoseconds::SYSTEM_TIME_NANOSECONDS_SPEC>;
#[doc = "System Time Nanoseconds Register"]
pub mod system_time_nanoseconds;
#[doc = "SYSTEM_TIME_SECONDS_UPDATE (rw) register accessor: System Time - Seconds Update Register\n\nYou can [`read`](crate::Reg::read) this register and get [`system_time_seconds_update::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`system_time_seconds_update::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@system_time_seconds_update`]
module"]
pub type SYSTEM_TIME_SECONDS_UPDATE = crate::Reg<system_time_seconds_update::SYSTEM_TIME_SECONDS_UPDATE_SPEC>;
#[doc = "System Time - Seconds Update Register"]
pub mod system_time_seconds_update;
#[doc = "SYSTEM_TIME_NANOSECONDS_UPDATE (rw) register accessor: System Time Nanoseconds Update Register\n\nYou can [`read`](crate::Reg::read) this register and get [`system_time_nanoseconds_update::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`system_time_nanoseconds_update::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@system_time_nanoseconds_update`]
module"]
pub type SYSTEM_TIME_NANOSECONDS_UPDATE = crate::Reg<system_time_nanoseconds_update::SYSTEM_TIME_NANOSECONDS_UPDATE_SPEC>;
#[doc = "System Time Nanoseconds Update Register"]
pub mod system_time_nanoseconds_update;
#[doc = "TIMESTAMP_ADDEND (rw) register accessor: Timestamp Addend Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timestamp_addend::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timestamp_addend::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timestamp_addend`]
module"]
pub type TIMESTAMP_ADDEND = crate::Reg<timestamp_addend::TIMESTAMP_ADDEND_SPEC>;
#[doc = "Timestamp Addend Register"]
pub mod timestamp_addend;
#[doc = "TARGET_TIME_SECONDS (rw) register accessor: Target Time Seconds Register\n\nYou can [`read`](crate::Reg::read) this register and get [`target_time_seconds::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`target_time_seconds::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@target_time_seconds`]
module"]
pub type TARGET_TIME_SECONDS = crate::Reg<target_time_seconds::TARGET_TIME_SECONDS_SPEC>;
#[doc = "Target Time Seconds Register"]
pub mod target_time_seconds;
#[doc = "TARGET_TIME_NANOSECONDS (rw) register accessor: Target Time Nanoseconds Register\n\nYou can [`read`](crate::Reg::read) this register and get [`target_time_nanoseconds::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`target_time_nanoseconds::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@target_time_nanoseconds`]
module"]
pub type TARGET_TIME_NANOSECONDS = crate::Reg<target_time_nanoseconds::TARGET_TIME_NANOSECONDS_SPEC>;
#[doc = "Target Time Nanoseconds Register"]
pub mod target_time_nanoseconds;
#[doc = "SYSTEM_TIME_HIGHER_WORD_SECONDS (rw) register accessor: System Time - Higher Word Seconds Register\n\nYou can [`read`](crate::Reg::read) this register and get [`system_time_higher_word_seconds::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`system_time_higher_word_seconds::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@system_time_higher_word_seconds`]
module"]
pub type SYSTEM_TIME_HIGHER_WORD_SECONDS = crate::Reg<system_time_higher_word_seconds::SYSTEM_TIME_HIGHER_WORD_SECONDS_SPEC>;
#[doc = "System Time - Higher Word Seconds Register"]
pub mod system_time_higher_word_seconds;
#[doc = "TIMESTAMP_STATUS (r) register accessor: Timestamp Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timestamp_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timestamp_status`]
module"]
pub type TIMESTAMP_STATUS = crate::Reg<timestamp_status::TIMESTAMP_STATUS_SPEC>;
#[doc = "Timestamp Status Register"]
pub mod timestamp_status;
#[doc = "BUS_MODE (rw) register accessor: Bus Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bus_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bus_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bus_mode`]
module"]
pub type BUS_MODE = crate::Reg<bus_mode::BUS_MODE_SPEC>;
#[doc = "Bus Mode Register"]
pub mod bus_mode;
#[doc = "TRANSMIT_POLL_DEMAND (rw) register accessor: Transmit Poll Demand Register\n\nYou can [`read`](crate::Reg::read) this register and get [`transmit_poll_demand::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`transmit_poll_demand::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@transmit_poll_demand`]
module"]
pub type TRANSMIT_POLL_DEMAND = crate::Reg<transmit_poll_demand::TRANSMIT_POLL_DEMAND_SPEC>;
#[doc = "Transmit Poll Demand Register"]
pub mod transmit_poll_demand;
#[doc = "RECEIVE_POLL_DEMAND (rw) register accessor: Receive Poll Demand Register\n\nYou can [`read`](crate::Reg::read) this register and get [`receive_poll_demand::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`receive_poll_demand::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@receive_poll_demand`]
module"]
pub type RECEIVE_POLL_DEMAND = crate::Reg<receive_poll_demand::RECEIVE_POLL_DEMAND_SPEC>;
#[doc = "Receive Poll Demand Register"]
pub mod receive_poll_demand;
#[doc = "RECEIVE_DESCRIPTOR_LIST_ADDRESS (rw) register accessor: Receive Descriptor Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`receive_descriptor_list_address::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`receive_descriptor_list_address::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@receive_descriptor_list_address`]
module"]
pub type RECEIVE_DESCRIPTOR_LIST_ADDRESS = crate::Reg<receive_descriptor_list_address::RECEIVE_DESCRIPTOR_LIST_ADDRESS_SPEC>;
#[doc = "Receive Descriptor Address Register"]
pub mod receive_descriptor_list_address;
#[doc = "TRANSMIT_DESCRIPTOR_LIST_ADDRESS (rw) register accessor: Transmit descripter Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`transmit_descriptor_list_address::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`transmit_descriptor_list_address::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@transmit_descriptor_list_address`]
module"]
pub type TRANSMIT_DESCRIPTOR_LIST_ADDRESS = crate::Reg<transmit_descriptor_list_address::TRANSMIT_DESCRIPTOR_LIST_ADDRESS_SPEC>;
#[doc = "Transmit descripter Address Register"]
pub mod transmit_descriptor_list_address;
#[doc = "STATUS (rw) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status Register"]
pub mod status;
#[doc = "OPERATION_MODE (rw) register accessor: Operation Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`operation_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`operation_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@operation_mode`]
module"]
pub type OPERATION_MODE = crate::Reg<operation_mode::OPERATION_MODE_SPEC>;
#[doc = "Operation Mode Register"]
pub mod operation_mode;
#[doc = "INTERRUPT_ENABLE (rw) register accessor: Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt_enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt_enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrupt_enable`]
module"]
pub type INTERRUPT_ENABLE = crate::Reg<interrupt_enable::INTERRUPT_ENABLE_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod interrupt_enable;
#[doc = "MISSED_FRAME_AND_BUFFER_OVERFLOW_COUNTER (r) register accessor: Missed Frame and Buffer Overflow Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`missed_frame_and_buffer_overflow_counter::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@missed_frame_and_buffer_overflow_counter`]
module"]
pub type MISSED_FRAME_AND_BUFFER_OVERFLOW_COUNTER = crate::Reg<missed_frame_and_buffer_overflow_counter::MISSED_FRAME_AND_BUFFER_OVERFLOW_COUNTER_SPEC>;
#[doc = "Missed Frame and Buffer Overflow Counter Register"]
pub mod missed_frame_and_buffer_overflow_counter;
#[doc = "RECEIVE_INTERRUPT_WATCHDOG_TIMER (rw) register accessor: Receive Interrupt Watchdog Timer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`receive_interrupt_watchdog_timer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`receive_interrupt_watchdog_timer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@receive_interrupt_watchdog_timer`]
module"]
pub type RECEIVE_INTERRUPT_WATCHDOG_TIMER = crate::Reg<receive_interrupt_watchdog_timer::RECEIVE_INTERRUPT_WATCHDOG_TIMER_SPEC>;
#[doc = "Receive Interrupt Watchdog Timer Register"]
pub mod receive_interrupt_watchdog_timer;
#[doc = "AHB_STATUS (r) register accessor: AHB Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_status`]
module"]
pub type AHB_STATUS = crate::Reg<ahb_status::AHB_STATUS_SPEC>;
#[doc = "AHB Status Register"]
pub mod ahb_status;
#[doc = "CURRENT_HOST_TRANSMIT_DESCRIPTOR (r) register accessor: Current Host Transmit Descriptor Register\n\nYou can [`read`](crate::Reg::read) this register and get [`current_host_transmit_descriptor::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@current_host_transmit_descriptor`]
module"]
pub type CURRENT_HOST_TRANSMIT_DESCRIPTOR = crate::Reg<current_host_transmit_descriptor::CURRENT_HOST_TRANSMIT_DESCRIPTOR_SPEC>;
#[doc = "Current Host Transmit Descriptor Register"]
pub mod current_host_transmit_descriptor;
#[doc = "CURRENT_HOST_RECEIVE_DESCRIPTOR (r) register accessor: Current Host Receive Descriptor Register\n\nYou can [`read`](crate::Reg::read) this register and get [`current_host_receive_descriptor::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@current_host_receive_descriptor`]
module"]
pub type CURRENT_HOST_RECEIVE_DESCRIPTOR = crate::Reg<current_host_receive_descriptor::CURRENT_HOST_RECEIVE_DESCRIPTOR_SPEC>;
#[doc = "Current Host Receive Descriptor Register"]
pub mod current_host_receive_descriptor;
#[doc = "CURRENT_HOST_TRANSMIT_BUFFER_ADDRESS (r) register accessor: Current Host Transmit Buffer Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`current_host_transmit_buffer_address::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@current_host_transmit_buffer_address`]
module"]
pub type CURRENT_HOST_TRANSMIT_BUFFER_ADDRESS = crate::Reg<current_host_transmit_buffer_address::CURRENT_HOST_TRANSMIT_BUFFER_ADDRESS_SPEC>;
#[doc = "Current Host Transmit Buffer Address Register"]
pub mod current_host_transmit_buffer_address;
#[doc = "CURRENT_HOST_RECEIVE_BUFFER_ADDRESS (r) register accessor: Current Host Receive Buffer Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`current_host_receive_buffer_address::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@current_host_receive_buffer_address`]
module"]
pub type CURRENT_HOST_RECEIVE_BUFFER_ADDRESS = crate::Reg<current_host_receive_buffer_address::CURRENT_HOST_RECEIVE_BUFFER_ADDRESS_SPEC>;
#[doc = "Current Host Receive Buffer Address Register"]
pub mod current_host_receive_buffer_address;
#[doc = "HW_FEATURE (rw) register accessor: HW Feature Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_feature::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_feature::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_feature`]
module"]
pub type HW_FEATURE = crate::Reg<hw_feature::HW_FEATURE_SPEC>;
#[doc = "HW Feature Register"]
pub mod hw_feature;

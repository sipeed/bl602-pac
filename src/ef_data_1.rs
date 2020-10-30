#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 128usize],
    #[doc = "0x80 - reg_key_slot_6_w0."]
    pub reg_key_slot_6_w0: crate::Reg<reg_key_slot_6_w0::REG_KEY_SLOT_6_W0_SPEC>,
    #[doc = "0x84 - reg_key_slot_6_w1."]
    pub reg_key_slot_6_w1: crate::Reg<reg_key_slot_6_w1::REG_KEY_SLOT_6_W1_SPEC>,
    #[doc = "0x88 - reg_key_slot_6_w2."]
    pub reg_key_slot_6_w2: crate::Reg<reg_key_slot_6_w2::REG_KEY_SLOT_6_W2_SPEC>,
    #[doc = "0x8c - reg_key_slot_6_w3."]
    pub reg_key_slot_6_w3: crate::Reg<reg_key_slot_6_w3::REG_KEY_SLOT_6_W3_SPEC>,
    #[doc = "0x90 - reg_key_slot_7_w0."]
    pub reg_key_slot_7_w0: crate::Reg<reg_key_slot_7_w0::REG_KEY_SLOT_7_W0_SPEC>,
    #[doc = "0x94 - reg_key_slot_7_w1."]
    pub reg_key_slot_7_w1: crate::Reg<reg_key_slot_7_w1::REG_KEY_SLOT_7_W1_SPEC>,
    #[doc = "0x98 - reg_key_slot_7_w2."]
    pub reg_key_slot_7_w2: crate::Reg<reg_key_slot_7_w2::REG_KEY_SLOT_7_W2_SPEC>,
    #[doc = "0x9c - reg_key_slot_7_w3."]
    pub reg_key_slot_7_w3: crate::Reg<reg_key_slot_7_w3::REG_KEY_SLOT_7_W3_SPEC>,
    #[doc = "0xa0 - reg_key_slot_8_w0."]
    pub reg_key_slot_8_w0: crate::Reg<reg_key_slot_8_w0::REG_KEY_SLOT_8_W0_SPEC>,
    #[doc = "0xa4 - reg_key_slot_8_w1."]
    pub reg_key_slot_8_w1: crate::Reg<reg_key_slot_8_w1::REG_KEY_SLOT_8_W1_SPEC>,
    #[doc = "0xa8 - reg_key_slot_8_w2."]
    pub reg_key_slot_8_w2: crate::Reg<reg_key_slot_8_w2::REG_KEY_SLOT_8_W2_SPEC>,
    #[doc = "0xac - reg_key_slot_8_w3."]
    pub reg_key_slot_8_w3: crate::Reg<reg_key_slot_8_w3::REG_KEY_SLOT_8_W3_SPEC>,
    #[doc = "0xb0 - reg_key_slot_9_w0."]
    pub reg_key_slot_9_w0: crate::Reg<reg_key_slot_9_w0::REG_KEY_SLOT_9_W0_SPEC>,
    #[doc = "0xb4 - reg_key_slot_9_w1."]
    pub reg_key_slot_9_w1: crate::Reg<reg_key_slot_9_w1::REG_KEY_SLOT_9_W1_SPEC>,
    #[doc = "0xb8 - reg_key_slot_9_w2."]
    pub reg_key_slot_9_w2: crate::Reg<reg_key_slot_9_w2::REG_KEY_SLOT_9_W2_SPEC>,
    #[doc = "0xbc - reg_key_slot_9_w3."]
    pub reg_key_slot_9_w3: crate::Reg<reg_key_slot_9_w3::REG_KEY_SLOT_9_W3_SPEC>,
    #[doc = "0xc0 - reg_key_slot_10_w0."]
    pub reg_key_slot_10_w0: crate::Reg<reg_key_slot_10_w0::REG_KEY_SLOT_10_W0_SPEC>,
    #[doc = "0xc4 - reg_key_slot_10_w1."]
    pub reg_key_slot_10_w1: crate::Reg<reg_key_slot_10_w1::REG_KEY_SLOT_10_W1_SPEC>,
    #[doc = "0xc8 - reg_key_slot_10_w2."]
    pub reg_key_slot_10_w2: crate::Reg<reg_key_slot_10_w2::REG_KEY_SLOT_10_W2_SPEC>,
    #[doc = "0xcc - reg_key_slot_10_w3."]
    pub reg_key_slot_10_w3: crate::Reg<reg_key_slot_10_w3::REG_KEY_SLOT_10_W3_SPEC>,
    #[doc = "0xd0 - reg_key_slot_11_w0."]
    pub reg_key_slot_11_w0: crate::Reg<reg_key_slot_11_w0::REG_KEY_SLOT_11_W0_SPEC>,
    #[doc = "0xd4 - reg_key_slot_11_w1."]
    pub reg_key_slot_11_w1: crate::Reg<reg_key_slot_11_w1::REG_KEY_SLOT_11_W1_SPEC>,
    #[doc = "0xd8 - reg_key_slot_11_w2."]
    pub reg_key_slot_11_w2: crate::Reg<reg_key_slot_11_w2::REG_KEY_SLOT_11_W2_SPEC>,
    #[doc = "0xdc - reg_key_slot_11_w3."]
    pub reg_key_slot_11_w3: crate::Reg<reg_key_slot_11_w3::REG_KEY_SLOT_11_W3_SPEC>,
    #[doc = "0xe0 - reg_data_1_lock."]
    pub reg_data_1_lock: crate::Reg<reg_data_1_lock::REG_DATA_1_LOCK_SPEC>,
}
#[doc = "reg_key_slot_6_w0 register accessor: an alias for `Reg<REG_KEY_SLOT_6_W0_SPEC>`"]
pub type REG_KEY_SLOT_6_W0 = crate::Reg<reg_key_slot_6_w0::REG_KEY_SLOT_6_W0_SPEC>;
#[doc = "reg_key_slot_6_w0."]
pub mod reg_key_slot_6_w0;
#[doc = "reg_key_slot_6_w1 register accessor: an alias for `Reg<REG_KEY_SLOT_6_W1_SPEC>`"]
pub type REG_KEY_SLOT_6_W1 = crate::Reg<reg_key_slot_6_w1::REG_KEY_SLOT_6_W1_SPEC>;
#[doc = "reg_key_slot_6_w1."]
pub mod reg_key_slot_6_w1;
#[doc = "reg_key_slot_6_w2 register accessor: an alias for `Reg<REG_KEY_SLOT_6_W2_SPEC>`"]
pub type REG_KEY_SLOT_6_W2 = crate::Reg<reg_key_slot_6_w2::REG_KEY_SLOT_6_W2_SPEC>;
#[doc = "reg_key_slot_6_w2."]
pub mod reg_key_slot_6_w2;
#[doc = "reg_key_slot_6_w3 register accessor: an alias for `Reg<REG_KEY_SLOT_6_W3_SPEC>`"]
pub type REG_KEY_SLOT_6_W3 = crate::Reg<reg_key_slot_6_w3::REG_KEY_SLOT_6_W3_SPEC>;
#[doc = "reg_key_slot_6_w3."]
pub mod reg_key_slot_6_w3;
#[doc = "reg_key_slot_7_w0 register accessor: an alias for `Reg<REG_KEY_SLOT_7_W0_SPEC>`"]
pub type REG_KEY_SLOT_7_W0 = crate::Reg<reg_key_slot_7_w0::REG_KEY_SLOT_7_W0_SPEC>;
#[doc = "reg_key_slot_7_w0."]
pub mod reg_key_slot_7_w0;
#[doc = "reg_key_slot_7_w1 register accessor: an alias for `Reg<REG_KEY_SLOT_7_W1_SPEC>`"]
pub type REG_KEY_SLOT_7_W1 = crate::Reg<reg_key_slot_7_w1::REG_KEY_SLOT_7_W1_SPEC>;
#[doc = "reg_key_slot_7_w1."]
pub mod reg_key_slot_7_w1;
#[doc = "reg_key_slot_7_w2 register accessor: an alias for `Reg<REG_KEY_SLOT_7_W2_SPEC>`"]
pub type REG_KEY_SLOT_7_W2 = crate::Reg<reg_key_slot_7_w2::REG_KEY_SLOT_7_W2_SPEC>;
#[doc = "reg_key_slot_7_w2."]
pub mod reg_key_slot_7_w2;
#[doc = "reg_key_slot_7_w3 register accessor: an alias for `Reg<REG_KEY_SLOT_7_W3_SPEC>`"]
pub type REG_KEY_SLOT_7_W3 = crate::Reg<reg_key_slot_7_w3::REG_KEY_SLOT_7_W3_SPEC>;
#[doc = "reg_key_slot_7_w3."]
pub mod reg_key_slot_7_w3;
#[doc = "reg_key_slot_8_w0 register accessor: an alias for `Reg<REG_KEY_SLOT_8_W0_SPEC>`"]
pub type REG_KEY_SLOT_8_W0 = crate::Reg<reg_key_slot_8_w0::REG_KEY_SLOT_8_W0_SPEC>;
#[doc = "reg_key_slot_8_w0."]
pub mod reg_key_slot_8_w0;
#[doc = "reg_key_slot_8_w1 register accessor: an alias for `Reg<REG_KEY_SLOT_8_W1_SPEC>`"]
pub type REG_KEY_SLOT_8_W1 = crate::Reg<reg_key_slot_8_w1::REG_KEY_SLOT_8_W1_SPEC>;
#[doc = "reg_key_slot_8_w1."]
pub mod reg_key_slot_8_w1;
#[doc = "reg_key_slot_8_w2 register accessor: an alias for `Reg<REG_KEY_SLOT_8_W2_SPEC>`"]
pub type REG_KEY_SLOT_8_W2 = crate::Reg<reg_key_slot_8_w2::REG_KEY_SLOT_8_W2_SPEC>;
#[doc = "reg_key_slot_8_w2."]
pub mod reg_key_slot_8_w2;
#[doc = "reg_key_slot_8_w3 register accessor: an alias for `Reg<REG_KEY_SLOT_8_W3_SPEC>`"]
pub type REG_KEY_SLOT_8_W3 = crate::Reg<reg_key_slot_8_w3::REG_KEY_SLOT_8_W3_SPEC>;
#[doc = "reg_key_slot_8_w3."]
pub mod reg_key_slot_8_w3;
#[doc = "reg_key_slot_9_w0 register accessor: an alias for `Reg<REG_KEY_SLOT_9_W0_SPEC>`"]
pub type REG_KEY_SLOT_9_W0 = crate::Reg<reg_key_slot_9_w0::REG_KEY_SLOT_9_W0_SPEC>;
#[doc = "reg_key_slot_9_w0."]
pub mod reg_key_slot_9_w0;
#[doc = "reg_key_slot_9_w1 register accessor: an alias for `Reg<REG_KEY_SLOT_9_W1_SPEC>`"]
pub type REG_KEY_SLOT_9_W1 = crate::Reg<reg_key_slot_9_w1::REG_KEY_SLOT_9_W1_SPEC>;
#[doc = "reg_key_slot_9_w1."]
pub mod reg_key_slot_9_w1;
#[doc = "reg_key_slot_9_w2 register accessor: an alias for `Reg<REG_KEY_SLOT_9_W2_SPEC>`"]
pub type REG_KEY_SLOT_9_W2 = crate::Reg<reg_key_slot_9_w2::REG_KEY_SLOT_9_W2_SPEC>;
#[doc = "reg_key_slot_9_w2."]
pub mod reg_key_slot_9_w2;
#[doc = "reg_key_slot_9_w3 register accessor: an alias for `Reg<REG_KEY_SLOT_9_W3_SPEC>`"]
pub type REG_KEY_SLOT_9_W3 = crate::Reg<reg_key_slot_9_w3::REG_KEY_SLOT_9_W3_SPEC>;
#[doc = "reg_key_slot_9_w3."]
pub mod reg_key_slot_9_w3;
#[doc = "reg_key_slot_10_w0 register accessor: an alias for `Reg<REG_KEY_SLOT_10_W0_SPEC>`"]
pub type REG_KEY_SLOT_10_W0 = crate::Reg<reg_key_slot_10_w0::REG_KEY_SLOT_10_W0_SPEC>;
#[doc = "reg_key_slot_10_w0."]
pub mod reg_key_slot_10_w0;
#[doc = "reg_key_slot_10_w1 register accessor: an alias for `Reg<REG_KEY_SLOT_10_W1_SPEC>`"]
pub type REG_KEY_SLOT_10_W1 = crate::Reg<reg_key_slot_10_w1::REG_KEY_SLOT_10_W1_SPEC>;
#[doc = "reg_key_slot_10_w1."]
pub mod reg_key_slot_10_w1;
#[doc = "reg_key_slot_10_w2 register accessor: an alias for `Reg<REG_KEY_SLOT_10_W2_SPEC>`"]
pub type REG_KEY_SLOT_10_W2 = crate::Reg<reg_key_slot_10_w2::REG_KEY_SLOT_10_W2_SPEC>;
#[doc = "reg_key_slot_10_w2."]
pub mod reg_key_slot_10_w2;
#[doc = "reg_key_slot_10_w3 register accessor: an alias for `Reg<REG_KEY_SLOT_10_W3_SPEC>`"]
pub type REG_KEY_SLOT_10_W3 = crate::Reg<reg_key_slot_10_w3::REG_KEY_SLOT_10_W3_SPEC>;
#[doc = "reg_key_slot_10_w3."]
pub mod reg_key_slot_10_w3;
#[doc = "reg_key_slot_11_w0 register accessor: an alias for `Reg<REG_KEY_SLOT_11_W0_SPEC>`"]
pub type REG_KEY_SLOT_11_W0 = crate::Reg<reg_key_slot_11_w0::REG_KEY_SLOT_11_W0_SPEC>;
#[doc = "reg_key_slot_11_w0."]
pub mod reg_key_slot_11_w0;
#[doc = "reg_key_slot_11_w1 register accessor: an alias for `Reg<REG_KEY_SLOT_11_W1_SPEC>`"]
pub type REG_KEY_SLOT_11_W1 = crate::Reg<reg_key_slot_11_w1::REG_KEY_SLOT_11_W1_SPEC>;
#[doc = "reg_key_slot_11_w1."]
pub mod reg_key_slot_11_w1;
#[doc = "reg_key_slot_11_w2 register accessor: an alias for `Reg<REG_KEY_SLOT_11_W2_SPEC>`"]
pub type REG_KEY_SLOT_11_W2 = crate::Reg<reg_key_slot_11_w2::REG_KEY_SLOT_11_W2_SPEC>;
#[doc = "reg_key_slot_11_w2."]
pub mod reg_key_slot_11_w2;
#[doc = "reg_key_slot_11_w3 register accessor: an alias for `Reg<REG_KEY_SLOT_11_W3_SPEC>`"]
pub type REG_KEY_SLOT_11_W3 = crate::Reg<reg_key_slot_11_w3::REG_KEY_SLOT_11_W3_SPEC>;
#[doc = "reg_key_slot_11_w3."]
pub mod reg_key_slot_11_w3;
#[doc = "reg_data_1_lock register accessor: an alias for `Reg<REG_DATA_1_LOCK_SPEC>`"]
pub type REG_DATA_1_LOCK = crate::Reg<reg_data_1_lock::REG_DATA_1_LOCK_SPEC>;
#[doc = "reg_data_1_lock."]
pub mod reg_data_1_lock;

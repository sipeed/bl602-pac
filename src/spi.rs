#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - spi_config."]
    pub spi_config: crate::Reg<spi_config::SPI_CONFIG_SPEC>,
    #[doc = "0x04 - spi_int_sts."]
    pub spi_int_sts: crate::Reg<spi_int_sts::SPI_INT_STS_SPEC>,
    #[doc = "0x08 - spi_bus_busy."]
    pub spi_bus_busy: crate::Reg<spi_bus_busy::SPI_BUS_BUSY_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - spi_prd_0."]
    pub spi_prd_0: crate::Reg<spi_prd_0::SPI_PRD_0_SPEC>,
    #[doc = "0x14 - spi_prd_1."]
    pub spi_prd_1: crate::Reg<spi_prd_1::SPI_PRD_1_SPEC>,
    #[doc = "0x18 - spi_rxd_ignr."]
    pub spi_rxd_ignr: crate::Reg<spi_rxd_ignr::SPI_RXD_IGNR_SPEC>,
    #[doc = "0x1c - spi_sto_value."]
    pub spi_sto_value: crate::Reg<spi_sto_value::SPI_STO_VALUE_SPEC>,
    _reserved7: [u8; 0x60],
    #[doc = "0x80 - spi_fifo_config_0."]
    pub spi_fifo_config_0: crate::Reg<spi_fifo_config_0::SPI_FIFO_CONFIG_0_SPEC>,
    #[doc = "0x84 - spi_fifo_config_1."]
    pub spi_fifo_config_1: crate::Reg<spi_fifo_config_1::SPI_FIFO_CONFIG_1_SPEC>,
    #[doc = "0x88 - spi_fifo_wdata."]
    pub spi_fifo_wdata: crate::Reg<spi_fifo_wdata::SPI_FIFO_WDATA_SPEC>,
    #[doc = "0x8c - spi_fifo_rdata."]
    pub spi_fifo_rdata: crate::Reg<spi_fifo_rdata::SPI_FIFO_RDATA_SPEC>,
}
#[doc = "spi_config register accessor: an alias for `Reg<SPI_CONFIG_SPEC>`"]
pub type SPI_CONFIG = crate::Reg<spi_config::SPI_CONFIG_SPEC>;
#[doc = "spi_config."]
pub mod spi_config;
#[doc = "spi_int_sts register accessor: an alias for `Reg<SPI_INT_STS_SPEC>`"]
pub type SPI_INT_STS = crate::Reg<spi_int_sts::SPI_INT_STS_SPEC>;
#[doc = "spi_int_sts."]
pub mod spi_int_sts;
#[doc = "spi_bus_busy register accessor: an alias for `Reg<SPI_BUS_BUSY_SPEC>`"]
pub type SPI_BUS_BUSY = crate::Reg<spi_bus_busy::SPI_BUS_BUSY_SPEC>;
#[doc = "spi_bus_busy."]
pub mod spi_bus_busy;
#[doc = "spi_prd_0 register accessor: an alias for `Reg<SPI_PRD_0_SPEC>`"]
pub type SPI_PRD_0 = crate::Reg<spi_prd_0::SPI_PRD_0_SPEC>;
#[doc = "spi_prd_0."]
pub mod spi_prd_0;
#[doc = "spi_prd_1 register accessor: an alias for `Reg<SPI_PRD_1_SPEC>`"]
pub type SPI_PRD_1 = crate::Reg<spi_prd_1::SPI_PRD_1_SPEC>;
#[doc = "spi_prd_1."]
pub mod spi_prd_1;
#[doc = "spi_rxd_ignr register accessor: an alias for `Reg<SPI_RXD_IGNR_SPEC>`"]
pub type SPI_RXD_IGNR = crate::Reg<spi_rxd_ignr::SPI_RXD_IGNR_SPEC>;
#[doc = "spi_rxd_ignr."]
pub mod spi_rxd_ignr;
#[doc = "spi_sto_value register accessor: an alias for `Reg<SPI_STO_VALUE_SPEC>`"]
pub type SPI_STO_VALUE = crate::Reg<spi_sto_value::SPI_STO_VALUE_SPEC>;
#[doc = "spi_sto_value."]
pub mod spi_sto_value;
#[doc = "spi_fifo_config_0 register accessor: an alias for `Reg<SPI_FIFO_CONFIG_0_SPEC>`"]
pub type SPI_FIFO_CONFIG_0 = crate::Reg<spi_fifo_config_0::SPI_FIFO_CONFIG_0_SPEC>;
#[doc = "spi_fifo_config_0."]
pub mod spi_fifo_config_0;
#[doc = "spi_fifo_config_1 register accessor: an alias for `Reg<SPI_FIFO_CONFIG_1_SPEC>`"]
pub type SPI_FIFO_CONFIG_1 = crate::Reg<spi_fifo_config_1::SPI_FIFO_CONFIG_1_SPEC>;
#[doc = "spi_fifo_config_1."]
pub mod spi_fifo_config_1;
#[doc = "spi_fifo_wdata register accessor: an alias for `Reg<SPI_FIFO_WDATA_SPEC>`"]
pub type SPI_FIFO_WDATA = crate::Reg<spi_fifo_wdata::SPI_FIFO_WDATA_SPEC>;
#[doc = "spi_fifo_wdata."]
pub mod spi_fifo_wdata;
#[doc = "spi_fifo_rdata register accessor: an alias for `Reg<SPI_FIFO_RDATA_SPEC>`"]
pub type SPI_FIFO_RDATA = crate::Reg<spi_fifo_rdata::SPI_FIFO_RDATA_SPEC>;
#[doc = "spi_fifo_rdata."]
pub mod spi_fifo_rdata;

#[doc = "Register `PDS_CTL` reader"]
pub struct R(crate::R<PDS_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDS_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDS_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDS_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDS_CTL` writer"]
pub struct W(crate::W<PDS_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDS_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<PDS_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDS_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_pds_ctrl_pll` reader - "]
pub struct CR_PDS_CTRL_PLL_R(crate::FieldReader<u8, u8>);
impl CR_PDS_CTRL_PLL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CR_PDS_CTRL_PLL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_PDS_CTRL_PLL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_pds_ctrl_pll` writer - "]
pub struct CR_PDS_CTRL_PLL_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_CTRL_PLL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
#[doc = "Field `cr_pds_ctrl_rf` reader - "]
pub struct CR_PDS_CTRL_RF_R(crate::FieldReader<u8, u8>);
impl CR_PDS_CTRL_RF_R {
    pub(crate) fn new(bits: u8) -> Self {
        CR_PDS_CTRL_RF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_PDS_CTRL_RF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_pds_ctrl_rf` writer - "]
pub struct CR_PDS_CTRL_RF_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_CTRL_RF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
#[doc = "Field `cr_pds_ldo_vol` reader - "]
pub struct CR_PDS_LDO_VOL_R(crate::FieldReader<u8, u8>);
impl CR_PDS_LDO_VOL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CR_PDS_LDO_VOL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_PDS_LDO_VOL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_pds_ldo_vol` writer - "]
pub struct CR_PDS_LDO_VOL_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_LDO_VOL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `cr_pds_pd_ldo11` reader - "]
pub struct CR_PDS_PD_LDO11_R(crate::FieldReader<bool, bool>);
impl CR_PDS_PD_LDO11_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_PDS_PD_LDO11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_PDS_PD_LDO11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_pds_pd_ldo11` writer - "]
pub struct CR_PDS_PD_LDO11_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_PD_LDO11_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `cr_np_wfi_mask` reader - "]
pub struct CR_NP_WFI_MASK_R(crate::FieldReader<bool, bool>);
impl CR_NP_WFI_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_NP_WFI_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_NP_WFI_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_np_wfi_mask` writer - "]
pub struct CR_NP_WFI_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_NP_WFI_MASK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `cr_pds_ldo_vsel_en` reader - "]
pub struct CR_PDS_LDO_VSEL_EN_R(crate::FieldReader<bool, bool>);
impl CR_PDS_LDO_VSEL_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_PDS_LDO_VSEL_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_PDS_LDO_VSEL_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_pds_ldo_vsel_en` writer - "]
pub struct CR_PDS_LDO_VSEL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_LDO_VSEL_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `cr_pds_rc32m_off_dis` reader - "]
pub struct CR_PDS_RC32M_OFF_DIS_R(crate::FieldReader<bool, bool>);
impl CR_PDS_RC32M_OFF_DIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_PDS_RC32M_OFF_DIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_PDS_RC32M_OFF_DIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_pds_rc32m_off_dis` writer - "]
pub struct CR_PDS_RC32M_OFF_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_RC32M_OFF_DIS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `cr_pds_rst_soc_en` reader - "]
pub struct CR_PDS_RST_SOC_EN_R(crate::FieldReader<bool, bool>);
impl CR_PDS_RST_SOC_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_PDS_RST_SOC_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_PDS_RST_SOC_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_pds_rst_soc_en` writer - "]
pub struct CR_PDS_RST_SOC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_RST_SOC_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `cr_pds_soc_enb_force_on` reader - "]
pub struct CR_PDS_SOC_ENB_FORCE_ON_R(crate::FieldReader<bool, bool>);
impl CR_PDS_SOC_ENB_FORCE_ON_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_PDS_SOC_ENB_FORCE_ON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_PDS_SOC_ENB_FORCE_ON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_pds_soc_enb_force_on` writer - "]
pub struct CR_PDS_SOC_ENB_FORCE_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_SOC_ENB_FORCE_ON_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `cr_pds_pd_xtal` reader - "]
pub struct CR_PDS_PD_XTAL_R(crate::FieldReader<bool, bool>);
impl CR_PDS_PD_XTAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_PDS_PD_XTAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_PDS_PD_XTAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_pds_pd_xtal` writer - "]
pub struct CR_PDS_PD_XTAL_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_PD_XTAL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `cr_pds_pwr_off` reader - "]
pub struct CR_PDS_PWR_OFF_R(crate::FieldReader<bool, bool>);
impl CR_PDS_PWR_OFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_PDS_PWR_OFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_PDS_PWR_OFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_pds_pwr_off` writer - "]
pub struct CR_PDS_PWR_OFF_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_PWR_OFF_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `cr_pds_wait_xtal_rdy` reader - "]
pub struct CR_PDS_WAIT_XTAL_RDY_R(crate::FieldReader<bool, bool>);
impl CR_PDS_WAIT_XTAL_RDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_PDS_WAIT_XTAL_RDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_PDS_WAIT_XTAL_RDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_pds_wait_xtal_rdy` writer - "]
pub struct CR_PDS_WAIT_XTAL_RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_WAIT_XTAL_RDY_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `cr_pds_iso_en` reader - "]
pub struct CR_PDS_ISO_EN_R(crate::FieldReader<bool, bool>);
impl CR_PDS_ISO_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_PDS_ISO_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_PDS_ISO_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_pds_iso_en` writer - "]
pub struct CR_PDS_ISO_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_ISO_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `cr_pds_mem_stby` reader - "]
pub struct CR_PDS_MEM_STBY_R(crate::FieldReader<bool, bool>);
impl CR_PDS_MEM_STBY_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_PDS_MEM_STBY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_PDS_MEM_STBY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_pds_mem_stby` writer - "]
pub struct CR_PDS_MEM_STBY_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_MEM_STBY_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `cr_pds_gate_clk` reader - "]
pub struct CR_PDS_GATE_CLK_R(crate::FieldReader<bool, bool>);
impl CR_PDS_GATE_CLK_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_PDS_GATE_CLK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_PDS_GATE_CLK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_pds_gate_clk` writer - "]
pub struct CR_PDS_GATE_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_GATE_CLK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `cr_pds_pd_bg_sys` reader - "]
pub struct CR_PDS_PD_BG_SYS_R(crate::FieldReader<bool, bool>);
impl CR_PDS_PD_BG_SYS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_PDS_PD_BG_SYS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_PDS_PD_BG_SYS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_pds_pd_bg_sys` writer - "]
pub struct CR_PDS_PD_BG_SYS_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_PD_BG_SYS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `cr_pds_pd_dcdc18` reader - "]
pub struct CR_PDS_PD_DCDC18_R(crate::FieldReader<bool, bool>);
impl CR_PDS_PD_DCDC18_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_PDS_PD_DCDC18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_PDS_PD_DCDC18_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_pds_pd_dcdc18` writer - "]
pub struct CR_PDS_PD_DCDC18_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_PD_DCDC18_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `cr_wifi_pds_save_state` reader - "]
pub struct CR_WIFI_PDS_SAVE_STATE_R(crate::FieldReader<bool, bool>);
impl CR_WIFI_PDS_SAVE_STATE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_WIFI_PDS_SAVE_STATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_WIFI_PDS_SAVE_STATE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_wifi_pds_save_state` writer - "]
pub struct CR_WIFI_PDS_SAVE_STATE_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_WIFI_PDS_SAVE_STATE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `cr_xtal_force_off` reader - "]
pub struct CR_XTAL_FORCE_OFF_R(crate::FieldReader<bool, bool>);
impl CR_XTAL_FORCE_OFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_XTAL_FORCE_OFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_XTAL_FORCE_OFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_xtal_force_off` writer - "]
pub struct CR_XTAL_FORCE_OFF_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_XTAL_FORCE_OFF_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `cr_sleep_forever` reader - "]
pub struct CR_SLEEP_FOREVER_R(crate::FieldReader<bool, bool>);
impl CR_SLEEP_FOREVER_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_SLEEP_FOREVER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_SLEEP_FOREVER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_sleep_forever` writer - "]
pub struct CR_SLEEP_FOREVER_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_SLEEP_FOREVER_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `pds_start_ps` reader - "]
pub struct PDS_START_PS_R(crate::FieldReader<bool, bool>);
impl PDS_START_PS_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDS_START_PS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDS_START_PS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pds_start_ps` writer - "]
pub struct PDS_START_PS_W<'a> {
    w: &'a mut W,
}
impl<'a> PDS_START_PS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn cr_pds_ctrl_pll(&self) -> CR_PDS_CTRL_PLL_R {
        CR_PDS_CTRL_PLL_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn cr_pds_ctrl_rf(&self) -> CR_PDS_CTRL_RF_R {
        CR_PDS_CTRL_RF_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn cr_pds_ldo_vol(&self) -> CR_PDS_LDO_VOL_R {
        CR_PDS_LDO_VOL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn cr_pds_pd_ldo11(&self) -> CR_PDS_PD_LDO11_R {
        CR_PDS_PD_LDO11_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn cr_np_wfi_mask(&self) -> CR_NP_WFI_MASK_R {
        CR_NP_WFI_MASK_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn cr_pds_ldo_vsel_en(&self) -> CR_PDS_LDO_VSEL_EN_R {
        CR_PDS_LDO_VSEL_EN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn cr_pds_rc32m_off_dis(&self) -> CR_PDS_RC32M_OFF_DIS_R {
        CR_PDS_RC32M_OFF_DIS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cr_pds_rst_soc_en(&self) -> CR_PDS_RST_SOC_EN_R {
        CR_PDS_RST_SOC_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn cr_pds_soc_enb_force_on(&self) -> CR_PDS_SOC_ENB_FORCE_ON_R {
        CR_PDS_SOC_ENB_FORCE_ON_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cr_pds_pd_xtal(&self) -> CR_PDS_PD_XTAL_R {
        CR_PDS_PD_XTAL_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cr_pds_pwr_off(&self) -> CR_PDS_PWR_OFF_R {
        CR_PDS_PWR_OFF_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn cr_pds_wait_xtal_rdy(&self) -> CR_PDS_WAIT_XTAL_RDY_R {
        CR_PDS_WAIT_XTAL_RDY_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cr_pds_iso_en(&self) -> CR_PDS_ISO_EN_R {
        CR_PDS_ISO_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cr_pds_mem_stby(&self) -> CR_PDS_MEM_STBY_R {
        CR_PDS_MEM_STBY_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_pds_gate_clk(&self) -> CR_PDS_GATE_CLK_R {
        CR_PDS_GATE_CLK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_pds_pd_bg_sys(&self) -> CR_PDS_PD_BG_SYS_R {
        CR_PDS_PD_BG_SYS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_pds_pd_dcdc18(&self) -> CR_PDS_PD_DCDC18_R {
        CR_PDS_PD_DCDC18_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_wifi_pds_save_state(&self) -> CR_WIFI_PDS_SAVE_STATE_R {
        CR_WIFI_PDS_SAVE_STATE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_xtal_force_off(&self) -> CR_XTAL_FORCE_OFF_R {
        CR_XTAL_FORCE_OFF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_sleep_forever(&self) -> CR_SLEEP_FOREVER_R {
        CR_SLEEP_FOREVER_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pds_start_ps(&self) -> PDS_START_PS_R {
        PDS_START_PS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn cr_pds_ctrl_pll(&mut self) -> CR_PDS_CTRL_PLL_W {
        CR_PDS_CTRL_PLL_W { w: self }
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn cr_pds_ctrl_rf(&mut self) -> CR_PDS_CTRL_RF_W {
        CR_PDS_CTRL_RF_W { w: self }
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn cr_pds_ldo_vol(&mut self) -> CR_PDS_LDO_VOL_W {
        CR_PDS_LDO_VOL_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn cr_pds_pd_ldo11(&mut self) -> CR_PDS_PD_LDO11_W {
        CR_PDS_PD_LDO11_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn cr_np_wfi_mask(&mut self) -> CR_NP_WFI_MASK_W {
        CR_NP_WFI_MASK_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn cr_pds_ldo_vsel_en(&mut self) -> CR_PDS_LDO_VSEL_EN_W {
        CR_PDS_LDO_VSEL_EN_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn cr_pds_rc32m_off_dis(&mut self) -> CR_PDS_RC32M_OFF_DIS_W {
        CR_PDS_RC32M_OFF_DIS_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cr_pds_rst_soc_en(&mut self) -> CR_PDS_RST_SOC_EN_W {
        CR_PDS_RST_SOC_EN_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn cr_pds_soc_enb_force_on(&mut self) -> CR_PDS_SOC_ENB_FORCE_ON_W {
        CR_PDS_SOC_ENB_FORCE_ON_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cr_pds_pd_xtal(&mut self) -> CR_PDS_PD_XTAL_W {
        CR_PDS_PD_XTAL_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cr_pds_pwr_off(&mut self) -> CR_PDS_PWR_OFF_W {
        CR_PDS_PWR_OFF_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn cr_pds_wait_xtal_rdy(&mut self) -> CR_PDS_WAIT_XTAL_RDY_W {
        CR_PDS_WAIT_XTAL_RDY_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cr_pds_iso_en(&mut self) -> CR_PDS_ISO_EN_W {
        CR_PDS_ISO_EN_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cr_pds_mem_stby(&mut self) -> CR_PDS_MEM_STBY_W {
        CR_PDS_MEM_STBY_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_pds_gate_clk(&mut self) -> CR_PDS_GATE_CLK_W {
        CR_PDS_GATE_CLK_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_pds_pd_bg_sys(&mut self) -> CR_PDS_PD_BG_SYS_W {
        CR_PDS_PD_BG_SYS_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_pds_pd_dcdc18(&mut self) -> CR_PDS_PD_DCDC18_W {
        CR_PDS_PD_DCDC18_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_wifi_pds_save_state(&mut self) -> CR_WIFI_PDS_SAVE_STATE_W {
        CR_WIFI_PDS_SAVE_STATE_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_xtal_force_off(&mut self) -> CR_XTAL_FORCE_OFF_W {
        CR_XTAL_FORCE_OFF_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_sleep_forever(&mut self) -> CR_SLEEP_FOREVER_W {
        CR_SLEEP_FOREVER_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pds_start_ps(&mut self) -> PDS_START_PS_W {
        PDS_START_PS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDS_CTL.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pds_ctl](index.html) module"]
pub struct PDS_CTL_SPEC;
impl crate::RegisterSpec for PDS_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pds_ctl::R](R) reader structure"]
impl crate::Readable for PDS_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pds_ctl::W](W) writer structure"]
impl crate::Writable for PDS_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDS_CTL to value 0x1a00_6b00"]
impl crate::Resettable for PDS_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1a00_6b00
    }
}

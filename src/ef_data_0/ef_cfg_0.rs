#[doc = "Register `ef_cfg_0` reader"]
pub struct R(crate::R<EF_CFG_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EF_CFG_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EF_CFG_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EF_CFG_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ef_cfg_0` writer"]
pub struct W(crate::W<EF_CFG_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EF_CFG_0_SPEC>;
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
impl From<crate::W<EF_CFG_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EF_CFG_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ef_dbg_mode` reader - "]
pub struct EF_DBG_MODE_R(crate::FieldReader<u8, u8>);
impl EF_DBG_MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        EF_DBG_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_DBG_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ef_dbg_mode` writer - "]
pub struct EF_DBG_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_DBG_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
#[doc = "Field `ef_dbg_jtag_0_dis` reader - "]
pub struct EF_DBG_JTAG_0_DIS_R(crate::FieldReader<u8, u8>);
impl EF_DBG_JTAG_0_DIS_R {
    pub(crate) fn new(bits: u8) -> Self {
        EF_DBG_JTAG_0_DIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_DBG_JTAG_0_DIS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ef_dbg_jtag_0_dis` writer - "]
pub struct EF_DBG_JTAG_0_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_DBG_JTAG_0_DIS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | ((value as u32 & 0x03) << 26);
        self.w
    }
}
#[doc = "Field `ef_dbg_jtag_1_dis` reader - "]
pub struct EF_DBG_JTAG_1_DIS_R(crate::FieldReader<u8, u8>);
impl EF_DBG_JTAG_1_DIS_R {
    pub(crate) fn new(bits: u8) -> Self {
        EF_DBG_JTAG_1_DIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_DBG_JTAG_1_DIS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ef_dbg_jtag_1_dis` writer - "]
pub struct EF_DBG_JTAG_1_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_DBG_JTAG_1_DIS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "Field `ef_efuse_dbg_dis` reader - "]
pub struct EF_EFUSE_DBG_DIS_R(crate::FieldReader<bool, bool>);
impl EF_EFUSE_DBG_DIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        EF_EFUSE_DBG_DIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_EFUSE_DBG_DIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ef_efuse_dbg_dis` writer - "]
pub struct EF_EFUSE_DBG_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_EFUSE_DBG_DIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `ef_se_dbg_dis` reader - "]
pub struct EF_SE_DBG_DIS_R(crate::FieldReader<bool, bool>);
impl EF_SE_DBG_DIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        EF_SE_DBG_DIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_SE_DBG_DIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ef_se_dbg_dis` writer - "]
pub struct EF_SE_DBG_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_SE_DBG_DIS_W<'a> {
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
#[doc = "Field `ef_cpu_rst_dbg_dis` reader - "]
pub struct EF_CPU_RST_DBG_DIS_R(crate::FieldReader<bool, bool>);
impl EF_CPU_RST_DBG_DIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        EF_CPU_RST_DBG_DIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_CPU_RST_DBG_DIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ef_cpu_rst_dbg_dis` writer - "]
pub struct EF_CPU_RST_DBG_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_CPU_RST_DBG_DIS_W<'a> {
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
#[doc = "Field `ef_cpu1_dis` reader - "]
pub struct EF_CPU1_DIS_R(crate::FieldReader<bool, bool>);
impl EF_CPU1_DIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        EF_CPU1_DIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_CPU1_DIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ef_cpu1_dis` writer - "]
pub struct EF_CPU1_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_CPU1_DIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `ef_sf_dis` reader - "]
pub struct EF_SF_DIS_R(crate::FieldReader<bool, bool>);
impl EF_SF_DIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        EF_SF_DIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_SF_DIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ef_sf_dis` writer - "]
pub struct EF_SF_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_SF_DIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `ef_cam_dis` reader - "]
pub struct EF_CAM_DIS_R(crate::FieldReader<bool, bool>);
impl EF_CAM_DIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        EF_CAM_DIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_CAM_DIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ef_cam_dis` writer - "]
pub struct EF_CAM_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_CAM_DIS_W<'a> {
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
#[doc = "Field `ef_0_key_enc_en` reader - "]
pub struct EF_0_KEY_ENC_EN_R(crate::FieldReader<bool, bool>);
impl EF_0_KEY_ENC_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        EF_0_KEY_ENC_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_0_KEY_ENC_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ef_0_key_enc_en` writer - "]
pub struct EF_0_KEY_ENC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_0_KEY_ENC_EN_W<'a> {
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
#[doc = "Field `ef_wifi_dis` reader - "]
pub struct EF_WIFI_DIS_R(crate::FieldReader<bool, bool>);
impl EF_WIFI_DIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        EF_WIFI_DIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_WIFI_DIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ef_wifi_dis` writer - "]
pub struct EF_WIFI_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_WIFI_DIS_W<'a> {
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
#[doc = "Field `ef_ble_dis` reader - "]
pub struct EF_BLE_DIS_R(crate::FieldReader<bool, bool>);
impl EF_BLE_DIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        EF_BLE_DIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_BLE_DIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ef_ble_dis` writer - "]
pub struct EF_BLE_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_BLE_DIS_W<'a> {
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
#[doc = "Field `ef_sdu_dis` reader - "]
pub struct EF_SDU_DIS_R(crate::FieldReader<bool, bool>);
impl EF_SDU_DIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        EF_SDU_DIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_SDU_DIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ef_sdu_dis` writer - "]
pub struct EF_SDU_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_SDU_DIS_W<'a> {
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
#[doc = "Field `ef_sw_usage_1` reader - "]
pub struct EF_SW_USAGE_1_R(crate::FieldReader<u8, u8>);
impl EF_SW_USAGE_1_R {
    pub(crate) fn new(bits: u8) -> Self {
        EF_SW_USAGE_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_SW_USAGE_1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ef_sw_usage_1` writer - "]
pub struct EF_SW_USAGE_1_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_SW_USAGE_1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `ef_boot_sel` reader - "]
pub struct EF_BOOT_SEL_R(crate::FieldReader<u8, u8>);
impl EF_BOOT_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        EF_BOOT_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_BOOT_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ef_boot_sel` writer - "]
pub struct EF_BOOT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_BOOT_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `ef_cpu0_enc_en` reader - "]
pub struct EF_CPU0_ENC_EN_R(crate::FieldReader<bool, bool>);
impl EF_CPU0_ENC_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        EF_CPU0_ENC_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_CPU0_ENC_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ef_cpu0_enc_en` writer - "]
pub struct EF_CPU0_ENC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_CPU0_ENC_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `ef_cpu1_enc_en` reader - "]
pub struct EF_CPU1_ENC_EN_R(crate::FieldReader<bool, bool>);
impl EF_CPU1_ENC_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        EF_CPU1_ENC_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_CPU1_ENC_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ef_cpu1_enc_en` writer - "]
pub struct EF_CPU1_ENC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_CPU1_ENC_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `ef_sboot_en` reader - "]
pub struct EF_SBOOT_EN_R(crate::FieldReader<u8, u8>);
impl EF_SBOOT_EN_R {
    pub(crate) fn new(bits: u8) -> Self {
        EF_SBOOT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_SBOOT_EN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ef_sboot_en` writer - "]
pub struct EF_SBOOT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_SBOOT_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `ef_sboot_sign_mode` reader - "]
pub struct EF_SBOOT_SIGN_MODE_R(crate::FieldReader<u8, u8>);
impl EF_SBOOT_SIGN_MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        EF_SBOOT_SIGN_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_SBOOT_SIGN_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ef_sboot_sign_mode` writer - "]
pub struct EF_SBOOT_SIGN_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_SBOOT_SIGN_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `ef_sf_aes_mode` reader - "]
pub struct EF_SF_AES_MODE_R(crate::FieldReader<u8, u8>);
impl EF_SF_AES_MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        EF_SF_AES_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_SF_AES_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ef_sf_aes_mode` writer - "]
pub struct EF_SF_AES_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_SF_AES_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn ef_dbg_mode(&self) -> EF_DBG_MODE_R {
        EF_DBG_MODE_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn ef_dbg_jtag_0_dis(&self) -> EF_DBG_JTAG_0_DIS_R {
        EF_DBG_JTAG_0_DIS_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn ef_dbg_jtag_1_dis(&self) -> EF_DBG_JTAG_1_DIS_R {
        EF_DBG_JTAG_1_DIS_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn ef_efuse_dbg_dis(&self) -> EF_EFUSE_DBG_DIS_R {
        EF_EFUSE_DBG_DIS_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn ef_se_dbg_dis(&self) -> EF_SE_DBG_DIS_R {
        EF_SE_DBG_DIS_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ef_cpu_rst_dbg_dis(&self) -> EF_CPU_RST_DBG_DIS_R {
        EF_CPU_RST_DBG_DIS_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ef_cpu1_dis(&self) -> EF_CPU1_DIS_R {
        EF_CPU1_DIS_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ef_sf_dis(&self) -> EF_SF_DIS_R {
        EF_SF_DIS_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn ef_cam_dis(&self) -> EF_CAM_DIS_R {
        EF_CAM_DIS_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn ef_0_key_enc_en(&self) -> EF_0_KEY_ENC_EN_R {
        EF_0_KEY_ENC_EN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ef_wifi_dis(&self) -> EF_WIFI_DIS_R {
        EF_WIFI_DIS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ef_ble_dis(&self) -> EF_BLE_DIS_R {
        EF_BLE_DIS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ef_sdu_dis(&self) -> EF_SDU_DIS_R {
        EF_SDU_DIS_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn ef_sw_usage_1(&self) -> EF_SW_USAGE_1_R {
        EF_SW_USAGE_1_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn ef_boot_sel(&self) -> EF_BOOT_SEL_R {
        EF_BOOT_SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ef_cpu0_enc_en(&self) -> EF_CPU0_ENC_EN_R {
        EF_CPU0_ENC_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ef_cpu1_enc_en(&self) -> EF_CPU1_ENC_EN_R {
        EF_CPU1_ENC_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn ef_sboot_en(&self) -> EF_SBOOT_EN_R {
        EF_SBOOT_EN_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn ef_sboot_sign_mode(&self) -> EF_SBOOT_SIGN_MODE_R {
        EF_SBOOT_SIGN_MODE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn ef_sf_aes_mode(&self) -> EF_SF_AES_MODE_R {
        EF_SF_AES_MODE_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn ef_dbg_mode(&mut self) -> EF_DBG_MODE_W {
        EF_DBG_MODE_W { w: self }
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn ef_dbg_jtag_0_dis(&mut self) -> EF_DBG_JTAG_0_DIS_W {
        EF_DBG_JTAG_0_DIS_W { w: self }
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn ef_dbg_jtag_1_dis(&mut self) -> EF_DBG_JTAG_1_DIS_W {
        EF_DBG_JTAG_1_DIS_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn ef_efuse_dbg_dis(&mut self) -> EF_EFUSE_DBG_DIS_W {
        EF_EFUSE_DBG_DIS_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn ef_se_dbg_dis(&mut self) -> EF_SE_DBG_DIS_W {
        EF_SE_DBG_DIS_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ef_cpu_rst_dbg_dis(&mut self) -> EF_CPU_RST_DBG_DIS_W {
        EF_CPU_RST_DBG_DIS_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ef_cpu1_dis(&mut self) -> EF_CPU1_DIS_W {
        EF_CPU1_DIS_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ef_sf_dis(&mut self) -> EF_SF_DIS_W {
        EF_SF_DIS_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn ef_cam_dis(&mut self) -> EF_CAM_DIS_W {
        EF_CAM_DIS_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn ef_0_key_enc_en(&mut self) -> EF_0_KEY_ENC_EN_W {
        EF_0_KEY_ENC_EN_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ef_wifi_dis(&mut self) -> EF_WIFI_DIS_W {
        EF_WIFI_DIS_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ef_ble_dis(&mut self) -> EF_BLE_DIS_W {
        EF_BLE_DIS_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ef_sdu_dis(&mut self) -> EF_SDU_DIS_W {
        EF_SDU_DIS_W { w: self }
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn ef_sw_usage_1(&mut self) -> EF_SW_USAGE_1_W {
        EF_SW_USAGE_1_W { w: self }
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn ef_boot_sel(&mut self) -> EF_BOOT_SEL_W {
        EF_BOOT_SEL_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ef_cpu0_enc_en(&mut self) -> EF_CPU0_ENC_EN_W {
        EF_CPU0_ENC_EN_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ef_cpu1_enc_en(&mut self) -> EF_CPU1_ENC_EN_W {
        EF_CPU1_ENC_EN_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn ef_sboot_en(&mut self) -> EF_SBOOT_EN_W {
        EF_SBOOT_EN_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn ef_sboot_sign_mode(&mut self) -> EF_SBOOT_SIGN_MODE_W {
        EF_SBOOT_SIGN_MODE_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn ef_sf_aes_mode(&mut self) -> EF_SF_AES_MODE_W {
        EF_SF_AES_MODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ef_cfg_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_cfg_0](index.html) module"]
pub struct EF_CFG_0_SPEC;
impl crate::RegisterSpec for EF_CFG_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ef_cfg_0::R](R) reader structure"]
impl crate::Readable for EF_CFG_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ef_cfg_0::W](W) writer structure"]
impl crate::Writable for EF_CFG_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ef_cfg_0 to value 0"]
impl crate::Resettable for EF_CFG_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

#[doc = "Register `ef_data_0_lock` reader"]
pub struct R(crate::R<EF_DATA_0_LOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EF_DATA_0_LOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EF_DATA_0_LOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EF_DATA_0_LOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ef_data_0_lock` writer"]
pub struct W(crate::W<EF_DATA_0_LOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EF_DATA_0_LOCK_SPEC>;
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
impl From<crate::W<EF_DATA_0_LOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EF_DATA_0_LOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rd_lock_key_slot_5` reader - "]
pub struct RD_LOCK_KEY_SLOT_5_R(crate::FieldReader<bool, bool>);
impl RD_LOCK_KEY_SLOT_5_R {
    pub(crate) fn new(bits: bool) -> Self {
        RD_LOCK_KEY_SLOT_5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_LOCK_KEY_SLOT_5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rd_lock_key_slot_5` writer - "]
pub struct RD_LOCK_KEY_SLOT_5_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_LOCK_KEY_SLOT_5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
#[doc = "Field `rd_lock_key_slot_4` reader - "]
pub struct RD_LOCK_KEY_SLOT_4_R(crate::FieldReader<bool, bool>);
impl RD_LOCK_KEY_SLOT_4_R {
    pub(crate) fn new(bits: bool) -> Self {
        RD_LOCK_KEY_SLOT_4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_LOCK_KEY_SLOT_4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rd_lock_key_slot_4` writer - "]
pub struct RD_LOCK_KEY_SLOT_4_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_LOCK_KEY_SLOT_4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `rd_lock_key_slot_3` reader - "]
pub struct RD_LOCK_KEY_SLOT_3_R(crate::FieldReader<bool, bool>);
impl RD_LOCK_KEY_SLOT_3_R {
    pub(crate) fn new(bits: bool) -> Self {
        RD_LOCK_KEY_SLOT_3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_LOCK_KEY_SLOT_3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rd_lock_key_slot_3` writer - "]
pub struct RD_LOCK_KEY_SLOT_3_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_LOCK_KEY_SLOT_3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `rd_lock_key_slot_2` reader - "]
pub struct RD_LOCK_KEY_SLOT_2_R(crate::FieldReader<bool, bool>);
impl RD_LOCK_KEY_SLOT_2_R {
    pub(crate) fn new(bits: bool) -> Self {
        RD_LOCK_KEY_SLOT_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_LOCK_KEY_SLOT_2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rd_lock_key_slot_2` writer - "]
pub struct RD_LOCK_KEY_SLOT_2_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_LOCK_KEY_SLOT_2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `rd_lock_key_slot_1` reader - "]
pub struct RD_LOCK_KEY_SLOT_1_R(crate::FieldReader<bool, bool>);
impl RD_LOCK_KEY_SLOT_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        RD_LOCK_KEY_SLOT_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_LOCK_KEY_SLOT_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rd_lock_key_slot_1` writer - "]
pub struct RD_LOCK_KEY_SLOT_1_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_LOCK_KEY_SLOT_1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `rd_lock_key_slot_0` reader - "]
pub struct RD_LOCK_KEY_SLOT_0_R(crate::FieldReader<bool, bool>);
impl RD_LOCK_KEY_SLOT_0_R {
    pub(crate) fn new(bits: bool) -> Self {
        RD_LOCK_KEY_SLOT_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_LOCK_KEY_SLOT_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rd_lock_key_slot_0` writer - "]
pub struct RD_LOCK_KEY_SLOT_0_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_LOCK_KEY_SLOT_0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `rd_lock_dbg_pwd` reader - "]
pub struct RD_LOCK_DBG_PWD_R(crate::FieldReader<bool, bool>);
impl RD_LOCK_DBG_PWD_R {
    pub(crate) fn new(bits: bool) -> Self {
        RD_LOCK_DBG_PWD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_LOCK_DBG_PWD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rd_lock_dbg_pwd` writer - "]
pub struct RD_LOCK_DBG_PWD_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_LOCK_DBG_PWD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `wr_lock_key_slot_5_h` reader - "]
pub struct WR_LOCK_KEY_SLOT_5_H_R(crate::FieldReader<bool, bool>);
impl WR_LOCK_KEY_SLOT_5_H_R {
    pub(crate) fn new(bits: bool) -> Self {
        WR_LOCK_KEY_SLOT_5_H_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WR_LOCK_KEY_SLOT_5_H_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `wr_lock_key_slot_5_h` writer - "]
pub struct WR_LOCK_KEY_SLOT_5_H_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_LOCK_KEY_SLOT_5_H_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `wr_lock_key_slot_4_h` reader - "]
pub struct WR_LOCK_KEY_SLOT_4_H_R(crate::FieldReader<bool, bool>);
impl WR_LOCK_KEY_SLOT_4_H_R {
    pub(crate) fn new(bits: bool) -> Self {
        WR_LOCK_KEY_SLOT_4_H_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WR_LOCK_KEY_SLOT_4_H_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `wr_lock_key_slot_4_h` writer - "]
pub struct WR_LOCK_KEY_SLOT_4_H_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_LOCK_KEY_SLOT_4_H_W<'a> {
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
#[doc = "Field `wr_lock_key_slot_3` reader - "]
pub struct WR_LOCK_KEY_SLOT_3_R(crate::FieldReader<bool, bool>);
impl WR_LOCK_KEY_SLOT_3_R {
    pub(crate) fn new(bits: bool) -> Self {
        WR_LOCK_KEY_SLOT_3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WR_LOCK_KEY_SLOT_3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `wr_lock_key_slot_3` writer - "]
pub struct WR_LOCK_KEY_SLOT_3_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_LOCK_KEY_SLOT_3_W<'a> {
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
#[doc = "Field `wr_lock_key_slot_2` reader - "]
pub struct WR_LOCK_KEY_SLOT_2_R(crate::FieldReader<bool, bool>);
impl WR_LOCK_KEY_SLOT_2_R {
    pub(crate) fn new(bits: bool) -> Self {
        WR_LOCK_KEY_SLOT_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WR_LOCK_KEY_SLOT_2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `wr_lock_key_slot_2` writer - "]
pub struct WR_LOCK_KEY_SLOT_2_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_LOCK_KEY_SLOT_2_W<'a> {
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
#[doc = "Field `wr_lock_key_slot_1` reader - "]
pub struct WR_LOCK_KEY_SLOT_1_R(crate::FieldReader<bool, bool>);
impl WR_LOCK_KEY_SLOT_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        WR_LOCK_KEY_SLOT_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WR_LOCK_KEY_SLOT_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `wr_lock_key_slot_1` writer - "]
pub struct WR_LOCK_KEY_SLOT_1_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_LOCK_KEY_SLOT_1_W<'a> {
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
#[doc = "Field `wr_lock_key_slot_0` reader - "]
pub struct WR_LOCK_KEY_SLOT_0_R(crate::FieldReader<bool, bool>);
impl WR_LOCK_KEY_SLOT_0_R {
    pub(crate) fn new(bits: bool) -> Self {
        WR_LOCK_KEY_SLOT_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WR_LOCK_KEY_SLOT_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `wr_lock_key_slot_0` writer - "]
pub struct WR_LOCK_KEY_SLOT_0_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_LOCK_KEY_SLOT_0_W<'a> {
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
#[doc = "Field `wr_lock_wifi_mac` reader - "]
pub struct WR_LOCK_WIFI_MAC_R(crate::FieldReader<bool, bool>);
impl WR_LOCK_WIFI_MAC_R {
    pub(crate) fn new(bits: bool) -> Self {
        WR_LOCK_WIFI_MAC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WR_LOCK_WIFI_MAC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `wr_lock_wifi_mac` writer - "]
pub struct WR_LOCK_WIFI_MAC_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_LOCK_WIFI_MAC_W<'a> {
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
#[doc = "Field `wr_lock_sw_usage_0` reader - "]
pub struct WR_LOCK_SW_USAGE_0_R(crate::FieldReader<bool, bool>);
impl WR_LOCK_SW_USAGE_0_R {
    pub(crate) fn new(bits: bool) -> Self {
        WR_LOCK_SW_USAGE_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WR_LOCK_SW_USAGE_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `wr_lock_sw_usage_0` writer - "]
pub struct WR_LOCK_SW_USAGE_0_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_LOCK_SW_USAGE_0_W<'a> {
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
#[doc = "Field `wr_lock_dbg_pwd` reader - "]
pub struct WR_LOCK_DBG_PWD_R(crate::FieldReader<bool, bool>);
impl WR_LOCK_DBG_PWD_R {
    pub(crate) fn new(bits: bool) -> Self {
        WR_LOCK_DBG_PWD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WR_LOCK_DBG_PWD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `wr_lock_dbg_pwd` writer - "]
pub struct WR_LOCK_DBG_PWD_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_LOCK_DBG_PWD_W<'a> {
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
#[doc = "Field `wr_lock_boot_mode` reader - "]
pub struct WR_LOCK_BOOT_MODE_R(crate::FieldReader<bool, bool>);
impl WR_LOCK_BOOT_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        WR_LOCK_BOOT_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WR_LOCK_BOOT_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `wr_lock_boot_mode` writer - "]
pub struct WR_LOCK_BOOT_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_LOCK_BOOT_MODE_W<'a> {
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
#[doc = "Field `wr_lock_key_slot_5_l` reader - "]
pub struct WR_LOCK_KEY_SLOT_5_L_R(crate::FieldReader<bool, bool>);
impl WR_LOCK_KEY_SLOT_5_L_R {
    pub(crate) fn new(bits: bool) -> Self {
        WR_LOCK_KEY_SLOT_5_L_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WR_LOCK_KEY_SLOT_5_L_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `wr_lock_key_slot_5_l` writer - "]
pub struct WR_LOCK_KEY_SLOT_5_L_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_LOCK_KEY_SLOT_5_L_W<'a> {
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
#[doc = "Field `wr_lock_key_slot_4_l` reader - "]
pub struct WR_LOCK_KEY_SLOT_4_L_R(crate::FieldReader<bool, bool>);
impl WR_LOCK_KEY_SLOT_4_L_R {
    pub(crate) fn new(bits: bool) -> Self {
        WR_LOCK_KEY_SLOT_4_L_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WR_LOCK_KEY_SLOT_4_L_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `wr_lock_key_slot_4_l` writer - "]
pub struct WR_LOCK_KEY_SLOT_4_L_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_LOCK_KEY_SLOT_4_L_W<'a> {
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
#[doc = "Field `ef_ana_trim_1` reader - "]
pub struct EF_ANA_TRIM_1_R(crate::FieldReader<u16, u16>);
impl EF_ANA_TRIM_1_R {
    pub(crate) fn new(bits: u16) -> Self {
        EF_ANA_TRIM_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_ANA_TRIM_1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ef_ana_trim_1` writer - "]
pub struct EF_ANA_TRIM_1_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_ANA_TRIM_1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff) | (value as u32 & 0x1fff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rd_lock_key_slot_5(&self) -> RD_LOCK_KEY_SLOT_5_R {
        RD_LOCK_KEY_SLOT_5_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rd_lock_key_slot_4(&self) -> RD_LOCK_KEY_SLOT_4_R {
        RD_LOCK_KEY_SLOT_4_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn rd_lock_key_slot_3(&self) -> RD_LOCK_KEY_SLOT_3_R {
        RD_LOCK_KEY_SLOT_3_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn rd_lock_key_slot_2(&self) -> RD_LOCK_KEY_SLOT_2_R {
        RD_LOCK_KEY_SLOT_2_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn rd_lock_key_slot_1(&self) -> RD_LOCK_KEY_SLOT_1_R {
        RD_LOCK_KEY_SLOT_1_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn rd_lock_key_slot_0(&self) -> RD_LOCK_KEY_SLOT_0_R {
        RD_LOCK_KEY_SLOT_0_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn rd_lock_dbg_pwd(&self) -> RD_LOCK_DBG_PWD_R {
        RD_LOCK_DBG_PWD_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn wr_lock_key_slot_5_h(&self) -> WR_LOCK_KEY_SLOT_5_H_R {
        WR_LOCK_KEY_SLOT_5_H_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn wr_lock_key_slot_4_h(&self) -> WR_LOCK_KEY_SLOT_4_H_R {
        WR_LOCK_KEY_SLOT_4_H_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn wr_lock_key_slot_3(&self) -> WR_LOCK_KEY_SLOT_3_R {
        WR_LOCK_KEY_SLOT_3_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn wr_lock_key_slot_2(&self) -> WR_LOCK_KEY_SLOT_2_R {
        WR_LOCK_KEY_SLOT_2_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn wr_lock_key_slot_1(&self) -> WR_LOCK_KEY_SLOT_1_R {
        WR_LOCK_KEY_SLOT_1_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn wr_lock_key_slot_0(&self) -> WR_LOCK_KEY_SLOT_0_R {
        WR_LOCK_KEY_SLOT_0_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn wr_lock_wifi_mac(&self) -> WR_LOCK_WIFI_MAC_R {
        WR_LOCK_WIFI_MAC_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn wr_lock_sw_usage_0(&self) -> WR_LOCK_SW_USAGE_0_R {
        WR_LOCK_SW_USAGE_0_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn wr_lock_dbg_pwd(&self) -> WR_LOCK_DBG_PWD_R {
        WR_LOCK_DBG_PWD_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn wr_lock_boot_mode(&self) -> WR_LOCK_BOOT_MODE_R {
        WR_LOCK_BOOT_MODE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn wr_lock_key_slot_5_l(&self) -> WR_LOCK_KEY_SLOT_5_L_R {
        WR_LOCK_KEY_SLOT_5_L_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn wr_lock_key_slot_4_l(&self) -> WR_LOCK_KEY_SLOT_4_L_R {
        WR_LOCK_KEY_SLOT_4_L_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 0:12"]
    #[inline(always)]
    pub fn ef_ana_trim_1(&self) -> EF_ANA_TRIM_1_R {
        EF_ANA_TRIM_1_R::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rd_lock_key_slot_5(&mut self) -> RD_LOCK_KEY_SLOT_5_W {
        RD_LOCK_KEY_SLOT_5_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rd_lock_key_slot_4(&mut self) -> RD_LOCK_KEY_SLOT_4_W {
        RD_LOCK_KEY_SLOT_4_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn rd_lock_key_slot_3(&mut self) -> RD_LOCK_KEY_SLOT_3_W {
        RD_LOCK_KEY_SLOT_3_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn rd_lock_key_slot_2(&mut self) -> RD_LOCK_KEY_SLOT_2_W {
        RD_LOCK_KEY_SLOT_2_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn rd_lock_key_slot_1(&mut self) -> RD_LOCK_KEY_SLOT_1_W {
        RD_LOCK_KEY_SLOT_1_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn rd_lock_key_slot_0(&mut self) -> RD_LOCK_KEY_SLOT_0_W {
        RD_LOCK_KEY_SLOT_0_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn rd_lock_dbg_pwd(&mut self) -> RD_LOCK_DBG_PWD_W {
        RD_LOCK_DBG_PWD_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn wr_lock_key_slot_5_h(&mut self) -> WR_LOCK_KEY_SLOT_5_H_W {
        WR_LOCK_KEY_SLOT_5_H_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn wr_lock_key_slot_4_h(&mut self) -> WR_LOCK_KEY_SLOT_4_H_W {
        WR_LOCK_KEY_SLOT_4_H_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn wr_lock_key_slot_3(&mut self) -> WR_LOCK_KEY_SLOT_3_W {
        WR_LOCK_KEY_SLOT_3_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn wr_lock_key_slot_2(&mut self) -> WR_LOCK_KEY_SLOT_2_W {
        WR_LOCK_KEY_SLOT_2_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn wr_lock_key_slot_1(&mut self) -> WR_LOCK_KEY_SLOT_1_W {
        WR_LOCK_KEY_SLOT_1_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn wr_lock_key_slot_0(&mut self) -> WR_LOCK_KEY_SLOT_0_W {
        WR_LOCK_KEY_SLOT_0_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn wr_lock_wifi_mac(&mut self) -> WR_LOCK_WIFI_MAC_W {
        WR_LOCK_WIFI_MAC_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn wr_lock_sw_usage_0(&mut self) -> WR_LOCK_SW_USAGE_0_W {
        WR_LOCK_SW_USAGE_0_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn wr_lock_dbg_pwd(&mut self) -> WR_LOCK_DBG_PWD_W {
        WR_LOCK_DBG_PWD_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn wr_lock_boot_mode(&mut self) -> WR_LOCK_BOOT_MODE_W {
        WR_LOCK_BOOT_MODE_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn wr_lock_key_slot_5_l(&mut self) -> WR_LOCK_KEY_SLOT_5_L_W {
        WR_LOCK_KEY_SLOT_5_L_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn wr_lock_key_slot_4_l(&mut self) -> WR_LOCK_KEY_SLOT_4_L_W {
        WR_LOCK_KEY_SLOT_4_L_W { w: self }
    }
    #[doc = "Bits 0:12"]
    #[inline(always)]
    pub fn ef_ana_trim_1(&mut self) -> EF_ANA_TRIM_1_W {
        EF_ANA_TRIM_1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ef_data_0_lock.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_data_0_lock](index.html) module"]
pub struct EF_DATA_0_LOCK_SPEC;
impl crate::RegisterSpec for EF_DATA_0_LOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ef_data_0_lock::R](R) reader structure"]
impl crate::Readable for EF_DATA_0_LOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ef_data_0_lock::W](W) writer structure"]
impl crate::Writable for EF_DATA_0_LOCK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ef_data_0_lock to value 0"]
impl crate::Resettable for EF_DATA_0_LOCK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

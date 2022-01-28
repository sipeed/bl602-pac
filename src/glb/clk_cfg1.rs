#[doc = "Register `clk_cfg1` reader"]
pub struct R(crate::R<CLK_CFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_CFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_CFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_CFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `clk_cfg1` writer"]
pub struct W(crate::W<CLK_CFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_CFG1_SPEC>;
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
impl From<crate::W<CLK_CFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_CFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ble_en` reader - Bluetooth clock enable"]
pub struct BLE_EN_R(crate::FieldReader<bool, bool>);
impl BLE_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        BLE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ble_en` writer - Bluetooth clock enable"]
pub struct BLE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> BLE_EN_W<'a> {
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
#[doc = "Field `ble_clk_sel` reader - "]
pub struct BLE_CLK_SEL_R(crate::FieldReader<u8, u8>);
impl BLE_CLK_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        BLE_CLK_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLE_CLK_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ble_clk_sel` writer - "]
pub struct BLE_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> BLE_CLK_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | ((value as u32 & 0x3f) << 16);
        self.w
    }
}
#[doc = "Field `wifi_mac_wt_div` reader - WiFi encryption clock divider"]
pub struct WIFI_MAC_WT_DIV_R(crate::FieldReader<u8, u8>);
impl WIFI_MAC_WT_DIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        WIFI_MAC_WT_DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WIFI_MAC_WT_DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `wifi_mac_wt_div` writer - WiFi encryption clock divider"]
pub struct WIFI_MAC_WT_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> WIFI_MAC_WT_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `wifi_mac_core_div` reader - WiFi core clock divider (0: 80MHz, 1: 40MHz)"]
pub struct WIFI_MAC_CORE_DIV_R(crate::FieldReader<u8, u8>);
impl WIFI_MAC_CORE_DIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        WIFI_MAC_CORE_DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WIFI_MAC_CORE_DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `wifi_mac_core_div` writer - WiFi core clock divider (0: 80MHz, 1: 40MHz)"]
pub struct WIFI_MAC_CORE_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> WIFI_MAC_CORE_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bit 24 - Bluetooth clock enable"]
    #[inline(always)]
    pub fn ble_en(&self) -> BLE_EN_R {
        BLE_EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn ble_clk_sel(&self) -> BLE_CLK_SEL_R {
        BLE_CLK_SEL_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 4:7 - WiFi encryption clock divider"]
    #[inline(always)]
    pub fn wifi_mac_wt_div(&self) -> WIFI_MAC_WT_DIV_R {
        WIFI_MAC_WT_DIV_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - WiFi core clock divider (0: 80MHz, 1: 40MHz)"]
    #[inline(always)]
    pub fn wifi_mac_core_div(&self) -> WIFI_MAC_CORE_DIV_R {
        WIFI_MAC_CORE_DIV_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 24 - Bluetooth clock enable"]
    #[inline(always)]
    pub fn ble_en(&mut self) -> BLE_EN_W {
        BLE_EN_W { w: self }
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn ble_clk_sel(&mut self) -> BLE_CLK_SEL_W {
        BLE_CLK_SEL_W { w: self }
    }
    #[doc = "Bits 4:7 - WiFi encryption clock divider"]
    #[inline(always)]
    pub fn wifi_mac_wt_div(&mut self) -> WIFI_MAC_WT_DIV_W {
        WIFI_MAC_WT_DIV_W { w: self }
    }
    #[doc = "Bits 0:3 - WiFi core clock divider (0: 80MHz, 1: 40MHz)"]
    #[inline(always)]
    pub fn wifi_mac_core_div(&mut self) -> WIFI_MAC_CORE_DIV_W {
        WIFI_MAC_CORE_DIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "clk_cfg1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_cfg1](index.html) module"]
pub struct CLK_CFG1_SPEC;
impl crate::RegisterSpec for CLK_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_cfg1::R](R) reader structure"]
impl crate::Readable for CLK_CFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_cfg1::W](W) writer structure"]
impl crate::Writable for CLK_CFG1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets clk_cfg1 to value 0x0110_0001"]
impl crate::Resettable for CLK_CFG1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0110_0001
    }
}

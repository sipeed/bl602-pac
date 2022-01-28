#[doc = "Register `WIFI_BT_COEX_CTRL` reader"]
pub struct R(crate::R<WIFI_BT_COEX_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WIFI_BT_COEX_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WIFI_BT_COEX_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WIFI_BT_COEX_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WIFI_BT_COEX_CTRL` writer"]
pub struct W(crate::W<WIFI_BT_COEX_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WIFI_BT_COEX_CTRL_SPEC>;
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
impl From<crate::W<WIFI_BT_COEX_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WIFI_BT_COEX_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `en_gpio_bt_coex` reader - "]
pub struct EN_GPIO_BT_COEX_R(crate::FieldReader<bool, bool>);
impl EN_GPIO_BT_COEX_R {
    pub(crate) fn new(bits: bool) -> Self {
        EN_GPIO_BT_COEX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN_GPIO_BT_COEX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `en_gpio_bt_coex` writer - "]
pub struct EN_GPIO_BT_COEX_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_GPIO_BT_COEX_W<'a> {
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
#[doc = "Field `coex_bt_bw` reader - "]
pub struct COEX_BT_BW_R(crate::FieldReader<bool, bool>);
impl COEX_BT_BW_R {
    pub(crate) fn new(bits: bool) -> Self {
        COEX_BT_BW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COEX_BT_BW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `coex_bt_bw` writer - "]
pub struct COEX_BT_BW_W<'a> {
    w: &'a mut W,
}
impl<'a> COEX_BT_BW_W<'a> {
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
#[doc = "Field `coex_bt_pti` reader - "]
pub struct COEX_BT_PTI_R(crate::FieldReader<u8, u8>);
impl COEX_BT_PTI_R {
    pub(crate) fn new(bits: u8) -> Self {
        COEX_BT_PTI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COEX_BT_PTI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `coex_bt_pti` writer - "]
pub struct COEX_BT_PTI_W<'a> {
    w: &'a mut W,
}
impl<'a> COEX_BT_PTI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 7)) | ((value as u32 & 0x0f) << 7);
        self.w
    }
}
#[doc = "Field `coex_bt_channel` reader - "]
pub struct COEX_BT_CHANNEL_R(crate::FieldReader<u8, u8>);
impl COEX_BT_CHANNEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        COEX_BT_CHANNEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COEX_BT_CHANNEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `coex_bt_channel` writer - "]
pub struct COEX_BT_CHANNEL_W<'a> {
    w: &'a mut W,
}
impl<'a> COEX_BT_CHANNEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn en_gpio_bt_coex(&self) -> EN_GPIO_BT_COEX_R {
        EN_GPIO_BT_COEX_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn coex_bt_bw(&self) -> COEX_BT_BW_R {
        COEX_BT_BW_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 7:10"]
    #[inline(always)]
    pub fn coex_bt_pti(&self) -> COEX_BT_PTI_R {
        COEX_BT_PTI_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn coex_bt_channel(&self) -> COEX_BT_CHANNEL_R {
        COEX_BT_CHANNEL_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn en_gpio_bt_coex(&mut self) -> EN_GPIO_BT_COEX_W {
        EN_GPIO_BT_COEX_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn coex_bt_bw(&mut self) -> COEX_BT_BW_W {
        COEX_BT_BW_W { w: self }
    }
    #[doc = "Bits 7:10"]
    #[inline(always)]
    pub fn coex_bt_pti(&mut self) -> COEX_BT_PTI_W {
        COEX_BT_PTI_W { w: self }
    }
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn coex_bt_channel(&mut self) -> COEX_BT_CHANNEL_W {
        COEX_BT_CHANNEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WIFI_BT_COEX_CTRL.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wifi_bt_coex_ctrl](index.html) module"]
pub struct WIFI_BT_COEX_CTRL_SPEC;
impl crate::RegisterSpec for WIFI_BT_COEX_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wifi_bt_coex_ctrl::R](R) reader structure"]
impl crate::Readable for WIFI_BT_COEX_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wifi_bt_coex_ctrl::W](W) writer structure"]
impl crate::Writable for WIFI_BT_COEX_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WIFI_BT_COEX_CTRL to value 0"]
impl crate::Resettable for WIFI_BT_COEX_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

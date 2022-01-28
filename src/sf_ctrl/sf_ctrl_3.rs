#[doc = "Register `sf_ctrl_3` reader"]
pub struct R(crate::R<SF_CTRL_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SF_CTRL_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SF_CTRL_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SF_CTRL_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sf_ctrl_3` writer"]
pub struct W(crate::W<SF_CTRL_3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SF_CTRL_3_SPEC>;
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
impl From<crate::W<SF_CTRL_3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SF_CTRL_3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sf_if_1_ack_lat` reader - "]
pub struct SF_IF_1_ACK_LAT_R(crate::FieldReader<u8, u8>);
impl SF_IF_1_ACK_LAT_R {
    pub(crate) fn new(bits: u8) -> Self {
        SF_IF_1_ACK_LAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_IF_1_ACK_LAT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_if_1_ack_lat` writer - "]
pub struct SF_IF_1_ACK_LAT_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_IF_1_ACK_LAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 29)) | ((value as u32 & 0x07) << 29);
        self.w
    }
}
#[doc = "Field `sf_cmds_wrap_mode` reader - "]
pub struct SF_CMDS_WRAP_MODE_R(crate::FieldReader<bool, bool>);
impl SF_CMDS_WRAP_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SF_CMDS_WRAP_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_CMDS_WRAP_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_cmds_wrap_mode` writer - "]
pub struct SF_CMDS_WRAP_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_CMDS_WRAP_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `sf_cmds_wrap_q_ini` reader - "]
pub struct SF_CMDS_WRAP_Q_INI_R(crate::FieldReader<bool, bool>);
impl SF_CMDS_WRAP_Q_INI_R {
    pub(crate) fn new(bits: bool) -> Self {
        SF_CMDS_WRAP_Q_INI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_CMDS_WRAP_Q_INI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_cmds_wrap_q_ini` writer - "]
pub struct SF_CMDS_WRAP_Q_INI_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_CMDS_WRAP_Q_INI_W<'a> {
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
#[doc = "Field `sf_cmds_bt_en` reader - "]
pub struct SF_CMDS_BT_EN_R(crate::FieldReader<bool, bool>);
impl SF_CMDS_BT_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SF_CMDS_BT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_CMDS_BT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_cmds_bt_en` writer - "]
pub struct SF_CMDS_BT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_CMDS_BT_EN_W<'a> {
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
#[doc = "Field `sf_cmds_bt_dly` reader - "]
pub struct SF_CMDS_BT_DLY_R(crate::FieldReader<u8, u8>);
impl SF_CMDS_BT_DLY_R {
    pub(crate) fn new(bits: u8) -> Self {
        SF_CMDS_BT_DLY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_CMDS_BT_DLY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_cmds_bt_dly` writer - "]
pub struct SF_CMDS_BT_DLY_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_CMDS_BT_DLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | ((value as u32 & 0x07) << 5);
        self.w
    }
}
#[doc = "Field `sf_cmds_en` reader - "]
pub struct SF_CMDS_EN_R(crate::FieldReader<bool, bool>);
impl SF_CMDS_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SF_CMDS_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_CMDS_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_cmds_en` writer - "]
pub struct SF_CMDS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_CMDS_EN_W<'a> {
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
#[doc = "Field `sf_cmds_wrap_len` reader - "]
pub struct SF_CMDS_WRAP_LEN_R(crate::FieldReader<u8, u8>);
impl SF_CMDS_WRAP_LEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        SF_CMDS_WRAP_LEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_CMDS_WRAP_LEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_cmds_wrap_len` writer - "]
pub struct SF_CMDS_WRAP_LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_CMDS_WRAP_LEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 29:31"]
    #[inline(always)]
    pub fn sf_if_1_ack_lat(&self) -> SF_IF_1_ACK_LAT_R {
        SF_IF_1_ACK_LAT_R::new(((self.bits >> 29) & 0x07) as u8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn sf_cmds_wrap_mode(&self) -> SF_CMDS_WRAP_MODE_R {
        SF_CMDS_WRAP_MODE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn sf_cmds_wrap_q_ini(&self) -> SF_CMDS_WRAP_Q_INI_R {
        SF_CMDS_WRAP_Q_INI_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn sf_cmds_bt_en(&self) -> SF_CMDS_BT_EN_R {
        SF_CMDS_BT_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 5:7"]
    #[inline(always)]
    pub fn sf_cmds_bt_dly(&self) -> SF_CMDS_BT_DLY_R {
        SF_CMDS_BT_DLY_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sf_cmds_en(&self) -> SF_CMDS_EN_R {
        SF_CMDS_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn sf_cmds_wrap_len(&self) -> SF_CMDS_WRAP_LEN_R {
        SF_CMDS_WRAP_LEN_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 29:31"]
    #[inline(always)]
    pub fn sf_if_1_ack_lat(&mut self) -> SF_IF_1_ACK_LAT_W {
        SF_IF_1_ACK_LAT_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn sf_cmds_wrap_mode(&mut self) -> SF_CMDS_WRAP_MODE_W {
        SF_CMDS_WRAP_MODE_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn sf_cmds_wrap_q_ini(&mut self) -> SF_CMDS_WRAP_Q_INI_W {
        SF_CMDS_WRAP_Q_INI_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn sf_cmds_bt_en(&mut self) -> SF_CMDS_BT_EN_W {
        SF_CMDS_BT_EN_W { w: self }
    }
    #[doc = "Bits 5:7"]
    #[inline(always)]
    pub fn sf_cmds_bt_dly(&mut self) -> SF_CMDS_BT_DLY_W {
        SF_CMDS_BT_DLY_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sf_cmds_en(&mut self) -> SF_CMDS_EN_W {
        SF_CMDS_EN_W { w: self }
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn sf_cmds_wrap_len(&mut self) -> SF_CMDS_WRAP_LEN_W {
        SF_CMDS_WRAP_LEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "sf_ctrl_3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_ctrl_3](index.html) module"]
pub struct SF_CTRL_3_SPEC;
impl crate::RegisterSpec for SF_CTRL_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sf_ctrl_3::R](R) reader structure"]
impl crate::Readable for SF_CTRL_3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sf_ctrl_3::W](W) writer structure"]
impl crate::Writable for SF_CTRL_3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets sf_ctrl_3 to value 0x2000_0046"]
impl crate::Resettable for SF_CTRL_3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2000_0046
    }
}

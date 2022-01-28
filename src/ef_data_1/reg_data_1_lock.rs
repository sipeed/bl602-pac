#[doc = "Register `reg_data_1_lock` reader"]
pub struct R(crate::R<REG_DATA_1_LOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REG_DATA_1_LOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REG_DATA_1_LOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REG_DATA_1_LOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `reg_data_1_lock` writer"]
pub struct W(crate::W<REG_DATA_1_LOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REG_DATA_1_LOCK_SPEC>;
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
impl From<crate::W<REG_DATA_1_LOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REG_DATA_1_LOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rd_lock_key_slot_9` reader - "]
pub struct RD_LOCK_KEY_SLOT_9_R(crate::FieldReader<bool, bool>);
impl RD_LOCK_KEY_SLOT_9_R {
    pub(crate) fn new(bits: bool) -> Self {
        RD_LOCK_KEY_SLOT_9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_LOCK_KEY_SLOT_9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rd_lock_key_slot_9` writer - "]
pub struct RD_LOCK_KEY_SLOT_9_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_LOCK_KEY_SLOT_9_W<'a> {
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
#[doc = "Field `rd_lock_key_slot_8` reader - "]
pub struct RD_LOCK_KEY_SLOT_8_R(crate::FieldReader<bool, bool>);
impl RD_LOCK_KEY_SLOT_8_R {
    pub(crate) fn new(bits: bool) -> Self {
        RD_LOCK_KEY_SLOT_8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_LOCK_KEY_SLOT_8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rd_lock_key_slot_8` writer - "]
pub struct RD_LOCK_KEY_SLOT_8_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_LOCK_KEY_SLOT_8_W<'a> {
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
#[doc = "Field `rd_lock_key_slot_7` reader - "]
pub struct RD_LOCK_KEY_SLOT_7_R(crate::FieldReader<bool, bool>);
impl RD_LOCK_KEY_SLOT_7_R {
    pub(crate) fn new(bits: bool) -> Self {
        RD_LOCK_KEY_SLOT_7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_LOCK_KEY_SLOT_7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rd_lock_key_slot_7` writer - "]
pub struct RD_LOCK_KEY_SLOT_7_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_LOCK_KEY_SLOT_7_W<'a> {
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
#[doc = "Field `rd_lock_key_slot_6` reader - "]
pub struct RD_LOCK_KEY_SLOT_6_R(crate::FieldReader<bool, bool>);
impl RD_LOCK_KEY_SLOT_6_R {
    pub(crate) fn new(bits: bool) -> Self {
        RD_LOCK_KEY_SLOT_6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_LOCK_KEY_SLOT_6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rd_lock_key_slot_6` writer - "]
pub struct RD_LOCK_KEY_SLOT_6_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_LOCK_KEY_SLOT_6_W<'a> {
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
#[doc = "Field `RESERVED_25_16` reader - "]
pub struct RESERVED_25_16_R(crate::FieldReader<u16, u16>);
impl RESERVED_25_16_R {
    pub(crate) fn new(bits: u16) -> Self {
        RESERVED_25_16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED_25_16_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED_25_16` writer - "]
pub struct RESERVED_25_16_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED_25_16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | ((value as u32 & 0x03ff) << 16);
        self.w
    }
}
#[doc = "Field `wr_lock_key_slot_9` reader - "]
pub struct WR_LOCK_KEY_SLOT_9_R(crate::FieldReader<bool, bool>);
impl WR_LOCK_KEY_SLOT_9_R {
    pub(crate) fn new(bits: bool) -> Self {
        WR_LOCK_KEY_SLOT_9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WR_LOCK_KEY_SLOT_9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `wr_lock_key_slot_9` writer - "]
pub struct WR_LOCK_KEY_SLOT_9_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_LOCK_KEY_SLOT_9_W<'a> {
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
#[doc = "Field `wr_lock_key_slot_8` reader - "]
pub struct WR_LOCK_KEY_SLOT_8_R(crate::FieldReader<bool, bool>);
impl WR_LOCK_KEY_SLOT_8_R {
    pub(crate) fn new(bits: bool) -> Self {
        WR_LOCK_KEY_SLOT_8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WR_LOCK_KEY_SLOT_8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `wr_lock_key_slot_8` writer - "]
pub struct WR_LOCK_KEY_SLOT_8_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_LOCK_KEY_SLOT_8_W<'a> {
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
#[doc = "Field `wr_lock_key_slot_7` reader - "]
pub struct WR_LOCK_KEY_SLOT_7_R(crate::FieldReader<bool, bool>);
impl WR_LOCK_KEY_SLOT_7_R {
    pub(crate) fn new(bits: bool) -> Self {
        WR_LOCK_KEY_SLOT_7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WR_LOCK_KEY_SLOT_7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `wr_lock_key_slot_7` writer - "]
pub struct WR_LOCK_KEY_SLOT_7_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_LOCK_KEY_SLOT_7_W<'a> {
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
#[doc = "Field `wr_lock_key_slot_6` reader - "]
pub struct WR_LOCK_KEY_SLOT_6_R(crate::FieldReader<bool, bool>);
impl WR_LOCK_KEY_SLOT_6_R {
    pub(crate) fn new(bits: bool) -> Self {
        WR_LOCK_KEY_SLOT_6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WR_LOCK_KEY_SLOT_6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `wr_lock_key_slot_6` writer - "]
pub struct WR_LOCK_KEY_SLOT_6_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_LOCK_KEY_SLOT_6_W<'a> {
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
#[doc = "Field `RESERVED_9_0` reader - "]
pub struct RESERVED_9_0_R(crate::FieldReader<u16, u16>);
impl RESERVED_9_0_R {
    pub(crate) fn new(bits: u16) -> Self {
        RESERVED_9_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED_9_0_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED_9_0` writer - "]
pub struct RESERVED_9_0_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED_9_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn rd_lock_key_slot_9(&self) -> RD_LOCK_KEY_SLOT_9_R {
        RD_LOCK_KEY_SLOT_9_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn rd_lock_key_slot_8(&self) -> RD_LOCK_KEY_SLOT_8_R {
        RD_LOCK_KEY_SLOT_8_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn rd_lock_key_slot_7(&self) -> RD_LOCK_KEY_SLOT_7_R {
        RD_LOCK_KEY_SLOT_7_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn rd_lock_key_slot_6(&self) -> RD_LOCK_KEY_SLOT_6_R {
        RD_LOCK_KEY_SLOT_6_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn reserved_25_16(&self) -> RESERVED_25_16_R {
        RESERVED_25_16_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn wr_lock_key_slot_9(&self) -> WR_LOCK_KEY_SLOT_9_R {
        WR_LOCK_KEY_SLOT_9_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn wr_lock_key_slot_8(&self) -> WR_LOCK_KEY_SLOT_8_R {
        WR_LOCK_KEY_SLOT_8_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn wr_lock_key_slot_7(&self) -> WR_LOCK_KEY_SLOT_7_R {
        WR_LOCK_KEY_SLOT_7_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn wr_lock_key_slot_6(&self) -> WR_LOCK_KEY_SLOT_6_R {
        WR_LOCK_KEY_SLOT_6_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn reserved_9_0(&self) -> RESERVED_9_0_R {
        RESERVED_9_0_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn rd_lock_key_slot_9(&mut self) -> RD_LOCK_KEY_SLOT_9_W {
        RD_LOCK_KEY_SLOT_9_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn rd_lock_key_slot_8(&mut self) -> RD_LOCK_KEY_SLOT_8_W {
        RD_LOCK_KEY_SLOT_8_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn rd_lock_key_slot_7(&mut self) -> RD_LOCK_KEY_SLOT_7_W {
        RD_LOCK_KEY_SLOT_7_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn rd_lock_key_slot_6(&mut self) -> RD_LOCK_KEY_SLOT_6_W {
        RD_LOCK_KEY_SLOT_6_W { w: self }
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn reserved_25_16(&mut self) -> RESERVED_25_16_W {
        RESERVED_25_16_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn wr_lock_key_slot_9(&mut self) -> WR_LOCK_KEY_SLOT_9_W {
        WR_LOCK_KEY_SLOT_9_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn wr_lock_key_slot_8(&mut self) -> WR_LOCK_KEY_SLOT_8_W {
        WR_LOCK_KEY_SLOT_8_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn wr_lock_key_slot_7(&mut self) -> WR_LOCK_KEY_SLOT_7_W {
        WR_LOCK_KEY_SLOT_7_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn wr_lock_key_slot_6(&mut self) -> WR_LOCK_KEY_SLOT_6_W {
        WR_LOCK_KEY_SLOT_6_W { w: self }
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn reserved_9_0(&mut self) -> RESERVED_9_0_W {
        RESERVED_9_0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "reg_data_1_lock.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_data_1_lock](index.html) module"]
pub struct REG_DATA_1_LOCK_SPEC;
impl crate::RegisterSpec for REG_DATA_1_LOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reg_data_1_lock::R](R) reader structure"]
impl crate::Readable for REG_DATA_1_LOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reg_data_1_lock::W](W) writer structure"]
impl crate::Writable for REG_DATA_1_LOCK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets reg_data_1_lock to value 0"]
impl crate::Resettable for REG_DATA_1_LOCK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

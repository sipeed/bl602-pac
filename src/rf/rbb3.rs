#[doc = "Register `rbb3` reader"]
pub struct R(crate::R<RBB3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RBB3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RBB3_SPEC>> for R {
    fn from(reader: crate::R<RBB3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rbb3` writer"]
pub struct W(crate::W<RBB3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RBB3_SPEC>;
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
impl core::convert::From<crate::W<RBB3_SPEC>> for W {
    fn from(writer: crate::W<RBB3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pwr_det_en` reader - "]
pub struct PWR_DET_EN_R(crate::FieldReader<bool, bool>);
impl PWR_DET_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWR_DET_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_DET_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwr_det_en` writer - "]
pub struct PWR_DET_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_DET_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Field `rxiqcal_en` reader - "]
pub struct RXIQCAL_EN_R(crate::FieldReader<bool, bool>);
impl RXIQCAL_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXIQCAL_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXIQCAL_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rxiqcal_en` writer - "]
pub struct RXIQCAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXIQCAL_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `rbb_bw` reader - "]
pub struct RBB_BW_R(crate::FieldReader<u8, u8>);
impl RBB_BW_R {
    pub(crate) fn new(bits: u8) -> Self {
        RBB_BW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBB_BW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rbb_bw` writer - "]
pub struct RBB_BW_W<'a> {
    w: &'a mut W,
}
impl<'a> RBB_BW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Field `rbb_tia_iqbias_short` reader - "]
pub struct RBB_TIA_IQBIAS_SHORT_R(crate::FieldReader<bool, bool>);
impl RBB_TIA_IQBIAS_SHORT_R {
    pub(crate) fn new(bits: bool) -> Self {
        RBB_TIA_IQBIAS_SHORT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBB_TIA_IQBIAS_SHORT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rbb_tia_iqbias_short` writer - "]
pub struct RBB_TIA_IQBIAS_SHORT_W<'a> {
    w: &'a mut W,
}
impl<'a> RBB_TIA_IQBIAS_SHORT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `rbb_bq_iqbias_short` reader - "]
pub struct RBB_BQ_IQBIAS_SHORT_R(crate::FieldReader<bool, bool>);
impl RBB_BQ_IQBIAS_SHORT_R {
    pub(crate) fn new(bits: bool) -> Self {
        RBB_BQ_IQBIAS_SHORT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBB_BQ_IQBIAS_SHORT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rbb_bq_iqbias_short` writer - "]
pub struct RBB_BQ_IQBIAS_SHORT_W<'a> {
    w: &'a mut W,
}
impl<'a> RBB_BQ_IQBIAS_SHORT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `rbb_vcm` reader - "]
pub struct RBB_VCM_R(crate::FieldReader<u8, u8>);
impl RBB_VCM_R {
    pub(crate) fn new(bits: u8) -> Self {
        RBB_VCM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBB_VCM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rbb_vcm` writer - "]
pub struct RBB_VCM_W<'a> {
    w: &'a mut W,
}
impl<'a> RBB_VCM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `rbb_bm_op` reader - "]
pub struct RBB_BM_OP_R(crate::FieldReader<u8, u8>);
impl RBB_BM_OP_R {
    pub(crate) fn new(bits: u8) -> Self {
        RBB_BM_OP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBB_BM_OP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rbb_bm_op` writer - "]
pub struct RBB_BM_OP_W<'a> {
    w: &'a mut W,
}
impl<'a> RBB_BM_OP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Field `rbb_deq` reader - "]
pub struct RBB_DEQ_R(crate::FieldReader<u8, u8>);
impl RBB_DEQ_R {
    pub(crate) fn new(bits: u8) -> Self {
        RBB_DEQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBB_DEQ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rbb_deq` writer - "]
pub struct RBB_DEQ_W<'a> {
    w: &'a mut W,
}
impl<'a> RBB_DEQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `rbb_bt_fif_tune` reader - "]
pub struct RBB_BT_FIF_TUNE_R(crate::FieldReader<u8, u8>);
impl RBB_BT_FIF_TUNE_R {
    pub(crate) fn new(bits: u8) -> Self {
        RBB_BT_FIF_TUNE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBB_BT_FIF_TUNE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rbb_bt_fif_tune` writer - "]
pub struct RBB_BT_FIF_TUNE_W<'a> {
    w: &'a mut W,
}
impl<'a> RBB_BT_FIF_TUNE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
#[doc = "Field `rbb_bt_mode` reader - "]
pub struct RBB_BT_MODE_R(crate::FieldReader<bool, bool>);
impl RBB_BT_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RBB_BT_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBB_BT_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rbb_bt_mode` writer - "]
pub struct RBB_BT_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> RBB_BT_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `rbb_bt_mode_hw` reader - "]
pub struct RBB_BT_MODE_HW_R(crate::FieldReader<bool, bool>);
impl RBB_BT_MODE_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        RBB_BT_MODE_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBB_BT_MODE_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rbb_bt_mode_hw` writer - "]
pub struct RBB_BT_MODE_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> RBB_BT_MODE_HW_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn pwr_det_en(&self) -> PWR_DET_EN_R {
        PWR_DET_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn rxiqcal_en(&self) -> RXIQCAL_EN_R {
        RXIQCAL_EN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn rbb_bw(&self) -> RBB_BW_R {
        RBB_BW_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn rbb_tia_iqbias_short(&self) -> RBB_TIA_IQBIAS_SHORT_R {
        RBB_TIA_IQBIAS_SHORT_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn rbb_bq_iqbias_short(&self) -> RBB_BQ_IQBIAS_SHORT_R {
        RBB_BQ_IQBIAS_SHORT_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn rbb_vcm(&self) -> RBB_VCM_R {
        RBB_VCM_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn rbb_bm_op(&self) -> RBB_BM_OP_R {
        RBB_BM_OP_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn rbb_deq(&self) -> RBB_DEQ_R {
        RBB_DEQ_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn rbb_bt_fif_tune(&self) -> RBB_BT_FIF_TUNE_R {
        RBB_BT_FIF_TUNE_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rbb_bt_mode(&self) -> RBB_BT_MODE_R {
        RBB_BT_MODE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rbb_bt_mode_hw(&self) -> RBB_BT_MODE_HW_R {
        RBB_BT_MODE_HW_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn pwr_det_en(&mut self) -> PWR_DET_EN_W {
        PWR_DET_EN_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn rxiqcal_en(&mut self) -> RXIQCAL_EN_W {
        RXIQCAL_EN_W { w: self }
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn rbb_bw(&mut self) -> RBB_BW_W {
        RBB_BW_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn rbb_tia_iqbias_short(&mut self) -> RBB_TIA_IQBIAS_SHORT_W {
        RBB_TIA_IQBIAS_SHORT_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn rbb_bq_iqbias_short(&mut self) -> RBB_BQ_IQBIAS_SHORT_W {
        RBB_BQ_IQBIAS_SHORT_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn rbb_vcm(&mut self) -> RBB_VCM_W {
        RBB_VCM_W { w: self }
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn rbb_bm_op(&mut self) -> RBB_BM_OP_W {
        RBB_BM_OP_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn rbb_deq(&mut self) -> RBB_DEQ_W {
        RBB_DEQ_W { w: self }
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn rbb_bt_fif_tune(&mut self) -> RBB_BT_FIF_TUNE_W {
        RBB_BT_FIF_TUNE_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rbb_bt_mode(&mut self) -> RBB_BT_MODE_W {
        RBB_BT_MODE_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rbb_bt_mode_hw(&mut self) -> RBB_BT_MODE_HW_W {
        RBB_BT_MODE_HW_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rbb3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rbb3](index.html) module"]
pub struct RBB3_SPEC;
impl crate::RegisterSpec for RBB3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rbb3::R](R) reader structure"]
impl crate::Readable for RBB3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rbb3::W](W) writer structure"]
impl crate::Writable for RBB3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets rbb3 to value 0"]
impl crate::Resettable for RBB3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

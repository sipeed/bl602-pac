#[doc = "Register `dfe_ctrl_0` reader"]
pub struct R(crate::R<DFE_CTRL_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFE_CTRL_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFE_CTRL_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFE_CTRL_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dfe_ctrl_0` writer"]
pub struct W(crate::W<DFE_CTRL_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFE_CTRL_0_SPEC>;
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
impl From<crate::W<DFE_CTRL_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFE_CTRL_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tx_dvga_gain_ctrl_hw` reader - "]
pub struct TX_DVGA_GAIN_CTRL_HW_R(crate::FieldReader<bool, bool>);
impl TX_DVGA_GAIN_CTRL_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_DVGA_GAIN_CTRL_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_DVGA_GAIN_CTRL_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tx_dvga_gain_ctrl_hw` writer - "]
pub struct TX_DVGA_GAIN_CTRL_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DVGA_GAIN_CTRL_HW_W<'a> {
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
#[doc = "Field `tx_dvga_gain_qdb` reader - "]
pub struct TX_DVGA_GAIN_QDB_R(crate::FieldReader<u8, u8>);
impl TX_DVGA_GAIN_QDB_R {
    pub(crate) fn new(bits: u8) -> Self {
        TX_DVGA_GAIN_QDB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_DVGA_GAIN_QDB_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tx_dvga_gain_qdb` writer - "]
pub struct TX_DVGA_GAIN_QDB_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DVGA_GAIN_QDB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 24)) | ((value as u32 & 0x7f) << 24);
        self.w
    }
}
#[doc = "Field `tx_iqc_gain_en` reader - "]
pub struct TX_IQC_GAIN_EN_R(crate::FieldReader<bool, bool>);
impl TX_IQC_GAIN_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_IQC_GAIN_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_IQC_GAIN_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tx_iqc_gain_en` writer - "]
pub struct TX_IQC_GAIN_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_IQC_GAIN_EN_W<'a> {
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
#[doc = "Field `tx_iqc_gain` reader - "]
pub struct TX_IQC_GAIN_R(crate::FieldReader<u16, u16>);
impl TX_IQC_GAIN_R {
    pub(crate) fn new(bits: u16) -> Self {
        TX_IQC_GAIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_IQC_GAIN_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tx_iqc_gain` writer - "]
pub struct TX_IQC_GAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_IQC_GAIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 12)) | ((value as u32 & 0x07ff) << 12);
        self.w
    }
}
#[doc = "Field `tx_iqc_phase_en` reader - "]
pub struct TX_IQC_PHASE_EN_R(crate::FieldReader<bool, bool>);
impl TX_IQC_PHASE_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_IQC_PHASE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_IQC_PHASE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tx_iqc_phase_en` writer - "]
pub struct TX_IQC_PHASE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_IQC_PHASE_EN_W<'a> {
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
#[doc = "Field `tx_iqc_phase` reader - "]
pub struct TX_IQC_PHASE_R(crate::FieldReader<u16, u16>);
impl TX_IQC_PHASE_R {
    pub(crate) fn new(bits: u16) -> Self {
        TX_IQC_PHASE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_IQC_PHASE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tx_iqc_phase` writer - "]
pub struct TX_IQC_PHASE_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_IQC_PHASE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn tx_dvga_gain_ctrl_hw(&self) -> TX_DVGA_GAIN_CTRL_HW_R {
        TX_DVGA_GAIN_CTRL_HW_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 24:30"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb(&self) -> TX_DVGA_GAIN_QDB_R {
        TX_DVGA_GAIN_QDB_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn tx_iqc_gain_en(&self) -> TX_IQC_GAIN_EN_R {
        TX_IQC_GAIN_EN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 12:22"]
    #[inline(always)]
    pub fn tx_iqc_gain(&self) -> TX_IQC_GAIN_R {
        TX_IQC_GAIN_R::new(((self.bits >> 12) & 0x07ff) as u16)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn tx_iqc_phase_en(&self) -> TX_IQC_PHASE_EN_R {
        TX_IQC_PHASE_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn tx_iqc_phase(&self) -> TX_IQC_PHASE_R {
        TX_IQC_PHASE_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn tx_dvga_gain_ctrl_hw(&mut self) -> TX_DVGA_GAIN_CTRL_HW_W {
        TX_DVGA_GAIN_CTRL_HW_W { w: self }
    }
    #[doc = "Bits 24:30"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb(&mut self) -> TX_DVGA_GAIN_QDB_W {
        TX_DVGA_GAIN_QDB_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn tx_iqc_gain_en(&mut self) -> TX_IQC_GAIN_EN_W {
        TX_IQC_GAIN_EN_W { w: self }
    }
    #[doc = "Bits 12:22"]
    #[inline(always)]
    pub fn tx_iqc_gain(&mut self) -> TX_IQC_GAIN_W {
        TX_IQC_GAIN_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn tx_iqc_phase_en(&mut self) -> TX_IQC_PHASE_EN_W {
        TX_IQC_PHASE_EN_W { w: self }
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn tx_iqc_phase(&mut self) -> TX_IQC_PHASE_W {
        TX_IQC_PHASE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "dfe_ctrl_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfe_ctrl_0](index.html) module"]
pub struct DFE_CTRL_0_SPEC;
impl crate::RegisterSpec for DFE_CTRL_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfe_ctrl_0::R](R) reader structure"]
impl crate::Readable for DFE_CTRL_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfe_ctrl_0::W](W) writer structure"]
impl crate::Writable for DFE_CTRL_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets dfe_ctrl_0 to value 0"]
impl crate::Resettable for DFE_CTRL_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

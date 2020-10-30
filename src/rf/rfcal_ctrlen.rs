#[doc = "Register `rfcal_ctrlen` reader"]
pub struct R(crate::R<RFCAL_CTRLEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RFCAL_CTRLEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RFCAL_CTRLEN_SPEC>> for R {
    fn from(reader: crate::R<RFCAL_CTRLEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rfcal_ctrlen` writer"]
pub struct W(crate::W<RFCAL_CTRLEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RFCAL_CTRLEN_SPEC>;
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
impl core::convert::From<crate::W<RFCAL_CTRLEN_SPEC>> for W {
    fn from(writer: crate::W<RFCAL_CTRLEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dpd_en` reader - "]
pub struct DPD_EN_R(crate::FieldReader<bool, bool>);
impl DPD_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DPD_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DPD_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dpd_en` writer - "]
pub struct DPD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DPD_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `tsencal_en` reader - "]
pub struct TSENCAL_EN_R(crate::FieldReader<bool, bool>);
impl TSENCAL_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSENCAL_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSENCAL_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tsencal_en` writer - "]
pub struct TSENCAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TSENCAL_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `pwdet_cal_en` reader - "]
pub struct PWDET_CAL_EN_R(crate::FieldReader<bool, bool>);
impl PWDET_CAL_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWDET_CAL_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWDET_CAL_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwdet_cal_en` writer - "]
pub struct PWDET_CAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWDET_CAL_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `riqcal_en` reader - "]
pub struct RIQCAL_EN_R(crate::FieldReader<bool, bool>);
impl RIQCAL_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RIQCAL_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RIQCAL_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `riqcal_en` writer - "]
pub struct RIQCAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RIQCAL_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `tiqcal_en` reader - "]
pub struct TIQCAL_EN_R(crate::FieldReader<bool, bool>);
impl TIQCAL_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIQCAL_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIQCAL_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tiqcal_en` writer - "]
pub struct TIQCAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIQCAL_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `lo_leakcal_en` reader - "]
pub struct LO_LEAKCAL_EN_R(crate::FieldReader<bool, bool>);
impl LO_LEAKCAL_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LO_LEAKCAL_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_LEAKCAL_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_leakcal_en` writer - "]
pub struct LO_LEAKCAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_LEAKCAL_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `rccal_en` reader - "]
pub struct RCCAL_EN_R(crate::FieldReader<bool, bool>);
impl RCCAL_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RCCAL_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCCAL_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rccal_en` writer - "]
pub struct RCCAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RCCAL_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `toscal_en` reader - "]
pub struct TOSCAL_EN_R(crate::FieldReader<bool, bool>);
impl TOSCAL_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TOSCAL_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOSCAL_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `toscal_en` writer - "]
pub struct TOSCAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TOSCAL_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `roscal_en` reader - "]
pub struct ROSCAL_EN_R(crate::FieldReader<bool, bool>);
impl ROSCAL_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ROSCAL_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROSCAL_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `roscal_en` writer - "]
pub struct ROSCAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ROSCAL_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `clkpll_cal_en` reader - "]
pub struct CLKPLL_CAL_EN_R(crate::FieldReader<bool, bool>);
impl CLKPLL_CAL_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLKPLL_CAL_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKPLL_CAL_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clkpll_cal_en` writer - "]
pub struct CLKPLL_CAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_CAL_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `roscal_inc_en` reader - "]
pub struct ROSCAL_INC_EN_R(crate::FieldReader<bool, bool>);
impl ROSCAL_INC_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ROSCAL_INC_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROSCAL_INC_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `roscal_inc_en` writer - "]
pub struct ROSCAL_INC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ROSCAL_INC_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `acal_inc_en` reader - "]
pub struct ACAL_INC_EN_R(crate::FieldReader<bool, bool>);
impl ACAL_INC_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACAL_INC_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACAL_INC_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `acal_inc_en` writer - "]
pub struct ACAL_INC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ACAL_INC_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `fcal_inc_en` reader - "]
pub struct FCAL_INC_EN_R(crate::FieldReader<bool, bool>);
impl FCAL_INC_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        FCAL_INC_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FCAL_INC_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `fcal_inc_en` writer - "]
pub struct FCAL_INC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FCAL_INC_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `acal_en` reader - "]
pub struct ACAL_EN_R(crate::FieldReader<bool, bool>);
impl ACAL_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACAL_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACAL_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `acal_en` writer - "]
pub struct ACAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ACAL_EN_W<'a> {
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
#[doc = "Field `fcal_en` reader - "]
pub struct FCAL_EN_R(crate::FieldReader<bool, bool>);
impl FCAL_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        FCAL_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FCAL_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `fcal_en` writer - "]
pub struct FCAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FCAL_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `dl_rfcal_table_en` reader - "]
pub struct DL_RFCAL_TABLE_EN_R(crate::FieldReader<bool, bool>);
impl DL_RFCAL_TABLE_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DL_RFCAL_TABLE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DL_RFCAL_TABLE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dl_rfcal_table_en` writer - "]
pub struct DL_RFCAL_TABLE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DL_RFCAL_TABLE_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `adc_oscal_en` reader - "]
pub struct ADC_OSCAL_EN_R(crate::FieldReader<bool, bool>);
impl ADC_OSCAL_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC_OSCAL_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_OSCAL_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adc_oscal_en` writer - "]
pub struct ADC_OSCAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_OSCAL_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `rcal_en_resv` reader - "]
pub struct RCAL_EN_RESV_R(crate::FieldReader<bool, bool>);
impl RCAL_EN_RESV_R {
    pub(crate) fn new(bits: bool) -> Self {
        RCAL_EN_RESV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCAL_EN_RESV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rcal_en_resv` writer - "]
pub struct RCAL_EN_RESV_W<'a> {
    w: &'a mut W,
}
impl<'a> RCAL_EN_RESV_W<'a> {
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
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn dpd_en(&self) -> DPD_EN_R {
        DPD_EN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tsencal_en(&self) -> TSENCAL_EN_R {
        TSENCAL_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn pwdet_cal_en(&self) -> PWDET_CAL_EN_R {
        PWDET_CAL_EN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn riqcal_en(&self) -> RIQCAL_EN_R {
        RIQCAL_EN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn tiqcal_en(&self) -> TIQCAL_EN_R {
        TIQCAL_EN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn lo_leakcal_en(&self) -> LO_LEAKCAL_EN_R {
        LO_LEAKCAL_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn rccal_en(&self) -> RCCAL_EN_R {
        RCCAL_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn toscal_en(&self) -> TOSCAL_EN_R {
        TOSCAL_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn roscal_en(&self) -> ROSCAL_EN_R {
        ROSCAL_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn clkpll_cal_en(&self) -> CLKPLL_CAL_EN_R {
        CLKPLL_CAL_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn roscal_inc_en(&self) -> ROSCAL_INC_EN_R {
        ROSCAL_INC_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn acal_inc_en(&self) -> ACAL_INC_EN_R {
        ACAL_INC_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn fcal_inc_en(&self) -> FCAL_INC_EN_R {
        FCAL_INC_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn acal_en(&self) -> ACAL_EN_R {
        ACAL_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn fcal_en(&self) -> FCAL_EN_R {
        FCAL_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dl_rfcal_table_en(&self) -> DL_RFCAL_TABLE_EN_R {
        DL_RFCAL_TABLE_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn adc_oscal_en(&self) -> ADC_OSCAL_EN_R {
        ADC_OSCAL_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rcal_en_resv(&self) -> RCAL_EN_RESV_R {
        RCAL_EN_RESV_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn dpd_en(&mut self) -> DPD_EN_W {
        DPD_EN_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tsencal_en(&mut self) -> TSENCAL_EN_W {
        TSENCAL_EN_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn pwdet_cal_en(&mut self) -> PWDET_CAL_EN_W {
        PWDET_CAL_EN_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn riqcal_en(&mut self) -> RIQCAL_EN_W {
        RIQCAL_EN_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn tiqcal_en(&mut self) -> TIQCAL_EN_W {
        TIQCAL_EN_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn lo_leakcal_en(&mut self) -> LO_LEAKCAL_EN_W {
        LO_LEAKCAL_EN_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn rccal_en(&mut self) -> RCCAL_EN_W {
        RCCAL_EN_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn toscal_en(&mut self) -> TOSCAL_EN_W {
        TOSCAL_EN_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn roscal_en(&mut self) -> ROSCAL_EN_W {
        ROSCAL_EN_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn clkpll_cal_en(&mut self) -> CLKPLL_CAL_EN_W {
        CLKPLL_CAL_EN_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn roscal_inc_en(&mut self) -> ROSCAL_INC_EN_W {
        ROSCAL_INC_EN_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn acal_inc_en(&mut self) -> ACAL_INC_EN_W {
        ACAL_INC_EN_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn fcal_inc_en(&mut self) -> FCAL_INC_EN_W {
        FCAL_INC_EN_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn acal_en(&mut self) -> ACAL_EN_W {
        ACAL_EN_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn fcal_en(&mut self) -> FCAL_EN_W {
        FCAL_EN_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dl_rfcal_table_en(&mut self) -> DL_RFCAL_TABLE_EN_W {
        DL_RFCAL_TABLE_EN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn adc_oscal_en(&mut self) -> ADC_OSCAL_EN_W {
        ADC_OSCAL_EN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rcal_en_resv(&mut self) -> RCAL_EN_RESV_W {
        RCAL_EN_RESV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Calibration mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfcal_ctrlen](index.html) module"]
pub struct RFCAL_CTRLEN_SPEC;
impl crate::RegisterSpec for RFCAL_CTRLEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rfcal_ctrlen::R](R) reader structure"]
impl crate::Readable for RFCAL_CTRLEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rfcal_ctrlen::W](W) writer structure"]
impl crate::Writable for RFCAL_CTRLEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets rfcal_ctrlen to value 0"]
impl crate::Resettable for RFCAL_CTRLEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

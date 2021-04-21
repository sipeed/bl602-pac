#[doc = "Register `rfcal_stateen` reader"]
pub struct R(crate::R<RFCAL_STATEEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RFCAL_STATEEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RFCAL_STATEEN_SPEC>> for R {
    fn from(reader: crate::R<RFCAL_STATEEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rfcal_stateen` writer"]
pub struct W(crate::W<RFCAL_STATEEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RFCAL_STATEEN_SPEC>;
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
impl core::convert::From<crate::W<RFCAL_STATEEN_SPEC>> for W {
    fn from(writer: crate::W<RFCAL_STATEEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rfcal_level` reader - "]
pub struct RFCAL_LEVEL_R(crate::FieldReader<u8, u8>);
impl RFCAL_LEVEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        RFCAL_LEVEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFCAL_LEVEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rfcal_level` writer - "]
pub struct RFCAL_LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RFCAL_LEVEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
#[doc = "Field `dpd_sten` reader - "]
pub struct DPD_STEN_R(crate::FieldReader<bool, bool>);
impl DPD_STEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DPD_STEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DPD_STEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dpd_sten` writer - "]
pub struct DPD_STEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DPD_STEN_W<'a> {
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
#[doc = "Field `tsencal_sten` reader - "]
pub struct TSENCAL_STEN_R(crate::FieldReader<bool, bool>);
impl TSENCAL_STEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSENCAL_STEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSENCAL_STEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tsencal_sten` writer - "]
pub struct TSENCAL_STEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TSENCAL_STEN_W<'a> {
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
#[doc = "Field `pwdet_cal_sten` reader - "]
pub struct PWDET_CAL_STEN_R(crate::FieldReader<bool, bool>);
impl PWDET_CAL_STEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWDET_CAL_STEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWDET_CAL_STEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwdet_cal_sten` writer - "]
pub struct PWDET_CAL_STEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWDET_CAL_STEN_W<'a> {
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
#[doc = "Field `riqcal_sten` reader - "]
pub struct RIQCAL_STEN_R(crate::FieldReader<bool, bool>);
impl RIQCAL_STEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RIQCAL_STEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RIQCAL_STEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `riqcal_sten` writer - "]
pub struct RIQCAL_STEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RIQCAL_STEN_W<'a> {
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
#[doc = "Field `tiqcal_sten` reader - "]
pub struct TIQCAL_STEN_R(crate::FieldReader<bool, bool>);
impl TIQCAL_STEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIQCAL_STEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIQCAL_STEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tiqcal_sten` writer - "]
pub struct TIQCAL_STEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIQCAL_STEN_W<'a> {
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
#[doc = "Field `lo_leakcal_sten` reader - "]
pub struct LO_LEAKCAL_STEN_R(crate::FieldReader<bool, bool>);
impl LO_LEAKCAL_STEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LO_LEAKCAL_STEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_LEAKCAL_STEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_leakcal_sten` writer - "]
pub struct LO_LEAKCAL_STEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_LEAKCAL_STEN_W<'a> {
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
#[doc = "Field `rccal_sten` reader - "]
pub struct RCCAL_STEN_R(crate::FieldReader<bool, bool>);
impl RCCAL_STEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RCCAL_STEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCCAL_STEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rccal_sten` writer - "]
pub struct RCCAL_STEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RCCAL_STEN_W<'a> {
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
#[doc = "Field `toscal_sten_resv` reader - "]
pub struct TOSCAL_STEN_RESV_R(crate::FieldReader<bool, bool>);
impl TOSCAL_STEN_RESV_R {
    pub(crate) fn new(bits: bool) -> Self {
        TOSCAL_STEN_RESV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOSCAL_STEN_RESV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `toscal_sten_resv` writer - "]
pub struct TOSCAL_STEN_RESV_W<'a> {
    w: &'a mut W,
}
impl<'a> TOSCAL_STEN_RESV_W<'a> {
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
#[doc = "Field `roscal_sten` reader - "]
pub struct ROSCAL_STEN_R(crate::FieldReader<bool, bool>);
impl ROSCAL_STEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ROSCAL_STEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROSCAL_STEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `roscal_sten` writer - "]
pub struct ROSCAL_STEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ROSCAL_STEN_W<'a> {
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
#[doc = "Field `clkpll_cal_sten` reader - "]
pub struct CLKPLL_CAL_STEN_R(crate::FieldReader<bool, bool>);
impl CLKPLL_CAL_STEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLKPLL_CAL_STEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKPLL_CAL_STEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clkpll_cal_sten` writer - "]
pub struct CLKPLL_CAL_STEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_CAL_STEN_W<'a> {
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
#[doc = "Field `inc_acal_sten` reader - "]
pub struct INC_ACAL_STEN_R(crate::FieldReader<bool, bool>);
impl INC_ACAL_STEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        INC_ACAL_STEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INC_ACAL_STEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `inc_acal_sten` writer - "]
pub struct INC_ACAL_STEN_W<'a> {
    w: &'a mut W,
}
impl<'a> INC_ACAL_STEN_W<'a> {
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
#[doc = "Field `inc_fcal_sten` reader - "]
pub struct INC_FCAL_STEN_R(crate::FieldReader<bool, bool>);
impl INC_FCAL_STEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        INC_FCAL_STEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INC_FCAL_STEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `inc_fcal_sten` writer - "]
pub struct INC_FCAL_STEN_W<'a> {
    w: &'a mut W,
}
impl<'a> INC_FCAL_STEN_W<'a> {
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
#[doc = "Field `acal_sten` reader - "]
pub struct ACAL_STEN_R(crate::FieldReader<bool, bool>);
impl ACAL_STEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACAL_STEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACAL_STEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `acal_sten` writer - "]
pub struct ACAL_STEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ACAL_STEN_W<'a> {
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
#[doc = "Field `fcal_sten` reader - "]
pub struct FCAL_STEN_R(crate::FieldReader<bool, bool>);
impl FCAL_STEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        FCAL_STEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FCAL_STEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `fcal_sten` writer - "]
pub struct FCAL_STEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FCAL_STEN_W<'a> {
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
#[doc = "Field `dl_rfcal_table_sten` reader - "]
pub struct DL_RFCAL_TABLE_STEN_R(crate::FieldReader<bool, bool>);
impl DL_RFCAL_TABLE_STEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DL_RFCAL_TABLE_STEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DL_RFCAL_TABLE_STEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dl_rfcal_table_sten` writer - "]
pub struct DL_RFCAL_TABLE_STEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DL_RFCAL_TABLE_STEN_W<'a> {
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
#[doc = "Field `adc_oscal_sten` reader - "]
pub struct ADC_OSCAL_STEN_R(crate::FieldReader<bool, bool>);
impl ADC_OSCAL_STEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC_OSCAL_STEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_OSCAL_STEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adc_oscal_sten` writer - "]
pub struct ADC_OSCAL_STEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_OSCAL_STEN_W<'a> {
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
#[doc = "Field `rcal_sten_resv` reader - "]
pub struct RCAL_STEN_RESV_R(crate::FieldReader<bool, bool>);
impl RCAL_STEN_RESV_R {
    pub(crate) fn new(bits: bool) -> Self {
        RCAL_STEN_RESV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCAL_STEN_RESV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rcal_sten_resv` writer - "]
pub struct RCAL_STEN_RESV_W<'a> {
    w: &'a mut W,
}
impl<'a> RCAL_STEN_RESV_W<'a> {
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
    pub fn rfcal_level(&self) -> RFCAL_LEVEL_R {
        RFCAL_LEVEL_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn dpd_sten(&self) -> DPD_STEN_R {
        DPD_STEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn tsencal_sten(&self) -> TSENCAL_STEN_R {
        TSENCAL_STEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn pwdet_cal_sten(&self) -> PWDET_CAL_STEN_R {
        PWDET_CAL_STEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn riqcal_sten(&self) -> RIQCAL_STEN_R {
        RIQCAL_STEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn tiqcal_sten(&self) -> TIQCAL_STEN_R {
        TIQCAL_STEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn lo_leakcal_sten(&self) -> LO_LEAKCAL_STEN_R {
        LO_LEAKCAL_STEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn rccal_sten(&self) -> RCCAL_STEN_R {
        RCCAL_STEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn toscal_sten_resv(&self) -> TOSCAL_STEN_RESV_R {
        TOSCAL_STEN_RESV_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn roscal_sten(&self) -> ROSCAL_STEN_R {
        ROSCAL_STEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn clkpll_cal_sten(&self) -> CLKPLL_CAL_STEN_R {
        CLKPLL_CAL_STEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn inc_acal_sten(&self) -> INC_ACAL_STEN_R {
        INC_ACAL_STEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn inc_fcal_sten(&self) -> INC_FCAL_STEN_R {
        INC_FCAL_STEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn acal_sten(&self) -> ACAL_STEN_R {
        ACAL_STEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn fcal_sten(&self) -> FCAL_STEN_R {
        FCAL_STEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dl_rfcal_table_sten(&self) -> DL_RFCAL_TABLE_STEN_R {
        DL_RFCAL_TABLE_STEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn adc_oscal_sten(&self) -> ADC_OSCAL_STEN_R {
        ADC_OSCAL_STEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rcal_sten_resv(&self) -> RCAL_STEN_RESV_R {
        RCAL_STEN_RESV_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn rfcal_level(&mut self) -> RFCAL_LEVEL_W {
        RFCAL_LEVEL_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn dpd_sten(&mut self) -> DPD_STEN_W {
        DPD_STEN_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn tsencal_sten(&mut self) -> TSENCAL_STEN_W {
        TSENCAL_STEN_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn pwdet_cal_sten(&mut self) -> PWDET_CAL_STEN_W {
        PWDET_CAL_STEN_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn riqcal_sten(&mut self) -> RIQCAL_STEN_W {
        RIQCAL_STEN_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn tiqcal_sten(&mut self) -> TIQCAL_STEN_W {
        TIQCAL_STEN_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn lo_leakcal_sten(&mut self) -> LO_LEAKCAL_STEN_W {
        LO_LEAKCAL_STEN_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn rccal_sten(&mut self) -> RCCAL_STEN_W {
        RCCAL_STEN_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn toscal_sten_resv(&mut self) -> TOSCAL_STEN_RESV_W {
        TOSCAL_STEN_RESV_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn roscal_sten(&mut self) -> ROSCAL_STEN_W {
        ROSCAL_STEN_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn clkpll_cal_sten(&mut self) -> CLKPLL_CAL_STEN_W {
        CLKPLL_CAL_STEN_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn inc_acal_sten(&mut self) -> INC_ACAL_STEN_W {
        INC_ACAL_STEN_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn inc_fcal_sten(&mut self) -> INC_FCAL_STEN_W {
        INC_FCAL_STEN_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn acal_sten(&mut self) -> ACAL_STEN_W {
        ACAL_STEN_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn fcal_sten(&mut self) -> FCAL_STEN_W {
        FCAL_STEN_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dl_rfcal_table_sten(&mut self) -> DL_RFCAL_TABLE_STEN_W {
        DL_RFCAL_TABLE_STEN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn adc_oscal_sten(&mut self) -> ADC_OSCAL_STEN_W {
        ADC_OSCAL_STEN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rcal_sten_resv(&mut self) -> RCAL_STEN_RESV_W {
        RCAL_STEN_RESV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rf calibration state enabl in full cal list\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfcal_stateen](index.html) module"]
pub struct RFCAL_STATEEN_SPEC;
impl crate::RegisterSpec for RFCAL_STATEEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rfcal_stateen::R](R) reader structure"]
impl crate::Readable for RFCAL_STATEEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rfcal_stateen::W](W) writer structure"]
impl crate::Writable for RFCAL_STATEEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets rfcal_stateen to value 0"]
impl crate::Resettable for RFCAL_STATEEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

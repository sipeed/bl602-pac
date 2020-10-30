#[doc = "Register `rfcal_status` reader"]
pub struct R(crate::R<RFCAL_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RFCAL_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RFCAL_STATUS_SPEC>> for R {
    fn from(reader: crate::R<RFCAL_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rfcal_status` writer"]
pub struct W(crate::W<RFCAL_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RFCAL_STATUS_SPEC>;
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
impl core::convert::From<crate::W<RFCAL_STATUS_SPEC>> for W {
    fn from(writer: crate::W<RFCAL_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dpd_status` reader - "]
pub struct DPD_STATUS_R(crate::FieldReader<u8, u8>);
impl DPD_STATUS_R {
    pub(crate) fn new(bits: u8) -> Self {
        DPD_STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DPD_STATUS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dpd_status` writer - "]
pub struct DPD_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> DPD_STATUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
#[doc = "Field `tenscal_status` reader - "]
pub struct TENSCAL_STATUS_R(crate::FieldReader<u8, u8>);
impl TENSCAL_STATUS_R {
    pub(crate) fn new(bits: u8) -> Self {
        TENSCAL_STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TENSCAL_STATUS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tenscal_status` writer - "]
pub struct TENSCAL_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> TENSCAL_STATUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Field `pwdet_cal_status` reader - "]
pub struct PWDET_CAL_STATUS_R(crate::FieldReader<u8, u8>);
impl PWDET_CAL_STATUS_R {
    pub(crate) fn new(bits: u8) -> Self {
        PWDET_CAL_STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWDET_CAL_STATUS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwdet_cal_status` writer - "]
pub struct PWDET_CAL_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> PWDET_CAL_STATUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "Field `riqcal_status_resv` reader - "]
pub struct RIQCAL_STATUS_RESV_R(crate::FieldReader<u8, u8>);
impl RIQCAL_STATUS_RESV_R {
    pub(crate) fn new(bits: u8) -> Self {
        RIQCAL_STATUS_RESV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RIQCAL_STATUS_RESV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `riqcal_status_resv` writer - "]
pub struct RIQCAL_STATUS_RESV_W<'a> {
    w: &'a mut W,
}
impl<'a> RIQCAL_STATUS_RESV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Field `tiqcal_status_resv` reader - "]
pub struct TIQCAL_STATUS_RESV_R(crate::FieldReader<u8, u8>);
impl TIQCAL_STATUS_RESV_R {
    pub(crate) fn new(bits: u8) -> Self {
        TIQCAL_STATUS_RESV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIQCAL_STATUS_RESV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tiqcal_status_resv` writer - "]
pub struct TIQCAL_STATUS_RESV_W<'a> {
    w: &'a mut W,
}
impl<'a> TIQCAL_STATUS_RESV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Field `lo_leakcal_status` reader - "]
pub struct LO_LEAKCAL_STATUS_R(crate::FieldReader<u8, u8>);
impl LO_LEAKCAL_STATUS_R {
    pub(crate) fn new(bits: u8) -> Self {
        LO_LEAKCAL_STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_LEAKCAL_STATUS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_leakcal_status` writer - "]
pub struct LO_LEAKCAL_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_LEAKCAL_STATUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Field `rccal_status` reader - "]
pub struct RCCAL_STATUS_R(crate::FieldReader<u8, u8>);
impl RCCAL_STATUS_R {
    pub(crate) fn new(bits: u8) -> Self {
        RCCAL_STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCCAL_STATUS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rccal_status` writer - "]
pub struct RCCAL_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> RCCAL_STATUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Field `tos_status` reader - "]
pub struct TOS_STATUS_R(crate::FieldReader<u8, u8>);
impl TOS_STATUS_R {
    pub(crate) fn new(bits: u8) -> Self {
        TOS_STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOS_STATUS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tos_status` writer - "]
pub struct TOS_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> TOS_STATUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `ros_status` reader - "]
pub struct ROS_STATUS_R(crate::FieldReader<u8, u8>);
impl ROS_STATUS_R {
    pub(crate) fn new(bits: u8) -> Self {
        ROS_STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROS_STATUS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ros_status` writer - "]
pub struct ROS_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> ROS_STATUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Field `clkpll_cal_status` reader - "]
pub struct CLKPLL_CAL_STATUS_R(crate::FieldReader<u8, u8>);
impl CLKPLL_CAL_STATUS_R {
    pub(crate) fn new(bits: u8) -> Self {
        CLKPLL_CAL_STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKPLL_CAL_STATUS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clkpll_cal_status` writer - "]
pub struct CLKPLL_CAL_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_CAL_STATUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `inc_acal_status` reader - "]
pub struct INC_ACAL_STATUS_R(crate::FieldReader<u8, u8>);
impl INC_ACAL_STATUS_R {
    pub(crate) fn new(bits: u8) -> Self {
        INC_ACAL_STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INC_ACAL_STATUS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `inc_acal_status` writer - "]
pub struct INC_ACAL_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> INC_ACAL_STATUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Field `inc_fcal_status` reader - "]
pub struct INC_FCAL_STATUS_R(crate::FieldReader<u8, u8>);
impl INC_FCAL_STATUS_R {
    pub(crate) fn new(bits: u8) -> Self {
        INC_FCAL_STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INC_FCAL_STATUS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `inc_fcal_status` writer - "]
pub struct INC_FCAL_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> INC_FCAL_STATUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `acal_status` reader - "]
pub struct ACAL_STATUS_R(crate::FieldReader<u8, u8>);
impl ACAL_STATUS_R {
    pub(crate) fn new(bits: u8) -> Self {
        ACAL_STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACAL_STATUS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `acal_status` writer - "]
pub struct ACAL_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> ACAL_STATUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `fcal_status` reader - "]
pub struct FCAL_STATUS_R(crate::FieldReader<u8, u8>);
impl FCAL_STATUS_R {
    pub(crate) fn new(bits: u8) -> Self {
        FCAL_STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FCAL_STATUS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `fcal_status` writer - "]
pub struct FCAL_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> FCAL_STATUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `adc_oscal_status` reader - "]
pub struct ADC_OSCAL_STATUS_R(crate::FieldReader<u8, u8>);
impl ADC_OSCAL_STATUS_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADC_OSCAL_STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_OSCAL_STATUS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adc_oscal_status` writer - "]
pub struct ADC_OSCAL_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_OSCAL_STATUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `rcal_status` reader - "]
pub struct RCAL_STATUS_R(crate::FieldReader<u8, u8>);
impl RCAL_STATUS_R {
    pub(crate) fn new(bits: u8) -> Self {
        RCAL_STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCAL_STATUS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rcal_status` writer - "]
pub struct RCAL_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> RCAL_STATUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn dpd_status(&self) -> DPD_STATUS_R {
        DPD_STATUS_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn tenscal_status(&self) -> TENSCAL_STATUS_R {
        TENSCAL_STATUS_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn pwdet_cal_status(&self) -> PWDET_CAL_STATUS_R {
        PWDET_CAL_STATUS_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn riqcal_status_resv(&self) -> RIQCAL_STATUS_RESV_R {
        RIQCAL_STATUS_RESV_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn tiqcal_status_resv(&self) -> TIQCAL_STATUS_RESV_R {
        TIQCAL_STATUS_RESV_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn lo_leakcal_status(&self) -> LO_LEAKCAL_STATUS_R {
        LO_LEAKCAL_STATUS_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn rccal_status(&self) -> RCCAL_STATUS_R {
        RCCAL_STATUS_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn tos_status(&self) -> TOS_STATUS_R {
        TOS_STATUS_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn ros_status(&self) -> ROS_STATUS_R {
        ROS_STATUS_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn clkpll_cal_status(&self) -> CLKPLL_CAL_STATUS_R {
        CLKPLL_CAL_STATUS_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn inc_acal_status(&self) -> INC_ACAL_STATUS_R {
        INC_ACAL_STATUS_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn inc_fcal_status(&self) -> INC_FCAL_STATUS_R {
        INC_FCAL_STATUS_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn acal_status(&self) -> ACAL_STATUS_R {
        ACAL_STATUS_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn fcal_status(&self) -> FCAL_STATUS_R {
        FCAL_STATUS_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn adc_oscal_status(&self) -> ADC_OSCAL_STATUS_R {
        ADC_OSCAL_STATUS_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn rcal_status(&self) -> RCAL_STATUS_R {
        RCAL_STATUS_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn dpd_status(&mut self) -> DPD_STATUS_W {
        DPD_STATUS_W { w: self }
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn tenscal_status(&mut self) -> TENSCAL_STATUS_W {
        TENSCAL_STATUS_W { w: self }
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn pwdet_cal_status(&mut self) -> PWDET_CAL_STATUS_W {
        PWDET_CAL_STATUS_W { w: self }
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn riqcal_status_resv(&mut self) -> RIQCAL_STATUS_RESV_W {
        RIQCAL_STATUS_RESV_W { w: self }
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn tiqcal_status_resv(&mut self) -> TIQCAL_STATUS_RESV_W {
        TIQCAL_STATUS_RESV_W { w: self }
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn lo_leakcal_status(&mut self) -> LO_LEAKCAL_STATUS_W {
        LO_LEAKCAL_STATUS_W { w: self }
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn rccal_status(&mut self) -> RCCAL_STATUS_W {
        RCCAL_STATUS_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn tos_status(&mut self) -> TOS_STATUS_W {
        TOS_STATUS_W { w: self }
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn ros_status(&mut self) -> ROS_STATUS_W {
        ROS_STATUS_W { w: self }
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn clkpll_cal_status(&mut self) -> CLKPLL_CAL_STATUS_W {
        CLKPLL_CAL_STATUS_W { w: self }
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn inc_acal_status(&mut self) -> INC_ACAL_STATUS_W {
        INC_ACAL_STATUS_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn inc_fcal_status(&mut self) -> INC_FCAL_STATUS_W {
        INC_FCAL_STATUS_W { w: self }
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn acal_status(&mut self) -> ACAL_STATUS_W {
        ACAL_STATUS_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn fcal_status(&mut self) -> FCAL_STATUS_W {
        FCAL_STATUS_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn adc_oscal_status(&mut self) -> ADC_OSCAL_STATUS_W {
        ADC_OSCAL_STATUS_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn rcal_status(&mut self) -> RCAL_STATUS_W {
        RCAL_STATUS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rfcal_status.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfcal_status](index.html) module"]
pub struct RFCAL_STATUS_SPEC;
impl crate::RegisterSpec for RFCAL_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rfcal_status::R](R) reader structure"]
impl crate::Readable for RFCAL_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rfcal_status::W](W) writer structure"]
impl crate::Writable for RFCAL_STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets rfcal_status to value 0"]
impl crate::Resettable for RFCAL_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

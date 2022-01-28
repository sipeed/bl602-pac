#[doc = "Register `rf_base_ctrl1` reader"]
pub struct R(crate::R<RF_BASE_CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_BASE_CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RF_BASE_CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RF_BASE_CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rf_base_ctrl1` writer"]
pub struct W(crate::W<RF_BASE_CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF_BASE_CTRL1_SPEC>;
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
impl From<crate::W<RF_BASE_CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RF_BASE_CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `mbg_trim` reader - "]
pub struct MBG_TRIM_R(crate::FieldReader<u8, u8>);
impl MBG_TRIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        MBG_TRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MBG_TRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `mbg_trim` writer - "]
pub struct MBG_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> MBG_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 27)) | ((value as u32 & 0x03) << 27);
        self.w
    }
}
#[doc = "Field `pud_pa_dly` reader - "]
pub struct PUD_PA_DLY_R(crate::FieldReader<u8, u8>);
impl PUD_PA_DLY_R {
    pub(crate) fn new(bits: u8) -> Self {
        PUD_PA_DLY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PUD_PA_DLY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pud_pa_dly` writer - "]
pub struct PUD_PA_DLY_W<'a> {
    w: &'a mut W,
}
impl<'a> PUD_PA_DLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Field `pud_iref_dly` reader - "]
pub struct PUD_IREF_DLY_R(crate::FieldReader<u8, u8>);
impl PUD_IREF_DLY_R {
    pub(crate) fn new(bits: u8) -> Self {
        PUD_IREF_DLY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PUD_IREF_DLY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pud_iref_dly` writer - "]
pub struct PUD_IREF_DLY_W<'a> {
    w: &'a mut W,
}
impl<'a> PUD_IREF_DLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `pud_vco_dly` reader - "]
pub struct PUD_VCO_DLY_R(crate::FieldReader<u8, u8>);
impl PUD_VCO_DLY_R {
    pub(crate) fn new(bits: u8) -> Self {
        PUD_VCO_DLY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PUD_VCO_DLY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pud_vco_dly` writer - "]
pub struct PUD_VCO_DLY_W<'a> {
    w: &'a mut W,
}
impl<'a> PUD_VCO_DLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Field `ppu_lead` reader - "]
pub struct PPU_LEAD_R(crate::FieldReader<u8, u8>);
impl PPU_LEAD_R {
    pub(crate) fn new(bits: u8) -> Self {
        PPU_LEAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PPU_LEAD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ppu_lead` writer - "]
pub struct PPU_LEAD_W<'a> {
    w: &'a mut W,
}
impl<'a> PPU_LEAD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `lo_sdm_rst_dly` reader - "]
pub struct LO_SDM_RST_DLY_R(crate::FieldReader<u8, u8>);
impl LO_SDM_RST_DLY_R {
    pub(crate) fn new(bits: u8) -> Self {
        LO_SDM_RST_DLY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_SDM_RST_DLY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_sdm_rst_dly` writer - "]
pub struct LO_SDM_RST_DLY_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_RST_DLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `aupll_sdm_rst_dly` reader - "]
pub struct AUPLL_SDM_RST_DLY_R(crate::FieldReader<u8, u8>);
impl AUPLL_SDM_RST_DLY_R {
    pub(crate) fn new(bits: u8) -> Self {
        AUPLL_SDM_RST_DLY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AUPLL_SDM_RST_DLY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `aupll_sdm_rst_dly` writer - "]
pub struct AUPLL_SDM_RST_DLY_W<'a> {
    w: &'a mut W,
}
impl<'a> AUPLL_SDM_RST_DLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 27:28"]
    #[inline(always)]
    pub fn mbg_trim(&self) -> MBG_TRIM_R {
        MBG_TRIM_R::new(((self.bits >> 27) & 0x03) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn pud_pa_dly(&self) -> PUD_PA_DLY_R {
        PUD_PA_DLY_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn pud_iref_dly(&self) -> PUD_IREF_DLY_R {
        PUD_IREF_DLY_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn pud_vco_dly(&self) -> PUD_VCO_DLY_R {
        PUD_VCO_DLY_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn ppu_lead(&self) -> PPU_LEAD_R {
        PPU_LEAD_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn lo_sdm_rst_dly(&self) -> LO_SDM_RST_DLY_R {
        LO_SDM_RST_DLY_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn aupll_sdm_rst_dly(&self) -> AUPLL_SDM_RST_DLY_R {
        AUPLL_SDM_RST_DLY_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 27:28"]
    #[inline(always)]
    pub fn mbg_trim(&mut self) -> MBG_TRIM_W {
        MBG_TRIM_W { w: self }
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn pud_pa_dly(&mut self) -> PUD_PA_DLY_W {
        PUD_PA_DLY_W { w: self }
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn pud_iref_dly(&mut self) -> PUD_IREF_DLY_W {
        PUD_IREF_DLY_W { w: self }
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn pud_vco_dly(&mut self) -> PUD_VCO_DLY_W {
        PUD_VCO_DLY_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn ppu_lead(&mut self) -> PPU_LEAD_W {
        PPU_LEAD_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn lo_sdm_rst_dly(&mut self) -> LO_SDM_RST_DLY_W {
        LO_SDM_RST_DLY_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn aupll_sdm_rst_dly(&mut self) -> AUPLL_SDM_RST_DLY_W {
        AUPLL_SDM_RST_DLY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ZRF Control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_base_ctrl1](index.html) module"]
pub struct RF_BASE_CTRL1_SPEC;
impl crate::RegisterSpec for RF_BASE_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_base_ctrl1::R](R) reader structure"]
impl crate::Readable for RF_BASE_CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf_base_ctrl1::W](W) writer structure"]
impl crate::Writable for RF_BASE_CTRL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets rf_base_ctrl1 to value 0"]
impl crate::Resettable for RF_BASE_CTRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

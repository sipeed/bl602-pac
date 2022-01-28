#[doc = "Register `pu_rst_clkpll` reader"]
pub struct R(crate::R<PU_RST_CLKPLL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PU_RST_CLKPLL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PU_RST_CLKPLL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PU_RST_CLKPLL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pu_rst_clkpll` writer"]
pub struct W(crate::W<PU_RST_CLKPLL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PU_RST_CLKPLL_SPEC>;
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
impl From<crate::W<PU_RST_CLKPLL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PU_RST_CLKPLL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pu_clkpll` reader - "]
pub struct PU_CLKPLL_R(crate::FieldReader<bool, bool>);
impl PU_CLKPLL_R {
    pub(crate) fn new(bits: bool) -> Self {
        PU_CLKPLL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_CLKPLL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pu_clkpll` writer - "]
pub struct PU_CLKPLL_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_CLKPLL_W<'a> {
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
#[doc = "Field `pu_clkpll_sfreg` reader - "]
pub struct PU_CLKPLL_SFREG_R(crate::FieldReader<bool, bool>);
impl PU_CLKPLL_SFREG_R {
    pub(crate) fn new(bits: bool) -> Self {
        PU_CLKPLL_SFREG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_CLKPLL_SFREG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pu_clkpll_sfreg` writer - "]
pub struct PU_CLKPLL_SFREG_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_CLKPLL_SFREG_W<'a> {
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
#[doc = "Field `clkpll_pu_cp` reader - "]
pub struct CLKPLL_PU_CP_R(crate::FieldReader<bool, bool>);
impl CLKPLL_PU_CP_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLKPLL_PU_CP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKPLL_PU_CP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clkpll_pu_cp` writer - "]
pub struct CLKPLL_PU_CP_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_PU_CP_W<'a> {
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
#[doc = "Field `clkpll_pu_pfd` reader - "]
pub struct CLKPLL_PU_PFD_R(crate::FieldReader<bool, bool>);
impl CLKPLL_PU_PFD_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLKPLL_PU_PFD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKPLL_PU_PFD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clkpll_pu_pfd` writer - "]
pub struct CLKPLL_PU_PFD_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_PU_PFD_W<'a> {
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
#[doc = "Field `clkpll_pu_clamp_op` reader - "]
pub struct CLKPLL_PU_CLAMP_OP_R(crate::FieldReader<bool, bool>);
impl CLKPLL_PU_CLAMP_OP_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLKPLL_PU_CLAMP_OP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKPLL_PU_CLAMP_OP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clkpll_pu_clamp_op` writer - "]
pub struct CLKPLL_PU_CLAMP_OP_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_PU_CLAMP_OP_W<'a> {
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
#[doc = "Field `clkpll_pu_fbdv` reader - "]
pub struct CLKPLL_PU_FBDV_R(crate::FieldReader<bool, bool>);
impl CLKPLL_PU_FBDV_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLKPLL_PU_FBDV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKPLL_PU_FBDV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clkpll_pu_fbdv` writer - "]
pub struct CLKPLL_PU_FBDV_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_PU_FBDV_W<'a> {
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
#[doc = "Field `clkpll_pu_postdiv` reader - "]
pub struct CLKPLL_PU_POSTDIV_R(crate::FieldReader<bool, bool>);
impl CLKPLL_PU_POSTDIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLKPLL_PU_POSTDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKPLL_PU_POSTDIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clkpll_pu_postdiv` writer - "]
pub struct CLKPLL_PU_POSTDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_PU_POSTDIV_W<'a> {
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
#[doc = "Field `clkpll_reset_refdiv` reader - "]
pub struct CLKPLL_RESET_REFDIV_R(crate::FieldReader<bool, bool>);
impl CLKPLL_RESET_REFDIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLKPLL_RESET_REFDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKPLL_RESET_REFDIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clkpll_reset_refdiv` writer - "]
pub struct CLKPLL_RESET_REFDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_RESET_REFDIV_W<'a> {
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
#[doc = "Field `clkpll_reset_fbdv` reader - "]
pub struct CLKPLL_RESET_FBDV_R(crate::FieldReader<bool, bool>);
impl CLKPLL_RESET_FBDV_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLKPLL_RESET_FBDV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKPLL_RESET_FBDV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clkpll_reset_fbdv` writer - "]
pub struct CLKPLL_RESET_FBDV_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_RESET_FBDV_W<'a> {
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
#[doc = "Field `clkpll_reset_postdiv` reader - "]
pub struct CLKPLL_RESET_POSTDIV_R(crate::FieldReader<bool, bool>);
impl CLKPLL_RESET_POSTDIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLKPLL_RESET_POSTDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKPLL_RESET_POSTDIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clkpll_reset_postdiv` writer - "]
pub struct CLKPLL_RESET_POSTDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_RESET_POSTDIV_W<'a> {
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
#[doc = "Field `clkpll_sdm_reset` reader - "]
pub struct CLKPLL_SDM_RESET_R(crate::FieldReader<bool, bool>);
impl CLKPLL_SDM_RESET_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLKPLL_SDM_RESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKPLL_SDM_RESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clkpll_sdm_reset` writer - "]
pub struct CLKPLL_SDM_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_SDM_RESET_W<'a> {
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
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pu_clkpll(&self) -> PU_CLKPLL_R {
        PU_CLKPLL_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pu_clkpll_sfreg(&self) -> PU_CLKPLL_SFREG_R {
        PU_CLKPLL_SFREG_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn clkpll_pu_cp(&self) -> CLKPLL_PU_CP_R {
        CLKPLL_PU_CP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn clkpll_pu_pfd(&self) -> CLKPLL_PU_PFD_R {
        CLKPLL_PU_PFD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn clkpll_pu_clamp_op(&self) -> CLKPLL_PU_CLAMP_OP_R {
        CLKPLL_PU_CLAMP_OP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn clkpll_pu_fbdv(&self) -> CLKPLL_PU_FBDV_R {
        CLKPLL_PU_FBDV_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn clkpll_pu_postdiv(&self) -> CLKPLL_PU_POSTDIV_R {
        CLKPLL_PU_POSTDIV_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn clkpll_reset_refdiv(&self) -> CLKPLL_RESET_REFDIV_R {
        CLKPLL_RESET_REFDIV_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn clkpll_reset_fbdv(&self) -> CLKPLL_RESET_FBDV_R {
        CLKPLL_RESET_FBDV_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn clkpll_reset_postdiv(&self) -> CLKPLL_RESET_POSTDIV_R {
        CLKPLL_RESET_POSTDIV_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn clkpll_sdm_reset(&self) -> CLKPLL_SDM_RESET_R {
        CLKPLL_SDM_RESET_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pu_clkpll(&mut self) -> PU_CLKPLL_W {
        PU_CLKPLL_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pu_clkpll_sfreg(&mut self) -> PU_CLKPLL_SFREG_W {
        PU_CLKPLL_SFREG_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn clkpll_pu_cp(&mut self) -> CLKPLL_PU_CP_W {
        CLKPLL_PU_CP_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn clkpll_pu_pfd(&mut self) -> CLKPLL_PU_PFD_W {
        CLKPLL_PU_PFD_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn clkpll_pu_clamp_op(&mut self) -> CLKPLL_PU_CLAMP_OP_W {
        CLKPLL_PU_CLAMP_OP_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn clkpll_pu_fbdv(&mut self) -> CLKPLL_PU_FBDV_W {
        CLKPLL_PU_FBDV_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn clkpll_pu_postdiv(&mut self) -> CLKPLL_PU_POSTDIV_W {
        CLKPLL_PU_POSTDIV_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn clkpll_reset_refdiv(&mut self) -> CLKPLL_RESET_REFDIV_W {
        CLKPLL_RESET_REFDIV_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn clkpll_reset_fbdv(&mut self) -> CLKPLL_RESET_FBDV_W {
        CLKPLL_RESET_FBDV_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn clkpll_reset_postdiv(&mut self) -> CLKPLL_RESET_POSTDIV_W {
        CLKPLL_RESET_POSTDIV_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn clkpll_sdm_reset(&mut self) -> CLKPLL_SDM_RESET_W {
        CLKPLL_SDM_RESET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "pu_rst_clkpll.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pu_rst_clkpll](index.html) module"]
pub struct PU_RST_CLKPLL_SPEC;
impl crate::RegisterSpec for PU_RST_CLKPLL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pu_rst_clkpll::R](R) reader structure"]
impl crate::Readable for PU_RST_CLKPLL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pu_rst_clkpll::W](W) writer structure"]
impl crate::Writable for PU_RST_CLKPLL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets pu_rst_clkpll to value 0x01f0"]
impl crate::Resettable for PU_RST_CLKPLL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01f0
    }
}

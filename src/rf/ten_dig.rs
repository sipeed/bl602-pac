#[doc = "Register `ten_dig` reader"]
pub struct R(crate::R<TEN_DIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TEN_DIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<TEN_DIG_SPEC>> for R {
    fn from(reader: crate::R<TEN_DIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ten_dig` writer"]
pub struct W(crate::W<TEN_DIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TEN_DIG_SPEC>;
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
impl core::convert::From<crate::W<TEN_DIG_SPEC>> for W {
    fn from(writer: crate::W<TEN_DIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rf_dtest_en` reader - "]
pub struct RF_DTEST_EN_R(crate::FieldReader<bool, bool>);
impl RF_DTEST_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RF_DTEST_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_DTEST_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_dtest_en` writer - "]
pub struct RF_DTEST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_DTEST_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `dtest_pull_down` reader - "]
pub struct DTEST_PULL_DOWN_R(crate::FieldReader<bool, bool>);
impl DTEST_PULL_DOWN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTEST_PULL_DOWN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTEST_PULL_DOWN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dtest_pull_down` writer - "]
pub struct DTEST_PULL_DOWN_W<'a> {
    w: &'a mut W,
}
impl<'a> DTEST_PULL_DOWN_W<'a> {
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
#[doc = "Field `dten_lo_fref` reader - "]
pub struct DTEN_LO_FREF_R(crate::FieldReader<bool, bool>);
impl DTEN_LO_FREF_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTEN_LO_FREF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTEN_LO_FREF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dten_lo_fref` writer - "]
pub struct DTEN_LO_FREF_W<'a> {
    w: &'a mut W,
}
impl<'a> DTEN_LO_FREF_W<'a> {
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
#[doc = "Field `dten_lo_fsdm` reader - "]
pub struct DTEN_LO_FSDM_R(crate::FieldReader<bool, bool>);
impl DTEN_LO_FSDM_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTEN_LO_FSDM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTEN_LO_FSDM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dten_lo_fsdm` writer - "]
pub struct DTEN_LO_FSDM_W<'a> {
    w: &'a mut W,
}
impl<'a> DTEN_LO_FSDM_W<'a> {
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
#[doc = "Field `dten_clkpll_fin` reader - "]
pub struct DTEN_CLKPLL_FIN_R(crate::FieldReader<bool, bool>);
impl DTEN_CLKPLL_FIN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTEN_CLKPLL_FIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTEN_CLKPLL_FIN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dten_clkpll_fin` writer - "]
pub struct DTEN_CLKPLL_FIN_W<'a> {
    w: &'a mut W,
}
impl<'a> DTEN_CLKPLL_FIN_W<'a> {
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
#[doc = "Field `dten_clkpll_fref` reader - "]
pub struct DTEN_CLKPLL_FREF_R(crate::FieldReader<bool, bool>);
impl DTEN_CLKPLL_FREF_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTEN_CLKPLL_FREF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTEN_CLKPLL_FREF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dten_clkpll_fref` writer - "]
pub struct DTEN_CLKPLL_FREF_W<'a> {
    w: &'a mut W,
}
impl<'a> DTEN_CLKPLL_FREF_W<'a> {
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
#[doc = "Field `dten_clkpll_fsdm` reader - "]
pub struct DTEN_CLKPLL_FSDM_R(crate::FieldReader<bool, bool>);
impl DTEN_CLKPLL_FSDM_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTEN_CLKPLL_FSDM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTEN_CLKPLL_FSDM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dten_clkpll_fsdm` writer - "]
pub struct DTEN_CLKPLL_FSDM_W<'a> {
    w: &'a mut W,
}
impl<'a> DTEN_CLKPLL_FSDM_W<'a> {
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
#[doc = "Field `dten_clkpll_clk32m` reader - "]
pub struct DTEN_CLKPLL_CLK32M_R(crate::FieldReader<bool, bool>);
impl DTEN_CLKPLL_CLK32M_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTEN_CLKPLL_CLK32M_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTEN_CLKPLL_CLK32M_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dten_clkpll_clk32m` writer - "]
pub struct DTEN_CLKPLL_CLK32M_W<'a> {
    w: &'a mut W,
}
impl<'a> DTEN_CLKPLL_CLK32M_W<'a> {
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
#[doc = "Field `dten_clkpll_clk96m` reader - "]
pub struct DTEN_CLKPLL_CLK96M_R(crate::FieldReader<bool, bool>);
impl DTEN_CLKPLL_CLK96M_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTEN_CLKPLL_CLK96M_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTEN_CLKPLL_CLK96M_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dten_clkpll_clk96m` writer - "]
pub struct DTEN_CLKPLL_CLK96M_W<'a> {
    w: &'a mut W,
}
impl<'a> DTEN_CLKPLL_CLK96M_W<'a> {
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
#[doc = "Field `dten_clkpll_postdiv_clk` reader - "]
pub struct DTEN_CLKPLL_POSTDIV_CLK_R(crate::FieldReader<bool, bool>);
impl DTEN_CLKPLL_POSTDIV_CLK_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTEN_CLKPLL_POSTDIV_CLK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTEN_CLKPLL_POSTDIV_CLK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dten_clkpll_postdiv_clk` writer - "]
pub struct DTEN_CLKPLL_POSTDIV_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> DTEN_CLKPLL_POSTDIV_CLK_W<'a> {
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
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn rf_dtest_en(&self) -> RF_DTEST_EN_R {
        RF_DTEST_EN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn dtest_pull_down(&self) -> DTEST_PULL_DOWN_R {
        DTEST_PULL_DOWN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dten_lo_fref(&self) -> DTEN_LO_FREF_R {
        DTEN_LO_FREF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn dten_lo_fsdm(&self) -> DTEN_LO_FSDM_R {
        DTEN_LO_FSDM_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dten_clkpll_fin(&self) -> DTEN_CLKPLL_FIN_R {
        DTEN_CLKPLL_FIN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dten_clkpll_fref(&self) -> DTEN_CLKPLL_FREF_R {
        DTEN_CLKPLL_FREF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dten_clkpll_fsdm(&self) -> DTEN_CLKPLL_FSDM_R {
        DTEN_CLKPLL_FSDM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dten_clkpll_clk32m(&self) -> DTEN_CLKPLL_CLK32M_R {
        DTEN_CLKPLL_CLK32M_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dten_clkpll_clk96m(&self) -> DTEN_CLKPLL_CLK96M_R {
        DTEN_CLKPLL_CLK96M_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dten_clkpll_postdiv_clk(&self) -> DTEN_CLKPLL_POSTDIV_CLK_R {
        DTEN_CLKPLL_POSTDIV_CLK_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn rf_dtest_en(&mut self) -> RF_DTEST_EN_W {
        RF_DTEST_EN_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn dtest_pull_down(&mut self) -> DTEST_PULL_DOWN_W {
        DTEST_PULL_DOWN_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dten_lo_fref(&mut self) -> DTEN_LO_FREF_W {
        DTEN_LO_FREF_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn dten_lo_fsdm(&mut self) -> DTEN_LO_FSDM_W {
        DTEN_LO_FSDM_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dten_clkpll_fin(&mut self) -> DTEN_CLKPLL_FIN_W {
        DTEN_CLKPLL_FIN_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dten_clkpll_fref(&mut self) -> DTEN_CLKPLL_FREF_W {
        DTEN_CLKPLL_FREF_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dten_clkpll_fsdm(&mut self) -> DTEN_CLKPLL_FSDM_W {
        DTEN_CLKPLL_FSDM_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dten_clkpll_clk32m(&mut self) -> DTEN_CLKPLL_CLK32M_W {
        DTEN_CLKPLL_CLK32M_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dten_clkpll_clk96m(&mut self) -> DTEN_CLKPLL_CLK96M_W {
        DTEN_CLKPLL_CLK96M_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dten_clkpll_postdiv_clk(&mut self) -> DTEN_CLKPLL_POSTDIV_CLK_W {
        DTEN_CLKPLL_POSTDIV_CLK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "digital test register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ten_dig](index.html) module"]
pub struct TEN_DIG_SPEC;
impl crate::RegisterSpec for TEN_DIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ten_dig::R](R) reader structure"]
impl crate::Readable for TEN_DIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ten_dig::W](W) writer structure"]
impl crate::Writable for TEN_DIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ten_dig to value 0"]
impl crate::Resettable for TEN_DIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

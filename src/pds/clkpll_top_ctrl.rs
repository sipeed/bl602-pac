#[doc = "Register `clkpll_top_ctrl` reader"]
pub struct R(crate::R<CLKPLL_TOP_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKPLL_TOP_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKPLL_TOP_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKPLL_TOP_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `clkpll_top_ctrl` writer"]
pub struct W(crate::W<CLKPLL_TOP_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKPLL_TOP_CTRL_SPEC>;
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
impl From<crate::W<CLKPLL_TOP_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKPLL_TOP_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `clkpll_vg13_sel` reader - "]
pub struct CLKPLL_VG13_SEL_R(crate::FieldReader<u8, u8>);
impl CLKPLL_VG13_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CLKPLL_VG13_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKPLL_VG13_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clkpll_vg13_sel` writer - "]
pub struct CLKPLL_VG13_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_VG13_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "Field `clkpll_vg11_sel` reader - "]
pub struct CLKPLL_VG11_SEL_R(crate::FieldReader<u8, u8>);
impl CLKPLL_VG11_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CLKPLL_VG11_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKPLL_VG11_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clkpll_vg11_sel` writer - "]
pub struct CLKPLL_VG11_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_VG11_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "Field `clkpll_refclk_sel` reader - "]
pub struct CLKPLL_REFCLK_SEL_R(crate::FieldReader<bool, bool>);
impl CLKPLL_REFCLK_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLKPLL_REFCLK_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKPLL_REFCLK_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clkpll_refclk_sel` writer - "]
pub struct CLKPLL_REFCLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_REFCLK_SEL_W<'a> {
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
#[doc = "Field `clkpll_xtal_rc32m_sel` reader - "]
pub struct CLKPLL_XTAL_RC32M_SEL_R(crate::FieldReader<bool, bool>);
impl CLKPLL_XTAL_RC32M_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLKPLL_XTAL_RC32M_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKPLL_XTAL_RC32M_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clkpll_xtal_rc32m_sel` writer - "]
pub struct CLKPLL_XTAL_RC32M_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_XTAL_RC32M_SEL_W<'a> {
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
#[doc = "Field `clkpll_refdiv_ratio` reader - "]
pub struct CLKPLL_REFDIV_RATIO_R(crate::FieldReader<u8, u8>);
impl CLKPLL_REFDIV_RATIO_R {
    pub(crate) fn new(bits: u8) -> Self {
        CLKPLL_REFDIV_RATIO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKPLL_REFDIV_RATIO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clkpll_refdiv_ratio` writer - "]
pub struct CLKPLL_REFDIV_RATIO_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_REFDIV_RATIO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `clkpll_postdiv` reader - "]
pub struct CLKPLL_POSTDIV_R(crate::FieldReader<u8, u8>);
impl CLKPLL_POSTDIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        CLKPLL_POSTDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKPLL_POSTDIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clkpll_postdiv` writer - "]
pub struct CLKPLL_POSTDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_POSTDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn clkpll_vg13_sel(&self) -> CLKPLL_VG13_SEL_R {
        CLKPLL_VG13_SEL_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn clkpll_vg11_sel(&self) -> CLKPLL_VG11_SEL_R {
        CLKPLL_VG11_SEL_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn clkpll_refclk_sel(&self) -> CLKPLL_REFCLK_SEL_R {
        CLKPLL_REFCLK_SEL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn clkpll_xtal_rc32m_sel(&self) -> CLKPLL_XTAL_RC32M_SEL_R {
        CLKPLL_XTAL_RC32M_SEL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn clkpll_refdiv_ratio(&self) -> CLKPLL_REFDIV_RATIO_R {
        CLKPLL_REFDIV_RATIO_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn clkpll_postdiv(&self) -> CLKPLL_POSTDIV_R {
        CLKPLL_POSTDIV_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn clkpll_vg13_sel(&mut self) -> CLKPLL_VG13_SEL_W {
        CLKPLL_VG13_SEL_W { w: self }
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn clkpll_vg11_sel(&mut self) -> CLKPLL_VG11_SEL_W {
        CLKPLL_VG11_SEL_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn clkpll_refclk_sel(&mut self) -> CLKPLL_REFCLK_SEL_W {
        CLKPLL_REFCLK_SEL_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn clkpll_xtal_rc32m_sel(&mut self) -> CLKPLL_XTAL_RC32M_SEL_W {
        CLKPLL_XTAL_RC32M_SEL_W { w: self }
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn clkpll_refdiv_ratio(&mut self) -> CLKPLL_REFDIV_RATIO_W {
        CLKPLL_REFDIV_RATIO_W { w: self }
    }
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn clkpll_postdiv(&mut self) -> CLKPLL_POSTDIV_W {
        CLKPLL_POSTDIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "clkpll_top_ctrl.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkpll_top_ctrl](index.html) module"]
pub struct CLKPLL_TOP_CTRL_SPEC;
impl crate::RegisterSpec for CLKPLL_TOP_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clkpll_top_ctrl::R](R) reader structure"]
impl crate::Readable for CLKPLL_TOP_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clkpll_top_ctrl::W](W) writer structure"]
impl crate::Writable for CLKPLL_TOP_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets clkpll_top_ctrl to value 0x0110_0414"]
impl crate::Resettable for CLKPLL_TOP_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0110_0414
    }
}

#[doc = "Register `fbdv` reader"]
pub struct R(crate::R<FBDV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FBDV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<FBDV_SPEC>> for R {
    fn from(reader: crate::R<FBDV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `fbdv` writer"]
pub struct W(crate::W<FBDV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FBDV_SPEC>;
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
impl core::convert::From<crate::W<FBDV_SPEC>> for W {
    fn from(writer: crate::W<FBDV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `lo_fbdv_rst_hw` reader - "]
pub struct LO_FBDV_RST_HW_R(crate::FieldReader<bool, bool>);
impl LO_FBDV_RST_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        LO_FBDV_RST_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_FBDV_RST_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_fbdv_rst_hw` writer - "]
pub struct LO_FBDV_RST_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_FBDV_RST_HW_W<'a> {
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
#[doc = "Field `lo_fbdv_rst` reader - "]
pub struct LO_FBDV_RST_R(crate::FieldReader<bool, bool>);
impl LO_FBDV_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        LO_FBDV_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_FBDV_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_fbdv_rst` writer - "]
pub struct LO_FBDV_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_FBDV_RST_W<'a> {
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
#[doc = "Field `lo_fbdv_sel_fb_clk` reader - "]
pub struct LO_FBDV_SEL_FB_CLK_R(crate::FieldReader<u8, u8>);
impl LO_FBDV_SEL_FB_CLK_R {
    pub(crate) fn new(bits: u8) -> Self {
        LO_FBDV_SEL_FB_CLK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_FBDV_SEL_FB_CLK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_fbdv_sel_fb_clk` writer - "]
pub struct LO_FBDV_SEL_FB_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_FBDV_SEL_FB_CLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `lo_fbdv_sel_sample_clk` reader - "]
pub struct LO_FBDV_SEL_SAMPLE_CLK_R(crate::FieldReader<u8, u8>);
impl LO_FBDV_SEL_SAMPLE_CLK_R {
    pub(crate) fn new(bits: u8) -> Self {
        LO_FBDV_SEL_SAMPLE_CLK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_FBDV_SEL_SAMPLE_CLK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_fbdv_sel_sample_clk` writer - "]
pub struct LO_FBDV_SEL_SAMPLE_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_FBDV_SEL_SAMPLE_CLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `lo_fbdv_halfstep_en` reader - "]
pub struct LO_FBDV_HALFSTEP_EN_R(crate::FieldReader<bool, bool>);
impl LO_FBDV_HALFSTEP_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LO_FBDV_HALFSTEP_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_FBDV_HALFSTEP_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_fbdv_halfstep_en` writer - "]
pub struct LO_FBDV_HALFSTEP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_FBDV_HALFSTEP_EN_W<'a> {
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
#[doc = "Field `lo_fbdv_halfstep_en_hw` reader - "]
pub struct LO_FBDV_HALFSTEP_EN_HW_R(crate::FieldReader<bool, bool>);
impl LO_FBDV_HALFSTEP_EN_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        LO_FBDV_HALFSTEP_EN_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_FBDV_HALFSTEP_EN_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_fbdv_halfstep_en_hw` writer - "]
pub struct LO_FBDV_HALFSTEP_EN_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_FBDV_HALFSTEP_EN_HW_W<'a> {
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
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn lo_fbdv_rst_hw(&self) -> LO_FBDV_RST_HW_R {
        LO_FBDV_RST_HW_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn lo_fbdv_rst(&self) -> LO_FBDV_RST_R {
        LO_FBDV_RST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn lo_fbdv_sel_fb_clk(&self) -> LO_FBDV_SEL_FB_CLK_R {
        LO_FBDV_SEL_FB_CLK_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn lo_fbdv_sel_sample_clk(&self) -> LO_FBDV_SEL_SAMPLE_CLK_R {
        LO_FBDV_SEL_SAMPLE_CLK_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn lo_fbdv_halfstep_en(&self) -> LO_FBDV_HALFSTEP_EN_R {
        LO_FBDV_HALFSTEP_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn lo_fbdv_halfstep_en_hw(&self) -> LO_FBDV_HALFSTEP_EN_HW_R {
        LO_FBDV_HALFSTEP_EN_HW_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn lo_fbdv_rst_hw(&mut self) -> LO_FBDV_RST_HW_W {
        LO_FBDV_RST_HW_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn lo_fbdv_rst(&mut self) -> LO_FBDV_RST_W {
        LO_FBDV_RST_W { w: self }
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn lo_fbdv_sel_fb_clk(&mut self) -> LO_FBDV_SEL_FB_CLK_W {
        LO_FBDV_SEL_FB_CLK_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn lo_fbdv_sel_sample_clk(&mut self) -> LO_FBDV_SEL_SAMPLE_CLK_W {
        LO_FBDV_SEL_SAMPLE_CLK_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn lo_fbdv_halfstep_en(&mut self) -> LO_FBDV_HALFSTEP_EN_W {
        LO_FBDV_HALFSTEP_EN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn lo_fbdv_halfstep_en_hw(&mut self) -> LO_FBDV_HALFSTEP_EN_HW_W {
        LO_FBDV_HALFSTEP_EN_HW_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "fbdv.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fbdv](index.html) module"]
pub struct FBDV_SPEC;
impl crate::RegisterSpec for FBDV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fbdv::R](R) reader structure"]
impl crate::Readable for FBDV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fbdv::W](W) writer structure"]
impl crate::Writable for FBDV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets fbdv to value 0"]
impl crate::Resettable for FBDV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

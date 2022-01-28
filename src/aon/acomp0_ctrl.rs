#[doc = "Register `acomp0_ctrl` reader"]
pub struct R(crate::R<ACOMP0_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACOMP0_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACOMP0_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACOMP0_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `acomp0_ctrl` writer"]
pub struct W(crate::W<ACOMP0_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACOMP0_CTRL_SPEC>;
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
impl From<crate::W<ACOMP0_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACOMP0_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `acomp0_muxen` reader - "]
pub struct ACOMP0_MUXEN_R(crate::FieldReader<bool, bool>);
impl ACOMP0_MUXEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACOMP0_MUXEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACOMP0_MUXEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `acomp0_muxen` writer - "]
pub struct ACOMP0_MUXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ACOMP0_MUXEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `acomp0_pos_sel` reader - "]
pub struct ACOMP0_POS_SEL_R(crate::FieldReader<u8, u8>);
impl ACOMP0_POS_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        ACOMP0_POS_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACOMP0_POS_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `acomp0_pos_sel` writer - "]
pub struct ACOMP0_POS_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ACOMP0_POS_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 22)) | ((value as u32 & 0x0f) << 22);
        self.w
    }
}
#[doc = "Field `acomp0_neg_sel` reader - "]
pub struct ACOMP0_NEG_SEL_R(crate::FieldReader<u8, u8>);
impl ACOMP0_NEG_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        ACOMP0_NEG_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACOMP0_NEG_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `acomp0_neg_sel` writer - "]
pub struct ACOMP0_NEG_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ACOMP0_NEG_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 18)) | ((value as u32 & 0x0f) << 18);
        self.w
    }
}
#[doc = "Field `acomp0_level_sel` reader - "]
pub struct ACOMP0_LEVEL_SEL_R(crate::FieldReader<u8, u8>);
impl ACOMP0_LEVEL_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        ACOMP0_LEVEL_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACOMP0_LEVEL_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `acomp0_level_sel` writer - "]
pub struct ACOMP0_LEVEL_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ACOMP0_LEVEL_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 12)) | ((value as u32 & 0x3f) << 12);
        self.w
    }
}
#[doc = "Field `acomp0_bias_prog` reader - "]
pub struct ACOMP0_BIAS_PROG_R(crate::FieldReader<u8, u8>);
impl ACOMP0_BIAS_PROG_R {
    pub(crate) fn new(bits: u8) -> Self {
        ACOMP0_BIAS_PROG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACOMP0_BIAS_PROG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `acomp0_bias_prog` writer - "]
pub struct ACOMP0_BIAS_PROG_W<'a> {
    w: &'a mut W,
}
impl<'a> ACOMP0_BIAS_PROG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Field `acomp0_hyst_selp` reader - "]
pub struct ACOMP0_HYST_SELP_R(crate::FieldReader<u8, u8>);
impl ACOMP0_HYST_SELP_R {
    pub(crate) fn new(bits: u8) -> Self {
        ACOMP0_HYST_SELP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACOMP0_HYST_SELP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `acomp0_hyst_selp` writer - "]
pub struct ACOMP0_HYST_SELP_W<'a> {
    w: &'a mut W,
}
impl<'a> ACOMP0_HYST_SELP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 7)) | ((value as u32 & 0x07) << 7);
        self.w
    }
}
#[doc = "Field `acomp0_hyst_seln` reader - "]
pub struct ACOMP0_HYST_SELN_R(crate::FieldReader<u8, u8>);
impl ACOMP0_HYST_SELN_R {
    pub(crate) fn new(bits: u8) -> Self {
        ACOMP0_HYST_SELN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACOMP0_HYST_SELN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `acomp0_hyst_seln` writer - "]
pub struct ACOMP0_HYST_SELN_W<'a> {
    w: &'a mut W,
}
impl<'a> ACOMP0_HYST_SELN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `acomp0_en` reader - "]
pub struct ACOMP0_EN_R(crate::FieldReader<bool, bool>);
impl ACOMP0_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACOMP0_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACOMP0_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `acomp0_en` writer - "]
pub struct ACOMP0_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ACOMP0_EN_W<'a> {
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
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn acomp0_muxen(&self) -> ACOMP0_MUXEN_R {
        ACOMP0_MUXEN_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 22:25"]
    #[inline(always)]
    pub fn acomp0_pos_sel(&self) -> ACOMP0_POS_SEL_R {
        ACOMP0_POS_SEL_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bits 18:21"]
    #[inline(always)]
    pub fn acomp0_neg_sel(&self) -> ACOMP0_NEG_SEL_R {
        ACOMP0_NEG_SEL_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 12:17"]
    #[inline(always)]
    pub fn acomp0_level_sel(&self) -> ACOMP0_LEVEL_SEL_R {
        ACOMP0_LEVEL_SEL_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn acomp0_bias_prog(&self) -> ACOMP0_BIAS_PROG_R {
        ACOMP0_BIAS_PROG_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 7:9"]
    #[inline(always)]
    pub fn acomp0_hyst_selp(&self) -> ACOMP0_HYST_SELP_R {
        ACOMP0_HYST_SELP_R::new(((self.bits >> 7) & 0x07) as u8)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn acomp0_hyst_seln(&self) -> ACOMP0_HYST_SELN_R {
        ACOMP0_HYST_SELN_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn acomp0_en(&self) -> ACOMP0_EN_R {
        ACOMP0_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn acomp0_muxen(&mut self) -> ACOMP0_MUXEN_W {
        ACOMP0_MUXEN_W { w: self }
    }
    #[doc = "Bits 22:25"]
    #[inline(always)]
    pub fn acomp0_pos_sel(&mut self) -> ACOMP0_POS_SEL_W {
        ACOMP0_POS_SEL_W { w: self }
    }
    #[doc = "Bits 18:21"]
    #[inline(always)]
    pub fn acomp0_neg_sel(&mut self) -> ACOMP0_NEG_SEL_W {
        ACOMP0_NEG_SEL_W { w: self }
    }
    #[doc = "Bits 12:17"]
    #[inline(always)]
    pub fn acomp0_level_sel(&mut self) -> ACOMP0_LEVEL_SEL_W {
        ACOMP0_LEVEL_SEL_W { w: self }
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn acomp0_bias_prog(&mut self) -> ACOMP0_BIAS_PROG_W {
        ACOMP0_BIAS_PROG_W { w: self }
    }
    #[doc = "Bits 7:9"]
    #[inline(always)]
    pub fn acomp0_hyst_selp(&mut self) -> ACOMP0_HYST_SELP_W {
        ACOMP0_HYST_SELP_W { w: self }
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn acomp0_hyst_seln(&mut self) -> ACOMP0_HYST_SELN_W {
        ACOMP0_HYST_SELN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn acomp0_en(&mut self) -> ACOMP0_EN_W {
        ACOMP0_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "acomp0_ctrl.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acomp0_ctrl](index.html) module"]
pub struct ACOMP0_CTRL_SPEC;
impl crate::RegisterSpec for ACOMP0_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [acomp0_ctrl::R](R) reader structure"]
impl crate::Readable for ACOMP0_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [acomp0_ctrl::W](W) writer structure"]
impl crate::Writable for ACOMP0_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets acomp0_ctrl to value 0"]
impl crate::Resettable for ACOMP0_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

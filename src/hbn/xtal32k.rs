#[doc = "Register `xtal32k` reader"]
pub struct R(crate::R<XTAL32K_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XTAL32K_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XTAL32K_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XTAL32K_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `xtal32k` writer"]
pub struct W(crate::W<XTAL32K_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XTAL32K_SPEC>;
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
impl From<crate::W<XTAL32K_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XTAL32K_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pu_xtal32k` reader - "]
pub struct PU_XTAL32K_R(crate::FieldReader<bool, bool>);
impl PU_XTAL32K_R {
    pub(crate) fn new(bits: bool) -> Self {
        PU_XTAL32K_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_XTAL32K_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pu_xtal32k` writer - "]
pub struct PU_XTAL32K_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_XTAL32K_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `pu_xtal32k_buf` reader - "]
pub struct PU_XTAL32K_BUF_R(crate::FieldReader<bool, bool>);
impl PU_XTAL32K_BUF_R {
    pub(crate) fn new(bits: bool) -> Self {
        PU_XTAL32K_BUF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_XTAL32K_BUF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pu_xtal32k_buf` writer - "]
pub struct PU_XTAL32K_BUF_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_XTAL32K_BUF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `xtal32k_ac_cap_short` reader - "]
pub struct XTAL32K_AC_CAP_SHORT_R(crate::FieldReader<bool, bool>);
impl XTAL32K_AC_CAP_SHORT_R {
    pub(crate) fn new(bits: bool) -> Self {
        XTAL32K_AC_CAP_SHORT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTAL32K_AC_CAP_SHORT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `xtal32k_ac_cap_short` writer - "]
pub struct XTAL32K_AC_CAP_SHORT_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL32K_AC_CAP_SHORT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `xtal32k_capbank` reader - "]
pub struct XTAL32K_CAPBANK_R(crate::FieldReader<u8, u8>);
impl XTAL32K_CAPBANK_R {
    pub(crate) fn new(bits: u8) -> Self {
        XTAL32K_CAPBANK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTAL32K_CAPBANK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `xtal32k_capbank` writer - "]
pub struct XTAL32K_CAPBANK_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL32K_CAPBANK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 11)) | ((value as u32 & 0x3f) << 11);
        self.w
    }
}
#[doc = "Field `xtal32k_inv_stre` reader - "]
pub struct XTAL32K_INV_STRE_R(crate::FieldReader<u8, u8>);
impl XTAL32K_INV_STRE_R {
    pub(crate) fn new(bits: u8) -> Self {
        XTAL32K_INV_STRE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTAL32K_INV_STRE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `xtal32k_inv_stre` writer - "]
pub struct XTAL32K_INV_STRE_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL32K_INV_STRE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | ((value as u32 & 0x03) << 9);
        self.w
    }
}
#[doc = "Field `xtal32k_otf_short` reader - "]
pub struct XTAL32K_OTF_SHORT_R(crate::FieldReader<bool, bool>);
impl XTAL32K_OTF_SHORT_R {
    pub(crate) fn new(bits: bool) -> Self {
        XTAL32K_OTF_SHORT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTAL32K_OTF_SHORT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `xtal32k_otf_short` writer - "]
pub struct XTAL32K_OTF_SHORT_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL32K_OTF_SHORT_W<'a> {
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
#[doc = "Field `xtal32k_outbuf_stre` reader - "]
pub struct XTAL32K_OUTBUF_STRE_R(crate::FieldReader<bool, bool>);
impl XTAL32K_OUTBUF_STRE_R {
    pub(crate) fn new(bits: bool) -> Self {
        XTAL32K_OUTBUF_STRE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTAL32K_OUTBUF_STRE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `xtal32k_outbuf_stre` writer - "]
pub struct XTAL32K_OUTBUF_STRE_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL32K_OUTBUF_STRE_W<'a> {
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
#[doc = "Field `xtal32k_reg` reader - "]
pub struct XTAL32K_REG_R(crate::FieldReader<u8, u8>);
impl XTAL32K_REG_R {
    pub(crate) fn new(bits: u8) -> Self {
        XTAL32K_REG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTAL32K_REG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `xtal32k_reg` writer - "]
pub struct XTAL32K_REG_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL32K_REG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | ((value as u32 & 0x03) << 5);
        self.w
    }
}
#[doc = "Field `xtal32k_amp_ctrl` reader - "]
pub struct XTAL32K_AMP_CTRL_R(crate::FieldReader<u8, u8>);
impl XTAL32K_AMP_CTRL_R {
    pub(crate) fn new(bits: u8) -> Self {
        XTAL32K_AMP_CTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTAL32K_AMP_CTRL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `xtal32k_amp_ctrl` writer - "]
pub struct XTAL32K_AMP_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL32K_AMP_CTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | ((value as u32 & 0x03) << 3);
        self.w
    }
}
#[doc = "Field `xtal32k_ext_sel` reader - "]
pub struct XTAL32K_EXT_SEL_R(crate::FieldReader<bool, bool>);
impl XTAL32K_EXT_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        XTAL32K_EXT_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTAL32K_EXT_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `xtal32k_ext_sel` writer - "]
pub struct XTAL32K_EXT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL32K_EXT_SEL_W<'a> {
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
impl R {
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn pu_xtal32k(&self) -> PU_XTAL32K_R {
        PU_XTAL32K_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn pu_xtal32k_buf(&self) -> PU_XTAL32K_BUF_R {
        PU_XTAL32K_BUF_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn xtal32k_ac_cap_short(&self) -> XTAL32K_AC_CAP_SHORT_R {
        XTAL32K_AC_CAP_SHORT_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 11:16"]
    #[inline(always)]
    pub fn xtal32k_capbank(&self) -> XTAL32K_CAPBANK_R {
        XTAL32K_CAPBANK_R::new(((self.bits >> 11) & 0x3f) as u8)
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn xtal32k_inv_stre(&self) -> XTAL32K_INV_STRE_R {
        XTAL32K_INV_STRE_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn xtal32k_otf_short(&self) -> XTAL32K_OTF_SHORT_R {
        XTAL32K_OTF_SHORT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn xtal32k_outbuf_stre(&self) -> XTAL32K_OUTBUF_STRE_R {
        XTAL32K_OUTBUF_STRE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn xtal32k_reg(&self) -> XTAL32K_REG_R {
        XTAL32K_REG_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn xtal32k_amp_ctrl(&self) -> XTAL32K_AMP_CTRL_R {
        XTAL32K_AMP_CTRL_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn xtal32k_ext_sel(&self) -> XTAL32K_EXT_SEL_R {
        XTAL32K_EXT_SEL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn pu_xtal32k(&mut self) -> PU_XTAL32K_W {
        PU_XTAL32K_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn pu_xtal32k_buf(&mut self) -> PU_XTAL32K_BUF_W {
        PU_XTAL32K_BUF_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn xtal32k_ac_cap_short(&mut self) -> XTAL32K_AC_CAP_SHORT_W {
        XTAL32K_AC_CAP_SHORT_W { w: self }
    }
    #[doc = "Bits 11:16"]
    #[inline(always)]
    pub fn xtal32k_capbank(&mut self) -> XTAL32K_CAPBANK_W {
        XTAL32K_CAPBANK_W { w: self }
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn xtal32k_inv_stre(&mut self) -> XTAL32K_INV_STRE_W {
        XTAL32K_INV_STRE_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn xtal32k_otf_short(&mut self) -> XTAL32K_OTF_SHORT_W {
        XTAL32K_OTF_SHORT_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn xtal32k_outbuf_stre(&mut self) -> XTAL32K_OUTBUF_STRE_W {
        XTAL32K_OUTBUF_STRE_W { w: self }
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn xtal32k_reg(&mut self) -> XTAL32K_REG_W {
        XTAL32K_REG_W { w: self }
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn xtal32k_amp_ctrl(&mut self) -> XTAL32K_AMP_CTRL_W {
        XTAL32K_AMP_CTRL_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn xtal32k_ext_sel(&mut self) -> XTAL32K_EXT_SEL_W {
        XTAL32K_EXT_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "xtal32k.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xtal32k](index.html) module"]
pub struct XTAL32K_SPEC;
impl crate::RegisterSpec for XTAL32K_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xtal32k::R](R) reader structure"]
impl crate::Readable for XTAL32K_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xtal32k::W](W) writer structure"]
impl crate::Writable for XTAL32K_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets xtal32k to value 0x000f_0228"]
impl crate::Resettable for XTAL32K_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x000f_0228
    }
}

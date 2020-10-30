#[doc = "Register `lodist` reader"]
pub struct R(crate::R<LODIST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LODIST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<LODIST_SPEC>> for R {
    fn from(reader: crate::R<LODIST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `lodist` writer"]
pub struct W(crate::W<LODIST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LODIST_SPEC>;
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
impl core::convert::From<crate::W<LODIST_SPEC>> for W {
    fn from(writer: crate::W<LODIST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `lo_lodist_rxbuf_stre` reader - "]
pub struct LO_LODIST_RXBUF_STRE_R(crate::FieldReader<bool, bool>);
impl LO_LODIST_RXBUF_STRE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LO_LODIST_RXBUF_STRE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_LODIST_RXBUF_STRE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_lodist_rxbuf_stre` writer - "]
pub struct LO_LODIST_RXBUF_STRE_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_LODIST_RXBUF_STRE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `lo_lodist_txbuf_stre` reader - "]
pub struct LO_LODIST_TXBUF_STRE_R(crate::FieldReader<bool, bool>);
impl LO_LODIST_TXBUF_STRE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LO_LODIST_TXBUF_STRE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_LODIST_TXBUF_STRE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_lodist_txbuf_stre` writer - "]
pub struct LO_LODIST_TXBUF_STRE_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_LODIST_TXBUF_STRE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `lo_osmx_cap` reader - "]
pub struct LO_OSMX_CAP_R(crate::FieldReader<u8, u8>);
impl LO_OSMX_CAP_R {
    pub(crate) fn new(bits: u8) -> Self {
        LO_OSMX_CAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_OSMX_CAP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_osmx_cap` writer - "]
pub struct LO_OSMX_CAP_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_OSMX_CAP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Field `lo_osmx_capbank_bias` reader - "]
pub struct LO_OSMX_CAPBANK_BIAS_R(crate::FieldReader<u8, u8>);
impl LO_OSMX_CAPBANK_BIAS_R {
    pub(crate) fn new(bits: u8) -> Self {
        LO_OSMX_CAPBANK_BIAS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_OSMX_CAPBANK_BIAS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_osmx_capbank_bias` writer - "]
pub struct LO_OSMX_CAPBANK_BIAS_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_OSMX_CAPBANK_BIAS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `lo_osmx_vbuf_stre` reader - "]
pub struct LO_OSMX_VBUF_STRE_R(crate::FieldReader<bool, bool>);
impl LO_OSMX_VBUF_STRE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LO_OSMX_VBUF_STRE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_OSMX_VBUF_STRE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_osmx_vbuf_stre` writer - "]
pub struct LO_OSMX_VBUF_STRE_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_OSMX_VBUF_STRE_W<'a> {
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
#[doc = "Field `lo_osmx_fix_cap` reader - "]
pub struct LO_OSMX_FIX_CAP_R(crate::FieldReader<bool, bool>);
impl LO_OSMX_FIX_CAP_R {
    pub(crate) fn new(bits: bool) -> Self {
        LO_OSMX_FIX_CAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_OSMX_FIX_CAP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_osmx_fix_cap` writer - "]
pub struct LO_OSMX_FIX_CAP_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_OSMX_FIX_CAP_W<'a> {
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
#[doc = "Field `lo_osmx_en_xgm` reader - "]
pub struct LO_OSMX_EN_XGM_R(crate::FieldReader<bool, bool>);
impl LO_OSMX_EN_XGM_R {
    pub(crate) fn new(bits: bool) -> Self {
        LO_OSMX_EN_XGM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_OSMX_EN_XGM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_osmx_en_xgm` writer - "]
pub struct LO_OSMX_EN_XGM_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_OSMX_EN_XGM_W<'a> {
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
#[doc = "Field `lo_osmx_xgm_boost` reader - "]
pub struct LO_OSMX_XGM_BOOST_R(crate::FieldReader<bool, bool>);
impl LO_OSMX_XGM_BOOST_R {
    pub(crate) fn new(bits: bool) -> Self {
        LO_OSMX_XGM_BOOST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_OSMX_XGM_BOOST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_osmx_xgm_boost` writer - "]
pub struct LO_OSMX_XGM_BOOST_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_OSMX_XGM_BOOST_W<'a> {
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
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn lo_lodist_rxbuf_stre(&self) -> LO_LODIST_RXBUF_STRE_R {
        LO_LODIST_RXBUF_STRE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn lo_lodist_txbuf_stre(&self) -> LO_LODIST_TXBUF_STRE_R {
        LO_LODIST_TXBUF_STRE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn lo_osmx_cap(&self) -> LO_OSMX_CAP_R {
        LO_OSMX_CAP_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn lo_osmx_capbank_bias(&self) -> LO_OSMX_CAPBANK_BIAS_R {
        LO_OSMX_CAPBANK_BIAS_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn lo_osmx_vbuf_stre(&self) -> LO_OSMX_VBUF_STRE_R {
        LO_OSMX_VBUF_STRE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn lo_osmx_fix_cap(&self) -> LO_OSMX_FIX_CAP_R {
        LO_OSMX_FIX_CAP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn lo_osmx_en_xgm(&self) -> LO_OSMX_EN_XGM_R {
        LO_OSMX_EN_XGM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn lo_osmx_xgm_boost(&self) -> LO_OSMX_XGM_BOOST_R {
        LO_OSMX_XGM_BOOST_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn lo_lodist_rxbuf_stre(&mut self) -> LO_LODIST_RXBUF_STRE_W {
        LO_LODIST_RXBUF_STRE_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn lo_lodist_txbuf_stre(&mut self) -> LO_LODIST_TXBUF_STRE_W {
        LO_LODIST_TXBUF_STRE_W { w: self }
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn lo_osmx_cap(&mut self) -> LO_OSMX_CAP_W {
        LO_OSMX_CAP_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn lo_osmx_capbank_bias(&mut self) -> LO_OSMX_CAPBANK_BIAS_W {
        LO_OSMX_CAPBANK_BIAS_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn lo_osmx_vbuf_stre(&mut self) -> LO_OSMX_VBUF_STRE_W {
        LO_OSMX_VBUF_STRE_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn lo_osmx_fix_cap(&mut self) -> LO_OSMX_FIX_CAP_W {
        LO_OSMX_FIX_CAP_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn lo_osmx_en_xgm(&mut self) -> LO_OSMX_EN_XGM_W {
        LO_OSMX_EN_XGM_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn lo_osmx_xgm_boost(&mut self) -> LO_OSMX_XGM_BOOST_W {
        LO_OSMX_XGM_BOOST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "lodist.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lodist](index.html) module"]
pub struct LODIST_SPEC;
impl crate::RegisterSpec for LODIST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lodist::R](R) reader structure"]
impl crate::Readable for LODIST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lodist::W](W) writer structure"]
impl crate::Writable for LODIST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets lodist to value 0"]
impl crate::Resettable for LODIST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

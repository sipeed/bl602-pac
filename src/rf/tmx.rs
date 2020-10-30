#[doc = "Register `tmx` reader"]
pub struct R(crate::R<TMX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TMX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<TMX_SPEC>> for R {
    fn from(reader: crate::R<TMX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tmx` writer"]
pub struct W(crate::W<TMX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TMX_SPEC>;
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
impl core::convert::From<crate::W<TMX_SPEC>> for W {
    fn from(writer: crate::W<TMX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tx_tsense_en` reader - "]
pub struct TX_TSENSE_EN_R(crate::FieldReader<bool, bool>);
impl TX_TSENSE_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_TSENSE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_TSENSE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tx_tsense_en` writer - "]
pub struct TX_TSENSE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_TSENSE_EN_W<'a> {
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
#[doc = "Field `tmx_bm_cas_bulk` reader - "]
pub struct TMX_BM_CAS_BULK_R(crate::FieldReader<u8, u8>);
impl TMX_BM_CAS_BULK_R {
    pub(crate) fn new(bits: u8) -> Self {
        TMX_BM_CAS_BULK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TMX_BM_CAS_BULK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tmx_bm_cas_bulk` writer - "]
pub struct TMX_BM_CAS_BULK_W<'a> {
    w: &'a mut W,
}
impl<'a> TMX_BM_CAS_BULK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Field `tmx_bm_cas` reader - "]
pub struct TMX_BM_CAS_R(crate::FieldReader<u8, u8>);
impl TMX_BM_CAS_R {
    pub(crate) fn new(bits: u8) -> Self {
        TMX_BM_CAS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TMX_BM_CAS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tmx_bm_cas` writer - "]
pub struct TMX_BM_CAS_W<'a> {
    w: &'a mut W,
}
impl<'a> TMX_BM_CAS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `tmx_bm_sw` reader - "]
pub struct TMX_BM_SW_R(crate::FieldReader<u8, u8>);
impl TMX_BM_SW_R {
    pub(crate) fn new(bits: u8) -> Self {
        TMX_BM_SW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TMX_BM_SW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tmx_bm_sw` writer - "]
pub struct TMX_BM_SW_W<'a> {
    w: &'a mut W,
}
impl<'a> TMX_BM_SW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `tmx_cs` reader - "]
pub struct TMX_CS_R(crate::FieldReader<u8, u8>);
impl TMX_CS_R {
    pub(crate) fn new(bits: u8) -> Self {
        TMX_CS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TMX_CS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tmx_cs` writer - "]
pub struct TMX_CS_W<'a> {
    w: &'a mut W,
}
impl<'a> TMX_CS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tx_tsense_en(&self) -> TX_TSENSE_EN_R {
        TX_TSENSE_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn tmx_bm_cas_bulk(&self) -> TMX_BM_CAS_BULK_R {
        TMX_BM_CAS_BULK_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn tmx_bm_cas(&self) -> TMX_BM_CAS_R {
        TMX_BM_CAS_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn tmx_bm_sw(&self) -> TMX_BM_SW_R {
        TMX_BM_SW_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn tmx_cs(&self) -> TMX_CS_R {
        TMX_CS_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tx_tsense_en(&mut self) -> TX_TSENSE_EN_W {
        TX_TSENSE_EN_W { w: self }
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn tmx_bm_cas_bulk(&mut self) -> TMX_BM_CAS_BULK_W {
        TMX_BM_CAS_BULK_W { w: self }
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn tmx_bm_cas(&mut self) -> TMX_BM_CAS_W {
        TMX_BM_CAS_W { w: self }
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn tmx_bm_sw(&mut self) -> TMX_BM_SW_W {
        TMX_BM_SW_W { w: self }
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn tmx_cs(&mut self) -> TMX_CS_W {
        TMX_CS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "tmx.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmx](index.html) module"]
pub struct TMX_SPEC;
impl crate::RegisterSpec for TMX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tmx::R](R) reader structure"]
impl crate::Readable for TMX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tmx::W](W) writer structure"]
impl crate::Writable for TMX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets tmx to value 0"]
impl crate::Resettable for TMX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

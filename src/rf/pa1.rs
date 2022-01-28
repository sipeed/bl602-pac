#[doc = "Register `pa1` reader"]
pub struct R(crate::R<PA1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PA1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PA1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PA1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pa1` writer"]
pub struct W(crate::W<PA1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PA1_SPEC>;
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
impl From<crate::W<PA1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PA1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pa_att_gc` reader - "]
pub struct PA_ATT_GC_R(crate::FieldReader<u8, u8>);
impl PA_ATT_GC_R {
    pub(crate) fn new(bits: u8) -> Self {
        PA_ATT_GC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA_ATT_GC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pa_att_gc` writer - "]
pub struct PA_ATT_GC_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_ATT_GC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
#[doc = "Field `pa_pwrmx_bm` reader - "]
pub struct PA_PWRMX_BM_R(crate::FieldReader<u8, u8>);
impl PA_PWRMX_BM_R {
    pub(crate) fn new(bits: u8) -> Self {
        PA_PWRMX_BM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA_PWRMX_BM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pa_pwrmx_bm` writer - "]
pub struct PA_PWRMX_BM_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_PWRMX_BM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | ((value as u32 & 0x07) << 24);
        self.w
    }
}
#[doc = "Field `pa_pwrmx_dac_pn_switch` reader - "]
pub struct PA_PWRMX_DAC_PN_SWITCH_R(crate::FieldReader<bool, bool>);
impl PA_PWRMX_DAC_PN_SWITCH_R {
    pub(crate) fn new(bits: bool) -> Self {
        PA_PWRMX_DAC_PN_SWITCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA_PWRMX_DAC_PN_SWITCH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pa_pwrmx_dac_pn_switch` writer - "]
pub struct PA_PWRMX_DAC_PN_SWITCH_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_PWRMX_DAC_PN_SWITCH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `pa_pwrmx_osdac` reader - "]
pub struct PA_PWRMX_OSDAC_R(crate::FieldReader<u8, u8>);
impl PA_PWRMX_OSDAC_R {
    pub(crate) fn new(bits: u8) -> Self {
        PA_PWRMX_OSDAC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA_PWRMX_OSDAC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pa_pwrmx_osdac` writer - "]
pub struct PA_PWRMX_OSDAC_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_PWRMX_OSDAC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 18)) | ((value as u32 & 0x0f) << 18);
        self.w
    }
}
#[doc = "Field `pa_lz_bias_en` reader - "]
pub struct PA_LZ_BIAS_EN_R(crate::FieldReader<bool, bool>);
impl PA_LZ_BIAS_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PA_LZ_BIAS_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA_LZ_BIAS_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pa_lz_bias_en` writer - "]
pub struct PA_LZ_BIAS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_LZ_BIAS_EN_W<'a> {
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
#[doc = "Field `pa_ib_fix` reader - "]
pub struct PA_IB_FIX_R(crate::FieldReader<bool, bool>);
impl PA_IB_FIX_R {
    pub(crate) fn new(bits: bool) -> Self {
        PA_IB_FIX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA_IB_FIX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pa_ib_fix` writer - "]
pub struct PA_IB_FIX_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_IB_FIX_W<'a> {
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
#[doc = "Field `pa_half_on` reader - "]
pub struct PA_HALF_ON_R(crate::FieldReader<bool, bool>);
impl PA_HALF_ON_R {
    pub(crate) fn new(bits: bool) -> Self {
        PA_HALF_ON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA_HALF_ON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pa_half_on` writer - "]
pub struct PA_HALF_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_HALF_ON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `pa_vbcas` reader - "]
pub struct PA_VBCAS_R(crate::FieldReader<u8, u8>);
impl PA_VBCAS_R {
    pub(crate) fn new(bits: u8) -> Self {
        PA_VBCAS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA_VBCAS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pa_vbcas` writer - "]
pub struct PA_VBCAS_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_VBCAS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
#[doc = "Field `pa_vbcore` reader - "]
pub struct PA_VBCORE_R(crate::FieldReader<u8, u8>);
impl PA_VBCORE_R {
    pub(crate) fn new(bits: u8) -> Self {
        PA_VBCORE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA_VBCORE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pa_vbcore` writer - "]
pub struct PA_VBCORE_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_VBCORE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `pa_iet` reader - "]
pub struct PA_IET_R(crate::FieldReader<u8, u8>);
impl PA_IET_R {
    pub(crate) fn new(bits: u8) -> Self {
        PA_IET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA_IET_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pa_iet` writer - "]
pub struct PA_IET_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_IET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `pa_etb_en` reader - "]
pub struct PA_ETB_EN_R(crate::FieldReader<bool, bool>);
impl PA_ETB_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PA_ETB_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA_ETB_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pa_etb_en` writer - "]
pub struct PA_ETB_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_ETB_EN_W<'a> {
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
#[doc = "Field `pa_iaq` reader - "]
pub struct PA_IAQ_R(crate::FieldReader<u8, u8>);
impl PA_IAQ_R {
    pub(crate) fn new(bits: u8) -> Self {
        PA_IAQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA_IAQ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pa_iaq` writer - "]
pub struct PA_IAQ_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_IAQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn pa_att_gc(&self) -> PA_ATT_GC_R {
        PA_ATT_GC_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn pa_pwrmx_bm(&self) -> PA_PWRMX_BM_R {
        PA_PWRMX_BM_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn pa_pwrmx_dac_pn_switch(&self) -> PA_PWRMX_DAC_PN_SWITCH_R {
        PA_PWRMX_DAC_PN_SWITCH_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 18:21"]
    #[inline(always)]
    pub fn pa_pwrmx_osdac(&self) -> PA_PWRMX_OSDAC_R {
        PA_PWRMX_OSDAC_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn pa_lz_bias_en(&self) -> PA_LZ_BIAS_EN_R {
        PA_LZ_BIAS_EN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pa_ib_fix(&self) -> PA_IB_FIX_R {
        PA_IB_FIX_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn pa_half_on(&self) -> PA_HALF_ON_R {
        PA_HALF_ON_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn pa_vbcas(&self) -> PA_VBCAS_R {
        PA_VBCAS_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn pa_vbcore(&self) -> PA_VBCORE_R {
        PA_VBCORE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn pa_iet(&self) -> PA_IET_R {
        PA_IET_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pa_etb_en(&self) -> PA_ETB_EN_R {
        PA_ETB_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn pa_iaq(&self) -> PA_IAQ_R {
        PA_IAQ_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn pa_att_gc(&mut self) -> PA_ATT_GC_W {
        PA_ATT_GC_W { w: self }
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn pa_pwrmx_bm(&mut self) -> PA_PWRMX_BM_W {
        PA_PWRMX_BM_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn pa_pwrmx_dac_pn_switch(&mut self) -> PA_PWRMX_DAC_PN_SWITCH_W {
        PA_PWRMX_DAC_PN_SWITCH_W { w: self }
    }
    #[doc = "Bits 18:21"]
    #[inline(always)]
    pub fn pa_pwrmx_osdac(&mut self) -> PA_PWRMX_OSDAC_W {
        PA_PWRMX_OSDAC_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn pa_lz_bias_en(&mut self) -> PA_LZ_BIAS_EN_W {
        PA_LZ_BIAS_EN_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pa_ib_fix(&mut self) -> PA_IB_FIX_W {
        PA_IB_FIX_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn pa_half_on(&mut self) -> PA_HALF_ON_W {
        PA_HALF_ON_W { w: self }
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn pa_vbcas(&mut self) -> PA_VBCAS_W {
        PA_VBCAS_W { w: self }
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn pa_vbcore(&mut self) -> PA_VBCORE_W {
        PA_VBCORE_W { w: self }
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn pa_iet(&mut self) -> PA_IET_W {
        PA_IET_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pa_etb_en(&mut self) -> PA_ETB_EN_W {
        PA_ETB_EN_W { w: self }
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn pa_iaq(&mut self) -> PA_IAQ_W {
        PA_IAQ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "pa1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pa1](index.html) module"]
pub struct PA1_SPEC;
impl crate::RegisterSpec for PA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pa1::R](R) reader structure"]
impl crate::Readable for PA1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pa1::W](W) writer structure"]
impl crate::Writable for PA1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets pa1 to value 0"]
impl crate::Resettable for PA1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

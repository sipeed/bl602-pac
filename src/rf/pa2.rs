#[doc = "Register `pa2` reader"]
pub struct R(crate::R<PA2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PA2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PA2_SPEC>> for R {
    fn from(reader: crate::R<PA2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pa2` writer"]
pub struct W(crate::W<PA2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PA2_SPEC>;
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
impl core::convert::From<crate::W<PA2_SPEC>> for W {
    fn from(writer: crate::W<PA2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pa_ib_fix_hw` reader - "]
pub struct PA_IB_FIX_HW_R(crate::FieldReader<bool, bool>);
impl PA_IB_FIX_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        PA_IB_FIX_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA_IB_FIX_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pa_ib_fix_hw` writer - "]
pub struct PA_IB_FIX_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_IB_FIX_HW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `pa_half_on_hw` reader - "]
pub struct PA_HALF_ON_HW_R(crate::FieldReader<bool, bool>);
impl PA_HALF_ON_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        PA_HALF_ON_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA_HALF_ON_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pa_half_on_hw` writer - "]
pub struct PA_HALF_ON_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_HALF_ON_HW_W<'a> {
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
#[doc = "Field `pa_vbcas_hw` reader - "]
pub struct PA_VBCAS_HW_R(crate::FieldReader<u8, u8>);
impl PA_VBCAS_HW_R {
    pub(crate) fn new(bits: u8) -> Self {
        PA_VBCAS_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA_VBCAS_HW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pa_vbcas_hw` writer - "]
pub struct PA_VBCAS_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_VBCAS_HW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Field `pa_vbcore_hw` reader - "]
pub struct PA_VBCORE_HW_R(crate::FieldReader<u8, u8>);
impl PA_VBCORE_HW_R {
    pub(crate) fn new(bits: u8) -> Self {
        PA_VBCORE_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA_VBCORE_HW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pa_vbcore_hw` writer - "]
pub struct PA_VBCORE_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_VBCORE_HW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `pa_iet_hw` reader - "]
pub struct PA_IET_HW_R(crate::FieldReader<u8, u8>);
impl PA_IET_HW_R {
    pub(crate) fn new(bits: u8) -> Self {
        PA_IET_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA_IET_HW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pa_iet_hw` writer - "]
pub struct PA_IET_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_IET_HW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `pa_etb_en_hw` reader - "]
pub struct PA_ETB_EN_HW_R(crate::FieldReader<bool, bool>);
impl PA_ETB_EN_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        PA_ETB_EN_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA_ETB_EN_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pa_etb_en_hw` writer - "]
pub struct PA_ETB_EN_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_ETB_EN_HW_W<'a> {
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
impl R {
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn pa_ib_fix_hw(&self) -> PA_IB_FIX_HW_R {
        PA_IB_FIX_HW_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pa_half_on_hw(&self) -> PA_HALF_ON_HW_R {
        PA_HALF_ON_HW_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn pa_vbcas_hw(&self) -> PA_VBCAS_HW_R {
        PA_VBCAS_HW_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn pa_vbcore_hw(&self) -> PA_VBCORE_HW_R {
        PA_VBCORE_HW_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn pa_iet_hw(&self) -> PA_IET_HW_R {
        PA_IET_HW_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pa_etb_en_hw(&self) -> PA_ETB_EN_HW_R {
        PA_ETB_EN_HW_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn pa_ib_fix_hw(&mut self) -> PA_IB_FIX_HW_W {
        PA_IB_FIX_HW_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pa_half_on_hw(&mut self) -> PA_HALF_ON_HW_W {
        PA_HALF_ON_HW_W { w: self }
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn pa_vbcas_hw(&mut self) -> PA_VBCAS_HW_W {
        PA_VBCAS_HW_W { w: self }
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn pa_vbcore_hw(&mut self) -> PA_VBCORE_HW_W {
        PA_VBCORE_HW_W { w: self }
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn pa_iet_hw(&mut self) -> PA_IET_HW_W {
        PA_IET_HW_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pa_etb_en_hw(&mut self) -> PA_ETB_EN_HW_W {
        PA_ETB_EN_HW_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RX normal bias mode registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pa2](index.html) module"]
pub struct PA2_SPEC;
impl crate::RegisterSpec for PA2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pa2::R](R) reader structure"]
impl crate::Readable for PA2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pa2::W](W) writer structure"]
impl crate::Writable for PA2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets pa2 to value 0"]
impl crate::Resettable for PA2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

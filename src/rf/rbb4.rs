#[doc = "Register `rbb4` reader"]
pub struct R(crate::R<RBB4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RBB4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RBB4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RBB4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rbb4` writer"]
pub struct W(crate::W<RBB4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RBB4_SPEC>;
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
impl From<crate::W<RBB4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RBB4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pkdet_out_latch` reader - "]
pub struct PKDET_OUT_LATCH_R(crate::FieldReader<bool, bool>);
impl PKDET_OUT_LATCH_R {
    pub(crate) fn new(bits: bool) -> Self {
        PKDET_OUT_LATCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PKDET_OUT_LATCH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pkdet_out_latch` writer - "]
pub struct PKDET_OUT_LATCH_W<'a> {
    w: &'a mut W,
}
impl<'a> PKDET_OUT_LATCH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `pkdet_out_raw` reader - "]
pub struct PKDET_OUT_RAW_R(crate::FieldReader<bool, bool>);
impl PKDET_OUT_RAW_R {
    pub(crate) fn new(bits: bool) -> Self {
        PKDET_OUT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PKDET_OUT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pkdet_out_raw` writer - "]
pub struct PKDET_OUT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> PKDET_OUT_RAW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `rbb_pkdet_en_hw` reader - "]
pub struct RBB_PKDET_EN_HW_R(crate::FieldReader<bool, bool>);
impl RBB_PKDET_EN_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        RBB_PKDET_EN_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBB_PKDET_EN_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rbb_pkdet_en_hw` writer - "]
pub struct RBB_PKDET_EN_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> RBB_PKDET_EN_HW_W<'a> {
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
#[doc = "Field `rbb_pkdet_out_rstn_hw` reader - "]
pub struct RBB_PKDET_OUT_RSTN_HW_R(crate::FieldReader<bool, bool>);
impl RBB_PKDET_OUT_RSTN_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        RBB_PKDET_OUT_RSTN_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBB_PKDET_OUT_RSTN_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rbb_pkdet_out_rstn_hw` writer - "]
pub struct RBB_PKDET_OUT_RSTN_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> RBB_PKDET_OUT_RSTN_HW_W<'a> {
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
#[doc = "Field `rbb_pkdet_en` reader - "]
pub struct RBB_PKDET_EN_R(crate::FieldReader<bool, bool>);
impl RBB_PKDET_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RBB_PKDET_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBB_PKDET_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rbb_pkdet_en` writer - "]
pub struct RBB_PKDET_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RBB_PKDET_EN_W<'a> {
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
#[doc = "Field `rbb_pkdet_out_rstn` reader - "]
pub struct RBB_PKDET_OUT_RSTN_R(crate::FieldReader<bool, bool>);
impl RBB_PKDET_OUT_RSTN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RBB_PKDET_OUT_RSTN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBB_PKDET_OUT_RSTN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rbb_pkdet_out_rstn` writer - "]
pub struct RBB_PKDET_OUT_RSTN_W<'a> {
    w: &'a mut W,
}
impl<'a> RBB_PKDET_OUT_RSTN_W<'a> {
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
#[doc = "Field `rbb_pkdet_vth` reader - "]
pub struct RBB_PKDET_VTH_R(crate::FieldReader<u8, u8>);
impl RBB_PKDET_VTH_R {
    pub(crate) fn new(bits: u8) -> Self {
        RBB_PKDET_VTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBB_PKDET_VTH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rbb_pkdet_vth` writer - "]
pub struct RBB_PKDET_VTH_W<'a> {
    w: &'a mut W,
}
impl<'a> RBB_PKDET_VTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn pkdet_out_latch(&self) -> PKDET_OUT_LATCH_R {
        PKDET_OUT_LATCH_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn pkdet_out_raw(&self) -> PKDET_OUT_RAW_R {
        PKDET_OUT_RAW_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn rbb_pkdet_en_hw(&self) -> RBB_PKDET_EN_HW_R {
        RBB_PKDET_EN_HW_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rbb_pkdet_out_rstn_hw(&self) -> RBB_PKDET_OUT_RSTN_HW_R {
        RBB_PKDET_OUT_RSTN_HW_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rbb_pkdet_en(&self) -> RBB_PKDET_EN_R {
        RBB_PKDET_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rbb_pkdet_out_rstn(&self) -> RBB_PKDET_OUT_RSTN_R {
        RBB_PKDET_OUT_RSTN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn rbb_pkdet_vth(&self) -> RBB_PKDET_VTH_R {
        RBB_PKDET_VTH_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn pkdet_out_latch(&mut self) -> PKDET_OUT_LATCH_W {
        PKDET_OUT_LATCH_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn pkdet_out_raw(&mut self) -> PKDET_OUT_RAW_W {
        PKDET_OUT_RAW_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn rbb_pkdet_en_hw(&mut self) -> RBB_PKDET_EN_HW_W {
        RBB_PKDET_EN_HW_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rbb_pkdet_out_rstn_hw(&mut self) -> RBB_PKDET_OUT_RSTN_HW_W {
        RBB_PKDET_OUT_RSTN_HW_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rbb_pkdet_en(&mut self) -> RBB_PKDET_EN_W {
        RBB_PKDET_EN_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rbb_pkdet_out_rstn(&mut self) -> RBB_PKDET_OUT_RSTN_W {
        RBB_PKDET_OUT_RSTN_W { w: self }
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn rbb_pkdet_vth(&mut self) -> RBB_PKDET_VTH_W {
        RBB_PKDET_VTH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rbb4.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rbb4](index.html) module"]
pub struct RBB4_SPEC;
impl crate::RegisterSpec for RBB4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rbb4::R](R) reader structure"]
impl crate::Readable for RBB4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rbb4::W](W) writer structure"]
impl crate::Writable for RBB4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets rbb4 to value 0"]
impl crate::Resettable for RBB4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

#[doc = "Register `vco4` reader"]
pub struct R(crate::R<VCO4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VCO4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VCO4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VCO4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `vco4` writer"]
pub struct W(crate::W<VCO4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VCO4_SPEC>;
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
impl From<crate::W<VCO4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VCO4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `fcal_inc_vctrl_ud` reader - "]
pub struct FCAL_INC_VCTRL_UD_R(crate::FieldReader<u8, u8>);
impl FCAL_INC_VCTRL_UD_R {
    pub(crate) fn new(bits: u8) -> Self {
        FCAL_INC_VCTRL_UD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FCAL_INC_VCTRL_UD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `fcal_inc_vctrl_ud` writer - "]
pub struct FCAL_INC_VCTRL_UD_W<'a> {
    w: &'a mut W,
}
impl<'a> FCAL_INC_VCTRL_UD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "Field `fcal_cnt_rdy` reader - "]
pub struct FCAL_CNT_RDY_R(crate::FieldReader<bool, bool>);
impl FCAL_CNT_RDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        FCAL_CNT_RDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FCAL_CNT_RDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `fcal_cnt_rdy` writer - "]
pub struct FCAL_CNT_RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> FCAL_CNT_RDY_W<'a> {
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
#[doc = "Field `fcal_inc_large_range` reader - "]
pub struct FCAL_INC_LARGE_RANGE_R(crate::FieldReader<bool, bool>);
impl FCAL_INC_LARGE_RANGE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FCAL_INC_LARGE_RANGE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FCAL_INC_LARGE_RANGE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `fcal_inc_large_range` writer - "]
pub struct FCAL_INC_LARGE_RANGE_W<'a> {
    w: &'a mut W,
}
impl<'a> FCAL_INC_LARGE_RANGE_W<'a> {
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
#[doc = "Field `fcal_inc_en_hw` reader - "]
pub struct FCAL_INC_EN_HW_R(crate::FieldReader<bool, bool>);
impl FCAL_INC_EN_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        FCAL_INC_EN_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FCAL_INC_EN_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `fcal_inc_en_hw` writer - "]
pub struct FCAL_INC_EN_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> FCAL_INC_EN_HW_W<'a> {
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
#[doc = "Field `fcal_cnt_start` reader - "]
pub struct FCAL_CNT_START_R(crate::FieldReader<bool, bool>);
impl FCAL_CNT_START_R {
    pub(crate) fn new(bits: bool) -> Self {
        FCAL_CNT_START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FCAL_CNT_START_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `fcal_cnt_start` writer - "]
pub struct FCAL_CNT_START_W<'a> {
    w: &'a mut W,
}
impl<'a> FCAL_CNT_START_W<'a> {
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
impl R {
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn fcal_inc_vctrl_ud(&self) -> FCAL_INC_VCTRL_UD_R {
        FCAL_INC_VCTRL_UD_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn fcal_cnt_rdy(&self) -> FCAL_CNT_RDY_R {
        FCAL_CNT_RDY_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn fcal_inc_large_range(&self) -> FCAL_INC_LARGE_RANGE_R {
        FCAL_INC_LARGE_RANGE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn fcal_inc_en_hw(&self) -> FCAL_INC_EN_HW_R {
        FCAL_INC_EN_HW_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn fcal_cnt_start(&self) -> FCAL_CNT_START_R {
        FCAL_CNT_START_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn fcal_inc_vctrl_ud(&mut self) -> FCAL_INC_VCTRL_UD_W {
        FCAL_INC_VCTRL_UD_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn fcal_cnt_rdy(&mut self) -> FCAL_CNT_RDY_W {
        FCAL_CNT_RDY_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn fcal_inc_large_range(&mut self) -> FCAL_INC_LARGE_RANGE_W {
        FCAL_INC_LARGE_RANGE_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn fcal_inc_en_hw(&mut self) -> FCAL_INC_EN_HW_W {
        FCAL_INC_EN_HW_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn fcal_cnt_start(&mut self) -> FCAL_CNT_START_W {
        FCAL_CNT_START_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "vco4.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vco4](index.html) module"]
pub struct VCO4_SPEC;
impl crate::RegisterSpec for VCO4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vco4::R](R) reader structure"]
impl crate::Readable for VCO4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vco4::W](W) writer structure"]
impl crate::Writable for VCO4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets vco4 to value 0"]
impl crate::Resettable for VCO4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

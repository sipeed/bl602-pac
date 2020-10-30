#[doc = "Register `rf_ical_ctrl0` reader"]
pub struct R(crate::R<RF_ICAL_CTRL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_ICAL_CTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RF_ICAL_CTRL0_SPEC>> for R {
    fn from(reader: crate::R<RF_ICAL_CTRL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rf_ical_ctrl0` writer"]
pub struct W(crate::W<RF_ICAL_CTRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF_ICAL_CTRL0_SPEC>;
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
impl core::convert::From<crate::W<RF_ICAL_CTRL0_SPEC>> for W {
    fn from(writer: crate::W<RF_ICAL_CTRL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rf_ical_f_ud_inv_en` reader - "]
pub struct RF_ICAL_F_UD_INV_EN_R(crate::FieldReader<bool, bool>);
impl RF_ICAL_F_UD_INV_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RF_ICAL_F_UD_INV_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_ICAL_F_UD_INV_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_ical_f_ud_inv_en` writer - "]
pub struct RF_ICAL_F_UD_INV_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_ICAL_F_UD_INV_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Field `rf_ical_a_ud_inv_en` reader - "]
pub struct RF_ICAL_A_UD_INV_EN_R(crate::FieldReader<bool, bool>);
impl RF_ICAL_A_UD_INV_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RF_ICAL_A_UD_INV_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_ICAL_A_UD_INV_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_ical_a_ud_inv_en` writer - "]
pub struct RF_ICAL_A_UD_INV_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_ICAL_A_UD_INV_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `rf_ical_f_cnt_n` reader - "]
pub struct RF_ICAL_F_CNT_N_R(crate::FieldReader<u16, u16>);
impl RF_ICAL_F_CNT_N_R {
    pub(crate) fn new(bits: u16) -> Self {
        RF_ICAL_F_CNT_N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_ICAL_F_CNT_N_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_ical_f_cnt_n` writer - "]
pub struct RF_ICAL_F_CNT_N_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_ICAL_F_CNT_N_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 20)) | (((value as u32) & 0x03ff) << 20);
        self.w
    }
}
#[doc = "Field `rf_ical_a_cnt_n` reader - "]
pub struct RF_ICAL_A_CNT_N_R(crate::FieldReader<u16, u16>);
impl RF_ICAL_A_CNT_N_R {
    pub(crate) fn new(bits: u16) -> Self {
        RF_ICAL_A_CNT_N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_ICAL_A_CNT_N_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_ical_a_cnt_n` writer - "]
pub struct RF_ICAL_A_CNT_N_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_ICAL_A_CNT_N_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 10)) | (((value as u32) & 0x03ff) << 10);
        self.w
    }
}
#[doc = "Field `rf_ical_r_cnt_n` reader - "]
pub struct RF_ICAL_R_CNT_N_R(crate::FieldReader<u16, u16>);
impl RF_ICAL_R_CNT_N_R {
    pub(crate) fn new(bits: u16) -> Self {
        RF_ICAL_R_CNT_N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_ICAL_R_CNT_N_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_ical_r_cnt_n` writer - "]
pub struct RF_ICAL_R_CNT_N_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_ICAL_R_CNT_N_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rf_ical_f_ud_inv_en(&self) -> RF_ICAL_F_UD_INV_EN_R {
        RF_ICAL_F_UD_INV_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rf_ical_a_ud_inv_en(&self) -> RF_ICAL_A_UD_INV_EN_R {
        RF_ICAL_A_UD_INV_EN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bits 20:29"]
    #[inline(always)]
    pub fn rf_ical_f_cnt_n(&self) -> RF_ICAL_F_CNT_N_R {
        RF_ICAL_F_CNT_N_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19"]
    #[inline(always)]
    pub fn rf_ical_a_cnt_n(&self) -> RF_ICAL_A_CNT_N_R {
        RF_ICAL_A_CNT_N_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn rf_ical_r_cnt_n(&self) -> RF_ICAL_R_CNT_N_R {
        RF_ICAL_R_CNT_N_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rf_ical_f_ud_inv_en(&mut self) -> RF_ICAL_F_UD_INV_EN_W {
        RF_ICAL_F_UD_INV_EN_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rf_ical_a_ud_inv_en(&mut self) -> RF_ICAL_A_UD_INV_EN_W {
        RF_ICAL_A_UD_INV_EN_W { w: self }
    }
    #[doc = "Bits 20:29"]
    #[inline(always)]
    pub fn rf_ical_f_cnt_n(&mut self) -> RF_ICAL_F_CNT_N_W {
        RF_ICAL_F_CNT_N_W { w: self }
    }
    #[doc = "Bits 10:19"]
    #[inline(always)]
    pub fn rf_ical_a_cnt_n(&mut self) -> RF_ICAL_A_CNT_N_W {
        RF_ICAL_A_CNT_N_W { w: self }
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn rf_ical_r_cnt_n(&mut self) -> RF_ICAL_R_CNT_N_W {
        RF_ICAL_R_CNT_N_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rf_ical_ctrl0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_ical_ctrl0](index.html) module"]
pub struct RF_ICAL_CTRL0_SPEC;
impl crate::RegisterSpec for RF_ICAL_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_ical_ctrl0::R](R) reader structure"]
impl crate::Readable for RF_ICAL_CTRL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf_ical_ctrl0::W](W) writer structure"]
impl crate::Writable for RF_ICAL_CTRL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets rf_ical_ctrl0 to value 0"]
impl crate::Resettable for RF_ICAL_CTRL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

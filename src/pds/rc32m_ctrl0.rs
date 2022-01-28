#[doc = "Register `rc32m_ctrl0` reader"]
pub struct R(crate::R<RC32M_CTRL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RC32M_CTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RC32M_CTRL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RC32M_CTRL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rc32m_ctrl0` writer"]
pub struct W(crate::W<RC32M_CTRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RC32M_CTRL0_SPEC>;
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
impl From<crate::W<RC32M_CTRL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RC32M_CTRL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rc32m_code_fr_ext` reader - "]
pub struct RC32M_CODE_FR_EXT_R(crate::FieldReader<u8, u8>);
impl RC32M_CODE_FR_EXT_R {
    pub(crate) fn new(bits: u8) -> Self {
        RC32M_CODE_FR_EXT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RC32M_CODE_FR_EXT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rc32m_code_fr_ext` writer - "]
pub struct RC32M_CODE_FR_EXT_W<'a> {
    w: &'a mut W,
}
impl<'a> RC32M_CODE_FR_EXT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 22)) | ((value as u32 & 0xff) << 22);
        self.w
    }
}
#[doc = "Field `rc32m_pd` reader - "]
pub struct RC32M_PD_R(crate::FieldReader<bool, bool>);
impl RC32M_PD_R {
    pub(crate) fn new(bits: bool) -> Self {
        RC32M_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RC32M_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rc32m_pd` writer - "]
pub struct RC32M_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> RC32M_PD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `rc32m_cal_en` reader - "]
pub struct RC32M_CAL_EN_R(crate::FieldReader<bool, bool>);
impl RC32M_CAL_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RC32M_CAL_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RC32M_CAL_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rc32m_cal_en` writer - "]
pub struct RC32M_CAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RC32M_CAL_EN_W<'a> {
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
#[doc = "Field `rc32m_ext_code_en` reader - "]
pub struct RC32M_EXT_CODE_EN_R(crate::FieldReader<bool, bool>);
impl RC32M_EXT_CODE_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RC32M_EXT_CODE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RC32M_EXT_CODE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rc32m_ext_code_en` writer - "]
pub struct RC32M_EXT_CODE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RC32M_EXT_CODE_EN_W<'a> {
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
#[doc = "Field `rc32m_refclk_half` reader - "]
pub struct RC32M_REFCLK_HALF_R(crate::FieldReader<bool, bool>);
impl RC32M_REFCLK_HALF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RC32M_REFCLK_HALF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RC32M_REFCLK_HALF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rc32m_refclk_half` writer - "]
pub struct RC32M_REFCLK_HALF_W<'a> {
    w: &'a mut W,
}
impl<'a> RC32M_REFCLK_HALF_W<'a> {
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
#[doc = "Field `rc32m_allow_cal` reader - "]
pub struct RC32M_ALLOW_CAL_R(crate::FieldReader<bool, bool>);
impl RC32M_ALLOW_CAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        RC32M_ALLOW_CAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RC32M_ALLOW_CAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rc32m_allow_cal` writer - "]
pub struct RC32M_ALLOW_CAL_W<'a> {
    w: &'a mut W,
}
impl<'a> RC32M_ALLOW_CAL_W<'a> {
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
#[doc = "Field `rc32m_dig_code_fr_cal` reader - "]
pub struct RC32M_DIG_CODE_FR_CAL_R(crate::FieldReader<u8, u8>);
impl RC32M_DIG_CODE_FR_CAL_R {
    pub(crate) fn new(bits: u8) -> Self {
        RC32M_DIG_CODE_FR_CAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RC32M_DIG_CODE_FR_CAL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rc32m_cal_precharge` reader - "]
pub struct RC32M_CAL_PRECHARGE_R(crate::FieldReader<bool, bool>);
impl RC32M_CAL_PRECHARGE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RC32M_CAL_PRECHARGE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RC32M_CAL_PRECHARGE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rc32m_cal_div` reader - "]
pub struct RC32M_CAL_DIV_R(crate::FieldReader<u8, u8>);
impl RC32M_CAL_DIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        RC32M_CAL_DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RC32M_CAL_DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rc32m_cal_div` writer - "]
pub struct RC32M_CAL_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> RC32M_CAL_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | ((value as u32 & 0x03) << 3);
        self.w
    }
}
#[doc = "Field `rc32m_cal_inprogress` reader - "]
pub struct RC32M_CAL_INPROGRESS_R(crate::FieldReader<bool, bool>);
impl RC32M_CAL_INPROGRESS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RC32M_CAL_INPROGRESS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RC32M_CAL_INPROGRESS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rc32m_rdy` reader - "]
pub struct RC32M_RDY_R(crate::FieldReader<bool, bool>);
impl RC32M_RDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        RC32M_RDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RC32M_RDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rc32m_cal_done` reader - "]
pub struct RC32M_CAL_DONE_R(crate::FieldReader<bool, bool>);
impl RC32M_CAL_DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RC32M_CAL_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RC32M_CAL_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 22:29"]
    #[inline(always)]
    pub fn rc32m_code_fr_ext(&self) -> RC32M_CODE_FR_EXT_R {
        RC32M_CODE_FR_EXT_R::new(((self.bits >> 22) & 0xff) as u8)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn rc32m_pd(&self) -> RC32M_PD_R {
        RC32M_PD_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn rc32m_cal_en(&self) -> RC32M_CAL_EN_R {
        RC32M_CAL_EN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn rc32m_ext_code_en(&self) -> RC32M_EXT_CODE_EN_R {
        RC32M_EXT_CODE_EN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn rc32m_refclk_half(&self) -> RC32M_REFCLK_HALF_R {
        RC32M_REFCLK_HALF_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn rc32m_allow_cal(&self) -> RC32M_ALLOW_CAL_R {
        RC32M_ALLOW_CAL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 6:13"]
    #[inline(always)]
    pub fn rc32m_dig_code_fr_cal(&self) -> RC32M_DIG_CODE_FR_CAL_R {
        RC32M_DIG_CODE_FR_CAL_R::new(((self.bits >> 6) & 0xff) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rc32m_cal_precharge(&self) -> RC32M_CAL_PRECHARGE_R {
        RC32M_CAL_PRECHARGE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn rc32m_cal_div(&self) -> RC32M_CAL_DIV_R {
        RC32M_CAL_DIV_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rc32m_cal_inprogress(&self) -> RC32M_CAL_INPROGRESS_R {
        RC32M_CAL_INPROGRESS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rc32m_rdy(&self) -> RC32M_RDY_R {
        RC32M_RDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rc32m_cal_done(&self) -> RC32M_CAL_DONE_R {
        RC32M_CAL_DONE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 22:29"]
    #[inline(always)]
    pub fn rc32m_code_fr_ext(&mut self) -> RC32M_CODE_FR_EXT_W {
        RC32M_CODE_FR_EXT_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn rc32m_pd(&mut self) -> RC32M_PD_W {
        RC32M_PD_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn rc32m_cal_en(&mut self) -> RC32M_CAL_EN_W {
        RC32M_CAL_EN_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn rc32m_ext_code_en(&mut self) -> RC32M_EXT_CODE_EN_W {
        RC32M_EXT_CODE_EN_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn rc32m_refclk_half(&mut self) -> RC32M_REFCLK_HALF_W {
        RC32M_REFCLK_HALF_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn rc32m_allow_cal(&mut self) -> RC32M_ALLOW_CAL_W {
        RC32M_ALLOW_CAL_W { w: self }
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn rc32m_cal_div(&mut self) -> RC32M_CAL_DIV_W {
        RC32M_CAL_DIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rc32m_ctrl0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rc32m_ctrl0](index.html) module"]
pub struct RC32M_CTRL0_SPEC;
impl crate::RegisterSpec for RC32M_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rc32m_ctrl0::R](R) reader structure"]
impl crate::Readable for RC32M_CTRL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rc32m_ctrl0::W](W) writer structure"]
impl crate::Writable for RC32M_CTRL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets rc32m_ctrl0 to value 0x1808_0018"]
impl crate::Resettable for RC32M_CTRL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1808_0018
    }
}

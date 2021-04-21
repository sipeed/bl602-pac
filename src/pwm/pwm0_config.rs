#[doc = "Register `pwm0_config` reader"]
pub struct R(crate::R<PWM0_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM0_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PWM0_CONFIG_SPEC>> for R {
    fn from(reader: crate::R<PWM0_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pwm0_config` writer"]
pub struct W(crate::W<PWM0_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM0_CONFIG_SPEC>;
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
impl core::convert::From<crate::W<PWM0_CONFIG_SPEC>> for W {
    fn from(writer: crate::W<PWM0_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pwm_sts_top` reader - "]
pub struct PWM_STS_TOP_R(crate::FieldReader<bool, bool>);
impl PWM_STS_TOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWM_STS_TOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWM_STS_TOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwm_stop_en` reader - "]
pub struct PWM_STOP_EN_R(crate::FieldReader<bool, bool>);
impl PWM_STOP_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWM_STOP_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWM_STOP_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwm_stop_en` writer - "]
pub struct PWM_STOP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_STOP_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `pwm_sw_mode` reader - "]
pub struct PWM_SW_MODE_R(crate::FieldReader<bool, bool>);
impl PWM_SW_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWM_SW_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWM_SW_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwm_sw_mode` writer - "]
pub struct PWM_SW_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_SW_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `pwm_sw_force_val` reader - "]
pub struct PWM_SW_FORCE_VAL_R(crate::FieldReader<bool, bool>);
impl PWM_SW_FORCE_VAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWM_SW_FORCE_VAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWM_SW_FORCE_VAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwm_sw_force_val` writer - "]
pub struct PWM_SW_FORCE_VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_SW_FORCE_VAL_W<'a> {
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
#[doc = "Field `pwm_stop_mode` reader - "]
pub struct PWM_STOP_MODE_R(crate::FieldReader<bool, bool>);
impl PWM_STOP_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWM_STOP_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWM_STOP_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwm_stop_mode` writer - "]
pub struct PWM_STOP_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_STOP_MODE_W<'a> {
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
#[doc = "Field `pwm_out_inv` reader - "]
pub struct PWM_OUT_INV_R(crate::FieldReader<bool, bool>);
impl PWM_OUT_INV_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWM_OUT_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWM_OUT_INV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwm_out_inv` writer - "]
pub struct PWM_OUT_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_OUT_INV_W<'a> {
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
#[doc = "Field `reg_clk_sel` reader - "]
pub struct REG_CLK_SEL_R(crate::FieldReader<u8, u8>);
impl REG_CLK_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        REG_CLK_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_CLK_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_clk_sel` writer - "]
pub struct REG_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_CLK_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn pwm_sts_top(&self) -> PWM_STS_TOP_R {
        PWM_STS_TOP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn pwm_stop_en(&self) -> PWM_STOP_EN_R {
        PWM_STOP_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pwm_sw_mode(&self) -> PWM_SW_MODE_R {
        PWM_SW_MODE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pwm_sw_force_val(&self) -> PWM_SW_FORCE_VAL_R {
        PWM_SW_FORCE_VAL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pwm_stop_mode(&self) -> PWM_STOP_MODE_R {
        PWM_STOP_MODE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pwm_out_inv(&self) -> PWM_OUT_INV_R {
        PWM_OUT_INV_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn reg_clk_sel(&self) -> REG_CLK_SEL_R {
        REG_CLK_SEL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn pwm_stop_en(&mut self) -> PWM_STOP_EN_W {
        PWM_STOP_EN_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pwm_sw_mode(&mut self) -> PWM_SW_MODE_W {
        PWM_SW_MODE_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pwm_sw_force_val(&mut self) -> PWM_SW_FORCE_VAL_W {
        PWM_SW_FORCE_VAL_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pwm_stop_mode(&mut self) -> PWM_STOP_MODE_W {
        PWM_STOP_MODE_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pwm_out_inv(&mut self) -> PWM_OUT_INV_W {
        PWM_OUT_INV_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn reg_clk_sel(&mut self) -> REG_CLK_SEL_W {
        REG_CLK_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "pwm0_config.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm0_config](index.html) module"]
pub struct PWM0_CONFIG_SPEC;
impl crate::RegisterSpec for PWM0_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm0_config::R](R) reader structure"]
impl crate::Readable for PWM0_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm0_config::W](W) writer structure"]
impl crate::Writable for PWM0_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets pwm0_config to value 0x08"]
impl crate::Resettable for PWM0_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x08
    }
}

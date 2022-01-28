#[doc = "Register `swrst_cfg2` reader"]
pub struct R(crate::R<SWRST_CFG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWRST_CFG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWRST_CFG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWRST_CFG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `swrst_cfg2` writer"]
pub struct W(crate::W<SWRST_CFG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWRST_CFG2_SPEC>;
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
impl From<crate::W<SWRST_CFG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWRST_CFG2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pka_clk_sel` reader - "]
pub struct PKA_CLK_SEL_R(crate::FieldReader<bool, bool>);
impl PKA_CLK_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        PKA_CLK_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PKA_CLK_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pka_clk_sel` writer - "]
pub struct PKA_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PKA_CLK_SEL_W<'a> {
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
#[doc = "Field `reg_ctrl_reset_dummy` reader - "]
pub struct REG_CTRL_RESET_DUMMY_R(crate::FieldReader<u8, u8>);
impl REG_CTRL_RESET_DUMMY_R {
    pub(crate) fn new(bits: u8) -> Self {
        REG_CTRL_RESET_DUMMY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_CTRL_RESET_DUMMY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_ctrl_reset_dummy` writer - "]
pub struct REG_CTRL_RESET_DUMMY_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_CTRL_RESET_DUMMY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `reg_ctrl_sys_reset` reader - "]
pub struct REG_CTRL_SYS_RESET_R(crate::FieldReader<bool, bool>);
impl REG_CTRL_SYS_RESET_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_CTRL_SYS_RESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_CTRL_SYS_RESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_ctrl_sys_reset` writer - "]
pub struct REG_CTRL_SYS_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_CTRL_SYS_RESET_W<'a> {
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
#[doc = "Field `reg_ctrl_cpu_reset` reader - "]
pub struct REG_CTRL_CPU_RESET_R(crate::FieldReader<bool, bool>);
impl REG_CTRL_CPU_RESET_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_CTRL_CPU_RESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_CTRL_CPU_RESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_ctrl_cpu_reset` writer - "]
pub struct REG_CTRL_CPU_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_CTRL_CPU_RESET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `reg_ctrl_pwron_rst` reader - "]
pub struct REG_CTRL_PWRON_RST_R(crate::FieldReader<bool, bool>);
impl REG_CTRL_PWRON_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_CTRL_PWRON_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_CTRL_PWRON_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_ctrl_pwron_rst` writer - "]
pub struct REG_CTRL_PWRON_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_CTRL_PWRON_RST_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn pka_clk_sel(&self) -> PKA_CLK_SEL_R {
        PKA_CLK_SEL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn reg_ctrl_reset_dummy(&self) -> REG_CTRL_RESET_DUMMY_R {
        REG_CTRL_RESET_DUMMY_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn reg_ctrl_sys_reset(&self) -> REG_CTRL_SYS_RESET_R {
        REG_CTRL_SYS_RESET_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_ctrl_cpu_reset(&self) -> REG_CTRL_CPU_RESET_R {
        REG_CTRL_CPU_RESET_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_ctrl_pwron_rst(&self) -> REG_CTRL_PWRON_RST_R {
        REG_CTRL_PWRON_RST_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn pka_clk_sel(&mut self) -> PKA_CLK_SEL_W {
        PKA_CLK_SEL_W { w: self }
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn reg_ctrl_reset_dummy(&mut self) -> REG_CTRL_RESET_DUMMY_W {
        REG_CTRL_RESET_DUMMY_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn reg_ctrl_sys_reset(&mut self) -> REG_CTRL_SYS_RESET_W {
        REG_CTRL_SYS_RESET_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_ctrl_cpu_reset(&mut self) -> REG_CTRL_CPU_RESET_W {
        REG_CTRL_CPU_RESET_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_ctrl_pwron_rst(&mut self) -> REG_CTRL_PWRON_RST_W {
        REG_CTRL_PWRON_RST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "swrst_cfg2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swrst_cfg2](index.html) module"]
pub struct SWRST_CFG2_SPEC;
impl crate::RegisterSpec for SWRST_CFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [swrst_cfg2::R](R) reader structure"]
impl crate::Readable for SWRST_CFG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [swrst_cfg2::W](W) writer structure"]
impl crate::Writable for SWRST_CFG2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets swrst_cfg2 to value 0"]
impl crate::Resettable for SWRST_CFG2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

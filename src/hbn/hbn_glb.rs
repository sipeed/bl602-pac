#[doc = "Register `HBN_GLB` reader"]
pub struct R(crate::R<HBN_GLB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HBN_GLB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HBN_GLB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HBN_GLB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HBN_GLB` writer"]
pub struct W(crate::W<HBN_GLB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HBN_GLB_SPEC>;
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
impl From<crate::W<HBN_GLB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HBN_GLB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sw_ldo11_aon_vout_sel` reader - "]
pub struct SW_LDO11_AON_VOUT_SEL_R(crate::FieldReader<u8, u8>);
impl SW_LDO11_AON_VOUT_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        SW_LDO11_AON_VOUT_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_LDO11_AON_VOUT_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sw_ldo11_aon_vout_sel` writer - "]
pub struct SW_LDO11_AON_VOUT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_LDO11_AON_VOUT_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
#[doc = "Field `sw_ldo11_rt_vout_sel` reader - "]
pub struct SW_LDO11_RT_VOUT_SEL_R(crate::FieldReader<u8, u8>);
impl SW_LDO11_RT_VOUT_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        SW_LDO11_RT_VOUT_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_LDO11_RT_VOUT_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sw_ldo11_rt_vout_sel` writer - "]
pub struct SW_LDO11_RT_VOUT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_LDO11_RT_VOUT_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `sw_ldo11soc_vout_sel_aon` reader - "]
pub struct SW_LDO11SOC_VOUT_SEL_AON_R(crate::FieldReader<u8, u8>);
impl SW_LDO11SOC_VOUT_SEL_AON_R {
    pub(crate) fn new(bits: u8) -> Self {
        SW_LDO11SOC_VOUT_SEL_AON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_LDO11SOC_VOUT_SEL_AON_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sw_ldo11soc_vout_sel_aon` writer - "]
pub struct SW_LDO11SOC_VOUT_SEL_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_LDO11SOC_VOUT_SEL_AON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `hbn_pu_rc32k` reader - "]
pub struct HBN_PU_RC32K_R(crate::FieldReader<bool, bool>);
impl HBN_PU_RC32K_R {
    pub(crate) fn new(bits: bool) -> Self {
        HBN_PU_RC32K_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HBN_PU_RC32K_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `hbn_pu_rc32k` writer - "]
pub struct HBN_PU_RC32K_W<'a> {
    w: &'a mut W,
}
impl<'a> HBN_PU_RC32K_W<'a> {
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
#[doc = "Field `hbn_f32k_sel` reader - "]
pub struct HBN_F32K_SEL_R(crate::FieldReader<u8, u8>);
impl HBN_F32K_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        HBN_F32K_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HBN_F32K_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `hbn_f32k_sel` writer - "]
pub struct HBN_F32K_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> HBN_F32K_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | ((value as u32 & 0x03) << 3);
        self.w
    }
}
#[doc = "Field `hbn_uart_clk_sel` reader - "]
pub struct HBN_UART_CLK_SEL_R(crate::FieldReader<bool, bool>);
impl HBN_UART_CLK_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        HBN_UART_CLK_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HBN_UART_CLK_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `hbn_uart_clk_sel` writer - "]
pub struct HBN_UART_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> HBN_UART_CLK_SEL_W<'a> {
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
#[doc = "Field `hbn_root_clk_sel` reader - "]
pub struct HBN_ROOT_CLK_SEL_R(crate::FieldReader<u8, u8>);
impl HBN_ROOT_CLK_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        HBN_ROOT_CLK_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HBN_ROOT_CLK_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `hbn_root_clk_sel` writer - "]
pub struct HBN_ROOT_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> HBN_ROOT_CLK_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn sw_ldo11_aon_vout_sel(&self) -> SW_LDO11_AON_VOUT_SEL_R {
        SW_LDO11_AON_VOUT_SEL_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn sw_ldo11_rt_vout_sel(&self) -> SW_LDO11_RT_VOUT_SEL_R {
        SW_LDO11_RT_VOUT_SEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn sw_ldo11soc_vout_sel_aon(&self) -> SW_LDO11SOC_VOUT_SEL_AON_R {
        SW_LDO11SOC_VOUT_SEL_AON_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn hbn_pu_rc32k(&self) -> HBN_PU_RC32K_R {
        HBN_PU_RC32K_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn hbn_f32k_sel(&self) -> HBN_F32K_SEL_R {
        HBN_F32K_SEL_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn hbn_uart_clk_sel(&self) -> HBN_UART_CLK_SEL_R {
        HBN_UART_CLK_SEL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn hbn_root_clk_sel(&self) -> HBN_ROOT_CLK_SEL_R {
        HBN_ROOT_CLK_SEL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn sw_ldo11_aon_vout_sel(&mut self) -> SW_LDO11_AON_VOUT_SEL_W {
        SW_LDO11_AON_VOUT_SEL_W { w: self }
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn sw_ldo11_rt_vout_sel(&mut self) -> SW_LDO11_RT_VOUT_SEL_W {
        SW_LDO11_RT_VOUT_SEL_W { w: self }
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn sw_ldo11soc_vout_sel_aon(&mut self) -> SW_LDO11SOC_VOUT_SEL_AON_W {
        SW_LDO11SOC_VOUT_SEL_AON_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn hbn_pu_rc32k(&mut self) -> HBN_PU_RC32K_W {
        HBN_PU_RC32K_W { w: self }
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn hbn_f32k_sel(&mut self) -> HBN_F32K_SEL_W {
        HBN_F32K_SEL_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn hbn_uart_clk_sel(&mut self) -> HBN_UART_CLK_SEL_W {
        HBN_UART_CLK_SEL_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn hbn_root_clk_sel(&mut self) -> HBN_ROOT_CLK_SEL_W {
        HBN_ROOT_CLK_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HBN_GLB.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hbn_glb](index.html) module"]
pub struct HBN_GLB_SPEC;
impl crate::RegisterSpec for HBN_GLB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hbn_glb::R](R) reader structure"]
impl crate::Readable for HBN_GLB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hbn_glb::W](W) writer structure"]
impl crate::Writable for HBN_GLB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HBN_GLB to value 0xaa0a_0020"]
impl crate::Resettable for HBN_GLB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xaa0a_0020
    }
}

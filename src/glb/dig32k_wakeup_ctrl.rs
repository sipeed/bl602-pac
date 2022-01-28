#[doc = "Register `DIG32K_WAKEUP_CTRL` reader"]
pub struct R(crate::R<DIG32K_WAKEUP_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIG32K_WAKEUP_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIG32K_WAKEUP_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIG32K_WAKEUP_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIG32K_WAKEUP_CTRL` writer"]
pub struct W(crate::W<DIG32K_WAKEUP_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIG32K_WAKEUP_CTRL_SPEC>;
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
impl From<crate::W<DIG32K_WAKEUP_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIG32K_WAKEUP_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_en_platform_wakeup` reader - "]
pub struct REG_EN_PLATFORM_WAKEUP_R(crate::FieldReader<bool, bool>);
impl REG_EN_PLATFORM_WAKEUP_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_EN_PLATFORM_WAKEUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_EN_PLATFORM_WAKEUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_en_platform_wakeup` writer - "]
pub struct REG_EN_PLATFORM_WAKEUP_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_EN_PLATFORM_WAKEUP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
#[doc = "Field `dig_clk_src_sel` reader - "]
pub struct DIG_CLK_SRC_SEL_R(crate::FieldReader<bool, bool>);
impl DIG_CLK_SRC_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIG_CLK_SRC_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIG_CLK_SRC_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dig_clk_src_sel` writer - "]
pub struct DIG_CLK_SRC_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DIG_CLK_SRC_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `dig_512k_comp` reader - "]
pub struct DIG_512K_COMP_R(crate::FieldReader<bool, bool>);
impl DIG_512K_COMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIG_512K_COMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIG_512K_COMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dig_512k_comp` writer - "]
pub struct DIG_512K_COMP_W<'a> {
    w: &'a mut W,
}
impl<'a> DIG_512K_COMP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `dig_512k_en` reader - "]
pub struct DIG_512K_EN_R(crate::FieldReader<bool, bool>);
impl DIG_512K_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIG_512K_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIG_512K_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dig_512k_en` writer - "]
pub struct DIG_512K_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DIG_512K_EN_W<'a> {
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
#[doc = "Field `dig_512k_div` reader - "]
pub struct DIG_512K_DIV_R(crate::FieldReader<u8, u8>);
impl DIG_512K_DIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        DIG_512K_DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIG_512K_DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dig_512k_div` writer - "]
pub struct DIG_512K_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> DIG_512K_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | ((value as u32 & 0x7f) << 16);
        self.w
    }
}
#[doc = "Field `dig_32k_comp` reader - "]
pub struct DIG_32K_COMP_R(crate::FieldReader<bool, bool>);
impl DIG_32K_COMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIG_32K_COMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIG_32K_COMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dig_32k_comp` writer - "]
pub struct DIG_32K_COMP_W<'a> {
    w: &'a mut W,
}
impl<'a> DIG_32K_COMP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `dig_32k_en` reader - "]
pub struct DIG_32K_EN_R(crate::FieldReader<bool, bool>);
impl DIG_32K_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIG_32K_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIG_32K_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dig_32k_en` writer - "]
pub struct DIG_32K_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DIG_32K_EN_W<'a> {
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
#[doc = "Field `dig_32k_div` reader - "]
pub struct DIG_32K_DIV_R(crate::FieldReader<u16, u16>);
impl DIG_32K_DIV_R {
    pub(crate) fn new(bits: u16) -> Self {
        DIG_32K_DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIG_32K_DIV_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dig_32k_div` writer - "]
pub struct DIG_32K_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> DIG_32K_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | (value as u32 & 0x07ff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn reg_en_platform_wakeup(&self) -> REG_EN_PLATFORM_WAKEUP_R {
        REG_EN_PLATFORM_WAKEUP_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn dig_clk_src_sel(&self) -> DIG_CLK_SRC_SEL_R {
        DIG_CLK_SRC_SEL_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn dig_512k_comp(&self) -> DIG_512K_COMP_R {
        DIG_512K_COMP_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn dig_512k_en(&self) -> DIG_512K_EN_R {
        DIG_512K_EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn dig_512k_div(&self) -> DIG_512K_DIV_R {
        DIG_512K_DIV_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn dig_32k_comp(&self) -> DIG_32K_COMP_R {
        DIG_32K_COMP_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dig_32k_en(&self) -> DIG_32K_EN_R {
        DIG_32K_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn dig_32k_div(&self) -> DIG_32K_DIV_R {
        DIG_32K_DIV_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn reg_en_platform_wakeup(&mut self) -> REG_EN_PLATFORM_WAKEUP_W {
        REG_EN_PLATFORM_WAKEUP_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn dig_clk_src_sel(&mut self) -> DIG_CLK_SRC_SEL_W {
        DIG_CLK_SRC_SEL_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn dig_512k_comp(&mut self) -> DIG_512K_COMP_W {
        DIG_512K_COMP_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn dig_512k_en(&mut self) -> DIG_512K_EN_W {
        DIG_512K_EN_W { w: self }
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn dig_512k_div(&mut self) -> DIG_512K_DIV_W {
        DIG_512K_DIV_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn dig_32k_comp(&mut self) -> DIG_32K_COMP_W {
        DIG_32K_COMP_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dig_32k_en(&mut self) -> DIG_32K_EN_W {
        DIG_32K_EN_W { w: self }
    }
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn dig_32k_div(&mut self) -> DIG_32K_DIV_W {
        DIG_32K_DIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DIG32K_WAKEUP_CTRL.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dig32k_wakeup_ctrl](index.html) module"]
pub struct DIG32K_WAKEUP_CTRL_SPEC;
impl crate::RegisterSpec for DIG32K_WAKEUP_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dig32k_wakeup_ctrl::R](R) reader structure"]
impl crate::Readable for DIG32K_WAKEUP_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dig32k_wakeup_ctrl::W](W) writer structure"]
impl crate::Writable for DIG32K_WAKEUP_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DIG32K_WAKEUP_CTRL to value 0x033e_13e8"]
impl crate::Resettable for DIG32K_WAKEUP_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x033e_13e8
    }
}

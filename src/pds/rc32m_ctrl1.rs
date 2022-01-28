#[doc = "Register `rc32m_ctrl1` reader"]
pub struct R(crate::R<RC32M_CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RC32M_CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RC32M_CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RC32M_CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rc32m_ctrl1` writer"]
pub struct W(crate::W<RC32M_CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RC32M_CTRL1_SPEC>;
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
impl From<crate::W<RC32M_CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RC32M_CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rc32m_reserved` reader - "]
pub struct RC32M_RESERVED_R(crate::FieldReader<u8, u8>);
impl RC32M_RESERVED_R {
    pub(crate) fn new(bits: u8) -> Self {
        RC32M_RESERVED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RC32M_RESERVED_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rc32m_reserved` writer - "]
pub struct RC32M_RESERVED_W<'a> {
    w: &'a mut W,
}
impl<'a> RC32M_RESERVED_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
#[doc = "Field `rc32m_clk_force_on` reader - "]
pub struct RC32M_CLK_FORCE_ON_R(crate::FieldReader<bool, bool>);
impl RC32M_CLK_FORCE_ON_R {
    pub(crate) fn new(bits: bool) -> Self {
        RC32M_CLK_FORCE_ON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RC32M_CLK_FORCE_ON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rc32m_clk_force_on` writer - "]
pub struct RC32M_CLK_FORCE_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> RC32M_CLK_FORCE_ON_W<'a> {
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
#[doc = "Field `rc32m_clk_inv` reader - "]
pub struct RC32M_CLK_INV_R(crate::FieldReader<bool, bool>);
impl RC32M_CLK_INV_R {
    pub(crate) fn new(bits: bool) -> Self {
        RC32M_CLK_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RC32M_CLK_INV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rc32m_clk_inv` writer - "]
pub struct RC32M_CLK_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> RC32M_CLK_INV_W<'a> {
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
#[doc = "Field `rc32m_clk_soft_rst` reader - "]
pub struct RC32M_CLK_SOFT_RST_R(crate::FieldReader<bool, bool>);
impl RC32M_CLK_SOFT_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        RC32M_CLK_SOFT_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RC32M_CLK_SOFT_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rc32m_clk_soft_rst` writer - "]
pub struct RC32M_CLK_SOFT_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> RC32M_CLK_SOFT_RST_W<'a> {
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
#[doc = "Field `rc32m_soft_rst` reader - "]
pub struct RC32M_SOFT_RST_R(crate::FieldReader<bool, bool>);
impl RC32M_SOFT_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        RC32M_SOFT_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RC32M_SOFT_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rc32m_soft_rst` writer - "]
pub struct RC32M_SOFT_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> RC32M_SOFT_RST_W<'a> {
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
#[doc = "Field `rc32m_test_en` reader - "]
pub struct RC32M_TEST_EN_R(crate::FieldReader<bool, bool>);
impl RC32M_TEST_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RC32M_TEST_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RC32M_TEST_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rc32m_test_en` writer - "]
pub struct RC32M_TEST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RC32M_TEST_EN_W<'a> {
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
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn rc32m_reserved(&self) -> RC32M_RESERVED_R {
        RC32M_RESERVED_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rc32m_clk_force_on(&self) -> RC32M_CLK_FORCE_ON_R {
        RC32M_CLK_FORCE_ON_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rc32m_clk_inv(&self) -> RC32M_CLK_INV_R {
        RC32M_CLK_INV_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rc32m_clk_soft_rst(&self) -> RC32M_CLK_SOFT_RST_R {
        RC32M_CLK_SOFT_RST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rc32m_soft_rst(&self) -> RC32M_SOFT_RST_R {
        RC32M_SOFT_RST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rc32m_test_en(&self) -> RC32M_TEST_EN_R {
        RC32M_TEST_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn rc32m_reserved(&mut self) -> RC32M_RESERVED_W {
        RC32M_RESERVED_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rc32m_clk_force_on(&mut self) -> RC32M_CLK_FORCE_ON_W {
        RC32M_CLK_FORCE_ON_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rc32m_clk_inv(&mut self) -> RC32M_CLK_INV_W {
        RC32M_CLK_INV_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rc32m_clk_soft_rst(&mut self) -> RC32M_CLK_SOFT_RST_W {
        RC32M_CLK_SOFT_RST_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rc32m_soft_rst(&mut self) -> RC32M_SOFT_RST_W {
        RC32M_SOFT_RST_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rc32m_test_en(&mut self) -> RC32M_TEST_EN_W {
        RC32M_TEST_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rc32m_ctrl1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rc32m_ctrl1](index.html) module"]
pub struct RC32M_CTRL1_SPEC;
impl crate::RegisterSpec for RC32M_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rc32m_ctrl1::R](R) reader structure"]
impl crate::Readable for RC32M_CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rc32m_ctrl1::W](W) writer structure"]
impl crate::Writable for RC32M_CTRL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets rc32m_ctrl1 to value 0"]
impl crate::Resettable for RC32M_CTRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

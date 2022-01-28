#[doc = "Register `i2c_sub_addr` reader"]
pub struct R(crate::R<I2C_SUB_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_SUB_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_SUB_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_SUB_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `i2c_sub_addr` writer"]
pub struct W(crate::W<I2C_SUB_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_SUB_ADDR_SPEC>;
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
impl From<crate::W<I2C_SUB_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C_SUB_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_i2c_sub_addr_b3` reader - "]
pub struct CR_I2C_SUB_ADDR_B3_R(crate::FieldReader<u8, u8>);
impl CR_I2C_SUB_ADDR_B3_R {
    pub(crate) fn new(bits: u8) -> Self {
        CR_I2C_SUB_ADDR_B3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_I2C_SUB_ADDR_B3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_i2c_sub_addr_b3` writer - "]
pub struct CR_I2C_SUB_ADDR_B3_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_I2C_SUB_ADDR_B3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
#[doc = "Field `cr_i2c_sub_addr_b2` reader - "]
pub struct CR_I2C_SUB_ADDR_B2_R(crate::FieldReader<u8, u8>);
impl CR_I2C_SUB_ADDR_B2_R {
    pub(crate) fn new(bits: u8) -> Self {
        CR_I2C_SUB_ADDR_B2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_I2C_SUB_ADDR_B2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_i2c_sub_addr_b2` writer - "]
pub struct CR_I2C_SUB_ADDR_B2_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_I2C_SUB_ADDR_B2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `cr_i2c_sub_addr_b1` reader - "]
pub struct CR_I2C_SUB_ADDR_B1_R(crate::FieldReader<u8, u8>);
impl CR_I2C_SUB_ADDR_B1_R {
    pub(crate) fn new(bits: u8) -> Self {
        CR_I2C_SUB_ADDR_B1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_I2C_SUB_ADDR_B1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_i2c_sub_addr_b1` writer - "]
pub struct CR_I2C_SUB_ADDR_B1_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_I2C_SUB_ADDR_B1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `cr_i2c_sub_addr_b0` reader - "]
pub struct CR_I2C_SUB_ADDR_B0_R(crate::FieldReader<u8, u8>);
impl CR_I2C_SUB_ADDR_B0_R {
    pub(crate) fn new(bits: u8) -> Self {
        CR_I2C_SUB_ADDR_B0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_I2C_SUB_ADDR_B0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_i2c_sub_addr_b0` writer - "]
pub struct CR_I2C_SUB_ADDR_B0_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_I2C_SUB_ADDR_B0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn cr_i2c_sub_addr_b3(&self) -> CR_I2C_SUB_ADDR_B3_R {
        CR_I2C_SUB_ADDR_B3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn cr_i2c_sub_addr_b2(&self) -> CR_I2C_SUB_ADDR_B2_R {
        CR_I2C_SUB_ADDR_B2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn cr_i2c_sub_addr_b1(&self) -> CR_I2C_SUB_ADDR_B1_R {
        CR_I2C_SUB_ADDR_B1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn cr_i2c_sub_addr_b0(&self) -> CR_I2C_SUB_ADDR_B0_R {
        CR_I2C_SUB_ADDR_B0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn cr_i2c_sub_addr_b3(&mut self) -> CR_I2C_SUB_ADDR_B3_W {
        CR_I2C_SUB_ADDR_B3_W { w: self }
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn cr_i2c_sub_addr_b2(&mut self) -> CR_I2C_SUB_ADDR_B2_W {
        CR_I2C_SUB_ADDR_B2_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn cr_i2c_sub_addr_b1(&mut self) -> CR_I2C_SUB_ADDR_B1_W {
        CR_I2C_SUB_ADDR_B1_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn cr_i2c_sub_addr_b0(&mut self) -> CR_I2C_SUB_ADDR_B0_W {
        CR_I2C_SUB_ADDR_B0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "i2c_sub_addr.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_sub_addr](index.html) module"]
pub struct I2C_SUB_ADDR_SPEC;
impl crate::RegisterSpec for I2C_SUB_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_sub_addr::R](R) reader structure"]
impl crate::Readable for I2C_SUB_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_sub_addr::W](W) writer structure"]
impl crate::Writable for I2C_SUB_ADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets i2c_sub_addr to value 0"]
impl crate::Resettable for I2C_SUB_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

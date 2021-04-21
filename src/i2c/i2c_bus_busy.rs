#[doc = "Register `i2c_bus_busy` reader"]
pub struct R(crate::R<I2C_BUS_BUSY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_BUS_BUSY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<I2C_BUS_BUSY_SPEC>> for R {
    fn from(reader: crate::R<I2C_BUS_BUSY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `i2c_bus_busy` writer"]
pub struct W(crate::W<I2C_BUS_BUSY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_BUS_BUSY_SPEC>;
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
impl core::convert::From<crate::W<I2C_BUS_BUSY_SPEC>> for W {
    fn from(writer: crate::W<I2C_BUS_BUSY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_i2c_bus_busy_clr` reader - "]
pub struct CR_I2C_BUS_BUSY_CLR_R(crate::FieldReader<bool, bool>);
impl CR_I2C_BUS_BUSY_CLR_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_I2C_BUS_BUSY_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_I2C_BUS_BUSY_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_i2c_bus_busy_clr` writer - "]
pub struct CR_I2C_BUS_BUSY_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_I2C_BUS_BUSY_CLR_W<'a> {
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
#[doc = "Field `sts_i2c_bus_busy` reader - "]
pub struct STS_I2C_BUS_BUSY_R(crate::FieldReader<bool, bool>);
impl STS_I2C_BUS_BUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        STS_I2C_BUS_BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STS_I2C_BUS_BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_i2c_bus_busy_clr(&self) -> CR_I2C_BUS_BUSY_CLR_R {
        CR_I2C_BUS_BUSY_CLR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sts_i2c_bus_busy(&self) -> STS_I2C_BUS_BUSY_R {
        STS_I2C_BUS_BUSY_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_i2c_bus_busy_clr(&mut self) -> CR_I2C_BUS_BUSY_CLR_W {
        CR_I2C_BUS_BUSY_CLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "i2c_bus_busy.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_bus_busy](index.html) module"]
pub struct I2C_BUS_BUSY_SPEC;
impl crate::RegisterSpec for I2C_BUS_BUSY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_bus_busy::R](R) reader structure"]
impl crate::Readable for I2C_BUS_BUSY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_bus_busy::W](W) writer structure"]
impl crate::Writable for I2C_BUS_BUSY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets i2c_bus_busy to value 0"]
impl crate::Resettable for I2C_BUS_BUSY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

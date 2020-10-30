#[doc = "Register `i2c_fifo_rdata` reader"]
pub struct R(crate::R<I2C_FIFO_RDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_FIFO_RDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<I2C_FIFO_RDATA_SPEC>> for R {
    fn from(reader: crate::R<I2C_FIFO_RDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `i2c_fifo_rdata` writer"]
pub struct W(crate::W<I2C_FIFO_RDATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_FIFO_RDATA_SPEC>;
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
impl core::convert::From<crate::W<I2C_FIFO_RDATA_SPEC>> for W {
    fn from(writer: crate::W<I2C_FIFO_RDATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `i2c_fifo_rdata` reader - "]
pub struct I2C_FIFO_RDATA_R(crate::FieldReader<u32, u32>);
impl I2C_FIFO_RDATA_R {
    pub(crate) fn new(bits: u32) -> Self {
        I2C_FIFO_RDATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_FIFO_RDATA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `i2c_fifo_rdata` writer - "]
pub struct I2C_FIFO_RDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_FIFO_RDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn i2c_fifo_rdata(&self) -> I2C_FIFO_RDATA_R {
        I2C_FIFO_RDATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn i2c_fifo_rdata(&mut self) -> I2C_FIFO_RDATA_W {
        I2C_FIFO_RDATA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "i2c_fifo_rdata.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_fifo_rdata](index.html) module"]
pub struct I2C_FIFO_RDATA_SPEC;
impl crate::RegisterSpec for I2C_FIFO_RDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_fifo_rdata::R](R) reader structure"]
impl crate::Readable for I2C_FIFO_RDATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_fifo_rdata::W](W) writer structure"]
impl crate::Writable for I2C_FIFO_RDATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets i2c_fifo_rdata to value 0"]
impl crate::Resettable for I2C_FIFO_RDATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

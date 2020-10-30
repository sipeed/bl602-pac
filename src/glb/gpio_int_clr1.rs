#[doc = "Register `GPIO_INT_CLR1` reader"]
pub struct R(crate::R<GPIO_INT_CLR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_INT_CLR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<GPIO_INT_CLR1_SPEC>> for R {
    fn from(reader: crate::R<GPIO_INT_CLR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_INT_CLR1` writer"]
pub struct W(crate::W<GPIO_INT_CLR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_INT_CLR1_SPEC>;
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
impl core::convert::From<crate::W<GPIO_INT_CLR1_SPEC>> for W {
    fn from(writer: crate::W<GPIO_INT_CLR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_gpio_int_clr1` reader - "]
pub struct REG_GPIO_INT_CLR1_R(crate::FieldReader<u32, u32>);
impl REG_GPIO_INT_CLR1_R {
    pub(crate) fn new(bits: u32) -> Self {
        REG_GPIO_INT_CLR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_GPIO_INT_CLR1_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_int_clr1` writer - "]
pub struct REG_GPIO_INT_CLR1_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_INT_CLR1_W<'a> {
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
    pub fn reg_gpio_int_clr1(&self) -> REG_GPIO_INT_CLR1_R {
        REG_GPIO_INT_CLR1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_gpio_int_clr1(&mut self) -> REG_GPIO_INT_CLR1_W {
        REG_GPIO_INT_CLR1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO_INT_CLR1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_int_clr1](index.html) module"]
pub struct GPIO_INT_CLR1_SPEC;
impl crate::RegisterSpec for GPIO_INT_CLR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_int_clr1::R](R) reader structure"]
impl crate::Readable for GPIO_INT_CLR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_int_clr1::W](W) writer structure"]
impl crate::Writable for GPIO_INT_CLR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIO_INT_CLR1 to value 0"]
impl crate::Resettable for GPIO_INT_CLR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

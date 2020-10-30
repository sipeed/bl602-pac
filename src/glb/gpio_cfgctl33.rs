#[doc = "Register `GPIO_CFGCTL33` reader"]
pub struct R(crate::R<GPIO_CFGCTL33_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_CFGCTL33_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<GPIO_CFGCTL33_SPEC>> for R {
    fn from(reader: crate::R<GPIO_CFGCTL33_SPEC>) -> Self {
        R(reader)
    }
}
impl R {}
#[doc = "GPIO_CFGCTL33.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_cfgctl33](index.html) module"]
pub struct GPIO_CFGCTL33_SPEC;
impl crate::RegisterSpec for GPIO_CFGCTL33_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_cfgctl33::R](R) reader structure"]
impl crate::Readable for GPIO_CFGCTL33_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GPIO_CFGCTL33 to value 0"]
impl crate::Resettable for GPIO_CFGCTL33_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

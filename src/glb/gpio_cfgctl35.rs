#[doc = "Register `GPIO_CFGCTL35` reader"]
pub struct R(crate::R<GPIO_CFGCTL35_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_CFGCTL35_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<GPIO_CFGCTL35_SPEC>> for R {
    fn from(reader: crate::R<GPIO_CFGCTL35_SPEC>) -> Self {
        R(reader)
    }
}
impl R {}
#[doc = "GPIO_CFGCTL35.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_cfgctl35](index.html) module"]
pub struct GPIO_CFGCTL35_SPEC;
impl crate::RegisterSpec for GPIO_CFGCTL35_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_cfgctl35::R](R) reader structure"]
impl crate::Readable for GPIO_CFGCTL35_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GPIO_CFGCTL35 to value 0"]
impl crate::Resettable for GPIO_CFGCTL35_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

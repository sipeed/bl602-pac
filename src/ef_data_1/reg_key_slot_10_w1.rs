#[doc = "Register `reg_key_slot_10_w1` reader"]
pub struct R(crate::R<REG_KEY_SLOT_10_W1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REG_KEY_SLOT_10_W1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<REG_KEY_SLOT_10_W1_SPEC>> for R {
    fn from(reader: crate::R<REG_KEY_SLOT_10_W1_SPEC>) -> Self {
        R(reader)
    }
}
impl R {}
#[doc = "reg_key_slot_10_w1.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_key_slot_10_w1](index.html) module"]
pub struct REG_KEY_SLOT_10_W1_SPEC;
impl crate::RegisterSpec for REG_KEY_SLOT_10_W1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reg_key_slot_10_w1::R](R) reader structure"]
impl crate::Readable for REG_KEY_SLOT_10_W1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets reg_key_slot_10_w1 to value 0"]
impl crate::Resettable for REG_KEY_SLOT_10_W1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

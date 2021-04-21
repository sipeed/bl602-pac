#[doc = "Register `TMSR3` reader"]
pub struct R(crate::R<TMSR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TMSR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<TMSR3_SPEC>> for R {
    fn from(reader: crate::R<TMSR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `tmsr_2` reader - "]
pub struct TMSR_2_R(crate::FieldReader<bool, bool>);
impl TMSR_2_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMSR_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TMSR_2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tmsr_1` reader - "]
pub struct TMSR_1_R(crate::FieldReader<bool, bool>);
impl TMSR_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMSR_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TMSR_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tmsr_0` reader - "]
pub struct TMSR_0_R(crate::FieldReader<bool, bool>);
impl TMSR_0_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMSR_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TMSR_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tmsr_2(&self) -> TMSR_2_R {
        TMSR_2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tmsr_1(&self) -> TMSR_1_R {
        TMSR_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tmsr_0(&self) -> TMSR_0_R {
        TMSR_0_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "TMSR3.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmsr3](index.html) module"]
pub struct TMSR3_SPEC;
impl crate::RegisterSpec for TMSR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tmsr3::R](R) reader structure"]
impl crate::Readable for TMSR3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TMSR3 to value 0"]
impl crate::Resettable for TMSR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

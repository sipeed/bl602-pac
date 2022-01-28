#[doc = "Register `rosdac_ctrl_hw1` reader"]
pub struct R(crate::R<ROSDAC_CTRL_HW1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ROSDAC_CTRL_HW1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ROSDAC_CTRL_HW1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ROSDAC_CTRL_HW1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rosdac_ctrl_hw1` writer"]
pub struct W(crate::W<ROSDAC_CTRL_HW1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ROSDAC_CTRL_HW1_SPEC>;
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
impl From<crate::W<ROSDAC_CTRL_HW1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ROSDAC_CTRL_HW1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rosdac_q_gc1` reader - "]
pub struct ROSDAC_Q_GC1_R(crate::FieldReader<u8, u8>);
impl ROSDAC_Q_GC1_R {
    pub(crate) fn new(bits: u8) -> Self {
        ROSDAC_Q_GC1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROSDAC_Q_GC1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rosdac_q_gc1` writer - "]
pub struct ROSDAC_Q_GC1_W<'a> {
    w: &'a mut W,
}
impl<'a> ROSDAC_Q_GC1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | ((value as u32 & 0x3f) << 24);
        self.w
    }
}
#[doc = "Field `rosdac_i_gc1` reader - "]
pub struct ROSDAC_I_GC1_R(crate::FieldReader<u8, u8>);
impl ROSDAC_I_GC1_R {
    pub(crate) fn new(bits: u8) -> Self {
        ROSDAC_I_GC1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROSDAC_I_GC1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rosdac_i_gc1` writer - "]
pub struct ROSDAC_I_GC1_W<'a> {
    w: &'a mut W,
}
impl<'a> ROSDAC_I_GC1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | ((value as u32 & 0x3f) << 16);
        self.w
    }
}
#[doc = "Field `rosdac_q_gc0` reader - "]
pub struct ROSDAC_Q_GC0_R(crate::FieldReader<u8, u8>);
impl ROSDAC_Q_GC0_R {
    pub(crate) fn new(bits: u8) -> Self {
        ROSDAC_Q_GC0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROSDAC_Q_GC0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rosdac_q_gc0` writer - "]
pub struct ROSDAC_Q_GC0_W<'a> {
    w: &'a mut W,
}
impl<'a> ROSDAC_Q_GC0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | ((value as u32 & 0x3f) << 8);
        self.w
    }
}
#[doc = "Field `rosdac_i_gc0` reader - "]
pub struct ROSDAC_I_GC0_R(crate::FieldReader<u8, u8>);
impl ROSDAC_I_GC0_R {
    pub(crate) fn new(bits: u8) -> Self {
        ROSDAC_I_GC0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROSDAC_I_GC0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rosdac_i_gc0` writer - "]
pub struct ROSDAC_I_GC0_W<'a> {
    w: &'a mut W,
}
impl<'a> ROSDAC_I_GC0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn rosdac_q_gc1(&self) -> ROSDAC_Q_GC1_R {
        ROSDAC_Q_GC1_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn rosdac_i_gc1(&self) -> ROSDAC_I_GC1_R {
        ROSDAC_I_GC1_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn rosdac_q_gc0(&self) -> ROSDAC_Q_GC0_R {
        ROSDAC_Q_GC0_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn rosdac_i_gc0(&self) -> ROSDAC_I_GC0_R {
        ROSDAC_I_GC0_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn rosdac_q_gc1(&mut self) -> ROSDAC_Q_GC1_W {
        ROSDAC_Q_GC1_W { w: self }
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn rosdac_i_gc1(&mut self) -> ROSDAC_I_GC1_W {
        ROSDAC_I_GC1_W { w: self }
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn rosdac_q_gc0(&mut self) -> ROSDAC_Q_GC0_W {
        ROSDAC_Q_GC0_W { w: self }
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn rosdac_i_gc0(&mut self) -> ROSDAC_I_GC0_W {
        ROSDAC_I_GC0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rosdac_ctrl_hw1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rosdac_ctrl_hw1](index.html) module"]
pub struct ROSDAC_CTRL_HW1_SPEC;
impl crate::RegisterSpec for ROSDAC_CTRL_HW1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rosdac_ctrl_hw1::R](R) reader structure"]
impl crate::Readable for ROSDAC_CTRL_HW1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rosdac_ctrl_hw1::W](W) writer structure"]
impl crate::Writable for ROSDAC_CTRL_HW1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets rosdac_ctrl_hw1 to value 0"]
impl crate::Resettable for ROSDAC_CTRL_HW1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

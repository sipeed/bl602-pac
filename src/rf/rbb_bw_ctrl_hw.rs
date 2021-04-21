#[doc = "Register `rbb_bw_ctrl_hw` reader"]
pub struct R(crate::R<RBB_BW_CTRL_HW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RBB_BW_CTRL_HW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RBB_BW_CTRL_HW_SPEC>> for R {
    fn from(reader: crate::R<RBB_BW_CTRL_HW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rbb_bw_ctrl_hw` writer"]
pub struct W(crate::W<RBB_BW_CTRL_HW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RBB_BW_CTRL_HW_SPEC>;
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
impl core::convert::From<crate::W<RBB_BW_CTRL_HW_SPEC>> for W {
    fn from(writer: crate::W<RBB_BW_CTRL_HW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rbb_bt_mode_ble` reader - "]
pub struct RBB_BT_MODE_BLE_R(crate::FieldReader<bool, bool>);
impl RBB_BT_MODE_BLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RBB_BT_MODE_BLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBB_BT_MODE_BLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rbb_bt_mode_ble` writer - "]
pub struct RBB_BT_MODE_BLE_W<'a> {
    w: &'a mut W,
}
impl<'a> RBB_BT_MODE_BLE_W<'a> {
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
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rbb_bt_mode_ble(&self) -> RBB_BT_MODE_BLE_R {
        RBB_BT_MODE_BLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rbb_bt_mode_ble(&mut self) -> RBB_BT_MODE_BLE_W {
        RBB_BT_MODE_BLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rbb_bw_ctrl_hw.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rbb_bw_ctrl_hw](index.html) module"]
pub struct RBB_BW_CTRL_HW_SPEC;
impl crate::RegisterSpec for RBB_BW_CTRL_HW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rbb_bw_ctrl_hw::R](R) reader structure"]
impl crate::Readable for RBB_BW_CTRL_HW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rbb_bw_ctrl_hw::W](W) writer structure"]
impl crate::Writable for RBB_BW_CTRL_HW_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets rbb_bw_ctrl_hw to value 0"]
impl crate::Resettable for RBB_BW_CTRL_HW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

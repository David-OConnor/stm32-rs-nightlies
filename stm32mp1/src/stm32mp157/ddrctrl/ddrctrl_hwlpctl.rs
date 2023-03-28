#[doc = "Register `DDRCTRL_HWLPCTL` reader"]
pub struct R(crate::R<DDRCTRL_HWLPCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_HWLPCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_HWLPCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_HWLPCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_HWLPCTL` writer"]
pub struct W(crate::W<DDRCTRL_HWLPCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_HWLPCTL_SPEC>;
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
impl From<crate::W<DDRCTRL_HWLPCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_HWLPCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HW_LP_EN` reader - HW_LP_EN"]
pub type HW_LP_EN_R = crate::BitReader<bool>;
#[doc = "Field `HW_LP_EN` writer - HW_LP_EN"]
pub type HW_LP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRCTRL_HWLPCTL_SPEC, bool, O>;
#[doc = "Field `HW_LP_EXIT_IDLE_EN` reader - HW_LP_EXIT_IDLE_EN"]
pub type HW_LP_EXIT_IDLE_EN_R = crate::BitReader<bool>;
#[doc = "Field `HW_LP_EXIT_IDLE_EN` writer - HW_LP_EXIT_IDLE_EN"]
pub type HW_LP_EXIT_IDLE_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DDRCTRL_HWLPCTL_SPEC, bool, O>;
#[doc = "Field `HW_LP_IDLE_X32` reader - HW_LP_IDLE_X32"]
pub type HW_LP_IDLE_X32_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HW_LP_IDLE_X32` writer - HW_LP_IDLE_X32"]
pub type HW_LP_IDLE_X32_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_HWLPCTL_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bit 0 - HW_LP_EN"]
    #[inline(always)]
    pub fn hw_lp_en(&self) -> HW_LP_EN_R {
        HW_LP_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HW_LP_EXIT_IDLE_EN"]
    #[inline(always)]
    pub fn hw_lp_exit_idle_en(&self) -> HW_LP_EXIT_IDLE_EN_R {
        HW_LP_EXIT_IDLE_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 16:27 - HW_LP_IDLE_X32"]
    #[inline(always)]
    pub fn hw_lp_idle_x32(&self) -> HW_LP_IDLE_X32_R {
        HW_LP_IDLE_X32_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - HW_LP_EN"]
    #[inline(always)]
    #[must_use]
    pub fn hw_lp_en(&mut self) -> HW_LP_EN_W<0> {
        HW_LP_EN_W::new(self)
    }
    #[doc = "Bit 1 - HW_LP_EXIT_IDLE_EN"]
    #[inline(always)]
    #[must_use]
    pub fn hw_lp_exit_idle_en(&mut self) -> HW_LP_EXIT_IDLE_EN_W<1> {
        HW_LP_EXIT_IDLE_EN_W::new(self)
    }
    #[doc = "Bits 16:27 - HW_LP_IDLE_X32"]
    #[inline(always)]
    #[must_use]
    pub fn hw_lp_idle_x32(&mut self) -> HW_LP_IDLE_X32_W<16> {
        HW_LP_IDLE_X32_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRCTRL hardware low power control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_hwlpctl](index.html) module"]
pub struct DDRCTRL_HWLPCTL_SPEC;
impl crate::RegisterSpec for DDRCTRL_HWLPCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_hwlpctl::R](R) reader structure"]
impl crate::Readable for DDRCTRL_HWLPCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_hwlpctl::W](W) writer structure"]
impl crate::Writable for DDRCTRL_HWLPCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDRCTRL_HWLPCTL to value 0x03"]
impl crate::Resettable for DDRCTRL_HWLPCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}

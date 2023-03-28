#[doc = "Register `DDRCTRL_SWCTL` reader"]
pub struct R(crate::R<DDRCTRL_SWCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_SWCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_SWCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_SWCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_SWCTL` writer"]
pub struct W(crate::W<DDRCTRL_SWCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_SWCTL_SPEC>;
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
impl From<crate::W<DDRCTRL_SWCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_SWCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SW_DONE` reader - SW_DONE"]
pub type SW_DONE_R = crate::BitReader<bool>;
#[doc = "Field `SW_DONE` writer - SW_DONE"]
pub type SW_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRCTRL_SWCTL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - SW_DONE"]
    #[inline(always)]
    pub fn sw_done(&self) -> SW_DONE_R {
        SW_DONE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SW_DONE"]
    #[inline(always)]
    #[must_use]
    pub fn sw_done(&mut self) -> SW_DONE_W<0> {
        SW_DONE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRCTRL software register programming control enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_swctl](index.html) module"]
pub struct DDRCTRL_SWCTL_SPEC;
impl crate::RegisterSpec for DDRCTRL_SWCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_swctl::R](R) reader structure"]
impl crate::Readable for DDRCTRL_SWCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_swctl::W](W) writer structure"]
impl crate::Writable for DDRCTRL_SWCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDRCTRL_SWCTL to value 0x01"]
impl crate::Resettable for DDRCTRL_SWCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}

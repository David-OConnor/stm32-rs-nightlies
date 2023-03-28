#[doc = "Register `CFR` writer"]
pub struct W(crate::W<CFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFR_SPEC>;
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
impl From<crate::W<CFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSOF0` writer - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register."]
pub type CSOF0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFR_SPEC, bool, O>;
#[doc = "Field `CSOF1` writer - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register."]
pub type CSOF1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFR_SPEC, bool, O>;
#[doc = "Field `CSOF2` writer - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register."]
pub type CSOF2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register."]
    #[inline(always)]
    #[must_use]
    pub fn csof0(&mut self) -> CSOF0_W<0> {
        CSOF0_W::new(self)
    }
    #[doc = "Bit 1 - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register."]
    #[inline(always)]
    #[must_use]
    pub fn csof1(&mut self) -> CSOF1_W<1> {
        CSOF1_W::new(self)
    }
    #[doc = "Bit 2 - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register."]
    #[inline(always)]
    #[must_use]
    pub fn csof2(&mut self) -> CSOF2_W<2> {
        CSOF2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMAMUX request line multiplexer interrupt clear flag register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfr](index.html) module"]
pub struct CFR_SPEC;
impl crate::RegisterSpec for CFR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cfr::W](W) writer structure"]
impl crate::Writable for CFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFR to value 0"]
impl crate::Resettable for CFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

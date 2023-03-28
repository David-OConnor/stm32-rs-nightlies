#[doc = "Register `RGCFR` writer"]
pub struct W(crate::W<RGCFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RGCFR_SPEC>;
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
impl From<crate::W<RGCFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RGCFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COF0` writer - Clear trigger event overrun flag Upon setting, this bit clears the corresponding overrun flag OFx in the DMAMUX_RGCSR register."]
pub type COF0_W<'a, const O: u8> = crate::BitWriter<'a, u32, RGCFR_SPEC, bool, O>;
#[doc = "Field `COF1` writer - Clear trigger event overrun flag Upon setting, this bit clears the corresponding overrun flag OFx in the DMAMUX_RGCSR register."]
pub type COF1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RGCFR_SPEC, bool, O>;
#[doc = "Field `COF2` writer - Clear trigger event overrun flag Upon setting, this bit clears the corresponding overrun flag OFx in the DMAMUX_RGCSR register."]
pub type COF2_W<'a, const O: u8> = crate::BitWriter<'a, u32, RGCFR_SPEC, bool, O>;
#[doc = "Field `COF3` writer - Clear trigger event overrun flag Upon setting, this bit clears the corresponding overrun flag OFx in the DMAMUX_RGCSR register."]
pub type COF3_W<'a, const O: u8> = crate::BitWriter<'a, u32, RGCFR_SPEC, bool, O>;
#[doc = "Field `COF4` writer - Clear trigger event overrun flag Upon setting, this bit clears the corresponding overrun flag OFx in the DMAMUX_RGCSR register."]
pub type COF4_W<'a, const O: u8> = crate::BitWriter<'a, u32, RGCFR_SPEC, bool, O>;
#[doc = "Field `COF5` writer - Clear trigger event overrun flag Upon setting, this bit clears the corresponding overrun flag OFx in the DMAMUX_RGCSR register."]
pub type COF5_W<'a, const O: u8> = crate::BitWriter<'a, u32, RGCFR_SPEC, bool, O>;
#[doc = "Field `COF6` writer - Clear trigger event overrun flag Upon setting, this bit clears the corresponding overrun flag OFx in the DMAMUX_RGCSR register."]
pub type COF6_W<'a, const O: u8> = crate::BitWriter<'a, u32, RGCFR_SPEC, bool, O>;
#[doc = "Field `COF7` writer - Clear trigger event overrun flag Upon setting, this bit clears the corresponding overrun flag OFx in the DMAMUX_RGCSR register."]
pub type COF7_W<'a, const O: u8> = crate::BitWriter<'a, u32, RGCFR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Clear trigger event overrun flag Upon setting, this bit clears the corresponding overrun flag OFx in the DMAMUX_RGCSR register."]
    #[inline(always)]
    #[must_use]
    pub fn cof0(&mut self) -> COF0_W<0> {
        COF0_W::new(self)
    }
    #[doc = "Bit 1 - Clear trigger event overrun flag Upon setting, this bit clears the corresponding overrun flag OFx in the DMAMUX_RGCSR register."]
    #[inline(always)]
    #[must_use]
    pub fn cof1(&mut self) -> COF1_W<1> {
        COF1_W::new(self)
    }
    #[doc = "Bit 2 - Clear trigger event overrun flag Upon setting, this bit clears the corresponding overrun flag OFx in the DMAMUX_RGCSR register."]
    #[inline(always)]
    #[must_use]
    pub fn cof2(&mut self) -> COF2_W<2> {
        COF2_W::new(self)
    }
    #[doc = "Bit 3 - Clear trigger event overrun flag Upon setting, this bit clears the corresponding overrun flag OFx in the DMAMUX_RGCSR register."]
    #[inline(always)]
    #[must_use]
    pub fn cof3(&mut self) -> COF3_W<3> {
        COF3_W::new(self)
    }
    #[doc = "Bit 4 - Clear trigger event overrun flag Upon setting, this bit clears the corresponding overrun flag OFx in the DMAMUX_RGCSR register."]
    #[inline(always)]
    #[must_use]
    pub fn cof4(&mut self) -> COF4_W<4> {
        COF4_W::new(self)
    }
    #[doc = "Bit 5 - Clear trigger event overrun flag Upon setting, this bit clears the corresponding overrun flag OFx in the DMAMUX_RGCSR register."]
    #[inline(always)]
    #[must_use]
    pub fn cof5(&mut self) -> COF5_W<5> {
        COF5_W::new(self)
    }
    #[doc = "Bit 6 - Clear trigger event overrun flag Upon setting, this bit clears the corresponding overrun flag OFx in the DMAMUX_RGCSR register."]
    #[inline(always)]
    #[must_use]
    pub fn cof6(&mut self) -> COF6_W<6> {
        COF6_W::new(self)
    }
    #[doc = "Bit 7 - Clear trigger event overrun flag Upon setting, this bit clears the corresponding overrun flag OFx in the DMAMUX_RGCSR register."]
    #[inline(always)]
    #[must_use]
    pub fn cof7(&mut self) -> COF7_W<7> {
        COF7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMAMux - DMA request generator clear flag register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rgcfr](index.html) module"]
pub struct RGCFR_SPEC;
impl crate::RegisterSpec for RGCFR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [rgcfr::W](W) writer structure"]
impl crate::Writable for RGCFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RGCFR to value 0"]
impl crate::Resettable for RGCFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

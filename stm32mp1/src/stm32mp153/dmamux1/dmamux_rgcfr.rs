#[doc = "Register `DMAMUX_RGCFR` writer"]
pub struct W(crate::W<DMAMUX_RGCFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAMUX_RGCFR_SPEC>;
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
impl From<crate::W<DMAMUX_RGCFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAMUX_RGCFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COF0` writer - COF0"]
pub type COF0_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAMUX_RGCFR_SPEC, bool, O>;
#[doc = "Field `COF1` writer - COF1"]
pub type COF1_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAMUX_RGCFR_SPEC, bool, O>;
#[doc = "Field `COF2` writer - COF2"]
pub type COF2_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAMUX_RGCFR_SPEC, bool, O>;
#[doc = "Field `COF3` writer - COF3"]
pub type COF3_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAMUX_RGCFR_SPEC, bool, O>;
#[doc = "Field `COF4` writer - COF4"]
pub type COF4_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAMUX_RGCFR_SPEC, bool, O>;
#[doc = "Field `COF5` writer - COF5"]
pub type COF5_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAMUX_RGCFR_SPEC, bool, O>;
#[doc = "Field `COF6` writer - COF6"]
pub type COF6_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAMUX_RGCFR_SPEC, bool, O>;
#[doc = "Field `COF7` writer - COF7"]
pub type COF7_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAMUX_RGCFR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - COF0"]
    #[inline(always)]
    #[must_use]
    pub fn cof0(&mut self) -> COF0_W<0> {
        COF0_W::new(self)
    }
    #[doc = "Bit 1 - COF1"]
    #[inline(always)]
    #[must_use]
    pub fn cof1(&mut self) -> COF1_W<1> {
        COF1_W::new(self)
    }
    #[doc = "Bit 2 - COF2"]
    #[inline(always)]
    #[must_use]
    pub fn cof2(&mut self) -> COF2_W<2> {
        COF2_W::new(self)
    }
    #[doc = "Bit 3 - COF3"]
    #[inline(always)]
    #[must_use]
    pub fn cof3(&mut self) -> COF3_W<3> {
        COF3_W::new(self)
    }
    #[doc = "Bit 4 - COF4"]
    #[inline(always)]
    #[must_use]
    pub fn cof4(&mut self) -> COF4_W<4> {
        COF4_W::new(self)
    }
    #[doc = "Bit 5 - COF5"]
    #[inline(always)]
    #[must_use]
    pub fn cof5(&mut self) -> COF5_W<5> {
        COF5_W::new(self)
    }
    #[doc = "Bit 6 - COF6"]
    #[inline(always)]
    #[must_use]
    pub fn cof6(&mut self) -> COF6_W<6> {
        COF6_W::new(self)
    }
    #[doc = "Bit 7 - COF7"]
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
#[doc = "DMAMUX request generator interrupt clear flag register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux_rgcfr](index.html) module"]
pub struct DMAMUX_RGCFR_SPEC;
impl crate::RegisterSpec for DMAMUX_RGCFR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [dmamux_rgcfr::W](W) writer structure"]
impl crate::Writable for DMAMUX_RGCFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMAMUX_RGCFR to value 0"]
impl crate::Resettable for DMAMUX_RGCFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

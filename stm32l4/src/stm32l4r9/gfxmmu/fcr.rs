#[doc = "Register `FCR` writer"]
pub struct W(crate::W<FCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCR_SPEC>;
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
impl From<crate::W<FCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CB0OF` writer - Clear buffer 0 overflow flag"]
pub type CB0OF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR_SPEC, bool, O>;
#[doc = "Field `CB1OF` writer - Clear buffer 1 overflow flag"]
pub type CB1OF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR_SPEC, bool, O>;
#[doc = "Field `CB2OF` writer - Clear buffer 2 overflow flag"]
pub type CB2OF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR_SPEC, bool, O>;
#[doc = "Field `CB3OF` writer - Clear buffer 3 overflow flag"]
pub type CB3OF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR_SPEC, bool, O>;
#[doc = "Field `CAMEF` writer - Clear AHB master error flag"]
pub type CAMEF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Clear buffer 0 overflow flag"]
    #[inline(always)]
    #[must_use]
    pub fn cb0of(&mut self) -> CB0OF_W<0> {
        CB0OF_W::new(self)
    }
    #[doc = "Bit 1 - Clear buffer 1 overflow flag"]
    #[inline(always)]
    #[must_use]
    pub fn cb1of(&mut self) -> CB1OF_W<1> {
        CB1OF_W::new(self)
    }
    #[doc = "Bit 2 - Clear buffer 2 overflow flag"]
    #[inline(always)]
    #[must_use]
    pub fn cb2of(&mut self) -> CB2OF_W<2> {
        CB2OF_W::new(self)
    }
    #[doc = "Bit 3 - Clear buffer 3 overflow flag"]
    #[inline(always)]
    #[must_use]
    pub fn cb3of(&mut self) -> CB3OF_W<3> {
        CB3OF_W::new(self)
    }
    #[doc = "Bit 4 - Clear AHB master error flag"]
    #[inline(always)]
    #[must_use]
    pub fn camef(&mut self) -> CAMEF_W<4> {
        CAMEF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Graphic MMU flag clear register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcr](index.html) module"]
pub struct FCR_SPEC;
impl crate::RegisterSpec for FCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [fcr::W](W) writer structure"]
impl crate::Writable for FCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FCR to value 0"]
impl crate::Resettable for FCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

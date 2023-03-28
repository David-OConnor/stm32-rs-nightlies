#[doc = "Register `BSRR` writer"]
pub struct W(crate::W<BSRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BSRR_SPEC>;
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
impl From<crate::W<BSRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BSRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Port x set bit y (y= 0..15)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BS3W_AW {
    #[doc = "1: Sets the corresponding ODRx bit"]
    Set = 1,
}
impl From<BS3W_AW> for bool {
    #[inline(always)]
    fn from(variant: BS3W_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BS3` writer - Port x set bit y (y= 0..15)"]
pub type BS3_W<'a, const O: u8> = crate::BitWriter<'a, u32, BSRR_SPEC, BS3W_AW, O>;
impl<'a, const O: u8> BS3_W<'a, O> {
    #[doc = "Sets the corresponding ODRx bit"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(BS3W_AW::Set)
    }
}
#[doc = "Port x reset bit y (y = 0..15)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR3W_AW {
    #[doc = "1: Resets the corresponding ODRx bit"]
    Reset = 1,
}
impl From<BR3W_AW> for bool {
    #[inline(always)]
    fn from(variant: BR3W_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BR3` writer - Port x reset bit y (y = 0..15)"]
pub type BR3_W<'a, const O: u8> = crate::BitWriter<'a, u32, BSRR_SPEC, BR3W_AW, O>;
impl<'a, const O: u8> BR3_W<'a, O> {
    #[doc = "Resets the corresponding ODRx bit"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(BR3W_AW::Reset)
    }
}
impl W {
    #[doc = "Bit 3 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn bs3(&mut self) -> BS3_W<3> {
        BS3_W::new(self)
    }
    #[doc = "Bit 19 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn br3(&mut self) -> BR3_W<19> {
        BR3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO port bit set/reset register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsrr](index.html) module"]
pub struct BSRR_SPEC;
impl crate::RegisterSpec for BSRR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [bsrr::W](W) writer structure"]
impl crate::Writable for BSRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BSRR to value 0"]
impl crate::Resettable for BSRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

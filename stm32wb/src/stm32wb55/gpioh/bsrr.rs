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
#[doc = "Field `BS0` writer - Port x set bit y (y= 0..15)"]
pub type BS0_W<'a, const O: u8> = crate::BitWriter<'a, u32, BSRR_SPEC, bool, O>;
#[doc = "Field `BS1` writer - Port x set bit y (y= 0..15)"]
pub type BS1_W<'a, const O: u8> = crate::BitWriter<'a, u32, BSRR_SPEC, bool, O>;
#[doc = "Field `BS3` writer - Port x set bit y (y= 0..15)"]
pub type BS3_W<'a, const O: u8> = crate::BitWriter<'a, u32, BSRR_SPEC, bool, O>;
#[doc = "Field `BR0` writer - Port x set bit y (y= 0..15)"]
pub type BR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, BSRR_SPEC, bool, O>;
#[doc = "Field `BR1` writer - Port x reset bit y (y = 0..15)"]
pub type BR1_W<'a, const O: u8> = crate::BitWriter<'a, u32, BSRR_SPEC, bool, O>;
#[doc = "Field `BR3` writer - Port x reset bit y (y = 0..15)"]
pub type BR3_W<'a, const O: u8> = crate::BitWriter<'a, u32, BSRR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn bs0(&mut self) -> BS0_W<0> {
        BS0_W::new(self)
    }
    #[doc = "Bit 1 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn bs1(&mut self) -> BS1_W<1> {
        BS1_W::new(self)
    }
    #[doc = "Bit 3 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn bs3(&mut self) -> BS3_W<3> {
        BS3_W::new(self)
    }
    #[doc = "Bit 16 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn br0(&mut self) -> BR0_W<16> {
        BR0_W::new(self)
    }
    #[doc = "Bit 17 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn br1(&mut self) -> BR1_W<17> {
        BR1_W::new(self)
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
